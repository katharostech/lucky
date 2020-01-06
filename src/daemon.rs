//! Contains the Lucky Daemon and RPC implementaiton used for client->daemon communication.
use anyhow::Context;
use serde::{Deserialize, Serialize};
use shiplift::Docker;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, RwLock,
};

use crate::docker::ContainerInfo;
use crate::rpc;
use crate::types::{HookScript, LuckyMetadata, ScriptStatus};

/// Daemon tools
mod tools;
// Built-in daemon hook handlers
mod hook_handlers;
// Daemon helper types
mod types;
use types::*;

#[derive(Debug, Default, Serialize, Deserialize)]
/// Contains the daemon state, which can be serialize and deserialized for persistance across
/// daemon crashes, upgrades, etc.
struct DaemonState {
    #[serde(rename = "script-statuses")]
    /// The statuses of all of the scripts
    script_statuses: HashMap<String, ScriptStatus>,
    /// The unit-local key-value store
    kv: HashMap<String, Cd<String>>,
    default_container: Option<Cd<ContainerInfo>>,
    /// Other containers that the daemon is supervising
    named_containers: HashMap<String, Cd<ContainerInfo>>,
}

/// The Lucky Daemon RPC service
struct LuckyDaemon {
    /// The charm directory
    charm_dir: PathBuf,
    /// The directory in which to store the daemon state
    state_dir: PathBuf,
    /// The contents of the charm's lucky.yaml config
    lucky_metadata: LuckyMetadata,
    /// Used to indicate that the server should stop listening.
    /// This will be set to true to indicate that the server should stop.
    stop_listening: Arc<AtomicBool>,
    /// The daemon state. This will be serialized and written to disc for persistance when the
    /// daemon crashes or is shutdown.  
    state: Arc<RwLock<DaemonState>>,
    /// The docker daemon connection if it has been loaded
    docker_conn: Arc<Mutex<Option<Arc<Mutex<Docker>>>>>,
}

impl LuckyDaemon {
    /// Create a new daemon instance
    ///
    /// `stop_listening` will be set to `true` by the daemon if it recieves a `StopDaemon` RPC. The
    /// actual stopping of the server itself is not handled by the daemon.
    fn new(
        lucky_metadata: LuckyMetadata,
        charm_dir: PathBuf,
        state_dir: PathBuf,
        stop_listening: Arc<AtomicBool>,
    ) -> Self {
        let daemon = LuckyDaemon {
            lucky_metadata,
            charm_dir,
            state_dir,
            stop_listening,
            state: Default::default(),
            docker_conn: Arc::new(Mutex::new(None)),
        };

        // Load daemon state
        tools::load_state(&daemon)
            .context("Could not load daemon state from filesystem")
            .unwrap_or_else(|e| log::error!("{:?}", e));

        // Update the Juju status
        crate::juju::set_status(tools::get_juju_status(&daemon)).unwrap_or_else(|e| {
            log::warn!("{:?}", e.context("Could not set juju status"));
        });

        log::trace!("Loaded daemon state: {:#?}", daemon.state.read().unwrap());

        daemon
    }

    pub(crate) fn set_script_status(
        &self,
        script_id: &str,
        status: ScriptStatus,
    ) -> anyhow::Result<()> {
        log::info!(r#"Set status[{}]: {}"#, script_id, status);
        self.state
            .write()
            .unwrap()
            .script_statuses
            .insert(script_id.into(), status);

        // Set the Juju status to the consolidated script statuses
        crate::juju::set_status(tools::get_juju_status(&self))?;

        Ok(())
    }

    /// Gets a handle to the daemon's Docker connection, creating a new one if one doesn't already
    /// exist.
    fn get_docker_conn(&self) -> anyhow::Result<Arc<Mutex<Docker>>> {
        let mut docker_conn = self.docker_conn.lock().unwrap();

        // If we have a connection already, return it
        if let Some(docker_conn) = &*docker_conn {
            Ok(docker_conn.clone())
        // If there is no connection
        } else {
            // Connect to docker
            log::debug!("Connecting to Docker");
            let conn = Docker::new();

            // Test getting Docker info
            log::trace!("Docker info: {:?}", crate::rt::block_on(conn.info())?);

            // Return connection
            let conn = Arc::new(Mutex::new(conn));
            *docker_conn = Some(conn.clone());
            Ok(conn)
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn _trigger_hook(
        &self,
        call: &mut dyn rpc::Call_TriggerHook,
        hook_name: &str,
        environment: &HashMap<String, String>,
    ) -> anyhow::Result<()> {
        // Run any built-in hook handler
        hook_handlers::handle_hook(&self, &hook_name).context(format!(
            r#"Error running internal hook handler for hook "{}""#,
            hook_name
        ))?;

        // Run hook scripts
        if let Some(hook_scripts) = self.lucky_metadata.hooks.get(hook_name) {
            // Execute all scripts registered for this hook
            for hook_script in hook_scripts {
                match hook_script {
                    HookScript::HostScript(script_name) => {
                        tools::run_host_script(self, call, script_name, &environment)?;
                    }
                    HookScript::ContainerScript(_script_name) => {
                        log::warn!("Container scripts not yet implemented");
                    }
                }

                // TODO: Don't apply container config when `docker: false` in lucky.yaml
                // Apply any container configuration changed by the script
                tools::apply_container_updates(self)?;
            }

            // Update the Juju status as Juju will clear it if we don't re-set it after hook
            // execution
            crate::juju::set_status(tools::get_juju_status(&self))?;

            // Finish reply
            call.set_continues(false);
            call.reply(None)?;

        // If the hook is not handled by the charm
        } else {
            // Update the Juju status
            crate::juju::set_status(tools::get_juju_status(&self))
                .or_else(|e| call.reply_error(e.to_string()))?;

            // Just reply without doing anything ( setting exit code to 0 )
            call.reply(None)?;
        }

        Ok(())
    }
}

impl rpc::VarlinkInterface for LuckyDaemon {
    /// Stop the Lucky daemon
    fn stop_daemon(&self, call: &mut dyn rpc::Call_StopDaemon) -> varlink::Result<()> {
        log::info!("Shutting down server");
        // Set the stop_listening=true.
        self.stop_listening.store(true, Ordering::SeqCst);

        // Reply and exit
        call.reply()?;
        Ok(())
    }

    /// Trigger a Juju hook
    fn trigger_hook(
        &self,
        call: &mut dyn rpc::Call_TriggerHook,
        hook_name: String,
        environment: HashMap<String, String>,
    ) -> varlink::Result<()> {
        // Set the hook environment variables
        for (var, value) in &environment {
            std::env::set_var(var, value);
        }

        log::info!("Triggering hook: {}", hook_name);

        // Trigger hook and handle error
        self._trigger_hook(call, &hook_name, &environment)
            .or_else(|e| {
                let e = format!("{:?}", e);
                log::error!("{}", e);
                call.reply_error(e)
            })?;

        // Unset the hook environment variables as they will be invalid when the hook exits
        for var in environment.keys() {
            std::env::remove_var(var);
        }

        log::info!("Done triggering hook: {}", hook_name);

        Ok(())
    }

    /// Set a script's status
    fn set_status(
        &self,
        call: &mut dyn rpc::Call_SetStatus,
        script_id: String,
        status: rpc::ScriptStatus,
    ) -> varlink::Result<()> {
        // Add status to script statuses
        let status: ScriptStatus = status.into();

        self.set_script_status(&script_id, status).or_else(|e| {
            let e = format!("{:?}", e);
            log::error!("{}", e);
            call.reply_error(e)
        })?;

        // Reply
        call.reply()?;
        Ok(())
    }

    /// Get a value in the unit local key-value store
    fn unit_kv_get(&self, call: &mut dyn rpc::Call_UnitKvGet, key: String) -> varlink::Result<()> {
        // Get with key
        let state = self.state.read().unwrap();
        let value = state.kv.get(&key);

        // Reply with value
        call.reply(value.map(|x| x.clone().into_inner()))?;

        Ok(())
    }

    fn unit_kv_get_all(&self, call: &mut dyn rpc::Call_UnitKvGetAll) -> varlink::Result<()> {
        // This call must be called with more
        if !call.wants_more() {
            call.reply_requires_more()?;
            return Ok(());
        }

        // Loop through key-value pairs and return result to client
        let state = self.state.read().unwrap();
        let pairs: Vec<(&String, &Cd<String>)> = state.kv.iter().collect();
        call.set_continues(true);
        let mut i = 0;
        let len = pairs.len();
        while i < len {
            // If this is the last pair
            if i == len - 1 {
                // Tell client not to expect more after this one
                call.set_continues(false);
            }
            // Reply with the pair
            call.reply(pairs[i].0.clone(), pairs[i].1.clone().into_inner())?;
            i += 1;
        }

        Ok(())
    }

    /// Set a value in the unit local key-value store
    fn unit_kv_set(
        &self,
        call: &mut dyn rpc::Call_UnitKvSet,
        key: String,
        value: Option<String>,
    ) -> varlink::Result<()> {
        let mut state = self.state.write().unwrap();

        // If a value has been provided
        if let Some(value) = value {
            log::debug!("Key-Value set: {} = {}", key, value);
            // Set key to value
            state.kv.insert(key, value.into());
        } else {
            log::debug!("Key-Value delete: {}", key);
            // Erase key
            state.kv.remove(&key);
        }

        // Reply empty
        call.reply()?;

        Ok(())
    }

    // The uncollapsed if is easier to understand in this case
    #[allow(clippy::collapsible_if)]
    fn container_image_set(
        &self,
        call: &mut dyn rpc::Call_ContainerImageSet,
        image: String,
        container_name: Option<String>,
    ) -> varlink::Result<()> {
        let mut state = self.state.write().unwrap();

        // If this is for a named container
        if let Some(name) = container_name {
            if let Some(container) = state.named_containers.get_mut(&name) {
                log::debug!("Set Docker image [{}]: {}", name, image);
                // Set the image on existing container
                container.config.image = image;
            } else {
                log::debug!("Adding new docker container: {}", name);
                log::debug!("Set Docker image [{}]: {}", name, image);
                // Create a new container with the given image
                let new_container = ContainerInfo::new(&image);
                state.named_containers.insert(name, new_container.into());
            }
        // If this is for the default container
        } else {
            if let Some(container) = &mut state.default_container {
                log::debug!("Set container image: {}", image);
                // Set the image on existing container
                container.config.image = image;
            } else {
                log::debug!("Adding container");
                log::debug!("Set container image: {}", image);
                // Create a new container with the given image
                let new_container = ContainerInfo::new(&image);
                state.default_container = Some(new_container.into());
            }
        }

        call.reply()?;
        Ok(())
    }

    // The uncollapsed if is easier to understand in this case
    #[allow(clippy::collapsible_if)]
    fn container_image_get(
        &self,
        call: &mut dyn rpc::Call_ContainerImageGet,
        container_name: Option<String>,
    ) -> varlink::Result<()> {
        let state = self.state.read().unwrap();

        // If this is for a named container
        if let Some(name) = container_name {
            if let Some(container) = state.named_containers.get(&name) {
                call.reply(Some(container.config.image.clone()))?;
            } else {
                call.reply(None)?;
            }
        // If this is for the default container
        } else {
            if let Some(container) = &state.default_container {
                call.reply(Some(container.config.image.clone()))?;
            } else {
                call.reply(None)?;
            }
        }

        Ok(())
    }

    fn container_apply(&self, call: &mut dyn rpc::Call_ContainerApply) -> varlink::Result<()> {
        // TODO: Don't apply container config when `docker: false` in lucky.yaml
        tools::apply_container_updates(self).or_else(|e| {
            let e = format!("{:?}", e);
            log::error!("{}", e);
            call.reply_error(e)
        })?;

        Ok(())
    }
}

impl Drop for LuckyDaemon {
    /// Persist the daeomon state before it is dropped
    fn drop(&mut self) {
        tools::flush_state(&self).unwrap_or_else(|e| log::error!("{:?}", e));
    }
}

//
// Client Helpers
//

/// Get the server service
pub(crate) fn get_service(
    lucky_metadata: LuckyMetadata,
    charm_dir: PathBuf,
    state_dir: PathBuf,
    stop_listening: Arc<AtomicBool>,
) -> varlink::VarlinkService {
    // Create a new daemon instance
    let daemon_instance = LuckyDaemon::new(lucky_metadata, charm_dir, state_dir, stop_listening);

    // Return the varlink service
    varlink::VarlinkService::new(
        "lucky.rpc",
        "lucky daemon",
        clap::crate_version!(),
        "https://github.com/katharostech/lucky",
        vec![Box::new(rpc::new(Box::new(daemon_instance)))],
    )
}

/// Get the client
pub(crate) fn get_client(connection: Arc<RwLock<varlink::Connection>>) -> rpc::VarlinkClient {
    // Return the varlink client
    rpc::VarlinkClient::new(connection)
}

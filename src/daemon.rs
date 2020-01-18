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
use crate::juju;
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
    /// The path to the socket that the daemon is listening on
    socket_path: PathBuf,
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

pub(crate) struct LuckyDaemonOptions {
    pub lucky_metadata: LuckyMetadata,
    pub charm_dir: PathBuf,
    pub state_dir: PathBuf,
    pub socket_path: PathBuf,
    pub stop_listening: Arc<AtomicBool>,
}

// Utility macro for handling anyhow results with `handle_err!(function_that_returns_anyhow_err())`
macro_rules! handle_err {
    ($expr:expr, $call:ident) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let e = format!("{:?}", e);
                log::error!("{}", e);
                return $call.reply_error(e);
            }
        }
    };
}

impl LuckyDaemon {
    /// Create a new daemon instance
    ///
    /// `stop_listening` will be set to `true` by the daemon if it recieves a `StopDaemon` RPC. The
    /// actual stopping of the server itself is not handled by the daemon.
    fn new(options: LuckyDaemonOptions) -> Self {
        let daemon = LuckyDaemon {
            lucky_metadata: options.lucky_metadata,
            charm_dir: options.charm_dir,
            state_dir: options.state_dir,
            socket_path: options.socket_path,
            stop_listening: options.stop_listening,
            state: Default::default(),
            docker_conn: Arc::new(Mutex::new(None)),
        };

        // Load daemon state
        tools::load_state(&daemon)
            .context("Could not load daemon state from filesystem")
            .unwrap_or_else(|e| log::error!("{:?}", e));

        // Update the Juju status
        crate::juju::set_status(tools::get_juju_status(&daemon.state.read().unwrap()))
            .unwrap_or_else(|e| {
                log::warn!("{:?}", e.context("Could not set juju status"));
            });

        log::trace!("Loaded daemon state: {:#?}", daemon.state.read().unwrap());

        daemon
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

                // If docker is enabled, update container configuration
                if self.lucky_metadata.use_docker {
                    tools::apply_container_updates(self)?;
                }
            }

            // Finish reply
            call.set_continues(false);
            call.reply(None)?;

        // If the hook is not handled by the charm
        } else {
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

        // Trigger hook
        handle_err!(self._trigger_hook(call, &hook_name, &environment), call);

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

        handle_err!(
            tools::set_script_status(&mut self.state.write().unwrap(), &script_id, status),
            call
        );

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
        let mut i = 0;
        let len = pairs.len();
        if len > 0 {
            call.set_continues(true);
            while i < len {
                // If this is the last pair
                if i == len - 1 {
                    // Tell client not to expect more after this one
                    call.set_continues(false);
                }
                // Reply with the pair
                call.reply(Some(rpc::UnitKvGetAll_Reply_pair {
                    key: pairs[i].0.clone(),
                    value: pairs[i].1.clone().into_inner(),
                }))?;
                i += 1;
            }
        } else {
            call.set_continues(false);
            call.reply(None)?;
        }

        Ok(())
    }

    fn get_private_address(
        &self,
        call: &mut dyn rpc::Call_GetPrivateAddress,
    ) -> varlink::Result<()> {
        call.reply(handle_err!(juju::unit_get_private_address(), call))?;

        Ok(())
    }

    fn get_public_address(&self, call: &mut dyn rpc::Call_GetPublicAddress) -> varlink::Result<()> {
        call.reply(handle_err!(juju::unit_get_public_address(), call))?;

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

    fn container_apply(&self, call: &mut dyn rpc::Call_ContainerApply) -> varlink::Result<()> {
        // TODO: Don't apply container config when `docker: false` in lucky.yaml
        handle_err!(tools::apply_container_updates(self), call);

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

    fn container_env_get(
        &self,
        call: &mut dyn rpc::Call_ContainerEnvGet,
        key: String,
        container_name: Option<String>,
    ) -> varlink::Result<()> {
        let state = self.state.read().unwrap();

        // Get the config for the requested container
        let container = match &container_name {
            Some(container_name) => state.named_containers.get(container_name),
            None => state.default_container.as_ref(),
        };

        // If the specified container exists
        if let Some(container) = container {
            // Reply with the environment variable's value
            call.reply(container.config.env_vars.get(&key).map(ToOwned::to_owned))?;
        } else {
            // Reply with None
            call.reply(None)?;
        }

        Ok(())
    }

    fn container_env_get_all(
        &self,
        call: &mut dyn rpc::Call_ContainerEnvGetAll,
        container_name: Option<String>,
    ) -> varlink::Result<()> {
        let state = self.state.read().unwrap();

        // This call must be called with more
        if !call.wants_more() {
            call.reply_requires_more()?;
            return Ok(());
        }

        // Get the config for the requested container
        let container = match &container_name {
            Some(container_name) => state.named_containers.get(container_name),
            None => state.default_container.as_ref(),
        };

        // If the container exists
        if let Some(container) = container {
            // Loop through key-value pairs and return result to client
            let pairs: Vec<(&String, &String)> = container.config.env_vars.iter().collect();
            let mut i = 0;
            let len = pairs.len();
            if len > 0 {
                call.set_continues(true);
                while i < len {
                    // If this is the last pair
                    if i == len - 1 {
                        // Tell client not to expect more after this one
                        call.set_continues(false);
                    }
                    // Reply with the pair
                    call.reply(Some(rpc::ContainerEnvGetAll_Reply_pair {
                        key: pairs[i].0.clone(),
                        value: pairs[i].1.clone(),
                    }))?;
                    i += 1;
                }
            // If there are no environment variables
            } else {
                // Return None
                call.set_continues(false);
                call.reply(None)?;
            }

        // If the container doesn't exist
        } else {
            // Reply None
            call.reply(None)?;
        }

        Ok(())
    }

    /// Set a value in the unit local key-value store
    fn container_env_set(
        &self,
        call: &mut dyn rpc::Call_ContainerEnvSet,
        key: String,
        value: Option<String>,
        container_name: Option<String>,
    ) -> varlink::Result<()> {
        let mut state = self.state.write().unwrap();

        // Get the config for the requested container
        let mut container = match &container_name {
            Some(container_name) => state.named_containers.get_mut(container_name),
            None => state.default_container.as_mut(),
        };

        if let Some(container) = &mut container {
            // If a value has been provided
            if let Some(value) = value {
                log::debug!("Container env set: {} = {}", key, value);
                // Set key to value
                container.config.env_vars.insert(key, value);
            } else {
                log::debug!("Container env deleted: {}", key);
                // Erase key
                container.config.env_vars.remove(&key);
            }

            // Reply empty
            call.reply()?;
        }

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
pub(crate) fn get_service(options: LuckyDaemonOptions) -> varlink::VarlinkService {
    // Create a new daemon instance
    let daemon_instance = LuckyDaemon::new(options);

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

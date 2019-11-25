//! Contains the Lucky RPC implementaiton used for client->daemon communication.
use anyhow::Context;
use serde::{Deserialize, Serialize};
use subprocess::{Exec, ExitStatus, Redirection};

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

use crate::types::{HookScript, LuckyMetadata, ScriptState, ScriptStatus};

#[allow(clippy::all)]
#[allow(bare_trait_objects)]
/// The varlink RPC code ( generated by build.rs from `rpc/lucky.rpc.varlink` )
pub(crate) mod lucky_rpc;
pub(crate) use lucky_rpc as rpc;

#[derive(Debug, Default, Serialize, Deserialize)]
/// Contains the daemon state, which can be serialize and deserialized for persistance across
/// daemon crashes, upgrades, etc.
struct DaemonState {
    #[serde(rename = "script-statuses")]
    /// The statuses of all of the scripts
    script_statuses: HashMap<String, ScriptStatus>,
    /// The unit-local key-value store
    kv: HashMap<String, String>,
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
}

impl LuckyDaemon {
    /// Create a new daemon instance
    ///
    /// stop_listening will be set to `true` by the daemon if it recieves a StopDaemon RPC. The
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
        };

        // Load daemon state
        daemon.load_state().unwrap_or_else(|e| {
            log::error!(
                "{:?}",
                e.context("Could not load daemon state from filesystem")
            );
        });

        // Update the Juju status
        crate::juju::set_status(daemon.get_juju_status()).unwrap_or_else(|e| {
            log::warn!(
                "{:?}",
                e.context("Could not set juju status")
            );
        });

        log::trace!("Loaded daemon state: {:#?}", daemon.state.read().unwrap());

        daemon
    }

    /// Load the daemon state from the filesystem
    fn load_state(&self) -> anyhow::Result<()> {
        let state_file_path = self.state_dir.join("state.yaml");
        if !state_file_path.exists() {
            return Ok(());
        }

        let state_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&state_file_path)
            .context(format!("Could not open state file: {:?}", state_file_path))?;

        *self.state.write().unwrap() = serde_yaml::from_reader(state_file).context(format!(
            "Could not parse state file as yaml: {:?}",
            state_file_path
        ))?;

        Ok(())
    }

    /// Write out the daemon state to fileystem
    fn flush_state(&self) -> anyhow::Result<()> {
        let state_file_path = self.state_dir.join("state.yaml");
        let mut state_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&state_file_path)?;

        // Write out comment to file
        state_file
            .write_all(
                b"# The daemon state will be written to this file when the daemon is shutdown\n",
            )
            .context(format!(
                "Failed writing to state file: {:?}",
                state_file_path
            ))?;

        // Serialize state to file
        serde_yaml::to_writer(state_file, &*self.state.read().unwrap()).context(format!(
            "Failed to serialize daemon state to file: {:?}",
            state_file_path
        ))?;

        Ok(())
    }

    /// Consolidate script statuses into one status that can be used as the global Juju Status
    fn get_juju_status(&self) -> ScriptStatus {
        // The resulting Juju state
        let mut juju_state = ScriptState::default();
        // The resulting Juju status message
        let mut juju_message = None;

        for status in self.state.read().unwrap().script_statuses.values() {
            // If this script state has a higher precedence
            if status.state > juju_state {
                // Set the Juju state to the more precedent state
                juju_state = status.state;
            }

            // If there is a message with the status
            if let Some(message) = &status.message {
                if let Some(current) = juju_message {
                    // Add this message to Juju message
                    juju_message = Some([current, message.clone()].join(", "));
                } else {
                    // Set Juju message to this message
                    juju_message = Some(message.clone());
                }
            }
        }

        // Return Juju status
        ScriptStatus {
            state: juju_state,
            message: juju_message,
        }
    }

    // Run one of the charm's host scripts
    fn run_host_script(
        &self,
        call: &mut dyn rpc::Call_TriggerHook,
        script_name: &str,
        environment: &HashMap<String, String>,
    ) -> varlink::Result<u32> {
        // Add bin dir to the PATH
        let path_env = std::env::var_os("PATH")
            .map(|mut p| {
                p.push(":");
                p.push(self.charm_dir.join("bin").as_os_str());
                p
            })
            .unwrap_or(self.charm_dir.join("bin").as_os_str().to_owned());
        // Build command
        let command_path = self.charm_dir.join("host_scripts").join(script_name);
        let mut command = Exec::cmd(&command_path)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .env("PATH", path_env)
            .env("LUCKY_CONTEXT", "client")
            .env("LUCKY_SCRIPT_ID", script_name);
        
        // Set environment for hook exececution
        for (k, v) in environment.iter() {
            command = command.env(k, v);
        }

        let mut process = match command.popen() {
            Ok(stream) => stream,
            Err(e) => {
                let e = anyhow::Error::from(e)
                    .context(format!("Error executing script: {:?}", command_path));
                call.reply_error(format!("{:?}", e))?;
                log_error(e);
                return Ok(1)
            }
        };

        let output_buffer = BufReader::new(process.stdout.as_ref().expect("Stdout not opened"));

        if call.wants_more() {
            call.set_continues(true);
        }

        for line in output_buffer.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    call.reply_error(format!("{:?}", e))?;
                    log_error(e.into());
                    return Ok(1)
                }
            };
            log::info!("output: {}", line);
            
            if call.wants_more() {
                call.reply(None, Some(line))?;
            }
        }

        // Wait for script to exit
        let exit_status = match process.wait() {
            Ok(status) => status,
            Err(e) => {
                call.reply_error(format!("{:?}", e))?;
                log_error(e.into());
                return Ok(1)
            }
        };

        match exit_status {
            ExitStatus::Exited(n) => Ok(n),
            _ => Ok(1),
        }
    }
}

impl Drop for LuckyDaemon {
    /// Persist the daeomon state before it is dropped
    fn drop(&mut self) {
        self.flush_state().unwrap_or_else(log_error);
    }
}

impl rpc::VarlinkInterface for LuckyDaemon {
    /// Trigger a Juju hook
    fn trigger_hook(
        &self,
        call: &mut dyn rpc::Call_TriggerHook,
        hook_name: String,
        environment: HashMap<String, String>
    ) -> varlink::Result<()> {
        log::info!("Triggering hook: {}", hook_name);

        let mut exit_code = 0;
        if let Some(hook_scripts) = self.lucky_metadata.hooks.get(&hook_name) {
            for hook_script in hook_scripts {
                match hook_script {
                    HookScript::HostScript(script_name) => {
                        let code = self.run_host_script(call, script_name, &environment)?;
                        if code != 0 {
                            exit_code = 1;
                        }
                    }
                    HookScript::ContainerScript(_script_name) => {
                        log::warn!("Container scripts not yet implemented");
                    }
                }
            }

            call.set_continues(false);
            call.reply(Some(exit_code), None)?;

        // If the hook is not handled by the charm
        } else {
            // Just reply without doing anything
            call.reply(None, None)?;
        }

        Ok(())
    }

    /// Stop the Lucky daemon
    fn stop_daemon(&self, call: &mut dyn rpc::Call_StopDaemon) -> varlink::Result<()> {
        log::info!("Shutting down server");
        // Set the stop_listening=true.
        self.stop_listening.store(true, Ordering::SeqCst);

        // Reply and exit
        call.reply()?;
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
        log::info!(r#"Setting status for script "{}": {}"#, script_id, status);
        self.state
            .write()
            .unwrap()
            .script_statuses
            .insert(script_id, status);

        // Set the Juju status to the consolidated script statuses
        crate::juju::set_status(self.get_juju_status())
            .or_else(|e| call.reply_error(e.to_string()))?;

        // Reply
        call.reply()?;
        Ok(())
    }
}

//
// Helpers
//

/// Convenience for handling errors in Results
fn log_error(e: anyhow::Error) {
    log::error!("{:?}", e);
}

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
        vec![Box::new(lucky_rpc::new(Box::new(daemon_instance)))],
    )
}

/// Get the client
pub(crate) fn get_client(connection: Arc<RwLock<varlink::Connection>>) -> rpc::VarlinkClient {
    // Return the varlink client
    rpc::VarlinkClient::new(connection)
}

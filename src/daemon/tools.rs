use anyhow::format_err;

use subprocess::{Exec, ExitStatus, Redirection};

use std::env;
use std::io::{BufRead, BufReader};

use super::*;

/// Load the daemon state from the filesystem
pub(super) fn load_state(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    let state_file_path = daemon.state_dir.join("state.yaml");
    if !state_file_path.exists() {
        return Ok(());
    }

    let state_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&state_file_path)
        .context(format!("Could not open state file: {:?}", state_file_path))?;

    *daemon.state.write().unwrap() = serde_yaml::from_reader(state_file).context(format!(
        "Could not parse state file as yaml: {:?}",
        state_file_path
    ))?;

    Ok(())
}

/// Write out the daemon state to fileystem
pub(super) fn flush_state(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    log::debug!("Flushing daemon state to disk");
    let state_file_path = daemon.state_dir.join("state.yaml");
    let mut state_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&state_file_path)?;

    // Write out comment to file
    state_file
        .write_all(b"# The daemon state will be written to this file when the daemon is shutdown\n")
        .context(format!(
            "Failed writing to state file: {:?}",
            state_file_path
        ))?;

    // Serialize state to file
    let state = &*daemon.state.read().unwrap();
    log::trace!("{:#?}", state);
    serde_yaml::to_writer(state_file, state).context(format!(
        "Failed to serialize daemon state to file: {:?}",
        state_file_path
    ))?;

    Ok(())
}

/// Consolidate script statuses into one status that can be used as the global Juju Status
pub(super) fn get_juju_status(daemon: &LuckyDaemon) -> ScriptStatus {
    // The resulting Juju state
    let mut juju_state = ScriptState::default();
    // The resulting Juju status message
    let mut juju_message = None;

    for status in daemon.state.read().unwrap().script_statuses.values() {
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
pub(super) fn run_host_script(
    daemon: &LuckyDaemon,
    call: &mut dyn rpc::Call_TriggerHook,
    script_name: &str,
    environment: &HashMap<String, String>,
) -> anyhow::Result<()> {
    // Add bin dir to the PATH
    let path_env = if let Some(path) = std::env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(daemon.charm_dir.join("bin"));
        env::join_paths(paths).context("Path contains invalid character")?
    } else {
        daemon.charm_dir.join("bin").as_os_str().to_owned()
    };

    // Build command
    let command_path = daemon.charm_dir.join("host_scripts").join(script_name);
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

    // Run script process
    let mut process = command
        .popen()
        .context(format!("Error executing script: {:?}", command_path))?;

    // Get script output buffer
    let output_buffer = BufReader::new(process.stdout.as_ref().expect("Stdout not opened"));

    // If the caller wants to get the streamed output
    if call.wants_more() {
        // Set the continues flag on the call to true
        call.set_continues(true);
    }

    // Loop through lines of output
    for line in output_buffer.lines() {
        let line = line?;
        log::info!("output: {}", line);

        // Send caller output if they asked for it
        if call.wants_more() {
            call.reply(Some(line))?;
        }
    }

    // Wait for script to exit
    let exit_status = process.wait()?;

    match exit_status {
        // If the command exited with a code, return the code
        ExitStatus::Exited(0) => Ok(()),
        // If process had an abnormal exit code just exit 1
        ExitStatus::Exited(n) => Err(format_err!(
            r#"Host script "{}" exited non-zero ({})"#,
            script_name,
            n
        )),
        ExitStatus::Signaled(signum) => Err(format_err!(
            r#"Host script "{}" terminated by signal ({})"#,
            script_name,
            signum
        )),
        status => Err(format_err!(
            r#"Host script "{}" failed: {:?}"#,
            script_name,
            status
        )),
    }
}

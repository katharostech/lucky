//! Contains functions used to interact with Juju through the hook environment
//! tools. Also contains Juju specific types such as the Juju metadata.yaml struct.

use anyhow::Context;

use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

use crate::process::run_cmd;
use crate::types::ScriptStatus;

/// Set the Juju status
///
/// Returns the command output
pub(crate) fn set_status(status: ScriptStatus) -> anyhow::Result<()> {
    run_cmd(
        "status-set",
        &[
            status.state.as_ref(),
            &status.message.unwrap_or_else(|| "".into()),
        ],
    )?;

    Ok(())
}

pub(crate) fn unit_get_private_address() -> anyhow::Result<String> {
    Ok(run_cmd("unit-get", &["private-address"])?)
}

pub(crate) fn unit_get_public_address() -> anyhow::Result<String> {
    Ok(run_cmd("unit-get", &["public-address"])?)
}

pub(crate) fn config_get() -> anyhow::Result<HashMap<String, serde_json::Value>> {
    let config_json = run_cmd("config-get", &["--format", "json", "--all"])?;
    let config = serde_json::from_str(&config_json).context("Could not parse config json")?;

    Ok(config)
}

/// Write out a message to the Juju Log. Setting `debug` to `true` will tell Juju the log is a
/// debug log.
///
/// If `juju-log` is not in the path, this function will silently ignore it.
///
/// If there is a problem while running `juju-log` the error will be printed to stderr.
///
/// This function blocks until the command exits.
pub(crate) fn juju_log(message: &str, debug: bool) {
    // build the juju-log command
    let mut cmd = Command::new("juju-log");
    if debug {
        cmd.arg("--debug");
    }
    cmd.arg(&message);

    // Run command and awit for it to exit
    if let Err(e) = cmd.output() {
        // Ignore it if juju-log isn't in the path
        if let std::io::ErrorKind::NotFound = e.kind() {
        }
        // Otherwise print a warning that we couldn't log
        else {
            writeln!(
                std::io::stderr(),
                "[WARN]: Could not log to juju-log: {}",
                e
            )
            .ok();
        }
    }
}

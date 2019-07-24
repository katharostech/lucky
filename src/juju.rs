//! Contains functions used to interact with Juju through the hook environment
//! tools. Also contains Juju specific types such as the Juju metadata.yaml struct.

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

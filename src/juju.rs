//! Module contains functions used to interact with Juju through the hook environment
//! tools. Also contains Juju specific types such as the Juju metadata.yaml struct.
use anyhow::Context;
use subprocess::{Exec, ExitStatus, Redirection};

use std::collections::HashMap;

use crate::types::ScriptStatus;

mod types;
pub(crate) use types::*;

/// Set the Juju status
///
/// Returns the command output
pub(crate) fn set_status(
    status: ScriptStatus,
    environment: Option<&HashMap<String, String>>,
) -> anyhow::Result<()> {
    run_command(
        "status-set",
        &[
            status.state.as_ref(),
            &status.message.unwrap_or_else(|| "".into()),
        ],
        environment,
    )?;

    Ok(())
}

//
// Helpers
//

/// This function encapsulates all of the common execution and error handling that is used when
/// execting Juju commands.
fn run_command(
    cmd: &str,
    args: &[&str],
    environment: Option<&HashMap<String, String>>,
) -> anyhow::Result<String> {
    let mut command = Exec::cmd(cmd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .args(args);

    if let Some(env) = environment {
        for (k, v) in env {
            command = command.env(k, v);
        }
    }

    let capture = command
        .capture()
        .context(format!(r#"Could not run command "{}""#, cmd))?;

    match capture.exit_status {
        ExitStatus::Exited(0) => (),
        ExitStatus::Exited(x) => anyhow::bail!(
            "Command exited with code {}: {}\nouput: {}",
            x,
            cmd,
            capture.stdout_str()
        ),
        _ => anyhow::bail!(
            "Running command {} failed with exit status: {:?}\noutput: {}",
            cmd,
            capture.exit_status,
            capture.stdout_str()
        ),
    }

    Ok(capture.stdout_str())
}

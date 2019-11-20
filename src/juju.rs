//! Module contains functions used to interact with Juju through the hook environment
//! tools.
use anyhow::Context;
use subprocess::{Exec, ExitStatus, Redirection};

use serde_json::from_str;

mod types;
pub(crate) use types::*;

/// This macro encapsulates all of the commen execution and error handling that is used when
/// execting Juju commands.
fn run_command(cmd: &str, args: &[&str]) -> anyhow::Result<String> {
    let capture = Exec::cmd(cmd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .args(args)
        .capture()
        .context(format!(r#"Could not run command "{}""#, cmd))?;

    match capture.exit_status {
        ExitStatus::Exited(0) => (),
        ExitStatus::Exited(x) => anyhow::bail!("Command exited with code {}: {}", x, cmd),
        _ => anyhow::bail!(
            "Running command {} failed with exit status: {:?}",
            cmd,
            capture.exit_status
        ),
    }

    Ok(capture.stdout_str())
}

/// Set the Juju status
///
/// Returns the command output
pub(crate) fn set_status(status: JujuStatus, message: &str) -> anyhow::Result<()> {
    run_command("status-set", &[status.as_ref(), message])?;

    Ok(())
}

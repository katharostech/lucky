//! Module contains functions used to interact with Juju through the hook environment
//! tools.
use anyhow::Context;
use subprocess::{Exec, ExitStatus, Redirection};

mod types;
pub(crate) use types::*;

/// This macro encapsulates all of the commen execution and error handling that is used when
/// execting Juju commands.
macro_rules! run_command {
    ($cmd:tt $($arg:expr) *) => {
        let capture = Exec::cmd($cmd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        $(
            .arg($arg)
        )*
        .capture()
        .context(format!(r#"Could not run command "{}""#, $cmd))?;

        match capture.exit_status {
            ExitStatus::Exited(0) => (),
            ExitStatus::Exited(x) => anyhow::bail!("Command exited with code {}: {}", x, $cmd),
            _ => anyhow::bail!(
                "Running command {} failed with exit status: {:?}",
                $cmd,
                capture.exit_status
            )
        }

        return Ok(capture.stdout_str())
    }
}

/// Set the Juju status
///
/// Returns the command output
pub(crate) fn set_status(status: JujuStatus, message: &str) -> anyhow::Result<String> {
    run_command!("status-set" status.as_ref() message);
}

//! Contains functions used to interact with Juju through the hook environment
//! tools. Also contains Juju specific types such as the Juju metadata.yaml struct.

use crate::types::ScriptStatus;
use crate::process::run_cmd;

mod types;
pub(crate) use types::*;

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

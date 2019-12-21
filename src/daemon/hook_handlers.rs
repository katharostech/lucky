//! Built-in handlers for Juju hooks that are executed by the daemon

use super::*;
use crate::types::{ScriptState, ScriptStatus};

pub(super) fn handle_hook(daemon: &LuckyDaemon, hook_name: &str) -> anyhow::Result<()> {
    match hook_name {
        "install" => handle_install(daemon),
        _ => Ok(()),
    }
}

fn handle_install(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    // If Docker is required
    if daemon.lucky_metadata.use_docker {
        daemon.set_script_status(
            "__internal__",
            ScriptStatus {
                state: ScriptState::Maintenance,
                message: Some("Installing docker".into()),
            },
        )?;

        // Make sure Docker is installed
        crate::docker::ensure_docker()?;

        daemon.set_script_status(
            "__internal__",
            ScriptStatus {
                state: ScriptState::Active,
                message: None,
            },
        )?;
    }

    Ok(())
}

//! Built-in handlers for Juju hooks that are executed by the daemon

use super::*;
use crate::types::{ScriptState, ScriptStatus};

pub(super) fn handle_hook(daemon: &LuckyDaemon, hook_name: &str) -> anyhow::Result<()> {
    match hook_name {
        "install" => handle_install(daemon),
        "stop" => handle_stop(daemon),
        _ => Ok(()),
    }
}

#[function_name::named]
fn handle_install(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    // If Docker support is enabled
    if daemon.lucky_metadata.use_docker {
        daemon_set_status!(daemon, ScriptState::Maintenance, "Installing docker");

        // Make sure Docker is installed
        crate::docker::ensure_docker()?;

        daemon_set_status!(daemon, ScriptState::Active);
    }

    Ok(())
}

fn handle_stop(_daemon: &LuckyDaemon) -> anyhow::Result<()> {
    // TODO: Clean up docker containers

    Ok(())
}

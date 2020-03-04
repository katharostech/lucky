//! Built-in handlers for Juju hooks that are executed by the daemon

use std::time::Duration;

use super::*;
use crate::docker::ContainerInfo;
use crate::rt::block_on;
use crate::types::{ScriptState, ScriptStatus};

pub(super) fn handle_pre_hook(daemon: &LuckyDaemon, hook_name: &str) -> anyhow::Result<()> {
    match hook_name {
        "install" => handle_pre_install(daemon),
        "config-changed" => handle_pre_config_changed(daemon),
        "upgrade-charm" => handle_pre_upgrade_charm(daemon),
        _ => Ok(()),
    }
}

pub(super) fn handle_post_hook(daemon: &LuckyDaemon, hook_name: &str) -> anyhow::Result<()> {
    match hook_name {
        "stop" => handle_post_stop(daemon),
        _ => Ok(()),
    }
}

#[function_name::named]
fn handle_pre_install(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    let mut state = daemon.state.write().unwrap();

    // Update the config cache
    update_config_cache(&mut state)?;

    // If Docker support is enabled
    if daemon.lucky_metadata.use_docker {
        daemon_set_status!(&mut state, ScriptState::Maintenance, "Installing docker");

        // Make sure Docker is installed
        crate::docker::ensure_docker()?;

        daemon_set_status!(&mut state, ScriptState::Active);
    }

    Ok(())
}

fn handle_pre_config_changed(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    let mut state = daemon.state.write().unwrap();

    // Update the configuration cache
    update_config_cache(&mut state)?;

    Ok(())
}

#[function_name::named]
fn handle_post_stop(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    let mut state = daemon.state.write().unwrap();
    let docker_conn = daemon.get_docker_conn()?;
    let docker_conn = docker_conn.lock().unwrap();

    daemon_set_status!(&mut state, ScriptState::Maintenance, "Removing containers");

    for mut container_info in state.named_containers.values_mut() {
        remove_container(&docker_conn, &mut container_info)?;
    }

    // Erase container config
    state.named_containers.clear();

    if let Some(container_info) = &mut state.default_container {
        remove_container(&docker_conn, container_info)?;
    }

    // Erase container config
    state.default_container = None;

    daemon_set_status!(&mut state, ScriptState::Active);
    Ok(())
}

#[function_name::named]
fn handle_pre_upgrade_charm(daemon: &LuckyDaemon) -> anyhow::Result<()> {
    let mut state = daemon.state.write().unwrap();
    daemon_set_status!(
        &mut state,
        ScriptState::Maintenance,
        "Updating containers after charm upgrade"
    );

    // Mark any containers as dirty because they need to be restarted
    if let Some(container) = &mut state.default_container {
        container.mark_dirty();
    }
    for container in &mut state.named_containers.values_mut() {
        container.mark_dirty();
    }

    // Drop state while we apply container updates
    drop(state);

    tools::apply_container_updates(&daemon)
        .context("Could not apply container updates during charm upgrade")?;

    // Set status to active
    let mut state = daemon.state.write().unwrap();
    daemon_set_status!(&mut state, ScriptState::Active);
    Ok(())
}

//
// Helpers
//

/// Update the daemons charm configuration cache with the valu
fn update_config_cache(state: &mut DaemonState) -> anyhow::Result<()> {
    log::debug!("Updating config cache");
    let charm_config = &mut state.charm_config;

    // Get updated charm config
    let latest_config = juju::config_get()?;

    // Loop through config
    for (k, v) in latest_config {
        // If it already exists
        if let Some(value) = charm_config.get_mut(&k) {
            // Update the value
            value.update(|value| *value = v);
        // If key does not already exist
        } else {
            // Insert the key
            charm_config.insert(k, Cd::new(v));
        }
    }

    Ok(())
}

/// Helper to remove a given container
fn remove_container(
    docker_conn: &shiplift::Docker,
    container_info: &mut Cd<ContainerInfo>,
) -> anyhow::Result<()> {
    // If container has an ID
    if let Some(id) = &container_info.id {
        let container = docker_conn.containers().get(id);

        // Stop the container
        log::debug!("Stopping container: {}", id);
        block_on(container.stop(Some(Duration::from_secs(10))))?;

        // Remove the container
        log::debug!("Removing container: {}", id);
        block_on(container.delete())?;

        // Unset the container id
        container_info.update(|info| info.id = None);
    }

    Ok(())
}

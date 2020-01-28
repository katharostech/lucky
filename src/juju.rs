//! Contains functions used to interact with Juju through the hook environment
//! tools. Also contains Juju specific types such as the Juju metadata.yaml struct.

use anyhow::{format_err, Context};

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

pub(crate) fn open_port(port_def: &str) -> anyhow::Result<String> {
    Ok(run_cmd("open-port", &[port_def])?)
}

pub(crate) fn close_port(port_def: &str) -> anyhow::Result<String> {
    Ok(run_cmd("close-port", &[port_def])?)
}

pub(crate) fn opened_ports() -> anyhow::Result<Vec<String>> {
    Ok(
        serde_json::from_str(&run_cmd("opened-ports", &["--format", "json"])?)
            .context("Could not parse json output of `opened-ports` command")?,
    )
}

pub(crate) fn relation_set(
    data: HashMap<String, String>,
    relation_id: Option<String>,
    app: bool,
) -> anyhow::Result<()> {
    let mut args: Vec<String> = vec![];

    if app {
        args.push("--app".into());
    }

    // Add relation option if specified
    if let Some(relation_id) = relation_id {
        args.push("-r".into());
        args.push(relation_id.into());
    }

    // Add data
    for (k, v) in data {
        args.push(format!("{}={}", k, v));
    }

    run_cmd(
        "relation-set",
        args.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
    )?;

    Ok(())
}

pub(crate) struct SpecificRelation {
    pub relation_id: String,
    pub remote_unit: String,
}

pub(crate) fn relation_get(
    relation: Option<SpecificRelation>,
    app: bool,
) -> anyhow::Result<HashMap<String, String>> {
    let mut args: Vec<String> = vec!["--format".into(), "json".into()];

    if app {
        args.push("--app".into());
    }

    // Add relation id
    if let Some(relation) = relation {
        args.append(&mut vec![
            "-r".into(),
            relation.relation_id,
            "-".into(),
            relation.remote_unit,
        ]);
    }

    // Run command
    let output = run_cmd(
        "relation-get",
        args.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
    )?;

    // Parse output
    Ok(serde_json::from_str(&output).context("Could not parse JSON response")?)
}

pub(crate) fn relation_list(relation_id: Option<String>) -> anyhow::Result<Vec<String>> {
    let mut args: Vec<String> = vec!["--format".into(), "json".into()];

    // Add relation id
    if let Some(relation_id) = relation_id {
        args.append(&mut vec!["-r".into(), relation_id.into()]);
    }

    // Run command
    let output = run_cmd(
        "relation-list",
        args.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
    )?;

    // Parse output
    Ok(serde_json::from_str(&output).context("Could not parse JSON")?)
}

pub(crate) fn relation_ids(relation_name: &str) -> anyhow::Result<Vec<String>> {
    // Run command
    let output = run_cmd("relation-ids", &["--format", "json", relation_name])?;

    // Parse output
    Ok(serde_json::from_str(&output).context("Could not parse JSON")?)
}

pub(crate) fn is_leader() -> anyhow::Result<bool> {
    // Run command
    let output = run_cmd("is-leader", &[])?;

    // Parse output
    match output.trim() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => {
            Err(format_err!("Unexpected response: {}", other)
                .context("Error running `is-leader` tool"))
        }
    }
}

pub(crate) fn leader_set(data: HashMap<String, String>) -> anyhow::Result<()> {
    let mut args: Vec<String> = vec![];

    // Add data
    for (k, v) in data {
        args.push(format!("{}={}", k, v));
    }

    run_cmd(
        "leader-set",
        args.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
    )?;

    Ok(())
}

pub(crate) fn leader_get() -> anyhow::Result<HashMap<String, String>> {
    // Run command
    let output = run_cmd("leader-get", &["--format", "json"])?;

    // Parse output
    Ok(serde_json::from_str(&output).context("Could not parse JSON response")?)
}

pub(crate) fn resource_get(resource_name: &str) -> anyhow::Result<String> {
    Ok(run_cmd("resource-get", &[resource_name])?.trim().into())
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

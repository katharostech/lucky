//! Docker related types

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// A container definition with extra metadata used by the Lucky daemon
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct Container {
    /// The id of the Docker container. Will be none if the container has not yet been run
    pub id: Option<String>,
    /// Marks this container as needing to be re-created with the updated config in def
    pub dirty: bool,
    /// The definition for the desired state of the container. This should match the actual state
    /// of the container if `dirty` is `false`.
    pub def: ContainerDef,
}

/// The definition of a Docker container run and all of its attributes such as image, volumes, etc.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct ContainerDef {
    /// The image specification, i.e. alpine/git
    pub image: String,
    /// The volumes for the container
    pub volumes: Vec<VolumeDef>,
    /// The port mappings for the container
    pub ports: Vec<PortDef>,
    /// The environment variables
    pub environment: HashMap<String, String>,
    /// The Docker entrypoint
    pub entrypoint: String,
    /// The Docker command
    pub command: String,
    /// Allocate a psuedo tty
    pub tty: bool,
    /// Keep a stdin pipe to the entrypoint open
    pub stdin_open: bool,
    /// Make container privileged
    pub privileged: bool,
    /// Restart policy
    pub restart: RestartPolicy,
}

/// A Docker volume definition
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct VolumeDef {
    pub source: String,
    pub target: String,
}

/// A Docker port mapping
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct PortDef {
    pub source: u16,
    pub target: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) enum RestartPolicy {
    /// Do not restart the container
    No,
    /// Restart container on failure with an optional maximum number of retries
    OnFailure(Option<u32>),
    /// Restart the container unless it was manually stopped
    UnlessStopped,
    /// Restart the container no matter what
    Always,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::No
    }
}

// Comment until we need it to supress clippy lint
// impl RestartPolicy {
//     /// Convert to a docker CLI representation
//     fn get_cli_arg(&self) -> String {
//         format!(
//             "--restart={}",
//             match self {
//                 RestartPolicy::No => "no".into(),
//                 RestartPolicy::OnFailure(Some(n)) => format!("on-failure:{}", n),
//                 RestartPolicy::OnFailure(None) => "on-failure".into(),
//                 RestartPolicy::Always => "always".into(),
//                 RestartPolicy::UnlessStopped => "unless-stopped".into(),
//             }
//         )
//     }
// }

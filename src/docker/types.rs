//! Docker related types

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// A struct made of a container definition and the container id
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Container {
    /// The id of the Docker container. Will be none if the container has not yet been run
    pub id: Option<String>,
    /// Marks this container as pending removal
    pub pending_removal: bool,
    /// The definition for the desired state of the container. This should match the actual state
    /// of the container if `dirty` is `false`.
    pub def: ContainerDef,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            id: None,
            pending_removal: false,
            def: Default::default(),
        }
    }
}

/// The definition of a Docker container run and all of its attributes such as image, volumes, etc.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct ContainerDef {
    /// The Docker command
    pub command: Option<String>,
    /// The image specification, i.e. alpine/git
    pub image: String,
    /// The Docker entrypoint
    pub entrypoint: Option<String>,
    /// The volumes for the container in the format expected by `docker run`
    pub volumes: Vec<String>,
    /// The port mappings for the container in the format expected by `docker run`
    pub ports: Vec<String>,
    /// The environment variables
    pub environment: HashMap<String, String>,
    /// Allocate a psuedo tty
    pub tty: bool,
    /// Keep a stdin pipe to the entrypoint open
    pub interactive: bool,
    /// Make container privileged
    pub privileged: bool,
    /// Restart policy
    pub restart: RestartPolicy,
}

impl ContainerDef {
    /// Returns an `Vec` of arguments to Docker to run the container with the given spec
    pub fn to_docker_args(&self) -> Vec<String> {
        let mut args = vec!["run".into(), "-d".into()];

        args.push(self.restart.to_docker_arg());

        if self.privileged {
            args.push("--privileged".into());
        }
        if self.interactive {
            args.push("--interactive".into());
        }
        if self.tty {
            args.push("--tty".into());
        }
        for (var, value) in &self.environment {
            args.push("--env".into());
            args.push(format!("{}={}", var, value));
        }
        for port in &self.ports {
            args.push("--publish".into());
            args.push(port.clone())
        }
        for volume in &self.volumes {
            args.push("--volume".into());
            args.push(volume.clone())
        }
        if let Some(entrypoint) = &self.entrypoint {
            args.push("--entrypoint".into());
            args.push(entrypoint.clone());
        }
        args.push(self.image.clone());
        if let Some(command) = &self.command {
            args.push(command.clone());
        }

        args
    }
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
impl RestartPolicy {
    /// Convert to a docker CLI representation
    fn to_docker_arg(&self) -> String {
        format!(
            "--restart={}",
            match self {
                RestartPolicy::No => "no".into(),
                RestartPolicy::OnFailure(Some(n)) => format!("on-failure:{}", n),
                RestartPolicy::OnFailure(None) => "on-failure".into(),
                RestartPolicy::Always => "always".into(),
                RestartPolicy::UnlessStopped => "unless-stopped".into(),
            }
        )
    }
}

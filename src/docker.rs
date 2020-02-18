//! Contains tools for installing and interracting with Docker
use anyhow::{bail, Context};
use regex::Regex;
use serde::{Deserialize, Serialize};
use shiplift::builder::ContainerOptions;
use shrinkwraprs::Shrinkwrap;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::process::{cmd_exists, run_cmd, run_cmd_with_retries};

use crate::VOLUME_DIR;

/// A struct made of a container definition and the container id
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub(crate) struct ContainerInfo {
    /// The id of the Docker container. Will be none if the container has not yet been run
    pub id: Option<String>,
    /// Marks this container as pending removal
    pub pending_removal: bool,
    /// Whether or not to pull the Docker image before running it
    pub pull_image: bool,
    /// The definition for the desired state of the container. This should match the actual state
    /// of the container if `dirty` is `false`.
    pub config: ContainerConfig,
}

impl ContainerInfo {
    pub fn new(image: &str) -> Self {
        ContainerInfo {
            id: None,
            pending_removal: false,
            pull_image: true,
            config: ContainerConfig::new(image),
        }
    }
}

#[derive(Shrinkwrap, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Clone, Debug)]
#[shrinkwrap(mutable)]
#[serde(transparent)]
/// A volume source path wrapper type to make it more difficult to mix-up sources and targets
pub struct VolumeSource(pub String);

#[derive(Shrinkwrap, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Clone, Debug)]
#[shrinkwrap(mutable)]
#[serde(transparent)]
/// A volume target path wrapper type to make it more difficult to mix-up sources and targets
pub struct VolumeTarget(pub String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Default, Clone, Debug)]
pub struct PortBinding {
    pub container_port: u32,
    pub host_port: u32,
    pub protocol: String,
}

impl std::fmt::Display for PortBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}/{}",
            self.host_port, self.container_port, self.protocol
        )
    }
}

impl FromStr for PortBinding {
    type Err = anyhow::Error;

    fn from_str(port_string: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^(?P<host_port>[0-9]{1,5}):(?P<container_port>[0-9]{1,5})(/(?P<protocol>(tcp|udp)))?$",
        ).expect("Could not compile regex");

        if let Some(captures) = re.captures(port_string) {
            Ok(PortBinding {
                host_port: captures
                    .name("host_port")
                    .expect("Expected host port")
                    .as_str()
                    .parse()
                    .expect("Could not parse int"),
                container_port: captures
                    .name("container_port")
                    .expect("expected container port")
                    .as_str()
                    .parse()
                    .expect("Could not parse int"),
                protocol: captures
                    .name("protocol")
                    .map_or("tcp".into(), |x| x.as_str().into()),
            })
        } else {
            Err(anyhow::format_err!("Could not parse port binding"))
        }
    }
}

/// The container configuration options such as image, volumes, ports, etc.
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
pub(crate) struct ContainerConfig {
    pub image: String,
    pub env_vars: HashMap<String, String>,
    pub entrypoint: Option<String>,
    pub command: Option<Vec<String>>,
    /// Volume mapping from target to source
    pub volumes: HashMap<VolumeTarget, VolumeSource>,
    // The port bindings
    pub ports: HashSet<PortBinding>,
    pub network: Option<String>,
}

impl ContainerConfig {
    pub fn new(image: &str) -> Self {
        ContainerConfig {
            image: image.to_owned(),
            ..Default::default()
        }
    }

    /// Get a `ContainerOptions` struct that can be given to shiplift to run the container
    ///
    /// The `charm_dir` is used as reference when mounting the container scripts into the container
    /// and the `socket_path` is used to mount the Lucky Daemon socket inside the container.
    pub fn to_container_options(
        &self,
        charm_dir: &Path,
        lucky_data_dir: &Path,
        socket_path: &Path,
    ) -> anyhow::Result<ContainerOptions> {
        let mut options = ContainerOptions::builder(&self.image);
        let mut volumes: Vec<String> = vec![];
        let mut env: Vec<String> = vec![];

        // Mount container scripts into the container
        volumes.push(format!(
            "{}:{}",
            charm_dir.join("container_scripts").to_string_lossy(),
            "/lucky/container_scripts"
        ));

        // Mount Lucky into the container
        let lucky_path = std::env::current_exe().context("Could not determine Lucky exe path")?;
        volumes.push(format!(
            "{}:{}",
            lucky_path.to_string_lossy(),
            "/usr/bin/lucky"
        ));

        // Mount the Lucky daemon socket into the container
        let container_socket_path = "/run/lucky.sock";
        volumes.push(format!(
            "{}:{}",
            socket_path.to_string_lossy(),
            container_socket_path
        ));

        // Add Socket path environment variable
        env.push(format!("LUCKY_DAEMON_SOCKET={}", container_socket_path));
        // Set lucky context to client
        env.push("LUCKY_CONTEXT=client".into());

        // Add the rest of the environment variables
        for (var, value) in &self.env_vars {
            env.push(format!("{}={}", var, value));
        }

        // Add entrypoint
        if let Some(entrypoint) = &self.entrypoint {
            options.entrypoint(&entrypoint);
        }

        // Add command
        if let Some(cmd) = &self.command {
            options.cmd(cmd.iter().map(AsRef::as_ref).collect());
        }

        // Add other specified volumes
        for (target, source) in &self.volumes {
            let host_path = if source.starts_with('/') {
                PathBuf::from(&**source)
            } else {
                lucky_data_dir.join(VOLUME_DIR).join(&**source)
            };

            // Create the host path
            if !host_path.exists() {
                fs::create_dir_all(&host_path).context(format!(
                    "Could not create dir: {}",
                    host_path.to_string_lossy()
                ))?;
            }

            // Add volume to container
            volumes.push(format!("{}:{}", host_path.to_string_lossy(), &**target));
        }

        // Add ports
        for PortBinding {
            container_port,
            protocol,
            host_port,
        } in &self.ports
        {
            options.expose(*container_port, protocol, *host_port);
        }

        // Set network
        if let Some(network) = &self.network {
            options.network_mode(network);
        }

        // Add volumes
        options.volumes(volumes.iter().map(AsRef::as_ref).collect());
        // Add environment
        options.env(env.iter().map(AsRef::as_ref).collect());

        // TODO: Right now we will always add the "restart unless-stopped" flag, but we should
        // parameterize this later.
        options.restart_policy("unless-stopped", 0 /* Maximum retry count */);

        // Build options
        Ok(options.build())
    }
}

/// Make sure Docker is installed an available
pub(crate) fn ensure_docker() -> anyhow::Result<()> {
    // Skip if docker is already installed
    if cmd_exists("docker", &["--version"])? {
        return Ok(());
    };

    // Install Docker
    // TODO: We need to figure out whether or not to use the Docker snap, which doesn't work on
    // LXD. We will also want to support Centos, but we might just have to install different
    // packages depending on the system.
    run_cmd_with_retries(
        "apt-get",
        &["install", "-y", "docker.io"],
        &Default::default(),
    )?;
    // Make sure docker is installed
    if !cmd_exists("docker", &["--version"])? {
        bail!("Could not install Docker");
    }

    // Get proxy settings from environment
    let mut proxy_settings = String::new();
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY").or_else(|_| std::env::var("http_proxy")) {
        proxy_settings.push_str(&format!("Environment=\"HTTP_PROXY={}\"\n", http_proxy));
    }
    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy"))
    {
        proxy_settings.push_str(&format!("Environment=\"HTTPS_PROXY={}\"\n", https_proxy));
    }

    // If there are any proxy settings
    if proxy_settings != "" {
        // Create the Docker service drop-in dir
        let dropin_dir = "/etc/systemd/system/docker.service.d/";
        fs::create_dir_all(dropin_dir).context(format!(
            "Could not create docker service drop-in config dir: {:?}",
            dropin_dir
        ))?;

        // Open the drop-in file
        let file_path = "/etc/systemd/system/docker.service.d/http-proxy.conf";
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)
            .context(format!(
                "Could not open Docker service dropin file: {:?}",
                file_path
            ))?;

        // Insert proxy settings
        file.write_all(format!("[Service]\n{}", proxy_settings).as_bytes())?;

        // Close file
        drop(file);

        // Reload Docker config
        run_cmd("systemctl", &["daemon-reload"])?;
        run_cmd("systemctl", &["restart", "docker"])?;
    }

    Ok(())
}

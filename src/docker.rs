//! Contains tools for installing and interracting with Docker
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use shiplift::builder::ContainerOptions;

use std::fs;
use std::io::Write;

use crate::process::{cmd_exists, run_cmd, run_cmd_with_retries};

/// A struct made of a container definition and the container id
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContainerInfo {
    /// The id of the Docker container. Will be none if the container has not yet been run
    pub id: Option<String>,
    /// Marks this container as pending removal
    pub pending_removal: bool,
    /// The definition for the desired state of the container. This should match the actual state
    /// of the container if `dirty` is `false`.
    pub config: ContainerConfig,
}

impl ContainerInfo {
    pub fn new(image: &str) -> Self {
        ContainerInfo {
            id: None,
            pending_removal: false,
            config: ContainerConfig::new(image),
        }
    }
}

/// The container configuration options such as image, volumes, ports, etc.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContainerConfig {
    pub image: String,
}

impl ContainerConfig {
    pub fn new(image: &str) -> Self {
        ContainerConfig {
            image: image.to_owned(),
        }
    }

    /// Get a `ContainerOptions` struct that can be given to shiplift to run the container
    pub fn to_container_options(&self) -> ContainerOptions {
        let options = ContainerOptions::builder(&self.image);

        options.build()
    }
}

/// Make sure Docker is installed an available
pub(crate) fn ensure_docker() -> anyhow::Result<()> {
    // Skip if docker is already installed
    if cmd_exists("docker", &["--version"])? {
        return Ok(());
    };

    // Install the Docker snap
    run_cmd_with_retries("snap", &["install", "docker"], &Default::default())?;
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
        let dropin_dir = "/etc/systemd/system/snap.docker.dockerd.service.d/";
        fs::create_dir_all(dropin_dir).context(format!(
            "Could not create docker service drop-in config dir: {:?}",
            dropin_dir
        ))?;

        // Open the drop-in file
        let file_path = "/etc/systemd/system/snap.docker.dockerd.service.d/http-proxy.conf";
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
        run_cmd("snap", &["restart", "docker"])?;
    }

    Ok(())
}

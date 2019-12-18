//! Contains tools for installing and interracting with Docker

use anyhow::{bail, Context};

use std::fs;
use std::io::Write;

use crate::process::{cmd_exists, run_cmd};

/// Make sure Docker is installed an available
pub(crate) fn ensure_docker() -> anyhow::Result<()> {
    // Skip if docker is already installed
    if cmd_exists("docker", &["--version"])? {
        return Ok(());
    };

    // Install the Docker snap
    run_cmd("snap", &["install", "docker"])?;
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

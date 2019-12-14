//! Contains tools for installing and interracting with Docker

use anyhow::Context;
use subprocess::{Exec, ExitStatus, PopenError, Redirection};

use std::io::ErrorKind as IoErrorKind;

/// Make sure Docker is installed an available
pub(crate) fn ensure_docker() -> anyhow::Result<()> {
    // Try running `docker --version`
    match Exec::cmd("docker").arg("--version").join() {
        Err(PopenError::IoError(e)) => match e.kind() {
            IoErrorKind::NotFound => (), // If Docker isn't found continue to install
            // If there is a different kind of error, report it
            _ => return Err(e).context("Error running \"docker --version\""),
        },
        Err(PopenError::Utf8Error(e)) => {
            return Err(e).context("Error running \"docker --version\"")
        }
        Err(PopenError::LogicError(e)) => panic!("Logic error spanwing docker script: {}", e),
        Ok(_) => return Ok(()),
    }

    // Install the Docker snap
    let capture = Exec::cmd("snap")
        .args(&["install", "docker"])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .context(r#"Could not run "snap install docker""#)?;

    // If the install was successful
    if let ExitStatus::Exited(0) = capture.exit_status {
        // Try running `docker --version` again
        match Exec::cmd("docker").arg("--version").join() {
            Err(PopenError::LogicError(e)) => panic!("Logic error spanwing docker script: {}", e),
            Err(e) => Err(e).context("Could not install docker"),
            Ok(_) => Ok(()),
        }

    // If the install failed
    } else {
        anyhow::bail!(
            "Running \"snap install docker\" failed.\n{}\noutput: {}",
            if let ExitStatus::Exited(n) = capture.exit_status {
                format!("Exit code: {}", n)
            } else {
                "".into()
            },
            capture.stdout_str()
        );
    }
}

use anyhow::{format_err, Context};
use subprocess::{Exec, ExitStatus, PopenError, Redirection};

use std::collections::HashMap;
use std::io::ErrorKind as IoErrorKind;

/// Test that a program exists and that running it succeeds
///
/// The function will attempt to execute the given command on the system and and will return
/// `Ok(true)` if the command exists and `Ok(false)` if the command is not found.
pub(crate) fn cmd_exists(command: &str, args: &[&str]) -> anyhow::Result<bool> {
    let cmd = Exec::cmd(command).args(args);
    let command_string = cmd.to_cmdline_lossy();
    let err_message = format!("Error running {}", command_string);

    match cmd.join() {
        Err(PopenError::IoError(e)) => match e.kind() {
            IoErrorKind::NotFound => Ok(false), // If Docker isn't found continue to install
            // If there is a different kind of error, report it
            _ => Err(e).context(err_message),
        },
        Err(PopenError::Utf8Error(e)) => Err(e).context(err_message),
        Err(PopenError::LogicError(e)) => panic!("Logic error spawning {}: {}", command_string, e),
        Ok(_) => Ok(true),
    }
}

/// Run a command synchronously with error context
///
/// A utility for running commands, merging their stdout and stderr, and using that output during
/// error reporting. Command will exit with an error if there is an IO error or if the command exits
/// non-zero.
fn _run_cmd(
    command: &str,
    args: &[&str],
    env: Option<&HashMap<String, String>>,
) -> anyhow::Result<String> {
    let mut cmd = Exec::cmd(command)
        .args(args)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge);
    if let Some(env) = env {
        cmd = cmd.env_extend(
            env.iter()
                .map(|(x, y)| (x.as_str(), y.as_str()))
                .collect::<Vec<(&str, &str)>>()
                .as_slice(),
        )
    }
    let command_string = cmd.to_cmdline_lossy();
    let err_message = format!("Error running {}", command_string);

    match cmd.capture() {
        Err(PopenError::IoError(e)) => Err(e).context(err_message),
        Err(PopenError::Utf8Error(e)) => Err(e).context(err_message),
        Err(PopenError::LogicError(e)) => panic!(
            "Logic error while running command {}: {}",
            command_string, e
        ),
        Ok(capture) => {
            if capture.success() {
                Ok(capture.stdout_str())
            } else {
                let exit_code_str = match capture.exit_status {
                    ExitStatus::Exited(code) => format!("({})", code),
                    ExitStatus::Signaled(sig) => format!("( Got signal: {} )", sig),
                    _ => "".into(),
                };

                Err(format_err!(
                    "Command exited non-zero {}. Output:\n{}",
                    exit_code_str,
                    capture.stdout_str()
                )
                .context(format!("Error running command {}", command_string)))
            }
        }
    }
}

pub(crate) fn run_cmd(command: &str, args: &[&str]) -> anyhow::Result<String> {
    _run_cmd(command, args, None)
}

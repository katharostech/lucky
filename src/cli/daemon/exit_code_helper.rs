//! This is a command that will run another command, output that commands output and finally, when
//! the command exits, it will print out the exit code of the command prefixed with the
//! `lucky::types::LUCKY_EXIT_CODE_HELPER_PREFIX`.
//!
//! This is a hack to get around this [issue](https://github.com/softprops/shiplift/issues/219).
//! Once that issue is resolved, we should remove this.
//!
//! Other affected code is the `lucky::daemon::tools::run_container_script()` function that uses
//! this command to get the exit code of container scripts.

use anyhow::{format_err, Context};
use clap::{App, Arg, ArgMatches};

use std::io::Write;
use std::process::Command;

use crate::cli::*;
use crate::types::LUCKY_EXIT_CODE_HELPER_PREFIX;

pub(super) struct ExitCodeHelperSubcommand;

impl<'a> CliCommand<'a> for ExitCodeHelperSubcommand {
    fn get_name(&self) -> &'static str {
        "exit-code-helper"
    }

    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about(concat!(
                "Run a command, print its output, and print the exit code prefixed by: ",
                "__LUCKY_CMD_EXIT_CODE__:"
            ))
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("command").multiple(true).required(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        let mut command = args
            .values_of("command")
            .expect("Missing required argument: command");
        let command_string = command.clone().collect::<Vec<_>>().as_slice().join(" ");

        // Run provided command. Stderr and stdout will be inherited from this process
        let status = Command::new(
            command
                .next()
                .ok_or(format_err!("Missing command argument"))?,
        )
        .args(command.collect::<Vec<&str>>().as_slice())
        // Make sure to set the context to client so scripts work like normal
        .env("LUCKY_CONTEXT", "client")
        .status()
        .context(format!("Failed to run command: {}", command_string))?;

        // If there is a valid exit code
        if let Some(code) = status.code() {
            // Print out exit status with prefix
            writeln!(
                std::io::stdout(),
                "{}{}",
                LUCKY_EXIT_CODE_HELPER_PREFIX,
                code
            )?;

        // For the sake of simplicity, exit 1 if the process was terminated by a signal and did
        // not have an exit code ( Unix-specific behavior )
        } else {
            // Print out exit code with prefix
            writeln!(std::io::stdout(), "{}1", LUCKY_EXIT_CODE_HELPER_PREFIX)?;
        }

        Ok(data)
    }
}

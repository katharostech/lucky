//! Commandline interface module

use clap::{App, ArgMatches};

mod types;
use types::*;

// Help utility
pub mod doc;

// Subcommands
mod charm;
mod client;
mod daemon;

/// Run the application
pub fn run() {
    // Initialize logger
    crate::log::init_logger();

    std::panic::catch_unwind(|| {
        // run program and report any errors
        if let Err(e) = execute() {
            if let Some(cli_error) = e.downcast_ref::<CliError>() {
                match cli_error {
                    CliError::Exit(0) => std::process::exit(0),
                    CliError::Exit(code) => {
                        log::error!("{:?}", e);
                        std::process::exit(*code);
                    }
                }
            } else {
                log::error!("{:?}", e);
                std::process::exit(1);
            }
        }
    })
    // Catch any panics and print an error message. This will appear after the message given by
    // colored backtrace.
    .or_else(|_| -> Result<(), ()> {
        log::error!(concat!(
            "The program has encountered a critical internal error and will now exit.\n",
            "This is a bug. TODO: Setup Taiga project for reporting errors!!\n"
        ));

        Ok(())
    })
    .expect("Panic while handling panic");
}

fn execute() -> anyhow::Result<()> {
    let cli: Box<dyn CliCommand>;

    // If there is a specified Lucky context
    if let Ok(context) = std::env::var("LUCKY_CONTEXT") {
        // Use the specified subcommand instead
        match context.as_ref() {
            "charm" => cli = Box::new(charm::CharmSubcommand),
            "daemon" => cli = Box::new(daemon::DaemonSubcommand),
            "client" => unimplemented!(),
            other => anyhow::bail!("Unrecognized LUCKY_CONTEXT: {}", other),
        };

    // Use the full lucky CLI
    } else {
        cli = Box::new(LuckyCli);
    }

    // Run the CLI
    let cmd = cli.get_cli();
    let args = cmd.get_matches();
    cli.run(&args)?;

    Ok(())
}

pub(crate) struct LuckyCli;

impl<'a> CliCommand<'a> for LuckyCli {
    fn get_name(&self) -> &'static str {
        "lucky"
    }

    fn get_command(&self) -> App<'a> {
        self.get_base_app()
            .version(crate::GIT_VERSION)
            .about("The Lucky charm framework for Juju.")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(charm::CharmSubcommand),
            Box::new(daemon::DaemonSubcommand),
            Box::new(client::ClientSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky",
            content: include_str!("cli/cli.md"),
        })
    }

    fn execute_command(&self, _args: &ArgMatches) -> anyhow::Result<()> {
        // Enable colored backtraces
        #[cfg(feature = "color-backtrace")]
        color_backtrace::install();

        Ok(())
    }
}

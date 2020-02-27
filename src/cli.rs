//! Commandline interface module
use anyhow::format_err;
use clap::{App, AppSettings, ArgMatches};

mod types;
use types::*;

// Help utility
pub mod doc;
// Misc. utilities
mod util;

// Subcommands
mod charm;
mod client;
#[cfg(feature = "daemon")]
mod daemon;

/// Run the CLI
pub fn run() {
    run_with_error_handler(run_cli);
}

fn run_cli() -> anyhow::Result<()> {
    let cli: Box<dyn CliCommand>;

    // If there is a specified Lucky context
    if let Ok(context) = std::env::var("LUCKY_CONTEXT") {
        // Use the specified subcommand instead
        match context.as_ref() {
            "charm" => cli = Box::new(charm::CharmSubcommand),
            #[cfg(feature = "daemon")]
            "daemon" => cli = Box::new(daemon::DaemonSubcommand),
            "client" => cli = Box::new(client::ClientSubcommand),
            other => anyhow::bail!("Unrecognized LUCKY_CONTEXT: {}", other),
        };

    // Use the full lucky CLI
    } else {
        cli = Box::new(LuckyCli);
    }

    // Show doc page if applicable
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.iter();
    args_iter.next(); // Skip first arg, which is the binary name
    cli.handle_doc_flags(args_iter)?;

    // Run CLI
    let cmd = cli.get_cli();
    let args = cmd.get_matches_from(&args);
    cli.run(&args, Default::default())?;

    Ok(())
}

pub(crate) struct LuckyCli;

impl<'a> CliCommand<'a> for LuckyCli {
    fn get_name(&self) -> &'static str {
        "lucky"
    }

    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .version(crate::LUCKY_VERSION)
            .setting(AppSettings::ArgRequiredElseHelp)
            .about("The Lucky charm framework for Juju.")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(charm::CharmSubcommand),
            Box::new(client::ClientSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky",
            content: include_str!("cli/cli_help/lucky.md"),
        })
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}

/// Run the documentation generator
pub fn doc_gen() {
    run_with_error_handler(run_doc_gen);
}

/// Generate CLI documentation
fn run_doc_gen() -> anyhow::Result<()> {
    println!("Starting doc gen");

    let cli = LuckyCli;
    doc::mdbook::generate_docs(
        &cli,
        match std::env::args().nth(1) {
            Some(arg) => arg,
            None => {
                anyhow::bail!("Out path argument required as first and only positional argument")
            }
        }
        .as_ref(),
    )?;

    println!("Doc gen finished");

    Ok(())
}

/// Run the given function with error handling and logging initialized
pub fn run_with_error_handler(f: fn() -> anyhow::Result<()>) {
    // Enable colored backtraces
    #[cfg(feature = "better-panic")]
    better_panic::Settings::auto().lineno_suffix(true).install();

    // Initialize logger
    crate::log::init_logger();

    std::panic::catch_unwind(|| {
        // run program and report any errors
        if let Err(e) = f() {
            // Handle special instructions from `CliError`s
            if let Some(cli_error) = e.downcast_ref::<CliError>() {
                match cli_error {
                    CliError::Exit(0) => std::process::exit(0),
                    CliError::Exit(code) => {
                        log::error!("{:?}", e);
                        std::process::exit(*code);
                    }
                }

            // Print varlink errors without the extra debug printing
            } else if let Some(varlink_error) = e.downcast_ref::<crate::rpc::Error>() {
                let e = format_err!("{}", varlink_error.kind());
                log::error!("{}", e);
                std::process::exit(1);

            // For all other errors just print out the default anyhow rendering of it
            } else {
                log::error!("{:?}", e);
                std::process::exit(1);
            }
        }
    })
    // Catch any panics and print an error message. This will appear after the message given by
    // colored backtrace.
    // TODO: Replace all uses of the concat macro fro wrapping strings with backslash escapes
    .or_else(|_| -> Result<(), ()> {
        log::error!(concat!(
            "The program has encountered a critical internal error and will now exit. ",
            "This is a bug. Please report it on our issue tracker:\n\n",
            "    https://tree.taiga.io/project/zicklag-lucky/issues"
        ));

        std::process::exit(1);
    })
    .expect("Panic while handling panic");
}

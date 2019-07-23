use anyhow::Context;
use clap::{App, Arg, ArgMatches};
use rand::{seq::IteratorRandom, thread_rng, Rng};

use std::io::Write;

use crate::cli::*;

const PASSWORD_CHARS: &'static str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub(super) struct RandomSubcommand;

impl<'a> CliCommand<'a> for RandomSubcommand {
    fn get_name(&self) -> &'static str {
        "random"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Generate random numbers and strings")
            .long_about(concat!(
                "Generate random numbers and strings. Without any arguments it generates a ",
                "random sequence of 24 letters and numbers."
            ))
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("range")
                .help("Generate a random integer in a range")
                .short('r')
                .long("range")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["lowest_number", "highest_number"])
                .conflicts_with("length"))
            .arg(Arg::with_name("float")
                .help("Generate a floating point range instead of an integer range")
                .short('f')
                .long("float")
                .requires("range"))
            .arg(Arg::with_name("length")
                .help("Set the length of the generated string")
                .short('l')
                .long("length")
                .takes_value(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        // Random number generator
        let mut rng = thread_rng();

        // Generate random range
        if let Some(mut range_vals) = args.values_of("range") {
            let lower_bound = range_vals
                .next()
                .expect("Missing required first value of \"range\" arg");
            let upper_bound = range_vals
                .next()
                .expect("Missing required second value of \"range\" arg");

            // Generate float range
            if args.is_present("float") {
                let lower_bound = lower_bound
                    .parse::<f32>()
                    .context("Could not parse lower bound of range")?;
                let upper_bound = upper_bound
                    .parse::<f32>()
                    .context("Could not parse upper bound of range")?;

                // print out range
                writeln!(
                    std::io::stdout(),
                    "{}",
                    rng.gen_range(lower_bound, upper_bound)
                )?;

            // Generate int rage
            } else {
                let lower_bound = lower_bound
                    .parse::<i32>()
                    .context("Could not parse lower bound of range")?;
                let upper_bound = upper_bound
                    .parse::<i32>()
                    .context("Could not parse upper bound of range")?;

                // Print out range
                writeln!(
                    std::io::stdout(),
                    "{}",
                    rng.gen_range(lower_bound, upper_bound)
                )?;
            }

        // Generate random password
        } else {
            let mut password = String::new();
            for _ in 0..args.value_of("length").map_or(Ok(24), str::parse)? {
                password.push(
                    PASSWORD_CHARS
                        .chars()
                        .choose(&mut rng)
                        .expect("Empty password chars iterator"),
                );
            }

            // Print out password
            writeln!(std::io::stdout(), "{}", password)?;
        }

        Ok(data)
    }
}

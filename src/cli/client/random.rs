use anyhow::Context;
use clap::{App, Arg, ArgMatches};
use get_port::{get_port_prefer, PortRange};
use rand::{seq::IteratorRandom, thread_rng, Rng};

use std::cmp::PartialOrd;
use std::io::Write;

use crate::cli::*;

const PASSWORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

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
                "Generate random passwords, numbers, and ports. Without any arguments it ",
                "generates a random sequence of 24 letters and numbers."
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
            .arg(Arg::with_name("available_port")
                .help("Get a random available port")
                .short('p')
                .long("available-port")
                .long_help(concat!(
                    "Get a random available port. This may be combined with --range to get a ",
                    "random available port in the given range. The default port range is ",
                    "1024-65535. If an available port cannot be found in the given range the next",
                    "available port starting at 1024 will be selected"
                ))
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

        // Generate available port in a range
        if args.is_present("available_port") {
            let port_range = if let Some(mut range_vals) = args.values_of("range") {
                let lower_bound = range_vals
                    .next()
                    .expect("Missing required first value of \"range\" arg");

                let upper_bound = range_vals
                    .next()
                    .expect("Missing required second value of \"range\" arg");

                let lower_bound = lower_bound
                    .parse::<u16>()
                    .context("Could not parse lower bound of range")?;
                let upper_bound = upper_bound
                    .parse::<u16>()
                    .context("Could not parse upper bound of range")?;

                verify_valid_range(&lower_bound, &upper_bound)?;

                PortRange {
                    min: lower_bound,
                    max: upper_bound + 1, // The max value is non-inclusive so we add 1 to it
                }
            } else {
                PortRange::default()
            };

            // Try to get a random port, or else just get the first available one in the range
            let port = get_port_prefer(vec![rng.gen_range(port_range.min, port_range.max)])
                .context("Failed to find an available port")?;

            writeln!(std::io::stdout(), "{}", port)?;

        // Generate a random number in a range
        } else if let Some(mut range_vals) = args.values_of("range") {
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

                verify_valid_range(&lower_bound, &upper_bound)?;

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

                verify_valid_range(&lower_bound, &upper_bound)?;

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

fn verify_valid_range<T>(lower: &T, upper: &T) -> anyhow::Result<()>
where
    T: std::fmt::Display + PartialOrd,
{
    if lower >= upper {
        anyhow::bail!(
            "Lower bound of range cannot be greather than or equal to upper bound: {} => {}",
            lower,
            upper
        );
    }

    Ok(())
}

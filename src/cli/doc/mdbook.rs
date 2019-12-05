//! Module for creating the content for an mdbook site from the Lucky CLI documentation

use clap::ArgSettings;
use regex::Regex;

use std::fmt::Write as FmtWrite;

use crate::cli::*;

/// A kind of command argument
enum ArgType {
    Flag,
    Opt, // Value name
    Positional,
}

lazy_static::lazy_static! {
    /// Matches an inline code snippet
    static ref INLINE_CODE: Regex =
        Regex::new(r"(?m)`(?P<text>.*?)`")
            .expect("Coud not compile regex");
}

pub(crate) fn print_doc<'a>(command: &dyn CliCommand<'a>) -> anyhow::Result<()> {
    let mut cli = command.get_app();

    let write_err = "Could not write to internal string buffer";
    let mut command_help = String::new();

    // Print command usage
    write!(
        command_help,
        "# {command_name}\n\n## Usage\n\n`{usage}`\n\n",
        command_name = cli.name.clone(),
        usage = cli // Here we have to parse out the `USAGE:\n` from `generate_usage()`
            .generate_usage()
            .split("\n")
            .nth(1)
            .expect("Error parsing command usage")
            .trim()
    )
    .expect(write_err);

    let table_head = "<table>\n<thead><tr><th>Arg</th><th>Environment Variable</th><th>Description</th><tr></thead>\n<tbody>\n";
    let table_tail = "</tbody>\n</table>\n\n";

    let mut flags_buf = String::new();
    let mut options_buf = String::new();
    let mut positionals_buf = String::new();

    let arg_map = cli.args;
    for arg in arg_map.args {
        let mut arg_type = ArgType::Flag;
        let mut arg_buffer = String::new();
        write!(arg_buffer, "<tr><td>").expect(write_err);

        let mut flags = vec![];
        if let Some(short) = arg.short {
            flags.push(format!("<code>-{}</code>", short));
        }
        if let Some(long) = arg.long {
            flags.push(format!("<code>--{}</code>", long));
        }

        if flags.len() != 0 {
            write!(arg_buffer, "{}", flags.join(", ")).expect(write_err);
            if arg.is_set(ArgSettings::TakesValue) {
                arg_type = ArgType::Opt;
                let value_names = arg
                    .val_names
                    .as_ref()
                    .map(|x| {
                        format!(
                            "<code>&lt;{}&gt;</code>",
                            x.values().map(|&x| x).collect::<Vec<&str>>().join("&gt; &lt;")
                        )
                    })
                    .unwrap_or_else(|| format!("<code>&lt;{}&gt;</code>", arg.name));
                write!(arg_buffer, " {}", &value_names).expect(write_err);
            }
            write!(arg_buffer, "</td>").expect(write_err);
        } else {
            write!(arg_buffer, "<code>&lt;{}&gt;</code></td>", arg.name).expect(write_err);
            arg_type = ArgType::Positional;
        }

        if let Some((env, _)) = arg.env {
            write!(
                arg_buffer,
                "<td><code>{}</code></td>",
                env.to_string_lossy()
            )
            .expect(write_err);
        } else {
            write!(arg_buffer, "<td></td>").expect(write_err);
        }

        write!(
            arg_buffer,
            "<td>{}</td></tr>\n",
            INLINE_CODE.replace_all(arg.long_help.unwrap_or(arg.help.unwrap_or("")), "<code>$text</code>")
        )
        .expect(write_err);

        write!(
            match arg_type {
                ArgType::Flag => &mut flags_buf,
                ArgType::Opt => &mut options_buf,
                ArgType::Positional => &mut positionals_buf,
            },
            "{}",
            arg_buffer
        )
        .expect(write_err);
    }

    // Write out flags table
    if flags_buf != "" {
        write!(command_help, "### Flags\n\n{}", table_head).expect(write_err);
        write!(command_help, "{}", flags_buf).expect(write_err);
        write!(command_help, "{}", table_tail).expect(write_err);
    }
    // Write out options table
    if options_buf != "" {
        write!(command_help, "### Options\n\n{}", table_head).expect(write_err);
        write!(command_help, "{}", options_buf).expect(write_err);
        write!(command_help, "{}", table_tail).expect(write_err);
    }
    // Write out positionals table
    if positionals_buf != "" {
        write!(command_help, "### Positionals\n\n{}", table_head).expect(write_err);
        write!(command_help, "{}", positionals_buf).expect(write_err);
        write!(command_help, "{}", table_tail).expect(write_err);
    }

    println!("{}", command_help);
    println!("----");

    for subcommand in command.get_subcommands() {
        print_doc(&*subcommand)?;
    }

    Ok(())
}

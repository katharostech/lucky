//! Module for creating the content for an mdbook site from the Lucky CLI documentation

use std::fmt::Write as FmtWrite;

use crate::cli::*;

pub(crate) fn print_doc<'a>(command: &dyn CliCommand<'a>) -> anyhow::Result<()> {
    let mut cli = command.get_app();

    let write_err = "Could not write to internal string buffer";
    let mut command_help = String::new();
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

    // Args
    write!(command_help, "## Args\n\n<table><tbody>").expect(write_err);

    let arg_map = cli.args;
    for arg in arg_map.args {
        write!(command_help, "<tr><td>").expect(write_err);

        let mut flags = vec![];
        if let Some(short) = arg.short {
            flags.push(format!("<code>-{}</code>", short));
        }
        if let Some(long) = arg.long {
            flags.push(format!("<code>--{}</code", long));
        }
        write!(command_help, "{}<td/>", flags.join(", ")).expect(write_err);
        write!(
            command_help,
            "<td>{}</td></tr>",
            arg.long_help.unwrap_or(arg.help.unwrap_or(""))
        )
        .expect(write_err);
        write!(command_help, "\n").expect(write_err);
    }

    write!(command_help, "</tbody></table>\n\n").expect(write_err);

    println!("{}", command_help);
    println!("----");

    for subcommand in command.get_subcommands() {
        print_doc(&*subcommand)?;
    }

    Ok(())
}

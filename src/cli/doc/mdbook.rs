//! Module for creating the content for an mdbook site from the Lucky CLI documentation

use clap::ArgSettings;
use regex::Regex;

use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::cli::*;

/// A kind of command argument
enum ArgType {
    Flag,
    Opt, // Value name
    Positional,
}

lazy_static::lazy_static! {
    /// Matches an inline code snippet
    static ref MD_INLINE_CODE: Regex =
        Regex::new(r"(?m)`(?P<text>.*?)`")
            .expect("Coud not compile regex");

    /// Matches the doc-gen automatically generated section of the mdbook's SUMMARY.md
    static ref CLI_DOC_INDEX: Regex =
        Regex::new(r"(?ms)\s*---\s*- \[Lucky CLI\].*")
            .expect("Coud not compile regex");
}

pub(crate) fn generate_docs<'a>(
    command: &impl CliCommand<'a>,
    outpath: &Path,
) -> anyhow::Result<()> {
    // The mdbook index for all of the CLI commands. Goes in the SUMMARY.md file
    let mut summary_index = String::from("\n\n---\n");

    // Generate the CLI doc files
    recurse_gen_cli_doc(command, &mut summary_index, outpath, outpath, 0)?;

    // Get the path to the SUMMARY.md file
    let summary_path = outpath.join("SUMMARY.md");
    if !summary_path.exists() {
        anyhow::bail!("SUMMARY.md not found in output directory");
    }

    // Open the SUMMARY.md file
    let mut summary_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(summary_path)?;
    let mut summary_contents = String::new();
    summary_file.read_to_string(&mut summary_contents)?;

    // If the SUMMARY.md file already has the CLI doc index in it ( like when doc-gen has already
    // been run )
    if CLI_DOC_INDEX.is_match(&summary_contents) {
        // Truncate the file
        summary_file.set_len(0)?;
        summary_file.seek(SeekFrom::Start(0))?;
        // Recreate the index, replacing the exsting CLI doc index with our updated one
        let new_file_contents = CLI_DOC_INDEX.replace(&summary_contents, summary_index.as_str());
        summary_file.write_all(new_file_contents.as_bytes())?;

    // If the SUMMARY.md file does not already have the CLI doc index in it
    } else {
        // Append the CLI doc to the end of the file
        summary_file.write_all(summary_index.as_bytes())?;
    }

    Ok(())
}

/// Recurse through CLI and generate doc pages for each command
///
/// `index` will have all of the links added to it for the mdbook SUMMARY.md
fn recurse_gen_cli_doc<'a>(
    command: &dyn CliCommand<'a>,
    mut index: &mut String,
    base_path: &Path,
    outpath: &Path,
    depth: usize,
) -> anyhow::Result<()> {
    let mut outpath = outpath.to_owned();
    let cli = command.get_app();

    // If this is the top level app
    if depth == 0 {
        // Add the index record
        index.push_str("\n- [Lucky CLI](./cli/lucky.md)");
        // Set the output path to the `cli` dir
        outpath = outpath.join("cli");
        std::fs::create_dir_all(&outpath)?;
    } else {
        // Add the index record
        index.push_str(
            format!(
                "\n{}- [{}](./{})",
                // Indent the link acording to depth
                "  ".repeat(depth),
                // Link name
                cli.name,
                // File path
                outpath
                    .strip_prefix(base_path)
                    .expect("Target path not subdir of base path")
                    .join(format!("{}.md", cli.name))
                    .to_string_lossy()
            )
            .as_str(),
        );
    }

    // Open markdown file
    let mut outfile = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(outpath.join(format!("{}.md", cli.name)))?;

    let mut content = String::new();
    // If the command has a doc page
    if let Some(doc) = command.get_doc() {
        // Add documentation with substituted command help message
        content.push_str(
            &doc.content
                .replace("${help_message}", &get_app_usage_md(command)),
        );

    // If there is no doc page for the command
    } else {
        // Add command usage documentation
        content.push_str(format!("# {}\n\n", cli.name).as_str());
        content.push_str(&get_app_usage_md(command));
    }

    // Write doc to file
    outfile.write_all(content.as_bytes())?;

    let subcommands = command.get_subcommands();
    // If there are subcommands
    if subcommands.len() != 0 {
        // Create subdirectory for subcommands
        let subdir = outpath.join(cli.name);
        std::fs::create_dir_all(&subdir)?;

        // Create documentation for the subcommands
        for subcommand in command.get_subcommands() {
            recurse_gen_cli_doc(&*subcommand, &mut index, base_path, &subdir, depth + 1)?;
        }
    }

    Ok(())
}

// Get the app usage markdown
fn get_app_usage_md<'a>(command: &dyn CliCommand<'a>) -> String {
    let mut app = command.get_app();
    let mut command_help = String::new();

    // Print command usage
    command_help.push_str(
        format!(
            "## Usage\n\n`{usage}`\n\n",
            usage = app // Here we have to parse out the `USAGE:\n` from `generate_usage()`
                .generate_usage()
                .split("\n")
                .nth(1)
                .expect("Error parsing command usage")
                .trim()
        )
        .as_str(),
    );

    // HTML table head and tail
    let table_head = "<table>\n<thead><tr><th>Arg</th><th>Environment Variable</th><th>Description</th><tr></thead>\n<tbody>\n";
    let table_tail = "</tbody>\n</table>\n\n";

    // Buffers for the arg table contents
    let mut flags_buf = String::new();
    let mut options_buf = String::new();
    let mut positionals_buf = String::new();

    // For each argument
    let arg_map = &app.args;
    for arg in &arg_map.args {
        // Skip help args
        if arg.name == "version" || arg.name == "help" || arg.name == "doc" {
            continue;
        }

        // The argument type ( flag, option, positional )
        let mut arg_type = ArgType::Flag;
        // The argument's table row buffer
        let mut arg_buffer = String::new();
        arg_buffer.push_str("<tr><td>");

        // Add short and long flags if present
        let mut flags = vec![];
        if let Some(short) = arg.short {
            flags.push(format!("<code>-{}</code>", short));
        }
        if let Some(long) = arg.long {
            flags.push(format!("<code>--{}</code>", long));
        }

        // If there are flags
        if flags.len() != 0 {
            // Add comma separated flags
            arg_buffer.push_str(flags.join(", ").as_str());

            // If the arg has a value
            if arg.is_set(ArgSettings::TakesValue) {
                // The arg is an option
                arg_type = ArgType::Opt;
                // Add value names if present
                let value_names = arg
                    .val_names
                    .as_ref()
                    .map(|x| {
                        format!(
                            "<code>&lt;{}&gt;</code>",
                            x.values()
                                .map(|&x| x)
                                .collect::<Vec<&str>>()
                                .join("&gt; &lt;")
                        )
                    })
                    .unwrap_or_else(|| format!("<code>&lt;{}&gt;</code>", arg.name));
                arg_buffer.push_str(&value_names);
            }
            arg_buffer.push_str("</td>");

        // If there are no flags
        } else {
            // Add this positional argument
            arg_type = ArgType::Positional;
            arg_buffer.push_str(format!("<code>&lt;{}&gt;</code></td>", arg.name).as_str());
        }

        // Add the environment variable if any
        if let Some((env, _)) = arg.env {
            arg_buffer
                .push_str(format!("<td><code>{}</code></td>", env.to_string_lossy()).as_str());
        } else {
            arg_buffer.push_str("<td></td>");
        }

        // Add help message
        arg_buffer.push_str(
            format!(
                "<td>{}</td></tr>\n",
                MD_INLINE_CODE.replace_all(
                    arg.long_help.unwrap_or(arg.help.unwrap_or("")),
                    "<code>$text</code>"
                )
            )
            .as_str(),
        );

        // Write argument to the correct arg table
        match arg_type {
            ArgType::Flag => &mut flags_buf,
            ArgType::Opt => &mut options_buf,
            ArgType::Positional => &mut positionals_buf,
        }
        .push_str(&arg_buffer);
    }

    // Write out flags table
    if flags_buf != "" {
        command_help.push_str("### Flags\n\n");
        command_help.push_str(table_head);
        command_help.push_str(&flags_buf);
        command_help.push_str(table_tail);
    }
    // Write out options table
    if options_buf != "" {
        command_help.push_str("### Options\n\n");
        command_help.push_str(table_head);
        command_help.push_str(&options_buf);
        command_help.push_str(table_tail);
    }
    // Write out positionals table
    if positionals_buf != "" {
        command_help.push_str("### Positionals\n\n");
        command_help.push_str(table_head);
        command_help.push_str(&positionals_buf);
        command_help.push_str(table_tail);
    }

    let subcommands = command.get_subcommands();
    // If this command has subcommands
    if subcommands.len() != 0 {
        // Add subcommands header
        command_help.push_str("### Subcommands\n\n");

        // Add subcommand links to list
        for subcommand in subcommands {
            let sub_app = subcommand.get_app();
            command_help.push_str(
                format!(
                    "- [{name}](./{parent}/{name}.md): {help}\n",
                    name = subcommand.get_name(),
                    parent = app.name,
                    help = sub_app.long_about.unwrap_or(sub_app.about.unwrap_or(""))
                )
                .as_str(),
            );
        }
    }

    command_help
}

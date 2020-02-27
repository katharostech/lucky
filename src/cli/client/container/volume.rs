use clap::{App, AppSettings, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct VolumeSubcommand;

impl<'a> CliCommand<'a> for VolumeSubcommand {
    fn get_name(&self) -> &'static str {
        "volume"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Configure container volumes")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(RemoveSubcommand),
            Box::new(AddSubcommand),
            Box::new(GetSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client_container_volume",
            content: include_str!("cli_help/volume.md"),
        })
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}

struct AddSubcommand;

impl<'a> CliCommand<'a> for AddSubcommand {
    fn get_name(&self) -> &'static str {
        "add"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Add a volume to the container")
            .long_about("Add a volume to the container.\n\n\
                         NOTE: This command does not exhibit the same behaviour docker does when \
                         mounting an empty volume to an non-empty directory in the container. \
                         See the doc page for more info."
            )
            .arg(Arg::with_name("source")
                .help("The source for the volume.")
                .long_help("The source for the volume: either a name for a named volume or an \
                            absolute path on the host"
                ))
            .arg(Arg::with_name("target")
                .help("The absolute path in the container to mount `source` to")
                .value_name("container_path"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let source = args
            .value_of("source")
            .expect("Missing required argument: source");
        let target = args
            .value_of("target")
            .expect("Missing required argument: target");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        client
            .container_volume_add(source.into(), target.into(), container.map(Into::into))
            .call()?;

        Ok(data)
    }
}

struct RemoveSubcommand;

impl<'a> CliCommand<'a> for RemoveSubcommand {
    fn get_name(&self) -> &'static str {
        "remove"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Remove a volume from the container")
            .arg(Arg::with_name("target")
                .help("The absolute path in the container of the volume you want to remove.")
                .value_name("container_path"))
            .arg(Arg::with_name("delete_data")
                .help("Delete the actual backing data for the volume")
                .short('D')
                .long("delete-data"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let target = args
            .value_of("target")
            .expect("Missing required argument: target");
        let delete_data = args.is_present("delete_data");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set script status
        client
            .container_volume_remove(target.into(), delete_data, container.map(Into::into))
            .call()?;

        Ok(data)
    }
}

struct GetSubcommand;

impl<'a> CliCommand<'a> for GetSubcommand {
    fn get_name(&self) -> &'static str {
        "get"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get the list of volumes or the source for specific volume")
            .unset_setting(AppSettings::ArgRequiredElseHelp)
            .long_about(concat!(
                "If `target` is provided and the volume exists, the source of the volume will be ",
                "printed. If target is not provided, a list of volumes in the format ",
                "source:target will be printed."
            ))
            .arg(Arg::with_name("target")
                .help("The absolute path, in the container, to the volume you want to get")
                .value_name("container_path")
                .required(false))
            .arg(Arg::with_name("container")
                .help("The name of the container to delete the environment variable for")
                .short('c')
                .long("container")
                .takes_value(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let target = args.value_of("target");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let volumes = client
            .container_volume_get_all(container.map(Into::into))
            .call()?
            .volumes;

        // If target is specified
        if let Some(target) = target {
            // Print the source associated with the target
            if let Some(volume) = volumes.iter().find(|v| v.target == target) {
                writeln!(std::io::stdout(), "{}", volume.source)?;
            }

        // If target is not specified
        } else {
            // Print all of the volumes
            for volume in volumes {
                writeln!(std::io::stdout(), "{}:{}", volume.source, volume.target)?;
            }
        }

        Ok(data)
    }
}

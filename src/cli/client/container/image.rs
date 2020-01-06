use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct ImageSubcommand;

impl<'a> CliCommand<'a> for ImageSubcommand {
    fn get_name(&self) -> &'static str {
        "image"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get and set the Docker image")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![Box::new(GetSubcommand), Box::new(SetSubcommand)]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
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
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Get the container image")
            .arg(Arg::with_name("name")
                .help("The name of the container to get the image of if not the default")
                .short('n')
                .long("name"))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let name = args.value_of("name");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Get the image for the specified container
        let response = client.container_image_get(name.map(Into::into)).call()?;

        // Write response
        writeln!(
            std::io::stdout(),
            "{}",
            response.image.unwrap_or_else(|| "".into())
        )?;

        Ok(data)
    }
}

struct SetSubcommand;

impl<'a> CliCommand<'a> for SetSubcommand {
    fn get_name(&self) -> &'static str {
        "set"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set the container image")
            .arg(Arg::with_name("image")
                .help("The container image")
                .required(true))
            .arg(Arg::with_name("name")
                .help("The name of the container to set the image for if not the default")
                .short('n')
                .long("name"))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let image = args
            .value_of("image")
            .expect("Missing required argument `image`");
        let name = args.value_of("name");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set the image for the specified container
        client
            .container_image_set(image.into(), name.map(Into::into))
            .call()?;

        Ok(data)
    }
}

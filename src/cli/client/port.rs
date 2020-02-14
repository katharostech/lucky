use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct PortSubcommand;

impl<'a> CliCommand<'a> for PortSubcommand {
    fn get_name(&self) -> &'static str {
        "port"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Open and close ports on the firewall")
            .long_about(concat!(
                "Open and close ports on the firewall. These only take effect when the charm is ",
                "\"exposed\" through the Juju GUI or CLI."
            ))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(OpenSubcommand),
            Box::new(CloseSubcommand),
            Box::new(GetOpenedSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}

struct OpenSubcommand;

impl<'a> CliCommand<'a> for OpenSubcommand {
    fn get_name(&self) -> &'static str {
        "open"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Open a port on the firewall")
            .long_about(concat!(
                "Open a port on the firewall. A port range may be specified with an optional ",
                "protocol as well ( TCP is default ). Example values: `8000-9000/udp`, `9000`."
            ))
            .arg(Arg::with_name("port")
                .help("The port to open")
                .required(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let port = args
            .value_of("port")
            .expect("Missing required argument: port");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Open the port
        client.port_open(port.into()).call()?;

        Ok(data)
    }
}

struct CloseSubcommand;

impl<'a> CliCommand<'a> for CloseSubcommand {
    fn get_name(&self) -> &'static str {
        "close"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Close a port on the firewall")
            .long_about(concat!(
                "Close a port on the firewall. A port range may be specified with an optional ",
                "protocol as well ( TCP is default ). Example values: `8000-9000/udp`, `9000`."
            ))
            .arg(Arg::with_name("port")
                .help("The port to close")
                .required_unless("all"))
            .arg(Arg::with_name("all")
                .help("Remove all port rules")
                .long("all")
                .short('A'))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let close_all = args.is_present("all");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        if close_all {
            // Close all ports
            client.port_close_all().call()?;
        } else {
            let port = args
                .value_of("port")
                .expect("Missing required argument: port");

            // Close the port
            client.port_close(port.into()).call()?;
        }

        Ok(data)
    }
}

struct GetOpenedSubcommand;

impl<'a> CliCommand<'a> for GetOpenedSubcommand {
    fn get_name(&self) -> &'static str {
        "get-opened"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Get the ports opened by this charm")
            .long_about(concat!(
                "Get the ports opened by this charm. This will return a line for each port or ",
                "port range that has been opened in the format: `port-or-range/protocol`. For ",
                "exmaple:\n\n8000/tcp\n10000-11000/udp"
            ))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        for port in client.port_get_opened().call()?.ports {
            writeln!(std::io::stdout(), "{}", port)?;
        }

        Ok(data)
    }
}

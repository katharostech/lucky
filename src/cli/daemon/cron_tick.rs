use anyhow::Context;
use clap::{App, ArgMatches};

use crate::cli::daemon::{get_daemon_client, get_daemon_connection_args, get_daemon_socket_path};
use crate::cli::*;
use crate::rpc::VarlinkClientInterface;

pub(super) struct CronTickSubcommand;

impl<'a> CliCommand<'a> for CronTickSubcommand {
    fn get_name(&self) -> &'static str {
        "cron-tick"
    }

    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Tick the cron scheduler and run pending cron jobs")
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .args(&get_daemon_connection_args())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        let socket_path = get_daemon_socket_path(args);

        let juju_context_id = std::env::var("JUJU_CONTEXT_ID").context(concat!(
            "JUJU_CONTEXT_ID environment var must be present. Maybe you need to run this ",
            "command using `juju-run`?"
        ))?;

        // Connect to lucky daemon
        let mut client = get_daemon_client(&socket_path)?;

        log::info!("Triggering cron schedule tick");

        // Just trigger the hook and exit
        client.cron_tick(juju_context_id).call()?;

        log::info!("Done with any pending cron jobs");

        Ok(data)
    }
}

use anyhow::Context;
use clap::{App, Arg, ArgMatches};
use subprocess::{Exec, Redirection};

use std::io::Read;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::sync_channel,
    Arc,
};

use crate::cli::daemon::{
    can_connect_daemon, get_daemon_connection_args, get_daemon_socket_path, try_connect_daemon,
};
use crate::cli::*;
use crate::config;

pub(crate) struct StartSubcommand;

impl<'a> CliCommand<'a> for StartSubcommand {
    fn get_name(&self) -> &'static str {
        "start"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
                .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Start the Lucky daemon")
            .arg(Arg::with_name("ignore_already_running")
                .long("ignore-already-running")
                .short('i')
                .help("Don't complain if the daemon is already running"))
            .arg(Arg::with_name("foreground")
                .long("foreground")
                .short('F')
                .help("Run in the foreground"))
            .arg(Arg::with_name("state_dir")
                .long("state-dir")
                .short('S')
                .takes_value(true)
                .help("The directory to store the unit state in")
                .long_help(concat!(
                    "The directory to store the unit state in. If this is left unspecified the ",
                    "state directory will be automatically determined from the unit name. For ",
                    "example, for a unit named `mysql/2`, the state dir will be ",
                    "`/var/lib/lucky/mysql_2_state`"
                ))
                .env("LUCKY_STATE_DIR"))
            .args(&get_daemon_connection_args())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches) -> anyhow::Result<()> {
        let unit_name = args
            .value_of("unit_name")
            .expect("Missing required arg: unit_name");

        let socket_path = get_daemon_socket_path(args);

        let listen_address = format!("unix:{};mode=700", socket_path);

        // Make sure a daemon is not already running
        if can_connect_daemon(&listen_address) {
            if args.is_present("ignore_already_running") {
                return Err(CliError::Exit(0).into());
            } else {
                anyhow::bail!("Daemon is already running");
            }
        }

        // Get the state dir
        let state_dir = args
            .value_of("state_dir")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(format!(
                    "/var/lib/lucky/{}_state",
                    unit_name.replace("/", "_")
                ))
            });

        // If we are running in the forground
        if args.is_present("foreground") {
            log::info!("Starting daemon in foreground");

            // The stop_listening flag is used to shutdown the server by setting it to `false`
            let stop_listening = Arc::new(AtomicBool::new(false));

            // Create state dir
            if !state_dir.exists() {
                std::fs::create_dir_all(&state_dir)
                    .context(format!("Could not create unit state dir: {:?}", state_dir))?;
            }

            // Get charm dir and lucky metadata
            let charm_dir = config::get_charm_dir()?;
            let lucky_metadata = config::load_yaml(&charm_dir, "lucky")?;

            log::trace!("loaded lucky.yml: {:#?}", lucky_metadata);

            // Get daemon service
            let service = crate::daemon::get_service(
                lucky_metadata,
                charm_dir,
                state_dir,
                stop_listening.clone(),
            );

            // Set signal handler for SIGINT/SIGTERM
            let stop = stop_listening.clone();
            ctrlc::set_handler(move || {
                log::info!("Shutting down server");
                stop.store(true, Ordering::Relaxed);
            })
            .context("Error setting signal handler for SIGINT/SIGTERM")?;

            // Start varlink server in its own thread
            let (sender, reciever) = sync_channel(0);
            let thread = std::thread::spawn(move || {
                let result = varlink::listen(
                    service,
                    &listen_address,
                    &varlink::ListenConfig {
                        max_worker_threads: num_cpus::get(),
                        stop_listening: Some(stop_listening.clone()),
                        ..Default::default()
                    },
                );

                sender
                    .send(result)
                    .expect("Could not send result over thread");
            });
            // Get the server start resut and wait for the thread to exit
            reciever
                .recv()
                .expect("Could not recieve result from thread")?;
            thread.join().expect("Could not join to thread");

        // If we should start in background
        } else {
            log::info!("Starting the lucky daemon");

            // Spawn another process for running the daemon in the background
            let mut output: Box<dyn Read> = Exec::cmd(std::env::current_exe()?)
                .args(&[
                    "daemon",
                    "start",
                    "--socket-path",
                    &socket_path,
                    "--unit-name",
                    &unit_name,
                    "--state-dir",
                    &state_dir.to_string_lossy(),
                    "-F",
                ])
                .stdout(Redirection::Pipe)
                .stderr(Redirection::Merge)
                .detached()
                .stream_stdout()
                .context("Could not start lucky daemon")?;

            // Make sure we can connect to the daemon
            try_connect_daemon(&socket_path)
                .and_then(|_| {
                    log::info!("Daemon started");
                    Ok(())
                })
                // If we can't connect to the daemon
                .or_else(move |_| {
                    let mut out = String::new();
                    output.read_to_string(&mut out).unwrap_or_else(|_| {
                        out = "Could not read daemon logs".into();
                        0
                    });
                    Err(anyhow::anyhow!(format!(
                        "Could not connect to daemon. Dameon logs:\n----\n{}",
                        out
                    )))
                })?;
        }

        Ok(())
    }
}

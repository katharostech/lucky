use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::sync_channel,
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::cli::daemon::{
    can_connect_daemon, get_daemon_connection_args, get_daemon_socket_path, try_connect_daemon,
};
use crate::cli::*;
use crate::config;
use crate::daemon::LuckyDaemonOptions;
use crate::log::{set_log_mode, LogMode::Daemon};
use crate::types::LuckyMetadata;

pub(super) struct StartSubcommand;

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
            .arg(Arg::with_name("data_dir")
                .long("data-dir")
                .short('S')
                .takes_value(true)
                .help("The directory to store the unit data in")
                .long_help(concat!(
                    "The directory to store the unit data in. If this is left unspecified the ",
                    "data directory will be automatically determined from the unit name. For ",
                    "example, for a unit named `mysql/2`, the data dir will be ",
                    "`/var/lib/lucky/mysql_2/`"
                ))
                .env("LUCKY_DATA_DIR"))
            .arg(Arg::with_name("log_file")
                .long("log-file")
                .short('L')
                .takes_value(true)
                .help("File to write daemon logs to")
                .env("LUCKY_LOG_FILE"))
            .args(&get_daemon_connection_args())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        if let Some(log_file) = args.value_of("log_file") {
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_file)
                .context(format!("Could not open log file: {:?}", log_file))?;

            crate::log::set_log_file(file);
        }

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

        // Get the data dir
        let data_dir = args.value_of("data_dir").map_or_else(
            || PathBuf::from(format!("/var/lib/lucky/{}", unit_name.replace("/", "_"))),
            PathBuf::from,
        );

        // If we are running in the forground
        if args.is_present("foreground") {
            // Set log mode to daemon
            set_log_mode(Daemon);
            log::info!("Starting daemon in foreground");

            // The stop_listening flag is used to shutdown the server by setting it to `false`
            let stop_listening = Arc::new(AtomicBool::new(false));

            // Create data dir
            if !data_dir.exists() {
                std::fs::create_dir_all(&data_dir)
                    .context(format!("Could not create unit data dir: {:?}", data_dir))?;
            }

            // Get charm dir and lucky metadata
            let charm_dir = config::get_charm_dir()?;
            let lucky_metadata: LuckyMetadata = config::load_yaml(&charm_dir, "lucky")?;

            // Collect cron schedules ( for scheduling cron tick )
            let cron_schedules: Vec<cron::Schedule> = lucky_metadata
                .cron_jobs
                .keys()
                .map(|x| x.parse())
                .collect::<Result<_, _>>()
                .map_err(|e| format_err!("Could not parse cron job: {}", e))?;

            log::trace!("loaded lucky.yml: {:#?}", lucky_metadata);

            // Get daemon service
            let service = crate::daemon::get_service(LuckyDaemonOptions {
                lucky_metadata,
                charm_dir,
                data_dir,
                stop_listening: stop_listening.clone(),
                socket_path: PathBuf::from(&socket_path),
            });

            // Set signal handler for SIGINT/SIGTERM
            let stop = stop_listening.clone();
            ctrlc::set_handler(move || {
                log::info!("Shutting down server");
                stop.store(true, Ordering::Relaxed);
            })
            .context("Error setting signal handler for SIGINT/SIGTERM")?;

            // Start varlink server in its own thread
            let stop_listening_ = stop_listening.clone();
            let (server_sender, server_receiver) = sync_channel(0);
            let server_thread = thread::spawn(move || {
                let result = varlink::listen(
                    service,
                    &listen_address,
                    &varlink::ListenConfig {
                        // We only need one thread because Juju only allows one charm context at a
                        // time anyway.
                        max_worker_threads: 1,
                        stop_listening: Some(stop_listening_),
                        ..Default::default()
                    },
                );

                server_sender
                    .send(result)
                    .expect("Could not send result over thread");
            });

            // Start the cron tick thread
            let unit_name_ = unit_name.to_string();
            let cron_thread = thread::Builder::new()
                .name("cron-tick".into())
                .spawn(move || cron_tick(&unit_name_, cron_schedules.as_slice(), &stop_listening))
                .context("Could not spawn cron-tick thread")?;

            // Get the server thread result
            server_receiver
                .recv()
                .expect("Could not recieve result from thread")?;

            // Wait on the server thread and the cron tick thread
            server_thread
                .join()
                .expect("Could not join to server thread");
            cron_thread.join().expect("Could not join to cron thread");

        // If we should start in background
        } else {
            log::info!("Starting the lucky daemon");

            let data_dir = data_dir.to_string_lossy();

            // Create the daemon process to run in the background
            let exe = std::env::current_exe()?;
            let mut cmd = Command::new(exe);
            cmd.stdout(Stdio::null())
                .stderr(Stdio::null())
                .env("LUCKY_CONTEXT", "daemon")
                .env("LUCKY_DATA_DIR", &*data_dir)
                .env("JUJU_UNIT_NAME", &unit_name)
                .env("LUCKY_DAEMON_SOCKET", &socket_path)
                .args(&["start", "-F"]);
            if let Some(log_file) = args.value_of("log_file") {
                cmd.env("LUCKY_LOG_FILE", log_file);
            }

            // Spawn process and stream output
            cmd.spawn().context("Could not start lucky daemon")?;

            // Make sure we can connect to the daemon
            try_connect_daemon(&socket_path)
                .and_then(|_| {
                    log::info!("Daemon started");
                    Ok(())
                })
                // If we can't connect to the daemon
                .or_else(move |_| {
                    Err(anyhow::format_err!(format!(
                        "Could not connect to daemon. {}",
                        {
                            // If there was a log file specified, print out the log
                            if let Some(log_file) = args.value_of("log_file") {
                                std::fs::read_to_string(log_file)
                                    .context(format!("Could not read log file: {}", log_file))?

                            // Otherwise suggest running with a log file
                            } else {
                                "Specify a log file when starting to see the daemon output.".into()
                            }
                        }
                    )))
                })?;
        }

        Ok(data)
    }
}

fn cron_tick(unit_name: &str, cron_schedules: &[cron::Schedule], stop: &Arc<AtomicBool>) {
    // Lucky exe path
    let lucky_exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            // TODO: Maybe the whole program should bomb out if cron can't start?
            log::error!(
                "Could not get current executable path, cron jobs will not run: {}",
                e
            );
            return;
        }
    };

    // Run the cron tick loop
    loop {
        // Exit loop if we are done
        if stop.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        // Make sure don't already have a Juju context ( i.e. we are in the middle )
        // of running a hook. If we do have a context, wait a second and try again.
        if std::env::var("JUJU_CONTEXT_ID").is_ok() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        // Use Juju run to create a Juju context and run the `lucky cron-tick`
        if let Err(e) = crate::process::run_cmd(
            "juju-run",
            &[
                &unit_name,
                &format!(
                    "LUCKY_CONTEXT=daemon {} {}",
                    &lucky_exe.as_os_str().to_string_lossy(),
                    "cron-tick"
                ),
            ],
        ) {
            log::error!("Error running cron-tick process: {:?}", e);
        }

        // The next cron job time
        let mut next_time = None;
        // Find closest next cron job time
        for schedule in cron_schedules {
            // If this schedule has an upcomming date
            if let Some(time) = schedule.upcoming(chrono::Local).next() {
                // If we already have a next_time
                if let Some(nt) = next_time {
                    // If this time is before the next time
                    if time < nt {
                        // Update next nearest time
                        next_time = Some(time);
                    }
                // If we don't have a next time
                } else {
                    // Set this upcomming time to the next time
                    next_time = Some(time);
                }
            }
        }

        // If we found a next job time
        if let Some(time) = next_time {
            // Get the time between now and the next job
            let sleep_duration = match (time - chrono::Local::now()).to_std() {
                Ok(duration) => duration,
                Err(e) => {
                    log::error!("Could not convert duration: {}", e);
                    continue;
                }
            };
            // Sleep until the time for the next job
            thread::sleep(sleep_duration);

        // If there are no more upcomming scheduled jobs
        } else {
            // Break out of the loop, we're done
            break;
        }
    }
}

use crossterm::style::{style, Color};
use log::{Level, LevelFilter, Metadata, Record};

use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::process::Command;

/// The Lucky logging implementation
///
/// This logger uses different output styles for the CLI and for the daemon. Also the daemon log
/// level can be independently controlled from the CLI by using the `LUCKY_DAEMON_LOG_LEVEL` and
/// `LUCKY_CLI_LOG_LEVEL` environment variables. The `LUCKY_LOG_LEVEL` environment variable can be
/// used to set a global default log level.
pub struct LuckyLogger;

impl log::Log for LuckyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Filter based on specific log level environment variables
        match metadata.target() {
            // Daemon logs
            target if target.starts_with("lucky::daemon") => {
                if let Ok(level) = std::env::var("LUCKY_DAEMON_LOG_LEVEL") {
                    metadata.level() <= level.parse().unwrap_or(LevelFilter::Trace)
                } else {
                    // Modified by the `LUCKY_LOG_LEVEL` env var
                    metadata.level() <= log::max_level()
                }
            }
            // CLI logs ( the default )
            _ => {
                if let Ok(level) = std::env::var("LUCKY_CLI_LOG_LEVEL") {
                    metadata.level() <= level.parse().unwrap_or(LevelFilter::Trace)
                } else {
                    // Modified by the `LUCKY_LOG_LEVEL` env var
                    metadata.level() <= log::max_level()
                }
            }
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let buffer_error = "Could not write to internal string buffer";
        match record.target() {
            // Daemon logs
            target if target.starts_with("lucky::daemon") => {
                let mut message = String::new();

                // Write module path if available
                if let Some(path) = record.module_path() {
                    write!(message, "[{}]", path).expect(buffer_error);
                }

                // Write log level
                write!(message, "[{}]", record.level()).expect(buffer_error);

                // Write file and line for trace messages
                if record.level() == Level::Trace
                    && record.file().is_some()
                    && record.line().is_some()
                {
                    write!(
                        message,
                        "[{}:{}]",
                        record.file().unwrap(),
                        record.line().unwrap()
                    )
                    .expect(buffer_error);
                }

                // Write message
                write!(message, ": {}", record.args()).expect(buffer_error);

                log_stderr(&message);
                log_juju(&message, record.level() >= LevelFilter::Debug);
            }
            // Cli Logs ( the default )
            _ => {
                // Format message
                let mut message = String::new();
                match record.level() {
                    // Print errors with newline and red `Error:` prefix
                    Level::Error => {
                        write!(
                            message,
                            "{} {}",
                            style("Error:").with(Color::Red),
                            record.args()
                        )
                        .expect(buffer_error);
                    }
                    // Print warnings with yellow `Warning:` prefix
                    Level::Warn => {
                        write!(
                            message,
                            "{} {}",
                            style("Warning:").with(Color::Yellow),
                            record.args()
                        )
                        .expect(buffer_error);
                    }
                    // Print info without decoration ( might want to change that, needs thought )
                    Level::Info => {
                        write!(message, "{}", record.args()).expect(buffer_error);
                    }
                    // Print debug with plain `Debug:` prefix
                    Level::Debug => {
                        write!(
                            message,
                            "{} {}",
                            style("Debug:").with(Color::DarkBlue),
                            record.args()
                        )
                        .expect(buffer_error);
                    }
                    // Print trace with grey `Trace:` prefix
                    Level::Trace => {
                        // Add `Trace:`
                        write!(message, "{}", style("Trace").with(Color::DarkGrey),)
                            .expect(buffer_error);

                        // Add source and line
                        if record.file().is_some() && record.line().is_some() {
                            write!(
                                message,
                                "{}",
                                style(format!(
                                    "[{}:{}]",
                                    record.file().unwrap(),
                                    record.line().unwrap()
                                ))
                                .with(Color::DarkGrey),
                            )
                            .expect(buffer_error);
                        }

                        // Add message
                        write!(
                            message,
                            "{} {}",
                            style(":").with(Color::DarkGrey),
                            record.args()
                        )
                        .expect(buffer_error);
                    }
                }

                // Log to stderr
                log_stderr(&message);
            }
        }
    }

    fn flush(&self) {}
}

/// Write a message out to stderr. Problems writing out are ignored.
fn log_stderr(message: &str) {
    writeln!(std::io::stderr(), "{}", message).ok();
}

/// Write out a message to the Juju Log. Setting `debug` to `true` will tell Juju the log is a
/// debug log.
///
/// If `juju-log` is not in the path, this function will silently ignore it.
///
/// If there is a problem while running `juju-log` the error will be printed to stderr.
fn log_juju(message: &str, debug: bool) {
    let mut cmd = Command::new("juju-log");
    if debug {
        cmd.arg("--debug");
    }
    cmd.arg(&message);

    if let Err(e) = cmd.spawn() {
        // Ignore it if juju-log isn't in the path
        if let std::io::ErrorKind::NotFound = e.kind() {
        }
        // Otherwise print a warning that we couldn't log
        else {
            writeln!(
                std::io::stderr(),
                "[WARN]: Could not log to juju-log: {}",
                e
            )
            .ok();
        }
    }
}

static LUCKY_LOGGER: LuckyLogger = LuckyLogger;

/// Initialize the logger and set the max log level from the `LUCKY_LOG_LEVEL` environment variable
pub(crate) fn init_logger() {
    match log::set_logger(&LUCKY_LOGGER) {
        Ok(()) => {
            if let Ok(level) = std::env::var("LUCKY_LOG_LEVEL") {
                log::set_max_level(level.parse().unwrap_or(log::LevelFilter::Debug));
            } else {
                log::set_max_level(log::LevelFilter::Info);
            }
        }
        Err(e) => panic!("Could not set logger: {}", e),
    }
}

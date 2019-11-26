use crossterm::style::{style, Color};
use log::{Level, LevelFilter, Metadata, Record};

use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::process::Command;
use std::sync::{Arc, RwLock};

/// The Lucky logging implementation
///
/// With this logger, the target is important as it determines how the output is formatted.
///
/// Currently the only respected target is "daemon" with all other logging being treated as "client"
/// logging. In this context, "client" logging refers to any logging output through the CLI and
/// meant primarily to be consumed by users on the command-line. "daemon" logging refers to output
/// that goes to the Juju log and possibly, in the future, log files.
pub struct LuckyLogger;

impl log::Log for LuckyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Max level is set by the LUCKY_LOG_LEVEL environment variable
        metadata.level() <= log::max_level() && {
            // Filter based on specific log level environment variables
            match metadata.target() {
                // Daemon logs
                "daemon" => {
                    if let Ok(level) = std::env::var("LUCKY_DAEMON_LOG_LEVEL") {
                        metadata.level() <= level.parse().unwrap_or(LevelFilter::Trace)
                    } else {
                        true
                    }
                }
                // Client logs, which is what we consider the default log target
                _ => {
                    if let Ok(level) = std::env::var("LUCKY_CLIENT_LOG_LEVEL") {
                        metadata.level() <= level.parse().unwrap_or(LevelFilter::Trace)
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match record.target() {
            "daemon" => {
                let mut message = String::new();

                // Write module path if available
                if let Some(path) = record.module_path() {
                    write!(message, "[{}]", path)
                        .expect("Could not write to internal string buffer");
                }

                // Write log level
                write!(message, "[{}]", record.level())
                    .expect("Could not write to internal string buffer");

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
                    .expect("Could not write to internal string buffer");
                }

                // Write message
                write!(message, ": {}", record.args())
                    .expect("Could not write to internal string buffer");

                log_stderr(&message);
                log_juju(&message, record.level() >= LevelFilter::Debug);
            }
            // Default to "client" style logging ( see doc comment for function )
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
                        .expect("Could not write to internal string buffer");
                    }
                    // Print warnings with yellow `Warning:` prefix
                    Level::Warn => {
                        write!(
                            message,
                            "{} {}",
                            style("Warning:").with(Color::Yellow),
                            record.args()
                        )
                        .expect("Could not write to internal string buffer");
                    }
                    // Print info without decoration ( might want to change that, needs thought )
                    Level::Info => {
                        write!(message, "{}", record.args())
                            .expect("Could not write to internal string buffer");
                    }
                    // Print debug with plain `Debug:` prefix
                    Level::Debug => {
                        write!(
                            message,
                            "{} {}",
                            style("Debug:").with(Color::DarkBlue),
                            record.args()
                        )
                        .expect("Could not write to internal string buffer");
                    }
                    // Print trace with grey `Trace:` prefix
                    Level::Trace => {
                        write!(
                            message,
                            "{} {}",
                            style("Trace:").with(Color::DarkGrey),
                            record.args()
                        )
                        .expect("Could not write to internal string buffer");
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
        match e.kind() {
            // Ignore it if juju-log isn't in the path
            std::io::ErrorKind::NotFound => (),
            // Otherwise print a warning that we couldn't log
            _ => {
                writeln!(
                    std::io::stderr(),
                    "[WARN]: Could not log to juju-log: {}",
                    e
                )
                .ok();
            }
        }
    }
}

static LUCKY_LOGGER: LuckyLogger = LuckyLogger;

/// Initialize the logger and set the max log level from the LUCKY_LOG_LEVEL environment variable
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

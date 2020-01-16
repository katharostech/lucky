//! Contains the lucky logging implementation
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Metadata, Record};

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

#[cfg(feature = "daemon")]
use crate::juju::juju_log;

/// The Lucky logging implementation
///
/// This logger uses different output styles in the CLI and Daemon logging modes. The default mode
/// is CLI, but the mode can be changed with `set_log_mode`.
///
/// The daemon log level can be independently controlled from the CLI log level by using the
/// `LUCKY_DAEMON_LOG_LEVEL` and `LUCKY_CLI_LOG_LEVEL` environment variables. The `LUCKY_LOG_LEVEL`
/// environment variable can be used to set a global default log level.
pub(crate) struct LuckyLogger {
    log_mode: Arc<RwLock<LogMode>>,
    log_file: Arc<RwLock<Option<File>>>,
}

impl LuckyLogger {
    fn new() -> Self {
        LuckyLogger {
            log_mode: Arc::new(RwLock::new(LogMode::Cli)),
            log_file: Arc::new(RwLock::new(None)),
        }
    }

    fn set_log_mode(&self, mode: LogMode) {
        let mut log_mode = self.log_mode.write().unwrap();

        *log_mode = mode;
    }

    fn set_log_file(&self, file: File) {
        let mut log_file = self.log_file.write().unwrap();

        *log_file = Some(file);
    }
}

/// The logging output mode to use
pub(crate) enum LogMode {
    /// The CLI logging mode
    Cli,
    /// The Daemon logging mode
    Daemon,
}

impl log::Log for LuckyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let log_mode = self.log_mode.read().unwrap();

        // Only log messages from Lucky itself, not dependent libraries.
        // Might want to change this later.
        if !metadata.target().starts_with("lucky::") {
            return false;
        }

        // Filter based on specific log level environment variables
        match *log_mode {
            // Daemon logs
            LogMode::Daemon => {
                if let Ok(level) = std::env::var("LUCKY_DAEMON_LOG_LEVEL") {
                    metadata.level() <= level.parse().unwrap_or(LevelFilter::Trace)
                } else {
                    // Modified by the `LUCKY_LOG_LEVEL` env var
                    metadata.level() <= log::max_level()
                }
            }
            // CLI logs
            LogMode::Cli => {
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
        let log_mode = self.log_mode.read().unwrap();
        match *log_mode {
            // Daemon logs
            LogMode::Daemon => {
                let mut message = String::new();

                // Write module path if available
                if let Some(path) = record.module_path() {
                    write!(message, "[{}]", path).expect(buffer_error);
                }

                // Write log level
                write!(message, "[{}]", record.level()).expect(buffer_error);

                // Write file and line for trace messages
                if (record.level() == Level::Trace || record.level() == Level::Debug)
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
                // Log to stderr
                log_to_stderr(&message);
                // Log to file
                if let Some(file) = &mut *self.log_file.write().unwrap() {
                    log_to_file(&message, file)
                }
                // Log to juju
                #[cfg(feature = "daemon")]
                juju_log(&message, record.level() >= LevelFilter::Debug);
            }
            // Cli Logs
            LogMode::Cli => {
                // Format message
                let mut message = String::new();
                match record.level() {
                    // Print errors with newline and red `Error:` prefix
                    Level::Error => {
                        write!(message, "{} {}", red("Error:"), record.args()).expect(buffer_error);
                    }
                    // Print warnings with yellow `Warning:` prefix
                    Level::Warn => {
                        write!(message, "{} {}", yellow("Warning:"), record.args())
                            .expect(buffer_error);
                    }
                    // Print info without decoration ( might want to change that, needs thought )
                    Level::Info => {
                        write!(message, "{}", record.args()).expect(buffer_error);
                    }
                    // Print debug with dark blue `Debug:` prefix
                    Level::Debug => {
                        // Add `Debug`
                        write!(message, "{}", dark_blue("Debug")).expect(buffer_error);

                        // Add source and line
                        if record.file().is_some() && record.line().is_some() {
                            write!(
                                message,
                                "{}",
                                dark_blue(&format!(
                                    "[{}:{}]",
                                    record.file().unwrap(),
                                    record.line().unwrap()
                                )),
                            )
                            .expect(buffer_error);
                        }

                        // Add message
                        write!(message, "{} {}", dark_blue(":"), record.args())
                            .expect(buffer_error);
                    }
                    // Print trace with grey `Trace:` prefix
                    Level::Trace => {
                        // Add `Trace:`
                        write!(message, "{}", dark_grey("Trace")).expect(buffer_error);

                        // Add source and line
                        if record.file().is_some() && record.line().is_some() {
                            write!(
                                message,
                                "{}",
                                dark_grey(&format!(
                                    "[{}:{}]",
                                    record.file().unwrap(),
                                    record.line().unwrap()
                                )),
                            )
                            .expect(buffer_error);
                        }

                        // Add message
                        write!(message, "{} {}", dark_grey(":"), record.args())
                            .expect(buffer_error);
                    }
                }

                // Log to stderr
                log_to_stderr(&message);
                // Log to file
                if let Some(file) = &mut *self.log_file.write().unwrap() {
                    log_to_file(&message, file)
                }
            }
        }
    }

    fn flush(&self) {}
}

/// Write a message out to stderr. Problems writing out are ignored.
fn log_to_stderr(message: &str) {
    writeln!(std::io::stderr(), "{}", message).ok();
}

/// Write a message out to log file. Problems writing out are ignored.
fn log_to_file(message: &str, file: &mut File) {
    writeln!(file, "{}", message).ok();
}

lazy_static! {
    static ref LUCKY_LOGGER: LuckyLogger = LuckyLogger::new();
}

/// Initialize the logger and set the max log level from the `LUCKY_LOG_LEVEL` environment variable
pub(crate) fn init_logger() {
    match log::set_logger(&*LUCKY_LOGGER) {
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

/// Set the logging mode for Lucky
pub(crate) fn set_log_mode(mode: LogMode) {
    LUCKY_LOGGER.set_log_mode(mode);
}

/// Set the logging mode for Lucky
pub(crate) fn set_log_file(file: File) {
    LUCKY_LOGGER.set_log_file(file);
}

//
// Color helpers
//
// These functions add color to the output if stderr is a tty
//

use atty::Stream::Stderr;
use crossterm::style::{style, Color};

fn red(s: &str) -> String {
    if atty::is(Stderr) {
        style(s).with(Color::Red).to_string()
    } else {
        s.to_string()
    }
}

fn yellow(s: &str) -> String {
    if atty::is(Stderr) {
        style(s).with(Color::Yellow).to_string()
    } else {
        s.to_string()
    }
}

fn dark_blue(s: &str) -> String {
    if atty::is(Stderr) {
        style(s).with(Color::DarkBlue).to_string()
    } else {
        s.to_string()
    }
}

fn dark_grey(s: &str) -> String {
    if atty::is(Stderr) {
        style(s).with(Color::DarkGrey).to_string()
    } else {
        s.to_string()
    }
}

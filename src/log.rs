use log::{Record, Level, Metadata};

/// Logger used for the Lucky daemon
pub(crate) struct DaemonLogger;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("[lucky_daemon][{}]: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
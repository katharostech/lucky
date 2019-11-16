use log::{Level, Metadata, Record};

/// Logger used for the Lucky daemon
pub(crate) struct DaemonLogger;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "[{}][{}]: {}",
                record.target(),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

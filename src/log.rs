use log::{Level, Metadata, Record};
use std::io::Write;
use std::process::Command;

/// Logger used for the Lucky daemon
pub(crate) struct DaemonLogger;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Log to Juju
            let mut cmd = Command::new("juju-log");
            if record.level() == Level::Debug {
                cmd.arg("--debug");
            }
            cmd.arg(format!(
                "[{}][{}]: {}",
                record.target(),
                record.level(),
                record.args()
            ));

            cmd.spawn()
                .map_err(|e| {
                    match e.kind() {
                        // Ignore it if juju-log isn't in the path
                        std::io::ErrorKind::NotFound => (),
                        _ => {
                            writeln!(
                                std::io::stderr(),
                                "[lucky::log][WARN]: Could not log to juju-log: {}",
                                e
                            )
                            .ok();
                            ()
                        }
                    }
                })
                .ok();

            // Log to standard out
            writeln!(
                std::io::stderr(),
                "[{}][{}]: {}",
                record.target(),
                record.level(),
                record.args()
            )
            .ok();
        }
    }

    fn flush(&self) {}
}

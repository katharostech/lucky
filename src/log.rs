use log::{Level, Metadata, Record};
use std::io::Write;
use std::process::Command;

/// Logger used for the Lucky daemon
pub(crate) struct DaemonLogger;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Log to Juju
            let mut cmd = Command::new("juju-log");
            if record.level() <= Level::Debug {
                cmd.arg("--debug");
            }
            cmd.arg(format!(
                "[{}]: {}",
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
                                "[WARN]: Could not log to juju-log: {}",
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
                "[{}]: {}",
                record.level(),
                record.args()
            )
            .ok();
        }
    }

    fn flush(&self) {}
}

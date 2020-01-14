//! Macros used throughout Lucky

// TODO: These should maybe be moved to a `lucky_macros` crate

/// Set the daemon status for the current function
///
/// This macro uses the function path to set the `script_id` of the status so that it is local to
/// the function.
///
/// This require that the function it is used in is annotated with `#[function_name::named]` from
/// the `function_name` crate.
///
/// # Example
///
/// ```
/// #[function_name::named]
/// fn do_something_for_daemon(daemon: &LuckyDaemon) {
///     daemon_set_status!(daemon, ScriptState::Maintenance, "Doing something");
///     // Do stuff
///     daemon_set_status!(daemon, ScriptState::Active);
/// }
macro_rules! daemon_set_status {
    ($daemon:expr, $state:expr) => {
        $daemon.set_script_status(
            concat!("__", module_path!(), "::", function_name!(), "__"),
            ScriptStatus {
                state: $state,
                message: None,
            },
        )?;
    };
    ($daemon:expr, $state:expr, $message:expr) => {
        $daemon.set_script_status(
            concat!("__", module_path!(), "::", function_name!(), "__"),
            ScriptStatus {
                state: $state,
                message: Some($message.into()),
            },
        )?;
    };
}

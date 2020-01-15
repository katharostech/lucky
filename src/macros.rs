//! Macros used throughout Lucky

// TODO: These should maybe be moved to a `lucky_macros` crate

/// Set the daemon status for the current function
///
/// This macro uses the function path to set the `script_id` of the status so that it is local to
/// the function.
///
/// This require that the function calling the macro is annotated with `#[function_name::named]`
/// from the `function_name` crate.
///
/// # Example
///
/// ```
/// #[function_name::named]
/// fn do_something_for_daemon(daemon: &LuckyDaemon) {
///     // Get a lock of the daemon state
///     let state = daemon.state.write().unwrap();
///
///     // Set the status
///     daemon_set_status!(&mut state, ScriptState::Maintenance, "Doing something");
///
///     // Do stuff
///
///     // Clear the status
///     daemon_set_status!(&mut state, ScriptState::Active);
/// }
macro_rules! daemon_set_status {
    ($daemon_state:expr, $script_state:expr) => {
        crate::daemon::tools::set_script_status(
            $daemon_state,
            concat!("__", module_path!(), "::", function_name!(), "__"),
            ScriptStatus {
                state: $script_state,
                message: None,
            },
        )?;
    };
    ($daemon_state:expr, $script_state:expr, $message:expr) => {
        crate::daemon::tools::set_script_status(
            $daemon_state,
            concat!("__", module_path!(), "::", function_name!(), "__"),
            ScriptStatus {
                state: $script_state,
                message: Some($message.into()),
            },
        )?;
    };
}

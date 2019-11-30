use thiserror::Error;

#[derive(Error, Debug)]
/// Lucky CLI error variants
pub(crate) enum CliError {
    #[error("Process exiting with code: {0}")]
    /// Indicates that the process should exit with the given code
    Exit(i32),
}

//! Error utilities

#[allow(clippy::needless_pass_by_value)]
/// Removes the "caused by" formatting of a varlink rpc error so that we are not dumping the debug
/// representation of Reply struct in front of our users.
pub(crate) fn map_rpc_err(e: crate::daemon::rpc::Error) -> anyhow::Error {
    anyhow::anyhow!("{}", e.0)
}

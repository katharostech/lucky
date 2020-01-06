//! Async runtime helpers

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::prelude::Future;
use tokio::runtime::Runtime;

lazy_static! {
    /// The Tokio runtime used to execute async code
    static ref RT: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(
        Runtime::new().expect("Could not start tokio runtime")
    ));
}

/// Run a future with the tokio executor
pub(crate) fn block_on<F, R, E>(future: F) -> Result<R, E>
where
    F: Send + 'static + Future<Item = R, Error = E>,
    R: Send + 'static,
    E: Send + 'static,
{
    let mut rt = RT.lock().unwrap();
    rt.block_on(future)
}

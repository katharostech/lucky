//! The Lucky CLI
//!
//! Lucky is a charm writing framework for [Juju](https://jaas.ai).

// Set compiler warning settings
#![warn(missing_docs)]
#![warn(future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::use_self)]
#![allow(clippy::too_many_lines)]

pub mod cli;
pub(crate) mod config;
pub(crate) mod daemon;
pub(crate) mod docker;
pub(crate) mod juju;
pub(crate) mod log;
pub(crate) mod process;
pub(crate) mod types;

use git_version::git_version;
// TODO: Not working right with drone builds to use the tag as the version
const GIT_VERSION: &str = git_version!();

//
// Async Helpers
//

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::prelude::Future;
use tokio::runtime::Runtime;
lazy_static! {
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

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

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
lazy_static! {
    static ref RT: Arc<Mutex<Runtime>> = Arc::new(Mutex::new(
        Runtime::new().expect("Could not start tokio runtime")
    ));
}

use git_version::git_version;
// TODO: Not working right with drone to use the tag as the version
const GIT_VERSION: &str = git_version!();

pub mod cli;
pub(crate) mod config;
pub(crate) mod daemon;
pub(crate) mod docker;
pub(crate) mod juju;
pub(crate) mod log;
pub(crate) mod process;
pub(crate) mod types;

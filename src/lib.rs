//! The Lucky CLI
//!
//! Lucky is a charm writing framework for [Juju](https://jaas.ai).

#![warn(missing_docs)]
#![warn(future_incompatible)]

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

pub mod cli;
pub(crate) mod config;
pub(crate) mod daemon;
pub(crate) mod juju;
pub(crate) mod log;
pub(crate) mod types;

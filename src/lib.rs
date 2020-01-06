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
pub(crate) mod log;
pub(crate) mod rpc;
pub(crate) mod types;

// Daemon only modules
#[cfg(feature = "daemon")]
pub(crate) mod daemon;
#[cfg(feature = "daemon")]
pub(crate) mod docker;
#[cfg(feature = "daemon")]
pub(crate) mod juju;
#[cfg(feature = "daemon")]
pub(crate) mod process;
#[cfg(feature = "daemon")]
pub(crate) mod rt;

use git_version::git_version;
// TODO: Not working right with drone builds to use the tag as the version
const GIT_VERSION: &str = git_version!();

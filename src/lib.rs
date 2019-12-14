//! The Lucky CLI
//!
//! Lucky is a charm writing framework for [Juju](https://jaas.ai).

// Set compiler warning settings
#![warn(missing_docs)]
#![warn(future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::use_self)]

use git_version::git_version;
// TODO: Not working right with drone to use the tag as the version
const GIT_VERSION: &str = git_version!();

pub mod cli;
pub(crate) mod config;
pub(crate) mod daemon;
pub(crate) mod docker;
pub(crate) mod juju;
pub(crate) mod log;
pub(crate) mod types;

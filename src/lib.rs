//! The Lucky CLI
//!
//! Lucky is a charm writing framework for [Juju](https://jaas.ai).

#![warn(missing_docs)]
#![warn(future_incompatible)]

pub mod cli;
pub(crate) mod daemon;
pub(crate) mod juju;
pub(crate) mod log;
pub(crate) mod types;

//! The Lucky CLI
//!
//! Lucky is a charm writing framework for [Juju](https://jaas.ai).

#![warn(missing_docs)]
#![warn(future_incompatible)]

pub mod cli;
pub(crate) mod daemon;
pub(crate) mod types;

pub(crate) const CHARM_TEMPLATE_ARCHIVE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/charm_template.zip"));

// Not used yet. Commenting to squelch warning
// pub(crate) const CHARM_HOOK_TEMPLATE: &[u8] = include_bytes!("../charm_hooks/hook-template.hbs");

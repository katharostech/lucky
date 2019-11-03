// TODO: enable #![warn(missing_docs)]
#![warn(future_incompatible)]

pub mod cli;

pub const CHARM_TEMPLATE_ARCHIVE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/charm_template.zip"));

pub const CHARM_HOOK_TEMPLATE: &[u8] = include_bytes!("../charm_hooks/hook-template.hbs");

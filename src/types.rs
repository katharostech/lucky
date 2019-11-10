//! Various types that are used throughout the application

use serde::Deserialize;
use std::collections::HashMap;

/// The list of the normal Juju hook names
pub(crate) const JUJU_NORMAL_HOOKS: &[&str] = &[
    "install",
    "config-changed",
    "leader-elected",
    "leader-settings-changed",
    "start",
    "stop",
    "upgrade-charm",
    "update-status",
    "collect-metrics",
];

#[allow(dead_code)]
/// The list of the Juju relation hooks with the `{}` where the relation name should be
pub(crate) const JUJU_RELATION_HOOKS: &[&str] = &[
    "{}-relation-joined",
    "{}-relation-changed",
    "{}-relation-departed",
    "{}-relation-broken",
];

#[allow(dead_code)]
/// The list of the Juju storage hooks with the `{}` where the storage name should be
pub(crate) const JUJU_STORAGE_HOOKS: &[&str] = &[
    "{}-storage-attached",
    "{}-storage-detaching",
];

/// The charm metadata as defined in a charm's `metadata.yaml` file
#[derive(Deserialize, Debug)]
pub(crate) struct CharmMetadata {
    pub name: String,
    #[serde(rename = "display-name")]
    pub display_name: String,
    pub summary: String,
    pub maintainer: String,
    pub description: String,
    pub tags: Vec<String>,
    pub subordinate: bool,
    pub provides: HashMap<String, RelationDef>,
    pub requires: HashMap<String, RelationDef>,
    pub peers: HashMap<String, RelationDef>,
}

/// The definition of a relation in the `metadata.yaml` file
#[derive(Deserialize, Debug)]
pub(crate) struct RelationDef {
    pub interface: String,
}

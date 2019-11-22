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
pub(crate) const JUJU_STORAGE_HOOKS: &[&str] = &["{}-storage-attached", "{}-storage-detaching"];

/// The charm metadata as defined in a charm's `metadata.yaml` file
#[derive(Deserialize, Debug)]
pub(crate) struct CharmMetadata {
    pub name: String,
    pub summary: Option<String>,
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub maintainer: Option<String>,
    pub maintainers: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub series: Option<String>,
    pub subordinate: Option<bool>,
    pub terms: Option<Vec<String>>,
    pub provides: Option<HashMap<String, RelationDef>>,
    pub requires: Option<HashMap<String, RelationDef>>,
    pub peers: Option<HashMap<String, RelationDef>>,
    pub storage: Option<HashMap<String, StorageDef>>,
    // TODO: Resources, payloads, and extra bindings
}

/// The definition of a relation in the `metadata.yaml` file
#[derive(Deserialize, Debug)]
pub(crate) struct RelationDef {
    pub interface: String,
}

/// See Juju Docs: https://discourse.jujucharms.com/t/writing-charms-that-use-storage/1128
#[derive(Deserialize, Debug)]
pub(crate) struct StorageDef {
    #[serde(rename = "type")]
    pub storage_type: StorageType,
    pub description: Option<String>,
    pub shared: Option<bool>,
    #[serde(rename = "read-only")]
    pub read_only: Option<bool>,
    #[serde(rename = "minimum-size")]
    pub minimum_size: Option<String>,
    pub location: Option<String>,
    pub multiple: Option<StorageMultiple>,
}

#[derive(Deserialize, Debug)]
pub(crate) enum StorageType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "block")]
    Block,
}

#[derive(Deserialize, Debug)]
pub(crate) struct StorageMultiple {
    pub range: String,
}

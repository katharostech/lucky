use serde::{Deserialize, Serialize};
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CharmMetadata {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provides: Option<HashMap<String, RelationDef>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<HashMap<String, RelationDef>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<HashMap<String, RelationDef>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<HashMap<String, StorageDef>>,
    // TODO: Resources, payloads, and extra bindings
}

/// The definition of a relation in the `metadata.yaml` file
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RelationDef {
    pub interface: String,
}

/// See [Juju Docs](https://discourse.jujucharms.com/t/writing-charms-that-use-storage/1128)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct StorageDef {
    #[serde(rename = "type")]
    pub storage_type: StorageType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<StorageMultiple>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum StorageType {
    Filesystem,
    Block,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StorageMultiple {
    pub range: String,
}

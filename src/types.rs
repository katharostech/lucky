//! Various types that are used throughout the application

use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
pub(crate) struct RelationDef {
    pub interface: String,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LuckyMetadata {
    pub hooks: HashMap<String, Vec<HookScript>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum HookScript {
    #[serde(rename = "host-script")]
    HostScript(String),
    #[serde(rename = "container-script")]
    ContainerScript(String),
}

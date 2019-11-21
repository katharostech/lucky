use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Debug, AsRefStr, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
/// A Lucky script state
pub(crate) enum ScriptState {
    /// The script is currently working on getting the service running
    Maintenance,
    /// The script cannot continue without extra user input
    Blocked,
    /// There is no error, but the script is wainting on some external resource before it can continue
    Waiting,
    /// The script is ready and providing the service
    Active,
}

#[derive(Debug)]
/// Encapsulates the scripts state and an optional message
pub(crate) struct ScriptStatus {
    pub state: ScriptState,
    pub message: Option<String>,
}

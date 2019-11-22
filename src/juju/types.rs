use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::daemon::rpc::ScriptStatus as RpcScriptStatus;
use crate::daemon::rpc::ScriptStatus_state as RpcScriptState;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, AsRefStr, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
/// A Lucky script state
///
/// The order of the variants are important in this case because the order of definition defines
/// the order of precendence. For example, the Juju status will only be `Active` if all of the
/// scripts are active. Any one script being a status of a higher precendence than all of the others
/// will cause the Juju status to be set to that status. This functionality is implemented in
/// `crate::daemon::LuckyDaemon::get_juju_status`.
pub(crate) enum ScriptState {
    /// The script is ready and providing the service
    Active,
    /// There is no error, but the script is wainting on some external resource before it can continue
    Waiting,
    /// The script is currently working on getting the service running
    Maintenance,
    /// The script cannot continue without extra user input
    Blocked,
}

impl Default for ScriptState {
    fn default() -> Self {
        Self::Active
    }
}

// Implement `from` and `into` for the RPC version of this enum
impl From<RpcScriptState> for ScriptState {
    fn from(state: RpcScriptState) -> Self {
        match state {
            RpcScriptState::Maintenance => Self::Maintenance,
            RpcScriptState::Blocked => Self::Blocked,
            RpcScriptState::Waiting => Self::Waiting,
            RpcScriptState::Active => Self::Active,
        }
    }
}
impl Into<RpcScriptState> for ScriptState {
    fn into(self) -> RpcScriptState {
        match self {
            Self::Maintenance => RpcScriptState::Maintenance,
            Self::Blocked => RpcScriptState::Blocked,
            Self::Waiting => RpcScriptState::Waiting,
            Self::Active => RpcScriptState::Active,
        }
    }
}

#[derive(Clone, Debug, Default)]
/// Encapsulates the scripts state and an optional message
pub(crate) struct ScriptStatus {
    pub state: ScriptState,
    pub message: Option<String>,
}

impl std::fmt::Display for ScriptStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}: {}", self.state.as_ref(), message)
        } else {
            write!(f, "{}", self.state.as_ref())
        }
        
    }
}

// Implement `from` and `into` for the RPC version of this struct
impl From<RpcScriptStatus> for ScriptStatus {
    fn from(status: RpcScriptStatus) -> Self {
        ScriptStatus {
            state: status.state.into(),
            message: status.message,
        }
    }
}
impl Into<RpcScriptStatus> for ScriptStatus {
    fn into(self) -> RpcScriptStatus {
        RpcScriptStatus {
            state: self.state.into(),
            message: self.message,
        }
    }
}

use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::daemon::rpc::ScriptStatus as RpcScriptStatus;
use crate::daemon::rpc::ScriptStatus_state as RpcScriptState;

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

#[derive(Debug)]
/// Encapsulates the scripts state and an optional message
pub(crate) struct ScriptStatus {
    pub state: ScriptState,
    pub message: Option<String>,
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

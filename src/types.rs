//! Types specific to Lucky that are used throughout the app

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::rpc::ScriptStatus as RpcScriptStatus;
use crate::rpc::ScriptStatus_state as RpcScriptState;

/// The prefix for the exit code in the exit code helper command
///
/// See `lucky::cli::daemon::exit_code_helper`.
pub(crate) const LUCKY_EXIT_CODE_HELPER_PREFIX: &str = "__LUCKY_CMD_EXIT_CODE__:";

/// Juju related types
pub(crate) mod juju;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    AsRefStr,
    EnumString,
    EnumVariantNames,
    Serialize,
    Deserialize,
)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

//
// Config files
//

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
/// The struct definition for the `lucky.yaml` file
pub(crate) struct LuckyMetadata {
    #[serde(default = "default_true")]
    /// Specifies whether or not to install Docker on the host and enable Docker-based features
    pub use_docker: bool,
    /// The hooks for the charm
    #[serde(default)]
    pub hooks: HashMap<String, Vec<CharmScript>>,
    /// The cron jobs for the charm
    pub cron_jobs: HashMap<String, Vec<CharmScript>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
/// A hook script in the `lucky.yaml` definition
pub(crate) enum CharmScript {
    /// A script that runs on the host from the `host_scripts` dir
    #[serde(rename_all = "kebab-case")]
    Host {
        host_script: String,
        #[serde(default = "Vec::new")]
        args: Vec<String>,
    },
    /// A script that runs on the host as inline bash
    #[serde(rename_all = "kebab-case")]
    InlineHost {
        inline_host_script: String,
        #[serde(default = "default_shell")]
        shell_command: Vec<String>,
    },
    /// A script that runs in the container from the `container_scripts` dir
    #[serde(rename_all = "kebab-case")]
    Container {
        container_script: String,
        container_name: Option<String>,
        #[serde(default = "Vec::new")]
        args: Vec<String>,
        /// This instructs Lucky to ignore the script if the container is not running yet
        #[serde(default = "default_false")]
        ignore_missing_container: bool,
    },
    /// A script that runs in the container as inline bash
    #[serde(rename_all = "kebab-case")]
    InlineContainer {
        inline_container_script: String,
        container_name: Option<String>,
        #[serde(default = "default_shell")]
        shell_command: Vec<String>,
        /// This instructs Lucky to ignore the script if the container is not running yet
        #[serde(default = "default_false")]
        ignore_missing_container: bool,
    },
}

//
// Helpers
//

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_shell() -> Vec<String> {
    vec!["/bin/bash".into(), "-c".into()]
}

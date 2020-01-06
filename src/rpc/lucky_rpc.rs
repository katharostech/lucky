#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Varlink_Error,
    VarlinkReply_Error,
    Error(Option<Error_Args>),
    RequiresMore(Option<RequiresMore_Args>),
}
include!("lucky_rpc_err_impl.rs");
pub struct Error(
    pub ErrorKind,
    pub Option<Box<dyn std::error::Error + 'static + Send + Sync>>,
    pub Option<&'static str>,
);
impl Error {
    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}
impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error(e, None, None)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.1
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error as StdError;
        if let Some(ref o) = self.2 {
            std::fmt::Display::fmt(o, f)?;
        }
        std::fmt::Debug::fmt(&self.0, f)?;
        if let Some(e) = self.source() {
            std::fmt::Display::fmt("\nCaused by:\n", f)?;
            std::fmt::Debug::fmt(&e, f)?;
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;
impl From<varlink::Error> for Error {
    fn from(e: varlink::Error) -> Self {
        match e.kind() {
            varlink::ErrorKind::VarlinkErrorReply(r) => Error(
                ErrorKind::from(r),
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
            _ => Error(
                ErrorKind::Varlink_Error,
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
        }
    }
}
#[allow(dead_code)]
impl Error {
    pub fn source_varlink_kind(&self) -> Option<&varlink::ErrorKind> {
        use std::error::Error as StdError;
        let mut s: &dyn StdError = self;
        while let Some(c) = s.source() {
            let k = self
                .source()
                .and_then(|e| e.downcast_ref::<varlink::Error>())
                .and_then(|e| Some(e.kind()));
            if k.is_some() {
                return k;
            }
            s = c;
        }
        None
    }
}
impl From<&varlink::Reply> for ErrorKind {
    #[allow(unused_variables)]
    fn from(e: &varlink::Reply) -> Self {
        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "lucky.rpc.Error" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p.clone()) {
                    Ok(v) => ErrorKind::Error(v),
                    Err(_) => ErrorKind::Error(None),
                },
                _ => ErrorKind::Error(None),
            },
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "lucky.rpc.RequiresMore" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p.clone()) {
                    Ok(v) => ErrorKind::RequiresMore(v),
                    Err(_) => ErrorKind::RequiresMore(None),
                },
                _ => ErrorKind::RequiresMore(None),
            },
            _ => ErrorKind::VarlinkReply_Error,
        }
    }
}
pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_error(&mut self, r#message: String) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "lucky.rpc.Error",
            Some(serde_json::to_value(Error_Args { r#message }).map_err(varlink::map_context!())?),
        ))
    }
    fn reply_requires_more(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error("lucky.rpc.RequiresMore", None))
    }
}
impl<'a> VarlinkCallError for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum r#ScriptStatus_state {
    r#Maintenance,
    r#Blocked,
    r#Waiting,
    r#Active,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#ScriptStatus {
    pub r#state: ScriptStatus_state,
    pub r#message: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Error_Args {
    pub r#message: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequiresMore_Args {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerApply_Reply {}
impl varlink::VarlinkReply for ContainerApply_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerApply_Args {}
pub trait Call_ContainerApply: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_ContainerApply for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerImageGet_Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#image: Option<String>,
}
impl varlink::VarlinkReply for ContainerImageGet_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerImageGet_Args {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#container_name: Option<String>,
}
pub trait Call_ContainerImageGet: VarlinkCallError {
    fn reply(&mut self, r#image: Option<String>) -> varlink::Result<()> {
        self.reply_struct(ContainerImageGet_Reply { r#image }.into())
    }
}
impl<'a> Call_ContainerImageGet for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerImageSet_Reply {}
impl varlink::VarlinkReply for ContainerImageSet_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContainerImageSet_Args {
    pub r#image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#container_name: Option<String>,
}
pub trait Call_ContainerImageSet: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_ContainerImageSet for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SetStatus_Reply {}
impl varlink::VarlinkReply for SetStatus_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SetStatus_Args {
    pub r#script_id: String,
    pub r#status: ScriptStatus,
}
pub trait Call_SetStatus: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_SetStatus for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopDaemon_Reply {}
impl varlink::VarlinkReply for StopDaemon_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopDaemon_Args {}
pub trait Call_StopDaemon: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_StopDaemon for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TriggerHook_Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#output: Option<String>,
}
impl varlink::VarlinkReply for TriggerHook_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TriggerHook_Args {
    pub r#hook_name: String,
    pub r#environment: varlink::StringHashMap<String>,
}
pub trait Call_TriggerHook: VarlinkCallError {
    fn reply(&mut self, r#output: Option<String>) -> varlink::Result<()> {
        self.reply_struct(TriggerHook_Reply { r#output }.into())
    }
}
impl<'a> Call_TriggerHook for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvGet_Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#value: Option<String>,
}
impl varlink::VarlinkReply for UnitKvGet_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvGet_Args {
    pub r#key: String,
}
pub trait Call_UnitKvGet: VarlinkCallError {
    fn reply(&mut self, r#value: Option<String>) -> varlink::Result<()> {
        self.reply_struct(UnitKvGet_Reply { r#value }.into())
    }
}
impl<'a> Call_UnitKvGet for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvGetAll_Reply {
    pub r#key: String,
    pub r#value: String,
}
impl varlink::VarlinkReply for UnitKvGetAll_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvGetAll_Args {}
pub trait Call_UnitKvGetAll: VarlinkCallError {
    fn reply(&mut self, r#key: String, r#value: String) -> varlink::Result<()> {
        self.reply_struct(UnitKvGetAll_Reply { r#key, r#value }.into())
    }
}
impl<'a> Call_UnitKvGetAll for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvSet_Reply {}
impl varlink::VarlinkReply for UnitKvSet_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitKvSet_Args {
    pub r#key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#value: Option<String>,
}
pub trait Call_UnitKvSet: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_UnitKvSet for varlink::Call<'a> {}
pub trait VarlinkInterface {
    fn container_apply(&self, call: &mut dyn Call_ContainerApply) -> varlink::Result<()>;
    fn container_image_get(
        &self,
        call: &mut dyn Call_ContainerImageGet,
        r#container_name: Option<String>,
    ) -> varlink::Result<()>;
    fn container_image_set(
        &self,
        call: &mut dyn Call_ContainerImageSet,
        r#image: String,
        r#container_name: Option<String>,
    ) -> varlink::Result<()>;
    fn set_status(
        &self,
        call: &mut dyn Call_SetStatus,
        r#script_id: String,
        r#status: ScriptStatus,
    ) -> varlink::Result<()>;
    fn stop_daemon(&self, call: &mut dyn Call_StopDaemon) -> varlink::Result<()>;
    fn trigger_hook(
        &self,
        call: &mut dyn Call_TriggerHook,
        r#hook_name: String,
        r#environment: varlink::StringHashMap<String>,
    ) -> varlink::Result<()>;
    fn unit_kv_get(&self, call: &mut dyn Call_UnitKvGet, r#key: String) -> varlink::Result<()>;
    fn unit_kv_get_all(&self, call: &mut dyn Call_UnitKvGetAll) -> varlink::Result<()>;
    fn unit_kv_set(
        &self,
        call: &mut dyn Call_UnitKvSet,
        r#key: String,
        r#value: Option<String>,
    ) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut dyn BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
pub trait VarlinkClientInterface {
    fn container_apply(
        &mut self,
    ) -> varlink::MethodCall<ContainerApply_Args, ContainerApply_Reply, Error>;
    fn container_image_get(
        &mut self,
        r#container_name: Option<String>,
    ) -> varlink::MethodCall<ContainerImageGet_Args, ContainerImageGet_Reply, Error>;
    fn container_image_set(
        &mut self,
        r#image: String,
        r#container_name: Option<String>,
    ) -> varlink::MethodCall<ContainerImageSet_Args, ContainerImageSet_Reply, Error>;
    fn set_status(
        &mut self,
        r#script_id: String,
        r#status: ScriptStatus,
    ) -> varlink::MethodCall<SetStatus_Args, SetStatus_Reply, Error>;
    fn stop_daemon(&mut self) -> varlink::MethodCall<StopDaemon_Args, StopDaemon_Reply, Error>;
    fn trigger_hook(
        &mut self,
        r#hook_name: String,
        r#environment: varlink::StringHashMap<String>,
    ) -> varlink::MethodCall<TriggerHook_Args, TriggerHook_Reply, Error>;
    fn unit_kv_get(
        &mut self,
        r#key: String,
    ) -> varlink::MethodCall<UnitKvGet_Args, UnitKvGet_Reply, Error>;
    fn unit_kv_get_all(
        &mut self,
    ) -> varlink::MethodCall<UnitKvGetAll_Args, UnitKvGetAll_Reply, Error>;
    fn unit_kv_set(
        &mut self,
        r#key: String,
        r#value: Option<String>,
    ) -> varlink::MethodCall<UnitKvSet_Args, UnitKvSet_Reply, Error>;
}
#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}
impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}
impl VarlinkClientInterface for VarlinkClient {
    fn container_apply(
        &mut self,
    ) -> varlink::MethodCall<ContainerApply_Args, ContainerApply_Reply, Error> {
        varlink::MethodCall::<ContainerApply_Args, ContainerApply_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.ContainerApply",
            ContainerApply_Args {},
        )
    }
    fn container_image_get(
        &mut self,
        r#container_name: Option<String>,
    ) -> varlink::MethodCall<ContainerImageGet_Args, ContainerImageGet_Reply, Error> {
        varlink::MethodCall::<ContainerImageGet_Args, ContainerImageGet_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.ContainerImageGet",
            ContainerImageGet_Args { r#container_name },
        )
    }
    fn container_image_set(
        &mut self,
        r#image: String,
        r#container_name: Option<String>,
    ) -> varlink::MethodCall<ContainerImageSet_Args, ContainerImageSet_Reply, Error> {
        varlink::MethodCall::<ContainerImageSet_Args, ContainerImageSet_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.ContainerImageSet",
            ContainerImageSet_Args {
                r#image,
                r#container_name,
            },
        )
    }
    fn set_status(
        &mut self,
        r#script_id: String,
        r#status: ScriptStatus,
    ) -> varlink::MethodCall<SetStatus_Args, SetStatus_Reply, Error> {
        varlink::MethodCall::<SetStatus_Args, SetStatus_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.SetStatus",
            SetStatus_Args {
                r#script_id,
                r#status,
            },
        )
    }
    fn stop_daemon(&mut self) -> varlink::MethodCall<StopDaemon_Args, StopDaemon_Reply, Error> {
        varlink::MethodCall::<StopDaemon_Args, StopDaemon_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.StopDaemon",
            StopDaemon_Args {},
        )
    }
    fn trigger_hook(
        &mut self,
        r#hook_name: String,
        r#environment: varlink::StringHashMap<String>,
    ) -> varlink::MethodCall<TriggerHook_Args, TriggerHook_Reply, Error> {
        varlink::MethodCall::<TriggerHook_Args, TriggerHook_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.TriggerHook",
            TriggerHook_Args {
                r#hook_name,
                r#environment,
            },
        )
    }
    fn unit_kv_get(
        &mut self,
        r#key: String,
    ) -> varlink::MethodCall<UnitKvGet_Args, UnitKvGet_Reply, Error> {
        varlink::MethodCall::<UnitKvGet_Args, UnitKvGet_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.UnitKvGet",
            UnitKvGet_Args { r#key },
        )
    }
    fn unit_kv_get_all(
        &mut self,
    ) -> varlink::MethodCall<UnitKvGetAll_Args, UnitKvGetAll_Reply, Error> {
        varlink::MethodCall::<UnitKvGetAll_Args, UnitKvGetAll_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.UnitKvGetAll",
            UnitKvGetAll_Args {},
        )
    }
    fn unit_kv_set(
        &mut self,
        r#key: String,
        r#value: Option<String>,
    ) -> varlink::MethodCall<UnitKvSet_Args, UnitKvSet_Reply, Error> {
        varlink::MethodCall::<UnitKvSet_Args, UnitKvSet_Reply, Error>::new(
            self.connection.clone(),
            "lucky.rpc.UnitKvSet",
            UnitKvSet_Args { r#key, r#value },
        )
    }
}
#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<dyn VarlinkInterface + Send + Sync>,
}
#[allow(dead_code)]
pub fn new(inner: Box<dyn VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}
impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "# The Lucky charm frameowrk for Juju.\n#\n# This is the varlink RPC schema definition for the Lucky daemon and client communication\n# protocol.\ninterface lucky.rpc\n\n# General catch-all error type\nerror Error(message: string)\n# Returned when a method must be called with `more`\nerror RequiresMore()\n\n# Trigger a Juju hook\n# \n# If this hook is called with --more it will return once for each line of output from the hook.\n#\n# If hook execution failed this will throw a `HookFailed` error\nmethod TriggerHook(hook_name: String, environment: [string]string)\n    -> (output: ?string)\n\n# Stops the deamon service\nmethod StopDaemon() -> ()\n\n# The status of a Lucky script\ntype ScriptStatus (\n    state: (Maintenance, Blocked, Waiting, Active),\n    message: ?string\n)\n\n# Sets a script's status\nmethod SetStatus(script_id: string, status: ScriptStatus) -> ()\n\n# Get a value in the Unit's local Key-Value store. Value will be null if the key is not set.\nmethod UnitKvGet(key: string) -> (value: ?string)\n\n# Get all of the key-value pairs that have been set. Must be called with --more or it will return\n# a `RequiresMore` error.\nmethod UnitKvGetAll() -> (key: string, value: string)\n\n# Set a value in the Unit's local Key-Value store. Setting `value` to null will erase the value.\nmethod UnitKvSet(key: string, value: ?string) -> ()\n\n# Set a container's image\nmethod ContainerImageSet(image: string, container_name: ?string) -> ()\n# Get a container's image. Image will be none if container doesn't exist.\nmethod ContainerImageGet(container_name: ?string) -> (image: ?string)\n# Apply updates to the container configuration\nmethod ContainerApply() -> ()"
    }
    fn get_name(&self) -> &'static str {
        "lucky.rpc"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut dyn BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "lucky.rpc.ContainerApply" => self
                .inner
                .container_apply(call as &mut dyn Call_ContainerApply),
            "lucky.rpc.ContainerImageGet" => {
                if let Some(args) = req.parameters.clone() {
                    let args: ContainerImageGet_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.container_image_get(
                        call as &mut dyn Call_ContainerImageGet,
                        args.r#container_name,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "lucky.rpc.ContainerImageSet" => {
                if let Some(args) = req.parameters.clone() {
                    let args: ContainerImageSet_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.container_image_set(
                        call as &mut dyn Call_ContainerImageSet,
                        args.r#image,
                        args.r#container_name,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "lucky.rpc.SetStatus" => {
                if let Some(args) = req.parameters.clone() {
                    let args: SetStatus_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.set_status(
                        call as &mut dyn Call_SetStatus,
                        args.r#script_id,
                        args.r#status,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "lucky.rpc.StopDaemon" => self.inner.stop_daemon(call as &mut dyn Call_StopDaemon),
            "lucky.rpc.TriggerHook" => {
                if let Some(args) = req.parameters.clone() {
                    let args: TriggerHook_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.trigger_hook(
                        call as &mut dyn Call_TriggerHook,
                        args.r#hook_name,
                        args.r#environment,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "lucky.rpc.UnitKvGet" => {
                if let Some(args) = req.parameters.clone() {
                    let args: UnitKvGet_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner
                        .unit_kv_get(call as &mut dyn Call_UnitKvGet, args.r#key)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "lucky.rpc.UnitKvGetAll" => self
                .inner
                .unit_kv_get_all(call as &mut dyn Call_UnitKvGetAll),
            "lucky.rpc.UnitKvSet" => {
                if let Some(args) = req.parameters.clone() {
                    let args: UnitKvSet_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.unit_kv_set(
                        call as &mut dyn Call_UnitKvSet,
                        args.r#key,
                        args.r#value,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}

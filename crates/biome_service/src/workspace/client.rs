use super::operation::workspace_operations;
use super::{
    ChangeFileParams, ChangeFileResult, CloseFileParams, DropPatternParams, FileExistsParams,
    FixFileParams, FixFileResult, FormatFileParams, FormatOnTypeParams, FormatRangeParams,
    GetControlFlowGraphParams, GetFormatterIRParams, GetModuleGraphParams, GetModuleGraphResult,
    GetSemanticModelParams, GetSyntaxTreeParams, GetSyntaxTreeResult, GoToDefinitionParams,
    GoToDefinitionResult, OpenFileParams, OpenFileResult, ProcessFileParams, ProcessFileResult,
    PullActionsParams, PullActionsResult, PullDiagnosticsAndActionsParams,
    PullDiagnosticsAndActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RenameParams,
    RenameResult, ScanProjectParams, ScanProjectResult, SearchPatternParams, SearchResults,
    SupportsFeatureParams, UpdateModuleGraphParams, UpdateSettingsParams, UpdateSettingsResult,
};
use crate::workspace::{
    CheckFileSizeParams, CheckFileSizeResult, CloseProjectParams, FileFeaturesResult,
    GetFileContentParams, GetRegisteredTypesParams, GetTypeInfoParams, OpenProjectParams,
    OpenProjectResult, ParsePatternParams, ParsePatternResult, PathIsIgnoredParams, RageParams,
    RageResult, ServerInfo,
};
use crate::{TransportError, Workspace, WorkspaceError};
use biome_formatter::rowan::Printed;
use biome_resolver::FsWithResolverProxy;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};
use tracing::instrument;

pub struct WorkspaceClient<T> {
    transport: T,
    request_id: AtomicU64,
    server_info: Option<ServerInfo>,
    fs: Box<dyn FsWithResolverProxy>,
}

pub trait WorkspaceTransport {
    fn request<P, R>(&self, request: TransportRequest<P>) -> Result<R, TransportError>
    where
        P: Serialize,
        R: DeserializeOwned;
}

#[derive(Debug)]
pub struct TransportRequest<P> {
    pub id: u64,
    pub method: &'static str,
    pub params: P,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// Information about the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
}

impl<T> WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    pub fn new(transport: T, fs: Box<dyn FsWithResolverProxy>) -> Result<Self, WorkspaceError> {
        let mut client = Self {
            transport,
            request_id: AtomicU64::new(0),
            server_info: None,
            fs,
        };

        // TODO: The current implementation of the JSON-RPC protocol in
        // tower_lsp doesn't allow any request to be sent before a call to
        // initialize, this is something we could be able to lift by using our
        // own RPC protocol implementation
        let value: InitializeResult = client.request(
            "initialize",
            json!({
                "capabilities": {},
                "clientInfo": {
                    "name": env!("CARGO_PKG_NAME"),
                    "version": biome_configuration::VERSION
                },
            }),
        )?;

        client.server_info = value.server_info;

        Ok(client)
    }

    /// Sends a request and waits for its response.
    ///
    /// A request that the server cancelled because of a concurrent change to
    /// its workspace is sent again, as
    /// [`RetryingWorkspace`](crate::workspace::RetryingWorkspace) does in
    /// process. The parameters are sent unchanged, so a position or range in a
    /// resent request applies to the content of the document at that time.
    fn request<P, R>(&self, method: &'static str, params: P) -> Result<R, WorkspaceError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        loop {
            match self.request_once(method, &params) {
                Err(WorkspaceError::TransportError(TransportError::ContentModified)) => {
                    thread::sleep(Duration::from_millis(1));
                }
                result => return result,
            }
        }
    }

    /// Sends a request once and waits for its response.
    fn request_once<P, R>(&self, method: &'static str, params: P) -> Result<R, WorkspaceError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = TransportRequest { id, method, params };

        let response = self.transport.request(request)?;

        Ok(response)
    }

    pub fn shutdown(self) -> Result<(), WorkspaceError> {
        self.request("biome/shutdown", ())
    }
}

/// Implements the method of each workspace operation on [WorkspaceClient].
macro_rules! client_workspace_methods {
    ($($(#[$meta:meta])* $name:ident($params:ty) -> $result:ty = $method:literal;)*) => {
        $(client_workspace_method!($name($params) -> $result = $method);)*
    };
}

macro_rules! client_workspace_method {
    (scan_project($params:ty) -> $result:ty = $method:literal) => {
        fn scan_project(&self, params: $params) -> Result<$result, WorkspaceError> {
            // As in `RetryingWorkspace`, a scan isn't sent again: the server runs it
            // in a scanner epoch, which queues setter-based writes until the scan
            // completes instead of cancelling it.
            self.request_once($method, params)
        }
    };
    (update_settings($params:ty) -> $result:ty = $method:literal) => {
        #[instrument(level = "info", skip_all)]
        fn update_settings(&self, params: $params) -> Result<$result, WorkspaceError> {
            self.request($method, params)
        }
    };
    ($name:ident($params:ty) -> $result:ty = $method:literal) => {
        fn $name(&self, params: $params) -> Result<$result, WorkspaceError> {
            self.request($method, params)
        }
    };
}

impl<T> Workspace for WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    workspace_operations!(client_workspace_methods);

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.fs.as_ref()
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }
}

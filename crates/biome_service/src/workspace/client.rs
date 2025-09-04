use super::{
    ChangeFileParams, ChangeFileResult, CloseFileParams, FileExitsParams, FixFileParams,
    FixFileResult, FormatFileParams, FormatOnTypeParams, FormatRangeParams,
    GetControlFlowGraphParams, GetFormatterIRParams, GetModuleGraphParams, GetModuleGraphResult,
    GetSemanticModelParams, GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams,
    OpenFileResult, PullActionsParams, PullActionsResult, PullDiagnosticsParams,
    PullDiagnosticsResult, RenameParams, RenameResult, ScanProjectParams, ScanProjectResult,
    SearchPatternParams, SearchResults, SupportsFeatureParams, UpdateModuleGraphParams,
    UpdateSettingsParams, UpdateSettingsResult,
};
use crate::workspace::{
    CheckFileSizeParams, CheckFileSizeResult, CloseProjectParams, FileFeaturesResult,
    GetFileContentParams, GetRegisteredTypesParams, GetTypeInfoParams, OpenProjectParams,
    OpenProjectResult, PathIsIgnoredParams, RageParams, RageResult, ServerInfo,
};
use crate::{TransportError, Workspace, WorkspaceError};
use biome_formatter::Printed;
use biome_resolver::FsWithResolverProxy;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicU64, Ordering},
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

    fn request<P, R>(&self, method: &'static str, params: P) -> Result<R, WorkspaceError>
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

impl<T> Workspace for WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    fn open_project(&self, params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError> {
        self.request("biome/open_project", params)
    }

    fn scan_project(&self, params: ScanProjectParams) -> Result<ScanProjectResult, WorkspaceError> {
        self.request("biome/scan_project", params)
    }

    #[instrument(level = "info", skip_all)]
    fn update_settings(
        &self,
        params: UpdateSettingsParams,
    ) -> Result<UpdateSettingsResult, WorkspaceError> {
        self.request("biome/update_settings", params)
    }

    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError> {
        self.request("biome/close_project", params)
    }

    fn open_file(&self, params: OpenFileParams) -> Result<OpenFileResult, WorkspaceError> {
        self.request("biome/open_file", params)
    }

    fn file_exists(&self, params: FileExitsParams) -> Result<bool, WorkspaceError> {
        self.request("biome/file_exists", params)
    }

    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        self.request("biome/file_features", params)
    }

    fn is_path_ignored(&self, params: PathIsIgnoredParams) -> Result<bool, WorkspaceError> {
        self.request("biome/is_path_ignored", params)
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        self.request("biome/get_syntax_tree", params)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError> {
        self.request("biome/get_control_flow_graph", params)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        self.request("biome/get_formatter_ir", params)
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        self.request("biome/get_type_info", params)
    }

    fn get_registered_types(
        &self,
        params: GetRegisteredTypesParams,
    ) -> Result<String, WorkspaceError> {
        self.request("biome/get_registered_types", params)
    }

    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError> {
        self.request("biome/get_semantic_model", params)
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.request("biome/get_file_content", params)
    }

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        self.request("biome/check_file_size", params)
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<ChangeFileResult, WorkspaceError> {
        self.request("biome/change_file", params)
    }

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.request("biome/pull_diagnostics", params)
    }

    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        self.request("biome/pull_actions", params)
    }

    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        self.request("biome/format_file", params)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        self.request("biome/format_range", params)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        self.request("biome/format_on_type", params)
    }

    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        self.request("biome/fix_file", params)
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError> {
        self.request("biome/rename", params)
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.request("biome/close_file", params)
    }

    fn update_module_graph(&self, params: UpdateModuleGraphParams) -> Result<(), WorkspaceError> {
        self.request("biome/update_module_graph", params)
    }

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.fs.as_ref()
    }

    fn parse_pattern(
        &self,
        params: super::ParsePatternParams,
    ) -> Result<super::ParsePatternResult, WorkspaceError> {
        self.request("biome/parse_pattern", params)
    }

    fn search_pattern(&self, params: SearchPatternParams) -> Result<SearchResults, WorkspaceError> {
        self.request("biome/search_pattern", params)
    }

    fn drop_pattern(&self, params: super::DropPatternParams) -> Result<(), WorkspaceError> {
        self.request("biome/drop_pattern", params)
    }

    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError> {
        self.request("biome/rage", params)
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    fn get_module_graph(
        &self,
        params: GetModuleGraphParams,
    ) -> Result<GetModuleGraphResult, WorkspaceError> {
        self.request("biome/get_module_graph", params)
    }
}

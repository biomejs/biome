use crate::workspace::{
    CheckFileSizeParams, CheckFileSizeResult, FileFeaturesResult, GetFileContentParams,
    IsPathIgnoredParams, OrganizeImportsParams, OrganizeImportsResult, ProjectKey, RageParams,
    RageResult, RegisterProjectFolderParams, ServerInfo, SetManifestForProjectParams,
    UnregisterProjectFolderParams,
};
use crate::{TransportError, Workspace, WorkspaceError};
use biome_formatter::Printed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicU64, Ordering},
};

use super::{
    ChangeFileParams, CloseFileParams, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameParams, RenameResult, SearchPatternParams,
    SearchResults, SupportsFeatureParams, UpdateSettingsParams,
};

pub struct WorkspaceClient<T> {
    transport: T,
    request_id: AtomicU64,
    server_info: Option<ServerInfo>,
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
    pub fn new(transport: T) -> Result<Self, WorkspaceError> {
        let mut client = Self {
            transport,
            request_id: AtomicU64::new(0),
            server_info: None,
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
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        self.request("biome/file_features", params)
    }
    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        self.request("biome/is_path_ignored", params)
    }
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        self.request("biome/update_settings", params)
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.request("biome/open_file", params)
    }

    fn set_manifest_for_project(
        &self,
        params: SetManifestForProjectParams,
    ) -> Result<(), WorkspaceError> {
        self.request("biome/set_manifest_for_project", params)
    }

    fn register_project_folder(
        &self,
        params: RegisterProjectFolderParams,
    ) -> Result<ProjectKey, WorkspaceError> {
        self.request("biome/register_project_folder", params)
    }

    fn unregister_project_folder(
        &self,
        params: UnregisterProjectFolderParams,
    ) -> Result<(), WorkspaceError> {
        self.request("biome/unregister_project_folder", params)
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

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.request("biome/get_file_content", params)
    }

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        self.request("biome/check_file_size", params)
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError> {
        self.request("biome/change_file", params)
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.request("biome/close_file", params)
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

    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError> {
        self.request("biome/rage", params)
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

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    fn organize_imports(
        &self,
        params: OrganizeImportsParams,
    ) -> Result<OrganizeImportsResult, WorkspaceError> {
        self.request("biome/organize_imports", params)
    }
}

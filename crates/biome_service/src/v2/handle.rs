use crate::scanner::WorkspaceWatcherBridge;
use crate::v2::request::{
    ControlRequest, WorkspaceCall, WorkspaceRequest, WorkspaceResponder, WorkspaceSender,
};
use crate::workspace::{
    ChangeFileParams, ChangeFileResult, CheckFileSizeParams, CheckFileSizeResult, CloseFileParams,
    CloseProjectParams, DropPatternParams, FileExistsParams, FileFeaturesResult, FixFileParams,
    FixFileResult, FormatFileParams, FormatOnTypeParams, FormatRangeParams,
    GetControlFlowGraphParams, GetFileContentParams, GetFormatterIRParams, GetModuleGraphParams,
    GetModuleGraphResult, GetRegisteredTypesParams, GetSemanticModelParams, GetSyntaxTreeParams,
    GetSyntaxTreeResult, GetTypeInfoParams, GoToDefinitionParams, GoToDefinitionResult,
    OpenFileParams, OpenFileResult, OpenProjectParams, OpenProjectResult, ParsePatternParams,
    ParsePatternResult, PathIsIgnoredParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsAndActionsParams, PullDiagnosticsAndActionsResult, PullDiagnosticsParams,
    PullDiagnosticsResult, RageParams, RageResult, RenameParams, RenameResult, ScanProjectParams,
    ScanProjectResult, SearchPatternParams, SearchResults, ServerInfo, SupportsFeatureParams,
    UpdateModuleGraphParams, UpdateSettingsParams, UpdateSettingsResult, Workspace,
};
use crate::{TransportError, WorkspaceError, projects::ProjectKey, scanner::ScanKind};
use biome_diagnostics::serde::Diagnostic;
use biome_formatter::Printed;
use biome_fs::{BiomePath, FileSystem, PathKind};
use biome_resolver::FsWithResolverProxy;
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkspaceHandle {
    sender: WorkspaceSender,
    fs: Arc<dyn FsWithResolverProxy>,
    server_info: Option<ServerInfo>,
}

#[derive(Clone)]
pub struct WorkspaceWatcher {
    sender: WorkspaceSender,
    fs: Arc<dyn FsWithResolverProxy>,
}

impl WorkspaceHandle {
    pub(crate) fn new(
        sender: WorkspaceSender,
        fs: Arc<dyn FsWithResolverProxy>,
        server_info: Option<ServerInfo>,
    ) -> Self {
        Self {
            sender,
            fs,
            server_info,
        }
    }

    fn request<R>(
        &self,
        build: impl FnOnce(WorkspaceResponder<R>) -> WorkspaceCall,
    ) -> Result<R, WorkspaceError> {
        let (tx, rx) = crossbeam::channel::bounded(1);
        let responder = WorkspaceResponder::new(tx);

        self.sender
            .send(WorkspaceRequest::Call {
                call: Box::new(build(responder)),
                sender: self.sender.clone(),
            })
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))?;

        rx.recv()
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))?
    }

    pub fn shutdown(&self) -> Result<(), WorkspaceError> {
        self.sender
            .send(WorkspaceRequest::Control(ControlRequest::Shutdown))
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))
    }
}

impl WorkspaceWatcher {
    pub(crate) fn new(sender: WorkspaceSender, fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self { sender, fs }
    }

    fn request<R>(
        &self,
        build: impl FnOnce(WorkspaceResponder<R>) -> crate::v2::request::WatcherRequest,
    ) -> Result<R, WorkspaceError> {
        let (tx, rx) = crossbeam::channel::bounded(1);
        let responder = WorkspaceResponder::new(tx);

        self.sender
            .send(WorkspaceRequest::Watcher(build(responder)))
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))?;

        rx.recv()
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))?
    }

    pub fn start_watcher(&self, mut watcher: crate::Watcher) {
        watcher.run(self);
    }

    pub fn shutdown(&self) -> Result<(), WorkspaceError> {
        self.sender
            .send(WorkspaceRequest::Control(ControlRequest::Shutdown))
            .map_err(|_| WorkspaceError::from(TransportError::ChannelClosed))
    }
}

impl WorkspaceWatcherBridge for WorkspaceWatcher {
    fn fs(&self) -> &dyn FileSystem {
        self.fs.as_ref()
    }

    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::FindProjectForPath(path.to_path_buf(), response)
        })
        .unwrap_or_default()
    }

    fn find_project_with_scan_kind_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(ProjectKey, ScanKind)> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::FindProjectWithScanKindForPath(
                path.to_path_buf(),
                response,
            )
        })
        .unwrap_or_default()
    }

    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError> {
        self.request(|tx| crate::v2::request::WatcherRequest::IsIgnored {
            project_key,
            scan_kind: scan_kind.clone(),
            path: path.to_path_buf(),
            path_kind,
            tx,
        })
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::IndexFile(project_key, path.into(), response)
        })
    }

    fn index_folder(&self, path: &Utf8Path) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::IndexFolder(path.to_path_buf(), response)
        })
    }

    fn insert_watched_folder(&self, path: Utf8PathBuf) -> bool {
        self.request(|response| {
            crate::v2::request::WatcherRequest::InsertWatchedFolder(path, response)
        })
        .unwrap_or_default()
    }

    fn remove_watched_folders_under(&self, path: &Utf8Path) -> Vec<Utf8PathBuf> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::RemoveWatchedFoldersUnder(
                path.to_path_buf(),
                response,
            )
        })
        .unwrap_or_default()
    }

    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::UnloadFile(
                path.to_path_buf(),
                project_key,
                response,
            )
        })
    }

    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        self.request(|response| {
            crate::v2::request::WatcherRequest::UnloadPath(
                path.to_path_buf(),
                project_key,
                response,
            )
        })
    }

    fn notify_stopped(&self) {
        let _ = self.sender.send(WorkspaceRequest::Watcher(
            crate::v2::request::WatcherRequest::NotifyStopped,
        ));
    }
}

impl Workspace for WorkspaceHandle {
    fn open_project(&self, params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::OpenProject(params, response))
    }

    fn scan_project(&self, params: ScanProjectParams) -> Result<ScanProjectResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::ScanProject(params, response))
    }

    fn update_settings(
        &self,
        params: UpdateSettingsParams,
    ) -> Result<UpdateSettingsResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::UpdateSettings(params, response))
    }

    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError> {
        self.request(|response| WorkspaceCall::CloseProject(params, response))
    }

    fn open_file(&self, params: OpenFileParams) -> Result<OpenFileResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::OpenFile(params, response))
    }

    fn file_exists(&self, params: FileExistsParams) -> Result<bool, WorkspaceError> {
        self.request(|response| WorkspaceCall::FileExists(params, response))
    }

    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::FileFeatures(params, response))
    }

    fn is_path_ignored(&self, params: PathIsIgnoredParams) -> Result<bool, WorkspaceError> {
        self.request(|response| WorkspaceCall::IsPathIgnored(params, response))
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetFileContent(params, response))
    }

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::CheckFileSize(params, response))
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<ChangeFileResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::ChangeFile(params, response))
    }

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::PullDiagnostics(params, response))
    }

    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::PullActions(params, response))
    }

    fn pull_diagnostics_and_actions(
        &self,
        params: PullDiagnosticsAndActionsParams,
    ) -> Result<PullDiagnosticsAndActionsResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::PullDiagnosticsAndActions(params, response))
    }

    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        self.request(|response| WorkspaceCall::FormatFile(params, response))
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        self.request(|response| WorkspaceCall::FormatRange(params, response))
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        self.request(|response| WorkspaceCall::FormatOnType(params, response))
    }

    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::FixFile(params, response))
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::Rename(params, response))
    }

    fn go_to_definition(
        &self,
        params: GoToDefinitionParams,
    ) -> Result<Option<GoToDefinitionResult>, WorkspaceError> {
        self.request(|response| WorkspaceCall::GoToDefinition(params, response))
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.request(|response| WorkspaceCall::CloseFile(params, response))
    }

    fn update_module_graph(&self, params: UpdateModuleGraphParams) -> Result<(), WorkspaceError> {
        self.request(|response| WorkspaceCall::UpdateModuleGraph(params, response))
    }

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.fs.as_ref()
    }

    fn parse_pattern(
        &self,
        params: ParsePatternParams,
    ) -> Result<ParsePatternResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::ParsePattern(params, response))
    }

    fn search_pattern(&self, params: SearchPatternParams) -> Result<SearchResults, WorkspaceError> {
        self.request(|response| WorkspaceCall::SearchPattern(params, response))
    }

    fn drop_pattern(&self, params: DropPatternParams) -> Result<(), WorkspaceError> {
        self.request(|response| WorkspaceCall::DropPattern(params, response))
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetSyntaxTree(params, response))
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetControlFlowGraph(params, response))
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetFormatterIr(params, response))
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetTypeInfo(params, response))
    }

    fn get_registered_types(
        &self,
        params: GetRegisteredTypesParams,
    ) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetRegisteredTypes(params, response))
    }

    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetSemanticModel(params, response))
    }

    fn get_module_graph(
        &self,
        params: GetModuleGraphParams,
    ) -> Result<GetModuleGraphResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::GetModuleGraph(params, response))
    }

    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError> {
        self.request(|response| WorkspaceCall::Rage(params, response))
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::request::{ControlRequest, WatcherRequest};
    use crate::workspace::{FileExistsParams, ServerInfo};
    use biome_fs::MemoryFileSystem;
    use camino::Utf8PathBuf;
    use crossbeam::channel::unbounded;
    use std::time::Duration;

    fn memory_fs() -> Arc<MemoryFileSystem> {
        Arc::new(MemoryFileSystem::default())
    }

    #[test]
    fn workspace_handle_forwards_workspace_requests() {
        let fs = memory_fs();
        let (sender, receiver) = unbounded();
        let handle = WorkspaceHandle::new(
            sender,
            fs,
            Some(ServerInfo {
                name: "test-server".to_string(),
                version: Some("1.0.0".to_string()),
            }),
        );
        let file_path = BiomePath::new("file.js");

        let thread_handle = std::thread::spawn({
            let handle = handle.clone();
            let file_path = file_path.clone();
            move || handle.file_exists(FileExistsParams { file_path })
        });

        let request = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("request should be sent");
        match request {
            WorkspaceRequest::Call { call, .. } => match *call {
                WorkspaceCall::FileExists(params, response) => {
                    assert_eq!(params.file_path, file_path);
                    response.send(Ok(true));
                }
                _ => panic!("unexpected request"),
            },
            _ => panic!("unexpected request"),
        }

        assert!(thread_handle.join().unwrap().unwrap());
        assert_eq!(handle.server_info().unwrap().name, "test-server");
    }

    #[test]
    fn workspace_handle_reports_channel_closed() {
        let fs = memory_fs();
        let (sender, receiver) = unbounded();
        let handle = WorkspaceHandle::new(sender, fs, None);
        drop(receiver);

        let error = handle
            .file_exists(FileExistsParams {
                file_path: BiomePath::new("missing.js"),
            })
            .unwrap_err();

        assert!(matches!(
            error,
            WorkspaceError::TransportError(TransportError::ChannelClosed)
        ));
    }

    #[test]
    fn workspace_handle_forwards_shutdown_control_request() {
        let fs = memory_fs();
        let (sender, receiver) = unbounded();
        let handle = WorkspaceHandle::new(sender, fs, None);

        handle.shutdown().unwrap();

        let request = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("request should be sent");
        assert!(matches!(
            request,
            WorkspaceRequest::Control(ControlRequest::Shutdown)
        ));
    }

    #[test]
    fn workspace_watcher_forwards_shutdown_control_request() {
        let fs = memory_fs();
        let (sender, receiver) = unbounded();
        let watcher = WorkspaceWatcher::new(sender, fs);

        watcher.shutdown().unwrap();

        let request = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("request should be sent");
        assert!(matches!(
            request,
            WorkspaceRequest::Control(ControlRequest::Shutdown)
        ));
    }

    #[test]
    fn workspace_handle_uses_local_filesystem() {
        let fs = memory_fs();
        let file_path = Utf8PathBuf::from("local.js");
        fs.insert(file_path.clone(), "let a = 1;");
        let (sender, _receiver) = unbounded();
        let handle = WorkspaceHandle::new(sender, fs, None);

        assert!(handle.fs().path_exists(&file_path));
    }

    #[test]
    fn workspace_watcher_forwards_watcher_requests() {
        let fs = memory_fs();
        let (sender, receiver) = unbounded();
        let watcher = WorkspaceWatcher::new(sender, fs);
        let folder_path = Utf8PathBuf::from("project");

        let thread_handle = std::thread::spawn({
            let watcher = watcher.clone();
            let folder_path = folder_path.clone();
            move || watcher.remove_watched_folders_under(&folder_path)
        });

        let request = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("request should be sent");
        match request {
            WorkspaceRequest::Watcher(WatcherRequest::RemoveWatchedFoldersUnder(
                path,
                response,
            )) => {
                assert_eq!(path, folder_path);
                response.send(Ok(vec![Utf8PathBuf::from("project/src")]));
            }
            _ => panic!("unexpected request"),
        }

        assert_eq!(
            thread_handle.join().unwrap(),
            vec![Utf8PathBuf::from("project/src")]
        );
    }

    #[test]
    fn workspace_watcher_uses_local_filesystem() {
        let fs = memory_fs();
        let file_path = Utf8PathBuf::from("watched.js");
        fs.insert(file_path.clone(), "let a = 1;");
        let (sender, _receiver) = unbounded();
        let watcher = WorkspaceWatcher::new(sender, fs);

        assert!(watcher.fs().path_exists(&file_path));
    }
}

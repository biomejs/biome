use crate::projects::ProjectKey;
use crate::scanner::ScanKind;
use crate::v2::staged::{FixFileOperation, StagedCommitMessage};
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
    ScanProjectResult, SearchPatternParams, SearchResults, SupportsFeatureParams,
    UpdateModuleGraphParams, UpdateSettingsParams, UpdateSettingsResult,
};
use crate::{TransportError, WorkspaceError};
use biome_formatter::Printed;
use biome_fs::{BiomePath, PathKind};
use camino::Utf8PathBuf;
use crossbeam::channel::Sender;

pub(crate) type WorkspaceSender = Sender<WorkspaceRequest>;

pub(crate) struct WorkspaceResponder<T> {
    tx: Sender<Result<T, WorkspaceError>>,
}

impl<T> WorkspaceResponder<T> {
    pub(crate) fn new(tx: Sender<Result<T, WorkspaceError>>) -> Self {
        Self { tx }
    }

    pub(crate) fn send(self, result: Result<T, WorkspaceError>) {
        let _ = self.tx.send(result);
    }
}

pub(crate) enum WorkspaceRequest {
    Call {
        call: Box<WorkspaceCall>,
        sender: WorkspaceSender,
    },
    Watcher(WatcherRequest),
    Commit(Box<WorkspaceCommit>),
    Control(ControlRequest),
}

pub(crate) enum WorkspaceCommit {
    FixFile(StagedCommitMessage<FixFileOperation>),
}

impl WorkspaceCommit {
    pub(crate) fn send_channel_closed(self) {
        match self {
            Self::FixFile(message) => {
                message.send(Err(WorkspaceError::from(TransportError::ChannelClosed)));
            }
        }
    }
}

pub(crate) enum WorkspaceCall {
    OpenProject(OpenProjectParams, WorkspaceResponder<OpenProjectResult>),
    ScanProject(ScanProjectParams, WorkspaceResponder<ScanProjectResult>),
    UpdateSettings(
        UpdateSettingsParams,
        WorkspaceResponder<UpdateSettingsResult>,
    ),
    CloseProject(CloseProjectParams, WorkspaceResponder<()>),

    OpenFile(OpenFileParams, WorkspaceResponder<OpenFileResult>),
    FileExists(FileExistsParams, WorkspaceResponder<bool>),
    FileFeatures(
        SupportsFeatureParams,
        WorkspaceResponder<FileFeaturesResult>,
    ),
    IsPathIgnored(PathIsIgnoredParams, WorkspaceResponder<bool>),
    GetFileContent(GetFileContentParams, WorkspaceResponder<String>),
    CheckFileSize(CheckFileSizeParams, WorkspaceResponder<CheckFileSizeResult>),
    ChangeFile(ChangeFileParams, WorkspaceResponder<ChangeFileResult>),
    PullDiagnostics(
        PullDiagnosticsParams,
        WorkspaceResponder<PullDiagnosticsResult>,
    ),
    PullActions(PullActionsParams, WorkspaceResponder<PullActionsResult>),
    PullDiagnosticsAndActions(
        PullDiagnosticsAndActionsParams,
        WorkspaceResponder<PullDiagnosticsAndActionsResult>,
    ),
    FormatFile(FormatFileParams, WorkspaceResponder<Printed>),
    FormatRange(FormatRangeParams, WorkspaceResponder<Printed>),
    FormatOnType(FormatOnTypeParams, WorkspaceResponder<Printed>),
    FixFile(FixFileParams, WorkspaceResponder<FixFileResult>),
    Rename(RenameParams, WorkspaceResponder<RenameResult>),
    GoToDefinition(
        GoToDefinitionParams,
        WorkspaceResponder<Option<GoToDefinitionResult>>,
    ),
    CloseFile(CloseFileParams, WorkspaceResponder<()>),
    UpdateModuleGraph(UpdateModuleGraphParams, WorkspaceResponder<()>),

    ParsePattern(ParsePatternParams, WorkspaceResponder<ParsePatternResult>),
    SearchPattern(SearchPatternParams, WorkspaceResponder<SearchResults>),
    DropPattern(DropPatternParams, WorkspaceResponder<()>),

    GetSyntaxTree(GetSyntaxTreeParams, WorkspaceResponder<GetSyntaxTreeResult>),
    GetControlFlowGraph(GetControlFlowGraphParams, WorkspaceResponder<String>),
    GetFormatterIr(GetFormatterIRParams, WorkspaceResponder<String>),
    GetTypeInfo(GetTypeInfoParams, WorkspaceResponder<String>),
    GetRegisteredTypes(GetRegisteredTypesParams, WorkspaceResponder<String>),
    GetSemanticModel(GetSemanticModelParams, WorkspaceResponder<String>),
    GetModuleGraph(
        GetModuleGraphParams,
        WorkspaceResponder<GetModuleGraphResult>,
    ),
    Rage(RageParams, WorkspaceResponder<RageResult>),
}

pub(crate) enum WatcherRequest {
    FindProjectForPath(Utf8PathBuf, WorkspaceResponder<Option<ProjectKey>>),
    FindProjectWithScanKindForPath(
        Utf8PathBuf,
        WorkspaceResponder<Option<(ProjectKey, ScanKind)>>,
    ),
    IsIgnored {
        project_key: ProjectKey,
        scan_kind: ScanKind,
        path: Utf8PathBuf,
        path_kind: Option<PathKind>,
        tx: WorkspaceResponder<bool>,
    },
    IndexFile(
        ProjectKey,
        BiomePath,
        WorkspaceResponder<Vec<biome_diagnostics::serde::Diagnostic>>,
    ),
    IndexFolder(
        Utf8PathBuf,
        WorkspaceResponder<Vec<biome_diagnostics::serde::Diagnostic>>,
    ),
    InsertWatchedFolder(Utf8PathBuf, WorkspaceResponder<bool>),
    RemoveWatchedFoldersUnder(Utf8PathBuf, WorkspaceResponder<Vec<Utf8PathBuf>>),
    UnloadFile(
        Utf8PathBuf,
        ProjectKey,
        WorkspaceResponder<Vec<biome_diagnostics::serde::Diagnostic>>,
    ),
    UnloadPath(
        Utf8PathBuf,
        ProjectKey,
        WorkspaceResponder<Vec<biome_diagnostics::serde::Diagnostic>>,
    ),
    NotifyStopped,
}

pub(crate) enum ControlRequest {
    Shutdown,
}

#[cfg(test)]
mod tests {
    use crate::v2::request::WorkspaceRequest;

    #[test]
    fn size_request() {
        let size = std::mem::size_of::<WorkspaceRequest>();
        assert_eq!(size, 88);
    }
}

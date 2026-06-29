use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerVisitorCache, Capabilities, CodeActionsParams, DiagnosticsAndActionsParams,
    FixAllParams, LintParams, LintResults, ResolveDefinitionParams, UpdateSnippetsNodes,
};
use crate::projects::{GetFileFeaturesParams, ProjectSnapshot};
use crate::settings::{EditorFeatures, SettingsHandle, SettingsWithEditor};
use crate::workspace::{
    DefinitionReference, FileFeaturesResult, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetRegisteredTypesParams, GetSemanticModelParams, GetSyntaxTreeParams, GetSyntaxTreeResult,
    GetTypeInfoParams, GoToDefinitionParams, GoToDefinitionResult, PathIsIgnoredParams, PatternId,
    PullActionsParams, PullActionsResult, PullDiagnosticsAndActionsParams,
    PullDiagnosticsAndActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RenameParams,
    RenameResult, SearchPatternParams, SearchQuery, SearchResults, Settings, SupportsFeatureParams,
};
use biome_analyze::{AnalyzerPluginVec, RuleCategories};
use biome_configuration::Configuration;
use biome_configuration::analyzer::AnalyzerSelector;
use biome_db::{AnyParsedSource, ParsedSnippet, ParsedSource};
use biome_diagnostics::{
    Diagnostic, DiagnosticExt, Severity, serde::Diagnostic as SerdeDiagnostic,
};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_project_layout::ProjectLayout;
use biome_rowan::{SendNode, TextRange, TextSize};
use biome_workspace_db::WorkspaceDb;
use camino::Utf8PathBuf;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

pub(crate) type LintFn = fn(LintParams) -> LintResults;
pub(crate) type CodeActionsFn = fn(CodeActionsParams) -> PullActionsResult;
pub(crate) type FixAllFn = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
pub(crate) type UpdateSnippetsFn =
    fn(AnyParsedSource, WorkspaceDb, Vec<UpdateSnippetsNodes>) -> Result<SendNode, WorkspaceError>;
pub(crate) type DiagnosticsAndActionsFn =
    fn(DiagnosticsAndActionsParams) -> PullDiagnosticsAndActionsResult;
pub(crate) type RenameFn = fn(
    &BiomePath,
    AnyParsedSource,
    TextSize,
    String,
    WorkspaceDb,
) -> Result<RenameResult, WorkspaceError>;
pub(crate) type ResolveDefinitionFn =
    for<'a> fn(ResolveDefinitionParams<'a>) -> Option<GoToDefinitionResult>;
pub(crate) type SearchFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &dyn SearchQuery,
    &SettingsWithEditor,
    PatternId,
    WorkspaceDb,
) -> Result<Vec<TextRange>, WorkspaceError>;
pub(crate) type DebugFormatterIrFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    WorkspaceDb,
) -> Result<String, WorkspaceError>;
pub(crate) type FormatFileFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;
pub(crate) type FormatEmbeddedFileFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    Vec<ParsedSnippet>,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;
pub(crate) type FormatRangeFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    TextRange,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;
pub(crate) type FormatOnTypeFn = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    TextSize,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;

/// A read-only task that can run away from the workspace owner.
pub(crate) struct WorkerTask<R> {
    run: Box<dyn FnOnce() -> Result<R, WorkspaceError> + Send + 'static>,
}

/// A read-only task that returns work for the owner to finish.
pub(crate) struct StagedWorkerTask<C, P> {
    run: Box<dyn FnOnce() -> Result<C, WorkspaceError> + Send + 'static>,
    stale_payload: P,
}

/// The result of running a staged task on a worker.
pub(crate) enum StagedSnapshotRun<C, P> {
    /// The worker finished and produced data for the owner to apply.
    Commit(C),
    /// The data was based on the old workspace state and must be recomputed.
    Stale(P),
}

/// The result of applying worker output on the owner.
pub(crate) enum StagedCommitResult<R, P, C> {
    /// The request is complete.
    Done(R),
    /// The workspace changed before the commit could be applied.
    Stale(P),
    /// Another worker step is needed before the request can finish.
    Continue(StagedWorkerTask<C, P>),
}

/// Shared behavior for read-only snapshots.
trait ReadOnlySnapshot<R>: Send + 'static {
    /// Runs the worker-side computation using only captured data.
    fn run(self) -> Result<R, WorkspaceError>;
}

/// Shared behavior for read-only snapshots that finish on the owner.
trait StagedReadOnlySnapshot<C, P>: Send + 'static {
    /// Returns the original request data needed to restart stale work.
    ///
    /// This is only needed for staged snapshots. If the worker sees that the workspace
    /// changed after the snapshot was captured, the owner uses this payload to rebuild
    /// the work from current state instead of reporting the request as cancelled.
    fn stale_payload(&self) -> P;
    /// Runs the worker-side computation and returns owner-side work to commit.
    fn run(self) -> Result<C, WorkspaceError>;
}

impl<R> WorkerTask<R> {
    fn from_snapshot(snapshot: impl ReadOnlySnapshot<R>) -> Self {
        Self {
            run: Box::new(|| snapshot.run()),
        }
    }

    pub(crate) fn run(self) -> Result<R, WorkspaceError> {
        salsa::Cancelled::catch(AssertUnwindSafe(|| (self.run)()))
            .map_err(|cancelled| WorkspaceError::cancelled(cancelled.to_string()))?
    }
}

impl<C, P> StagedWorkerTask<C, P>
where
    P: Send + 'static,
{
    fn from_snapshot(snapshot: impl StagedReadOnlySnapshot<C, P>) -> Self {
        let stale_payload = snapshot.stale_payload();
        Self {
            run: Box::new(|| snapshot.run()),
            stale_payload,
        }
    }

    pub(crate) fn run(self) -> Result<StagedSnapshotRun<C, P>, WorkspaceError> {
        match salsa::Cancelled::catch(AssertUnwindSafe(|| (self.run)())) {
            Ok(result) => result.map(StagedSnapshotRun::Commit),
            Err(salsa::Cancelled::PendingWrite) => Ok(StagedSnapshotRun::Stale(self.stale_payload)),
            Err(cancelled) => Err(WorkspaceError::cancelled(cancelled.to_string())),
        }
    }
}

#[cfg(test)]
pub(crate) fn test_worker_task<R: 'static>(
    run: impl FnOnce() -> Result<R, WorkspaceError> + Send + 'static,
) -> WorkerTask<R> {
    WorkerTask::from_snapshot(TestSnapshot { run: Box::new(run) })
}

#[cfg(test)]
struct TestSnapshot<R> {
    run: Box<dyn FnOnce() -> Result<R, WorkspaceError> + Send + 'static>,
}

#[cfg(test)]
impl<R: 'static> ReadOnlySnapshot<R> for TestSnapshot<R> {
    fn run(self) -> Result<R, WorkspaceError> {
        (self.run)()
    }
}

/// Data needed to print a syntax tree.
pub(crate) struct GetSyntaxTreeSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    debug_syntax_tree: fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> GetSyntaxTreeResult,
}

impl GetSyntaxTreeSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: GetSyntaxTreeParams,
        parse: ParsedSource,
        debug_syntax_tree: fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> GetSyntaxTreeResult,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            debug_syntax_tree,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<GetSyntaxTreeResult> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to print a control-flow graph.
pub(crate) struct GetControlFlowGraphSnapshot {
    db: WorkspaceDb,
    parse: ParsedSource,
    cursor: TextSize,
    debug_control_flow: fn(AnyParsedSource, TextSize, WorkspaceDb) -> String,
}

impl GetControlFlowGraphSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: GetControlFlowGraphParams,
        parse: ParsedSource,
        debug_control_flow: fn(AnyParsedSource, TextSize, WorkspaceDb) -> String,
    ) -> Self {
        Self {
            db,
            parse,
            cursor: params.cursor,
            debug_control_flow,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<String> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to print formatter internals.
pub(crate) struct GetFormatterIrSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    document_file_source: DocumentFileSource,
    settings: Settings,
    debug_formatter_ir: DebugFormatterIrFn,
}

impl GetFormatterIrSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: GetFormatterIRParams,
        parse: ParsedSource,
        document_file_source: DocumentFileSource,
        settings: Settings,
        debug_formatter_ir: DebugFormatterIrFn,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            document_file_source,
            settings,
            debug_formatter_ir,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<String> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to print type information.
pub(crate) struct GetTypeInfoSnapshot {
    db: WorkspaceDb,
    parse: ParsedSource,
    debug_type_info: fn(AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>,
}

impl GetTypeInfoSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        _params: GetTypeInfoParams,
        parse: ParsedSource,
        debug_type_info: fn(AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>,
    ) -> Self {
        Self {
            db,
            parse,
            debug_type_info,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<String> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to print registered types.
pub(crate) struct GetRegisteredTypesSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    debug_registered_types:
        fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>,
}

impl GetRegisteredTypesSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: GetRegisteredTypesParams,
        parse: ParsedSource,
        debug_registered_types: fn(
            &BiomePath,
            AnyParsedSource,
            WorkspaceDb,
        ) -> Result<String, WorkspaceError>,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            debug_registered_types,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<String> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to print the semantic model.
pub(crate) struct GetSemanticModelSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    debug_semantic_model:
        fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>,
}

impl GetSemanticModelSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: GetSemanticModelParams,
        parse: ParsedSource,
        debug_semantic_model: fn(
            &BiomePath,
            AnyParsedSource,
            WorkspaceDb,
        ) -> Result<String, WorkspaceError>,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            debug_semantic_model,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<String> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to decide which features apply to a file.
pub(crate) struct FileFeaturesSnapshot {
    project: ProjectSnapshot,
    params: SupportsFeatureParams,
    language: DocumentFileSource,
    settings: Settings,
    capabilities: Capabilities,
}

impl FileFeaturesSnapshot {
    pub(crate) fn new(
        project: ProjectSnapshot,
        params: SupportsFeatureParams,
        language: DocumentFileSource,
        settings: Settings,
        capabilities: Capabilities,
    ) -> Self {
        Self {
            project,
            params,
            language,
            settings,
            capabilities,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<FileFeaturesResult> {
        WorkerTask::from_snapshot(self)
    }
}

/// Data needed to decide whether a path is ignored.
pub(crate) struct PathIsIgnoredSnapshot {
    project: ProjectSnapshot,
    params: PathIsIgnoredParams,
}

impl PathIsIgnoredSnapshot {
    pub(crate) fn new(project: ProjectSnapshot, params: PathIsIgnoredParams) -> Self {
        Self { project, params }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<bool> {
        WorkerTask::from_snapshot(self)
    }
}

/// A parsed embedded file and the fixer that can handle it.
struct FixFileSnippet {
    parsed_source: ParsedSnippet,
    language: DocumentFileSource,
    fix_all: Option<FixAllFn>,
}

/// Owner-built input for the first fix-file worker step.
pub(crate) struct FixFileSnapshotInput {
    pub(crate) db: WorkspaceDb,
    pub(crate) params: FixFileParams,
    pub(crate) parse: ParsedSource,
    pub(crate) embedded_snippets: Vec<(ParsedSnippet, DocumentFileSource, Option<FixAllFn>)>,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) fix_all: FixAllFn,
    pub(crate) update_snippets: Option<UpdateSnippetsFn>,
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) revision: u64,
}

/// Data needed to apply fixes, starting with embedded files.
pub(crate) struct FixFileSnapshot {
    db: WorkspaceDb,
    params: FixFileParams,
    parse: ParsedSource,
    embedded_snippets: Vec<FixFileSnippet>,
    working_directory: Utf8PathBuf,
    settings: Settings,
    language: DocumentFileSource,
    fix_all: FixAllFn,
    update_snippets: Option<UpdateSnippetsFn>,
    plugins: AnalyzerPluginVec,
    project_layout: Arc<ProjectLayout>,
    revision: u64,
}

impl FixFileSnapshot {
    pub(crate) fn new(input: FixFileSnapshotInput) -> Self {
        let FixFileSnapshotInput {
            db,
            params,
            parse,
            embedded_snippets,
            working_directory,
            settings,
            language,
            fix_all,
            update_snippets,
            plugins,
            project_layout,
            revision,
        } = input;

        Self {
            db,
            params,
            parse,
            embedded_snippets: embedded_snippets
                .into_iter()
                .map(|(parsed_source, language, fix_all)| FixFileSnippet {
                    parsed_source,
                    language,
                    fix_all,
                })
                .collect(),
            working_directory,
            settings,
            language,
            fix_all,
            update_snippets,
            plugins,
            project_layout,
            revision,
        }
    }

    pub(crate) fn into_staged_worker_task(self) -> StagedWorkerTask<FixFileCommit, FixFileParams> {
        StagedWorkerTask::from_snapshot(self)
    }
}

/// Owner-built input for the final fix-file worker step.
pub(crate) struct FixFileRootSnapshotInput {
    pub(crate) db: WorkspaceDb,
    pub(crate) params: FixFileParams,
    pub(crate) parse: ParsedSource,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) fix_all: FixAllFn,
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) revision: u64,
    pub(crate) errors: usize,
    pub(crate) actions: Vec<crate::workspace::FixAction>,
    pub(crate) skipped_suggested_fixes: u32,
}

/// Data needed to apply fixes to the root file.
pub(crate) struct FixFileRootSnapshot {
    db: WorkspaceDb,
    params: FixFileParams,
    parse: ParsedSource,
    working_directory: Utf8PathBuf,
    settings: Settings,
    language: DocumentFileSource,
    fix_all: FixAllFn,
    plugins: AnalyzerPluginVec,
    project_layout: Arc<ProjectLayout>,
    revision: u64,
    errors: usize,
    actions: Vec<crate::workspace::FixAction>,
    skipped_suggested_fixes: u32,
}

impl FixFileRootSnapshot {
    pub(crate) fn new(input: FixFileRootSnapshotInput) -> Self {
        let FixFileRootSnapshotInput {
            db,
            params,
            parse,
            working_directory,
            settings,
            language,
            fix_all,
            plugins,
            project_layout,
            revision,
            errors,
            actions,
            skipped_suggested_fixes,
        } = input;

        Self {
            db,
            params,
            parse,
            working_directory,
            settings,
            language,
            fix_all,
            plugins,
            project_layout,
            revision,
            errors,
            actions,
            skipped_suggested_fixes,
        }
    }

    pub(crate) fn into_staged_worker_task(self) -> StagedWorkerTask<FixFileCommit, FixFileParams> {
        StagedWorkerTask::from_snapshot(self)
    }
}

/// Work produced by fix-file workers for the owner.
pub(crate) enum FixFileCommit {
    /// Update embedded snippets first, then continue with the root file.
    ApplyEmbedded(Box<FixFileEmbeddedCommit>),
    /// Finish the request with the computed fix result.
    Finish(Box<FixFileFinishCommit>),
}

/// Owner work needed after embedded fixes have been computed.
pub(crate) struct FixFileEmbeddedCommit {
    pub(crate) params: FixFileParams,
    pub(crate) new_root: SendNode,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) fix_all: FixAllFn,
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) revision: u64,
    pub(crate) errors: usize,
    pub(crate) actions: Vec<crate::workspace::FixAction>,
    pub(crate) skipped_suggested_fixes: u32,
}

/// Owner work needed after root fixes have been computed.
pub(crate) struct FixFileFinishCommit {
    pub(crate) params: FixFileParams,
    pub(crate) revision: u64,
    pub(crate) result: FixFileResult,
}

/// Which formatter entry point should run for a file.
pub(crate) enum FormatFileKind {
    /// Format the root file only.
    Plain(FormatFileFn),
    /// Format the root file and its embedded snippets.
    Embedded(FormatEmbeddedFileFn),
}

/// Data needed to format a whole file.
pub(crate) struct FormatFileSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    document_file_source: DocumentFileSource,
    settings: Settings,
    inline_config: Option<Configuration>,
    embedded_nodes: Vec<ParsedSnippet>,
    format_kind: FormatFileKind,
}

impl FormatFileSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: FormatFileParams,
        parse: ParsedSource,
        document_file_source: DocumentFileSource,
        settings: Settings,
        embedded_nodes: Vec<ParsedSnippet>,
        format_kind: FormatFileKind,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            document_file_source,
            settings,
            inline_config: params.inline_config,
            embedded_nodes,
            format_kind,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<Printed> {
        WorkerTask::from_snapshot(self)
    }
}

pub(crate) struct FormatRangeSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    document_file_source: DocumentFileSource,
    settings: Settings,
    inline_config: Option<Configuration>,
    range: TextRange,
    format_range: FormatRangeFn,
}

impl FormatRangeSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: FormatRangeParams,
        parse: ParsedSource,
        document_file_source: DocumentFileSource,
        settings: Settings,
        format_range: FormatRangeFn,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            document_file_source,
            settings,
            inline_config: params.inline_config,
            range: params.range,
            format_range,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<Printed> {
        WorkerTask::from_snapshot(self)
    }
}

pub(crate) struct FormatOnTypeSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    document_file_source: DocumentFileSource,
    settings: Settings,
    inline_config: Option<Configuration>,
    offset: TextSize,
    format_on_type: FormatOnTypeFn,
}

impl FormatOnTypeSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: FormatOnTypeParams,
        parse: ParsedSource,
        document_file_source: DocumentFileSource,
        settings: Settings,
        format_on_type: FormatOnTypeFn,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            document_file_source,
            settings,
            inline_config: params.inline_config,
            offset: params.offset,
            format_on_type,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<Printed> {
        WorkerTask::from_snapshot(self)
    }
}

struct DiagnosticSnippet {
    parsed_source: ParsedSnippet,
    language: DocumentFileSource,
    lint: Option<LintFn>,
}

pub(crate) struct PullDiagnosticsSnapshotInput {
    pub(crate) db: WorkspaceDb,
    pub(crate) params: PullDiagnosticsParams,
    pub(crate) parse: ParsedSource,
    pub(crate) embedded_snippets: Vec<(ParsedSnippet, DocumentFileSource, Option<LintFn>)>,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) lint: Option<LintFn>,
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) project_layout: Arc<ProjectLayout>,
}

pub(crate) struct PullDiagnosticsSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    embedded_snippets: Vec<DiagnosticSnippet>,
    working_directory: Utf8PathBuf,
    settings: Settings,
    inline_config: Option<Configuration>,
    language: DocumentFileSource,
    categories: RuleCategories,
    only: Vec<AnalyzerSelector>,
    skip: Vec<AnalyzerSelector>,
    enabled_rules: Vec<AnalyzerSelector>,
    pull_code_actions: bool,
    max_diagnostics: Option<u32>,
    diagnostic_level: Severity,
    enforce_assist: bool,
    lint: Option<LintFn>,
    plugins: AnalyzerPluginVec,
    project_layout: Arc<ProjectLayout>,
    analyzer_cache: AnalyzerVisitorCache,
}

impl PullDiagnosticsSnapshot {
    pub(crate) fn new(input: PullDiagnosticsSnapshotInput) -> Self {
        let PullDiagnosticsSnapshotInput {
            db,
            params,
            parse,
            embedded_snippets,
            working_directory,
            settings,
            language,
            lint,
            plugins,
            project_layout,
        } = input;

        Self {
            db,
            path: params.path,
            parse,
            embedded_snippets: embedded_snippets
                .into_iter()
                .map(|(parsed_source, language, lint)| DiagnosticSnippet {
                    parsed_source,
                    language,
                    lint,
                })
                .collect(),
            working_directory,
            settings,
            inline_config: params.inline_config,
            language,
            categories: params.categories,
            only: params.only,
            skip: params.skip,
            enabled_rules: params.enabled_rules,
            pull_code_actions: params.include_code_fix,
            max_diagnostics: params.max_diagnostics,
            diagnostic_level: params.diagnostic_level,
            enforce_assist: params.enforce_assist,
            lint,
            plugins,
            project_layout,
            analyzer_cache: AnalyzerVisitorCache::default(),
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<PullDiagnosticsResult> {
        WorkerTask::from_snapshot(self)
    }
}

struct DiagnosticsAndActionsSnippet {
    parsed_source: ParsedSnippet,
    language: DocumentFileSource,
    pull_diagnostics_and_actions: Option<DiagnosticsAndActionsFn>,
}

pub(crate) struct PullDiagnosticsAndActionsSnapshotInput {
    pub(crate) db: WorkspaceDb,
    pub(crate) params: PullDiagnosticsAndActionsParams,
    pub(crate) parse: ParsedSource,
    pub(crate) embedded_snippets: Vec<(
        ParsedSnippet,
        DocumentFileSource,
        Option<DiagnosticsAndActionsFn>,
    )>,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) pull_diagnostics_and_actions: Option<DiagnosticsAndActionsFn>,
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) project_layout: Arc<ProjectLayout>,
}

pub(crate) struct PullDiagnosticsAndActionsSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    embedded_snippets: Vec<DiagnosticsAndActionsSnippet>,
    working_directory: Utf8PathBuf,
    settings: Settings,
    inline_config: Option<Configuration>,
    language: DocumentFileSource,
    categories: RuleCategories,
    only: Vec<AnalyzerSelector>,
    skip: Vec<AnalyzerSelector>,
    enabled_rules: Vec<AnalyzerSelector>,
    pull_diagnostics_and_actions: Option<DiagnosticsAndActionsFn>,
    plugins: AnalyzerPluginVec,
    project_layout: Arc<ProjectLayout>,
}

impl PullDiagnosticsAndActionsSnapshot {
    pub(crate) fn new(input: PullDiagnosticsAndActionsSnapshotInput) -> Self {
        let PullDiagnosticsAndActionsSnapshotInput {
            db,
            params,
            parse,
            embedded_snippets,
            working_directory,
            settings,
            language,
            pull_diagnostics_and_actions,
            plugins,
            project_layout,
        } = input;

        Self {
            db,
            path: params.path,
            parse,
            embedded_snippets: embedded_snippets
                .into_iter()
                .map(|(parsed_source, language, pull_diagnostics_and_actions)| {
                    DiagnosticsAndActionsSnippet {
                        parsed_source,
                        language,
                        pull_diagnostics_and_actions,
                    }
                })
                .collect(),
            working_directory,
            settings,
            inline_config: params.inline_config,
            language,
            categories: params.categories,
            only: params.only,
            skip: params.skip,
            enabled_rules: params.enabled_rules,
            pull_diagnostics_and_actions,
            plugins,
            project_layout,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<PullDiagnosticsAndActionsResult> {
        WorkerTask::from_snapshot(self)
    }
}

struct CodeActionsSnippet {
    parsed_source: ParsedSnippet,
    language: DocumentFileSource,
    code_actions: Option<CodeActionsFn>,
}

pub(crate) struct PullActionsSnapshotInput {
    pub(crate) db: WorkspaceDb,
    pub(crate) params: PullActionsParams,
    pub(crate) parse: ParsedSource,
    pub(crate) embedded_snippets: Vec<(ParsedSnippet, DocumentFileSource, Option<CodeActionsFn>)>,
    pub(crate) working_directory: Utf8PathBuf,
    pub(crate) settings: Settings,
    pub(crate) language: DocumentFileSource,
    pub(crate) code_actions: CodeActionsFn,
    pub(crate) project_layout: Arc<ProjectLayout>,
}

pub(crate) struct PullActionsSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    embedded_snippets: Vec<CodeActionsSnippet>,
    working_directory: Utf8PathBuf,
    settings: Settings,
    inline_config: Option<Configuration>,
    language: DocumentFileSource,
    range: Option<TextRange>,
    only: Vec<AnalyzerSelector>,
    skip: Vec<AnalyzerSelector>,
    enabled_rules: Vec<AnalyzerSelector>,
    categories: RuleCategories,
    compute_actions: bool,
    code_actions: CodeActionsFn,
    project_layout: Arc<ProjectLayout>,
    analyzer_cache: AnalyzerVisitorCache,
}

impl PullActionsSnapshot {
    pub(crate) fn new(input: PullActionsSnapshotInput) -> Self {
        let PullActionsSnapshotInput {
            db,
            params,
            parse,
            embedded_snippets,
            working_directory,
            settings,
            language,
            code_actions,
            project_layout,
        } = input;

        Self {
            db,
            path: params.path,
            parse,
            embedded_snippets: embedded_snippets
                .into_iter()
                .map(
                    |(parsed_source, language, code_actions)| CodeActionsSnippet {
                        parsed_source,
                        language,
                        code_actions,
                    },
                )
                .collect(),
            working_directory,
            settings,
            inline_config: params.inline_config,
            language,
            range: params.range,
            only: params.only,
            skip: params.skip,
            enabled_rules: params.enabled_rules,
            categories: params.categories,
            compute_actions: params.compute_actions,
            code_actions,
            project_layout,
            analyzer_cache: AnalyzerVisitorCache::default(),
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<PullActionsResult> {
        WorkerTask::from_snapshot(self)
    }
}

pub(crate) struct RenameSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    symbol_at: TextSize,
    new_name: String,
    rename: RenameFn,
}

impl RenameSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: RenameParams,
        parse: ParsedSource,
        rename: RenameFn,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            symbol_at: params.symbol_at,
            new_name: params.new_name,
            rename,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<RenameResult> {
        WorkerTask::from_snapshot(self)
    }
}

struct DefinitionSnippet {
    parsed_source: ParsedSnippet,
    resolve_definition: Option<ResolveDefinitionFn>,
}

pub(crate) struct GoToDefinitionSnapshot {
    db: Option<WorkspaceDb>,
    path: Option<BiomePath>,
    parse: Option<ParsedSource>,
    embedded_snippets: Vec<DefinitionSnippet>,
    definition_ref: Option<DefinitionReference>,
    resolve_definition: Option<ResolveDefinitionFn>,
    capability_error: Option<WorkspaceError>,
}

impl GoToDefinitionSnapshot {
    pub(crate) fn disabled() -> Self {
        Self {
            db: None,
            path: None,
            parse: None,
            embedded_snippets: Vec::new(),
            definition_ref: None,
            resolve_definition: None,
            capability_error: None,
        }
    }

    pub(crate) fn new(
        db: WorkspaceDb,
        params: GoToDefinitionParams,
        parse: ParsedSource,
        embedded_snippets: Vec<(ParsedSnippet, Option<ResolveDefinitionFn>)>,
        definition_ref: Option<DefinitionReference>,
        resolve_definition: Option<ResolveDefinitionFn>,
        capability_error: Option<WorkspaceError>,
    ) -> Self {
        Self {
            db: Some(db),
            path: Some(params.path),
            parse: Some(parse),
            embedded_snippets: embedded_snippets
                .into_iter()
                .map(|(parsed_source, resolve_definition)| DefinitionSnippet {
                    parsed_source,
                    resolve_definition,
                })
                .collect(),
            definition_ref,
            resolve_definition,
            capability_error,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<Option<GoToDefinitionResult>> {
        WorkerTask::from_snapshot(self)
    }
}

pub(crate) struct SearchPatternSnapshot {
    db: WorkspaceDb,
    path: BiomePath,
    parse: ParsedSource,
    document_file_source: DocumentFileSource,
    settings: Settings,
    pattern: PatternId,
    provider: Arc<dyn SearchQuery>,
    search: SearchFn,
}

impl SearchPatternSnapshot {
    pub(crate) fn new(
        db: WorkspaceDb,
        params: SearchPatternParams,
        parse: ParsedSource,
        document_file_source: DocumentFileSource,
        settings: Settings,
        provider: Arc<dyn SearchQuery>,
        search: SearchFn,
    ) -> Self {
        Self {
            db,
            path: params.path,
            parse,
            document_file_source,
            settings,
            pattern: params.pattern,
            provider,
            search,
        }
    }

    pub(crate) fn into_worker_task(self) -> WorkerTask<SearchResults> {
        WorkerTask::from_snapshot(self)
    }
}

macro_rules! read_only_snapshot {
    ($snapshot:ty, $result:ty, $run:ident) => {
        impl ReadOnlySnapshot<$result> for $snapshot {
            fn run(self) -> Result<$result, WorkspaceError> {
                $run(self)
            }
        }
    };
}

macro_rules! staged_read_only_snapshot {
    ($snapshot:ty, $commit:ty, $payload:ty, $run:ident) => {
        impl StagedReadOnlySnapshot<$commit, $payload> for $snapshot {
            fn stale_payload(&self) -> $payload {
                self.params.clone()
            }

            fn run(self) -> Result<$commit, WorkspaceError> {
                $run(self)
            }
        }
    };
}

read_only_snapshot!(
    GetSyntaxTreeSnapshot,
    GetSyntaxTreeResult,
    run_get_syntax_tree
);
read_only_snapshot!(
    GetControlFlowGraphSnapshot,
    String,
    run_get_control_flow_graph
);
read_only_snapshot!(GetFormatterIrSnapshot, String, run_get_formatter_ir);
read_only_snapshot!(GetTypeInfoSnapshot, String, run_get_type_info);
read_only_snapshot!(GetRegisteredTypesSnapshot, String, run_get_registered_types);
read_only_snapshot!(GetSemanticModelSnapshot, String, run_get_semantic_model);
read_only_snapshot!(FileFeaturesSnapshot, FileFeaturesResult, run_file_features);
read_only_snapshot!(PathIsIgnoredSnapshot, bool, run_path_is_ignored);
staged_read_only_snapshot!(FixFileSnapshot, FixFileCommit, FixFileParams, run_fix_file);
staged_read_only_snapshot!(
    FixFileRootSnapshot,
    FixFileCommit,
    FixFileParams,
    run_fix_file_root
);
read_only_snapshot!(FormatFileSnapshot, Printed, run_format_file);
read_only_snapshot!(FormatRangeSnapshot, Printed, run_format_range);
read_only_snapshot!(FormatOnTypeSnapshot, Printed, run_format_on_type);
read_only_snapshot!(
    PullDiagnosticsSnapshot,
    PullDiagnosticsResult,
    run_pull_diagnostics
);
read_only_snapshot!(
    PullDiagnosticsAndActionsSnapshot,
    PullDiagnosticsAndActionsResult,
    run_pull_diagnostics_and_actions
);
read_only_snapshot!(PullActionsSnapshot, PullActionsResult, run_pull_actions);
read_only_snapshot!(RenameSnapshot, RenameResult, run_rename);
read_only_snapshot!(
    GoToDefinitionSnapshot,
    Option<GoToDefinitionResult>,
    run_go_to_definition
);
read_only_snapshot!(SearchPatternSnapshot, SearchResults, run_search_pattern);

fn run_get_syntax_tree(
    snapshot: GetSyntaxTreeSnapshot,
) -> Result<GetSyntaxTreeResult, WorkspaceError> {
    Ok((snapshot.debug_syntax_tree)(
        &snapshot.path,
        snapshot.parse.into(),
        snapshot.db,
    ))
}

fn run_get_control_flow_graph(
    snapshot: GetControlFlowGraphSnapshot,
) -> Result<String, WorkspaceError> {
    Ok((snapshot.debug_control_flow)(
        snapshot.parse.into(),
        snapshot.cursor,
        snapshot.db,
    ))
}

fn run_get_formatter_ir(snapshot: GetFormatterIrSnapshot) -> Result<String, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (Option::<Configuration>::None, EditorFeatures::default()),
    );
    (snapshot.debug_formatter_ir)(
        &snapshot.path,
        &snapshot.document_file_source,
        snapshot.parse.into(),
        &settings,
        snapshot.db,
    )
}

fn run_get_type_info(snapshot: GetTypeInfoSnapshot) -> Result<String, WorkspaceError> {
    (snapshot.debug_type_info)(snapshot.parse.into(), snapshot.db)
}

fn run_get_registered_types(
    snapshot: GetRegisteredTypesSnapshot,
) -> Result<String, WorkspaceError> {
    (snapshot.debug_registered_types)(&snapshot.path, snapshot.parse.into(), snapshot.db)
}

fn run_get_semantic_model(snapshot: GetSemanticModelSnapshot) -> Result<String, WorkspaceError> {
    (snapshot.debug_semantic_model)(&snapshot.path, snapshot.parse.into(), snapshot.db)
}

fn run_file_features(snapshot: FileFeaturesSnapshot) -> Result<FileFeaturesResult, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.params.inline_config, EditorFeatures::default()),
    );

    snapshot.project.get_file_features(GetFileFeaturesParams {
        project_key: snapshot.params.project_key,
        path: snapshot.params.path.as_path(),
        requested_features: snapshot.params.features,
        language: snapshot.language,
        capabilities: &snapshot.capabilities,
        handle: &settings,
        skip_ignore_check: snapshot.params.skip_ignore_check,
        not_requested_features: snapshot.params.not_requested_features,
    })
}

fn run_path_is_ignored(snapshot: PathIsIgnoredSnapshot) -> Result<bool, WorkspaceError> {
    let path = snapshot.params.path.as_path();
    if snapshot.project.is_top_level_config(path) {
        return Ok(false);
    }

    Ok(snapshot.project.is_ignored(
        path,
        snapshot.params.is_dir,
        snapshot.params.features,
        snapshot.params.ignore_kind,
    ))
}

fn run_fix_file(snapshot: FixFileSnapshot) -> Result<FixFileCommit, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (
            snapshot.params.inline_config.clone(),
            EditorFeatures::default(),
        ),
    );

    if let Some(update_snippets) = snapshot.update_snippets {
        let mut errors = 0;
        let mut actions = vec![];
        let mut skipped_suggested_fixes = 0;
        let mut new_snippets = vec![];

        for embedded_snippet in snapshot.embedded_snippets {
            let Some(fix_all) = embedded_snippet.fix_all else {
                continue;
            };
            let range = embedded_snippet.parsed_source.element_range(&snapshot.db);

            let results = fix_all(FixAllParams {
                parsed_source: embedded_snippet.parsed_source.into(),
                fix_file_mode: snapshot.params.fix_file_mode,
                settings: &settings,
                should_format: snapshot.params.should_format,
                biome_path: &snapshot.params.path,
                workspace_db: snapshot.db.clone(),
                project_layout: snapshot.project_layout.clone(),
                document_file_source: embedded_snippet.language,
                only: &snapshot.params.only,
                skip: &snapshot.params.skip,
                rule_categories: snapshot.params.rule_categories,
                suppression_reason: snapshot.params.suppression_reason.clone(),
                enabled_rules: &snapshot.params.enabled_rules,
                plugins: snapshot.plugins.clone(),
                working_directory: Some(snapshot.working_directory.as_path()),
                embeds_initial_indent: 0,
            })?;

            actions.extend(results.actions);
            errors += results.errors;
            skipped_suggested_fixes += results.skipped_suggested_fixes;

            new_snippets.push(UpdateSnippetsNodes {
                range,
                new_code: results.code,
                needs_reindent: snapshot.params.should_format,
            });
        }

        let new_root = update_snippets(snapshot.parse.into(), snapshot.db, new_snippets)?;

        return Ok(FixFileCommit::ApplyEmbedded(Box::new(
            FixFileEmbeddedCommit {
                params: snapshot.params,
                new_root,
                working_directory: snapshot.working_directory,
                settings: snapshot.settings,
                language: snapshot.language,
                fix_all: snapshot.fix_all,
                plugins: snapshot.plugins,
                project_layout: snapshot.project_layout,
                revision: snapshot.revision,
                errors,
                actions,
                skipped_suggested_fixes,
            },
        )));
    }

    run_fix_file_root(FixFileRootSnapshot {
        db: snapshot.db,
        params: snapshot.params,
        parse: snapshot.parse,
        working_directory: snapshot.working_directory,
        settings: snapshot.settings,
        language: snapshot.language,
        fix_all: snapshot.fix_all,
        plugins: snapshot.plugins,
        project_layout: snapshot.project_layout,
        revision: snapshot.revision,
        errors: 0,
        actions: vec![],
        skipped_suggested_fixes: 0,
    })
}

fn run_fix_file_root(mut snapshot: FixFileRootSnapshot) -> Result<FixFileCommit, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (
            snapshot.params.inline_config.clone(),
            EditorFeatures::default(),
        ),
    );

    let fix_result = (snapshot.fix_all)(FixAllParams {
        parsed_source: snapshot.parse.into(),
        fix_file_mode: snapshot.params.fix_file_mode,
        settings: &settings,
        should_format: snapshot.params.should_format,
        biome_path: &snapshot.params.path,
        workspace_db: snapshot.db,
        project_layout: snapshot.project_layout,
        document_file_source: snapshot.language,
        only: &snapshot.params.only,
        skip: &snapshot.params.skip,
        rule_categories: snapshot.params.rule_categories,
        suppression_reason: snapshot.params.suppression_reason.clone(),
        enabled_rules: &snapshot.params.enabled_rules,
        plugins: snapshot.plugins,
        working_directory: Some(snapshot.working_directory.as_path()),
        embeds_initial_indent: 0,
    })?;

    snapshot.actions.extend(fix_result.actions);
    snapshot.errors += fix_result.errors;
    snapshot.skipped_suggested_fixes += fix_result.skipped_suggested_fixes;

    Ok(FixFileCommit::Finish(Box::new(FixFileFinishCommit {
        params: snapshot.params,
        revision: snapshot.revision,
        result: FixFileResult {
            errors: snapshot.errors,
            code: fix_result.code,
            actions: snapshot.actions,
            skipped_suggested_fixes: snapshot.skipped_suggested_fixes,
        },
    })))
}

fn run_format_file(snapshot: FormatFileSnapshot) -> Result<Printed, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.inline_config, EditorFeatures::default()),
    );

    match snapshot.format_kind {
        FormatFileKind::Plain(format) => format(
            &snapshot.path,
            &snapshot.document_file_source,
            snapshot.parse.into(),
            &settings,
            snapshot.db,
        ),
        FormatFileKind::Embedded(format_embedded) => format_embedded(
            &snapshot.path,
            &snapshot.document_file_source,
            snapshot.parse.into(),
            &settings,
            snapshot.embedded_nodes,
            snapshot.db,
        ),
    }
}

fn run_format_range(snapshot: FormatRangeSnapshot) -> Result<Printed, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.inline_config, EditorFeatures::default()),
    );

    (snapshot.format_range)(
        &snapshot.path,
        &snapshot.document_file_source,
        snapshot.parse.into(),
        &settings,
        snapshot.range,
        snapshot.db,
    )
}

fn run_format_on_type(snapshot: FormatOnTypeSnapshot) -> Result<Printed, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.inline_config, EditorFeatures::default()),
    );

    (snapshot.format_on_type)(
        &snapshot.path,
        &snapshot.document_file_source,
        snapshot.parse.into(),
        &settings,
        snapshot.offset,
        snapshot.db,
    )
}

fn run_pull_diagnostics(
    snapshot: PullDiagnosticsSnapshot,
) -> Result<PullDiagnosticsResult, WorkspaceError> {
    let parse_errors = snapshot.parse.error_count(&snapshot.db);

    let (diagnostics, errors, warnings, infos, skipped_diagnostics) =
        if (snapshot.categories.is_lint() || snapshot.categories.is_assist())
            && let Some(lint) = snapshot.lint
        {
            let settings = SettingsHandle::new(
                &snapshot.settings,
                (snapshot.inline_config, EditorFeatures::default()),
            );
            let results = lint(LintParams {
                parsed_source: snapshot.parse.into(),
                settings: &settings,
                path: &snapshot.path,
                only: &snapshot.only,
                skip: &snapshot.skip,
                language: snapshot.language,
                categories: snapshot.categories,
                workspace_db: snapshot.db.clone(),
                project_layout: snapshot.project_layout.clone(),
                suppression_reason: None,
                enabled_selectors: &snapshot.enabled_rules,
                pull_code_actions: snapshot.pull_code_actions,
                plugins: snapshot.plugins.clone(),
                working_directory: Some(snapshot.working_directory.as_path()),
                max_diagnostics: snapshot.max_diagnostics,
                diagnostic_level: snapshot.diagnostic_level,
                enforce_assist: snapshot.enforce_assist,
                analyzer_cache: &snapshot.analyzer_cache,
            });

            let LintResults {
                mut diagnostics,
                mut errors,
                mut skipped_diagnostics,
                mut warnings,
                mut infos,
            } = results;

            for embedded_node in snapshot.embedded_snippets {
                let Some(lint) = embedded_node.lint else {
                    continue;
                };

                let results = lint(LintParams {
                    parsed_source: embedded_node.parsed_source.into(),
                    settings: &settings,
                    path: &snapshot.path,
                    only: &snapshot.only,
                    skip: &snapshot.skip,
                    language: embedded_node.language,
                    categories: snapshot.categories,
                    workspace_db: snapshot.db.clone(),
                    project_layout: snapshot.project_layout.clone(),
                    suppression_reason: None,
                    enabled_selectors: &snapshot.enabled_rules,
                    pull_code_actions: snapshot.pull_code_actions,
                    plugins: snapshot.plugins.clone(),
                    working_directory: Some(snapshot.working_directory.as_path()),
                    max_diagnostics: snapshot.max_diagnostics,
                    diagnostic_level: snapshot.diagnostic_level,
                    enforce_assist: snapshot.enforce_assist,
                    analyzer_cache: &snapshot.analyzer_cache,
                });

                diagnostics.extend(results.diagnostics);
                skipped_diagnostics += results.skipped_diagnostics;
                errors += results.errors;
                warnings += results.warnings;
                infos += results.infos;
            }

            (diagnostics, errors, warnings, infos, skipped_diagnostics)
        } else {
            let mut parse_diagnostics: Vec<_> = snapshot
                .parse
                .serde_diagnostics(&snapshot.db)
                .into_iter()
                .filter(|diag| diag.severity() >= snapshot.diagnostic_level)
                .collect();
            let mut errors = parse_diagnostics
                .iter()
                .filter(|diag| diag.severity() >= Severity::Error)
                .count();

            for embedded_node in snapshot.embedded_snippets {
                let diagnostics: Vec<_> = embedded_node
                    .parsed_source
                    .serde_diagnostics(&snapshot.db)
                    .into_iter()
                    .filter(|diag| diag.severity() >= snapshot.diagnostic_level)
                    .collect();
                errors += diagnostics
                    .iter()
                    .filter(|diag| diag.severity() >= Severity::Error)
                    .count();
                parse_diagnostics.extend(diagnostics);
            }

            (parse_diagnostics, errors, 0, 0, 0)
        };

    tracing::info!(
        "Pulled {:?} diagnostic(s), skipped {:?} diagnostic(s) from {}",
        diagnostics.len(),
        skipped_diagnostics,
        snapshot.path
    );

    Ok(PullDiagnosticsResult {
        diagnostics: diagnostics
            .into_iter()
            .map(|diag| {
                let diag = diag.with_file_path(snapshot.path.to_string());
                SerdeDiagnostic::new(diag)
            })
            .collect(),
        errors,
        warnings,
        infos,
        parse_errors,
        skipped_diagnostics: skipped_diagnostics.into(),
    })
}

fn run_pull_diagnostics_and_actions(
    snapshot: PullDiagnosticsAndActionsSnapshot,
) -> Result<PullDiagnosticsAndActionsResult, WorkspaceError> {
    if !(snapshot.categories.is_lint() || snapshot.categories.is_assist()) {
        return Ok(PullDiagnosticsAndActionsResult {
            diagnostics: vec![],
        });
    }

    let Some(pull_diagnostics_and_actions) = snapshot.pull_diagnostics_and_actions else {
        return Ok(PullDiagnosticsAndActionsResult {
            diagnostics: vec![],
        });
    };

    let handle = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.inline_config, EditorFeatures::default()),
    );
    let mut final_result = pull_diagnostics_and_actions(DiagnosticsAndActionsParams {
        parsed_source: snapshot.parse.into(),
        settings: &handle,
        path: &snapshot.path,
        only: &snapshot.only,
        skip: &snapshot.skip,
        language: snapshot.language,
        categories: snapshot.categories,
        workspace_db: snapshot.db.clone(),
        project_layout: snapshot.project_layout.clone(),
        suppression_reason: None,
        enabled_selectors: &snapshot.enabled_rules,
        plugins: snapshot.plugins.clone(),
        working_directory: Some(snapshot.working_directory.as_path()),
    });

    for embedded_node in snapshot.embedded_snippets {
        let Some(pull_diagnostics_and_actions) = embedded_node.pull_diagnostics_and_actions else {
            continue;
        };

        let snippet_result = pull_diagnostics_and_actions(DiagnosticsAndActionsParams {
            parsed_source: embedded_node.parsed_source.into(),
            settings: &handle,
            path: &snapshot.path,
            only: &snapshot.only,
            skip: &snapshot.skip,
            language: embedded_node.language,
            categories: snapshot.categories,
            workspace_db: snapshot.db.clone(),
            project_layout: snapshot.project_layout.clone(),
            suppression_reason: None,
            enabled_selectors: &snapshot.enabled_rules,
            plugins: snapshot.plugins.clone(),
            working_directory: Some(snapshot.working_directory.as_path()),
        });

        final_result.diagnostics.extend(snippet_result.diagnostics);
    }

    Ok(final_result)
}

fn run_pull_actions(snapshot: PullActionsSnapshot) -> Result<PullActionsResult, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (snapshot.inline_config, EditorFeatures::default()),
    );
    let mut result = (snapshot.code_actions)(CodeActionsParams {
        parsed_source: snapshot.parse.into(),
        range: snapshot.range,
        settings: &settings,
        path: &snapshot.path,
        workspace_db: snapshot.db.clone(),
        project_layout: snapshot.project_layout.clone(),
        language: snapshot.language,
        only: &snapshot.only,
        skip: &snapshot.skip,
        suppression_reason: None,
        enabled_rules: &snapshot.enabled_rules,
        plugins: Vec::new(),
        categories: snapshot.categories,
        working_directory: Some(snapshot.working_directory.as_path()),
        compute_actions: snapshot.compute_actions,
        analyzer_cache: &snapshot.analyzer_cache,
    });

    for embedded_snippet in snapshot.embedded_snippets {
        let Some(code_actions) = embedded_snippet.code_actions else {
            continue;
        };

        let embedded_actions_result = code_actions(CodeActionsParams {
            parsed_source: embedded_snippet.parsed_source.into(),
            range: snapshot.range,
            settings: &settings,
            path: &snapshot.path,
            workspace_db: snapshot.db.clone(),
            project_layout: snapshot.project_layout.clone(),
            language: embedded_snippet.language,
            only: &snapshot.only,
            skip: &snapshot.skip,
            suppression_reason: None,
            enabled_rules: &snapshot.enabled_rules,
            plugins: Vec::new(),
            categories: snapshot.categories,
            working_directory: Some(snapshot.working_directory.as_path()),
            compute_actions: snapshot.compute_actions,
            analyzer_cache: &snapshot.analyzer_cache,
        });

        result.actions.extend(embedded_actions_result.actions);
    }

    Ok(result)
}

fn run_rename(snapshot: RenameSnapshot) -> Result<RenameResult, WorkspaceError> {
    (snapshot.rename)(
        &snapshot.path,
        snapshot.parse.into(),
        snapshot.symbol_at,
        snapshot.new_name,
        snapshot.db,
    )
}

fn run_go_to_definition(
    snapshot: GoToDefinitionSnapshot,
) -> Result<Option<GoToDefinitionResult>, WorkspaceError> {
    let (Some(db), Some(path), Some(parse)) = (snapshot.db, snapshot.path, snapshot.parse) else {
        return Ok(None);
    };
    let Some(definition_ref) = snapshot.definition_ref else {
        return Ok(None);
    };

    for snippet in snapshot.embedded_snippets {
        if let DefinitionReference::LocalEmbedded { range, .. } = &definition_ref {
            let offset = snippet.parsed_source.content_offset(&db);
            let parent_range = *range + offset;
            if !snippet
                .parsed_source
                .content_range(&db)
                .contains_range(parent_range)
            {
                continue;
            }
        }

        let Some(resolve_definition) = snippet.resolve_definition else {
            continue;
        };

        let result = resolve_definition(ResolveDefinitionParams {
            path: &path,
            definition_ref: &definition_ref,
            workspace_db: db.clone(),
            parsed_source: snippet.parsed_source.into(),
        });

        if let Some(result) = result
            && !result.matches.is_empty()
        {
            return Ok(Some(result));
        }
    }

    let Some(resolve_definition) = snapshot.resolve_definition else {
        return Err(snapshot
            .capability_error
            .unwrap_or_else(WorkspaceError::feature_not_enabled));
    };

    Ok(resolve_definition(ResolveDefinitionParams {
        path: &path,
        definition_ref: &definition_ref,
        workspace_db: db,
        parsed_source: parse.into(),
    }))
}

fn run_search_pattern(snapshot: SearchPatternSnapshot) -> Result<SearchResults, WorkspaceError> {
    let settings = SettingsHandle::new(
        &snapshot.settings,
        (Option::<Configuration>::None, EditorFeatures::default()),
    );
    let matches = (snapshot.search)(
        &snapshot.path,
        &snapshot.document_file_source,
        snapshot.parse.into(),
        snapshot.provider.as_ref(),
        &settings,
        snapshot.pattern,
        snapshot.db,
    )?;

    Ok(SearchResults {
        path: snapshot.path,
        matches,
    })
}

#[cfg(test)]
mod tests {
    use super::{StagedReadOnlySnapshot, StagedSnapshotRun, StagedWorkerTask, test_worker_task};
    use crate::WorkspaceError;
    use std::panic::resume_unwind;

    struct TestStagedSnapshot {
        pending_write: bool,
    }

    impl StagedReadOnlySnapshot<&'static str, &'static str> for TestStagedSnapshot {
        fn stale_payload(&self) -> &'static str {
            "stale payload"
        }

        fn run(self) -> Result<&'static str, WorkspaceError> {
            if self.pending_write {
                resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
            }

            Ok("commit")
        }
    }

    #[test]
    fn worker_task_runs_owned_closure() {
        let task = test_worker_task(|| Ok::<_, WorkspaceError>(42));

        assert_eq!(task.run().unwrap(), 42);
    }

    #[test]
    fn worker_task_maps_salsa_cancellation_to_workspace_error() {
        let task = test_worker_task(|| -> Result<(), WorkspaceError> {
            resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
        });

        let error = task.run().unwrap_err();
        let WorkspaceError::Cancelled(cancelled) = error else {
            panic!("expected cancellation error, got {error}");
        };

        assert_eq!(cancelled.reason, "cancelled because of pending write");
    }

    #[test]
    #[should_panic(expected = "regular panic")]
    fn worker_task_propagates_regular_panics() {
        let task = test_worker_task(|| -> Result<(), WorkspaceError> {
            panic!("regular panic");
        });

        let _ = task.run();
    }

    #[test]
    fn staged_worker_task_maps_pending_write_to_stale() {
        let task = StagedWorkerTask::from_snapshot(TestStagedSnapshot {
            pending_write: true,
        });

        let result = task.run().unwrap();
        let StagedSnapshotRun::Stale(payload) = result else {
            panic!("expected stale result");
        };

        assert_eq!(payload, "stale payload");
    }

    #[test]
    fn staged_worker_task_returns_commit() {
        let task = StagedWorkerTask::from_snapshot(TestStagedSnapshot {
            pending_write: false,
        });

        let result = task.run().unwrap();
        let StagedSnapshotRun::Commit(commit) = result else {
            panic!("expected commit result");
        };

        assert_eq!(commit, "commit");
    }
}

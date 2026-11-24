use super::{
    document::{Document, DocumentView},
    views::new_views_map,
    *,
};
use crate::Watcher;
use crate::configuration::{LoadedConfiguration, ProjectScanComputer, read_config};
use crate::db::{DbReadGuard, DbState, ViewId, WorkspaceDb};
use crate::diagnostics::{
    FileTooLarge, NoIgnoreFileFound, TailwindStylesheetNotFound, VcsDiagnostic,
};
use crate::embed::EmbedContent;
#[cfg(feature = "lang_js")]
use crate::file_handlers::AstroFileHandler;
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DiagnosticsAndActionsParams, Features, FixAllParams,
    FixedFileResult, LintParams, LintResults, ParseEmbeddedParams, ParseResult, ParsedOrigin,
    ParsedSnippetOrigin, ResolveBindingParams, ResolveDefinitionParams, SnippetsIterator,
    UpdateSnippetsNodes,
};
#[cfg(all(feature = "lang_js", feature = "lang_html"))]
use crate::file_handlers::{SvelteFileHandler, VueFileHandler};
use crate::module_graph::ModuleDependencies;
#[cfg(all(feature = "module_graph", feature = "lang_css"))]
use crate::module_graph::resolve_css_module;
#[cfg(all(feature = "module_graph", feature = "lang_html"))]
use crate::module_graph::resolve_html_module;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
use crate::module_graph::resolve_js_module;
#[cfg(feature = "module_graph")]
use crate::module_graph::{ModuleDb, ModuleInfoKind, module_dependencies};
use crate::projects::{GetFileFeaturesParams, ProjectDb, ProjectKey};
use crate::scanner::{
    IndexRequestKind, IndexTrigger, ProjectScan, ScanOptions, Scanner, ScannerWatcherBridge,
    WatcherInstruction, WorkspaceScannerBridge,
};
use crate::settings::{
    EditorFeature, EditorFeatures, GitIgnoreMatcher, ModuleGraphResolutionKind,
    SettingsEditorState, SettingsHandle, SettingsQuery, SettingsSelectionKey, SettingsWithEditor,
    VcsIgnoredPatterns, settings_without_overrides,
};
use crate::workspace::{
    ChangeFileParams, ChangeFileResult, CheckFileSizeParams, CheckFileSizeResult, CloseFileParams,
    CloseProjectParams, DropPatternParams, FeaturesBuilder, FileContent, FileExistsParams,
    FileFeaturesResult, FixFileParams, FixFileResult, FormatFileParams, FormatOnTypeParams,
    FormatRangeParams, GetControlFlowGraphParams, GetFileContentParams, GetFormatterIRParams,
    GetModuleGraphParams, GetModuleGraphResult, GetRegisteredTypesParams, GetSemanticModelParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, GetTypeInfoParams, GoToDefinitionParams,
    GoToDefinitionResult, IgnoreKind, OpenFileParams, OpenFileResult, OpenProjectParams,
    OpenProjectResult, ParsePatternParams, ParsePatternResult, PathIsIgnoredParams,
    PullActionsParams, PullActionsResult, PullDiagnosticsAndActionsParams,
    PullDiagnosticsAndActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RageEntry,
    RageParams, RageResult, RenameParams, RenameResult, ScanKind, ScanProjectParams,
    ScanProjectResult, SearchPatternParams, SearchResults, ServerInfo, ServiceNotification,
    Settings, SupportsFeatureParams, UpdateModuleGraphParams, UpdateSettingsParams,
    UpdateSettingsResult,
};
use crate::{Workspace, WorkspaceError};
use biome_configuration::bool::Bool;
use biome_configuration::max_size::MaxSize;
use biome_configuration::vcs::VcsClientKind;
use biome_configuration::{BiomeDiagnostic, Configuration, ConfigurationPathHint};
#[cfg(all(feature = "module_graph", feature = "lang_css"))]
use biome_css_syntax::AnyCssRoot;
use biome_db::Source;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::print_diagnostic_to_string;
use biome_diagnostics::{
    Diagnostic, DiagnosticExt, Error, Severity, serde::Diagnostic as SerdeDiagnostic,
};
#[cfg(feature = "html_embeds")]
use biome_embeds::{EmbeddedData, EmbeddedSnippet, collect_embedded_data};
use biome_formatter::rowan::Printed;
use biome_fs::{BiomePath, ConfigName, PathKind, normalize_path, resolve_symbolic_link_target};
#[cfg(all(feature = "module_graph", feature = "lang_html"))]
use biome_html_syntax::HtmlRoot;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
use biome_js_semantic::js_semantic_model;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
use biome_js_syntax::AnyJsRoot;
use biome_json_parser::JsonParserOptions;
#[cfg(feature = "lang_css")]
use biome_languages::CssFileSource;
#[cfg(all(feature = "lang_html", feature = "lang_js"))]
use biome_languages::HtmlFileSource;
#[cfg(feature = "lang_js")]
use biome_languages::JsFileSource;
#[cfg(feature = "lang_js")]
use biome_languages::javascript::{JsEmbeddingKind, ModuleKind};
use biome_languages::{DocumentFileSource, JsonFileSource, LanguageDb};
#[cfg(feature = "lang_js")]
use biome_package::PackageType;
use biome_package::{Catalogs, PackageJson};
use biome_parser::AnyParse;
use biome_parser::db::SourceExt;
use biome_parser::db::{AnalyzedSource, SnippetRef};
use biome_parser::diagnostic::ParseDiagnostic;
#[cfg(feature = "plugins")]
use biome_plugin_loader::Plugins;
#[cfg(feature = "plugins")]
use biome_plugin_loader::{BiomePlugin, PluginCache, PluginDiagnostic};
use biome_project_layout::ProjectLayout;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::NodeCache;
use biome_text_edit::{CompressedOp, DiffOp, TextEdit};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::Sender;
use papaya::{Compute, HashMap, Operation};
use parking_lot::Mutex;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::ops::{Deref, Range};
use std::panic::RefUnwindSafe;
#[cfg(feature = "module_graph")]
use std::rc::Rc;
use std::sync::Arc;
#[cfg(test)]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::watch;
use tracing::{instrument, warn};

pub struct WorkspaceServer {
    /// features available throughout the application
    features: Features,

    /// The layout of projects and their internal packages.
    project_layout: Arc<ProjectLayout>,

    /// Keeps all loaded plugins in memory, per project.
    #[cfg(feature = "plugins")]
    plugin_caches: Arc<HashMap<Utf8PathBuf, PluginCache>>,

    /// Stores the document (text content + version number) associated with a URL
    documents: HashMap<Utf8PathBuf, Document, FxBuildHasher>,

    search_provider: Arc<dyn SearchQuery>,

    /// Scanner instance.
    pub(crate) scanner: Scanner,

    /// File system implementation.
    fs: Arc<dyn FsWithResolverProxy>,

    /// Channel sender for sending notifications of service data updates.
    notification_tx: watch::Sender<ServiceNotification>,

    /// The clients whose files were closed, such as the ones that
    /// disconnected. Requests that a client sent before its files were closed
    /// can still be running, and the files they open or change aren't kept.
    ///
    /// Two operations race when a client disconnects:
    ///
    /// 1. A request of the client publishes a view, then checks whether the
    ///    client is closed.
    /// 2. The close records the client as closed, then removes its views.
    ///
    /// The lock orders them: either the request finds its client closed and
    /// removes the view itself, or the close finds the view and removes it.
    closed_clients: Mutex<FxHashSet<ClientId>>,

    /// Tracks which clients still use each project so one client cannot unload another's context.
    project_clients: Mutex<FxHashMap<ProjectKey, FxHashSet<ClientId>>>,

    #[cfg(test)]
    cancel_change_file_after_document_update: AtomicBool,

    /// Makes the next [Workspace::change_file] close the files of its client
    /// after it reads the document and before it publishes the change.
    #[cfg(test)]
    close_client_files_before_change_publish: AtomicBool,

    /// Pauses a close after it updates the document views.
    #[cfg(test)]
    pause_close_after_document_update: AtomicBool,

    /// Counts closes that reached the pause after updating document views.
    #[cfg(test)]
    close_document_updates: AtomicUsize,

    #[cfg(test)]
    scanner_test_state: ScannerTestState,
}

/// A convenient wrapper around a [WorkspaceServer] that holds salsa database state.
///
/// Used when creating a fresh new server.
pub struct LocalWorkspace {
    server: WorkspaceServer,
    db_state: DbState,
}

/// A workspace server and the Salsa database used by one operation, on
/// behalf of one client.
pub struct WorkspaceServerWithDb<'a> {
    server: &'a WorkspaceServer,
    db_state: &'a DbState,
    /// The client that makes the requests. It reads and writes its own editor
    /// views. See [views](crate::workspace::views).
    client: ClientId,
}

struct ProcessFileState {
    parsed: ParsedOrigin,
    file_source: DocumentFileSource,
    db: WorkspaceDb,
}

/// Synchronization and cancellation controls for scanner concurrency tests.
///
/// The controls expose deterministic boundaries at file-index entry, project
/// settings lookup, file commit, and project-scan entry.
#[cfg(test)]
#[derive(Default)]
struct ScannerTestState {
    /// Number of scanner-driven file indexing attempts.
    file_index_attempts: AtomicUsize,
    /// Number of project settings reads made by scanner-driven file indexing.
    file_settings_read_attempts: AtomicUsize,
    /// Whether scanner file indexing should pause after reaching settings lookup.
    pause_file_settings_read: AtomicBool,
    /// Number of scanner-driven file commit attempts.
    file_commit_attempts: AtomicUsize,
    /// Number of project scan attempts.
    project_scan_attempts: AtomicUsize,
    /// Whether the first project scan should be cancelled before it starts.
    cancel_first_scan_attempt: AtomicBool,
    /// Number of VCS ignore refreshes that read their ignore files and are about to publish.
    vcs_ignore_publish_attempts: AtomicUsize,
    /// Whether VCS ignore refreshes should pause after reading their ignore files.
    pause_vcs_ignore_publish: AtomicBool,
}

#[cfg(test)]
impl ScannerTestState {
    /// Records entry into scanner-driven file indexing.
    fn enter_file_indexing(&self, reason: OpenFileReason) {
        if reason.is_index() {
            self.file_index_attempts.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Records a project settings read made by scanner-driven file indexing.
    fn enter_file_settings_read(&self, reason: OpenFileReason) {
        if reason.is_index() {
            self.file_settings_read_attempts
                .fetch_add(1, Ordering::AcqRel);
            while self.pause_file_settings_read.load(Ordering::Acquire) {
                std::thread::yield_now();
            }
        }
    }

    /// Records a VCS ignore refresh that read its ignore files and is about to publish them.
    fn enter_vcs_ignore_publish(&self) {
        self.vcs_ignore_publish_attempts
            .fetch_add(1, Ordering::AcqRel);
        while self.pause_vcs_ignore_publish.load(Ordering::Acquire) {
            std::thread::yield_now();
        }
    }

    /// Records scanner-driven file commits after parsing and before publication.
    fn enter_file_commit(&self, reason: OpenFileReason) {
        if reason.is_index() {
            self.file_commit_attempts.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Records entry into a project scan and optionally cancels its first attempt.
    ///
    /// The cancellation lets tests verify that [`RetryingWorkspace`] propagates
    /// an interrupted project scan instead of restarting the complete traversal.
    fn enter_project_scan(&self) {
        self.project_scan_attempts.fetch_add(1, Ordering::AcqRel);
        if self.cancel_first_scan_attempt.swap(false, Ordering::AcqRel) {
            std::panic::resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
        }
    }
}

impl ProcessFileState {
    fn iter_snippets(&self) -> SnippetsIterator<'_> {
        match &self.parsed {
            ParsedOrigin::Workspace(AnalyzedSource::Source(source)) => {
                SnippetsIterator::from_source(*source, &self.db)
            }
            ParsedOrigin::Workspace(AnalyzedSource::Snippet(_)) => {
                unreachable!("a process-file state must contain a document")
            }
            ParsedOrigin::Interned { snippets, .. } => SnippetsIterator::Interned(snippets.iter()),
        }
    }

    #[cfg(feature = "html_embeds")]
    fn collect_embedded_data(&self) -> Option<Arc<EmbeddedData>> {
        let ParsedOrigin::Interned {
            parse, snippets, ..
        } = &self.parsed
        else {
            return None;
        };

        Some(Arc::new(collect_embedded_data(
            self.file_source,
            parse,
            snippets
                .iter()
                .filter_map(|snippet| {
                    let ParsedSnippetOrigin::Interned {
                        parse,
                        content,
                        file_source,
                    } = snippet
                    else {
                        return None;
                    };
                    Some(EmbeddedSnippet::new(
                        parse,
                        content.content_range,
                        *file_source,
                    ))
                })
                .collect(),
        )))
    }

    /// Returns `true` when the document, or a snippet whose syntax errors stop
    /// formatting the document, has syntax errors.
    fn has_errors_for_formatting(&self) -> bool {
        self.parsed
            .diagnostics(&self.db)
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
            || self
                .iter_snippets()
                .blocking_formatting(self.file_source)
                .any(|snippet| snippet.has_errors())
    }

    /// Returns `true` when parsing the formatted document can report
    /// diagnostics, for the document or for one of the snippets whose syntax
    /// errors stop formatting the document.
    ///
    /// Formatting doesn't add syntax errors to code that has none, so this is
    /// the case when the parser already reported a diagnostic of any severity.
    /// JSON is the exception: its formatter prints trailing commas when
    /// `json.formatter.trailingCommas` is `all`, and the parser rejects them
    /// unless its options allow them. A JSON document or snippet therefore
    /// always counts.
    fn can_have_parse_diagnostics_after_formatting(&self) -> bool {
        matches!(self.file_source, DocumentFileSource::Json(_))
            || !self.parsed.diagnostics(&self.db).is_empty()
            || self
                .iter_snippets()
                .blocking_formatting(self.file_source)
                .any(|snippet| {
                    !snippet.diagnostics().is_empty()
                        || snippet
                            .file_source(&self.db)
                            .is_none_or(|source| matches!(source, DocumentFileSource::Json(_)))
                })
    }

    /// Counts the syntax errors of the document and of the snippets whose errors
    /// count for it: the snippets that stop formatting the document, and when
    /// `analyzed` is `true`, the snippets that the linter analyzes.
    fn error_count(&self, settings: &Settings, path: &BiomePath, analyzed: bool) -> usize {
        let mut counted: FxHashSet<_> = self
            .iter_snippets()
            .blocking_formatting(self.file_source)
            .map(|snippet| snippet.content_range())
            .collect();
        if analyzed {
            counted.extend(
                self.iter_snippets()
                    .for_analysis(
                        &self.parsed,
                        self.file_source,
                        settings,
                        path.as_path(),
                        &self.db,
                    )
                    .map(|snippet| snippet.content_range()),
            );
        }
        self.parsed
            .diagnostics(&self.db)
            .iter()
            .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
            .count()
            + self
                .iter_snippets()
                .filter(|snippet| counted.contains(&snippet.content_range()))
                .map(|snippet| snippet.error_count())
                .sum::<usize>()
    }
}

/// The resoulution of the operation.
#[cfg(feature = "module_graph")]
enum ResolvedModuleGraphUpdate {
    Upsert {
        kind: ModuleInfoKind,
        dependencies: ModuleDependencies,
        diagnostics: Vec<Error>,
    },
    Remove,
    Noop,
}

impl Deref for LocalWorkspace {
    type Target = WorkspaceServer;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl Deref for WorkspaceServerWithDb<'_> {
    type Target = WorkspaceServer;

    fn deref(&self) -> &Self::Target {
        self.server
    }
}

fn resolve_git_path(base: &Utf8Path, path: &str) -> Utf8PathBuf {
    let path = Utf8Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    };

    normalize_path(&resolved_path)
}

fn resolve_git_info_exclude_path(
    fs: &dyn FsWithResolverProxy,
    directory: &Utf8Path,
) -> Option<Utf8PathBuf> {
    let git_dir = resolve_git_dir(fs, directory)?;
    let git_common_dir = resolve_git_common_dir(fs, &git_dir);

    // Git treats `$GIT_DIR/info/exclude` as repository-local ignore rules. In a
    // linked worktree, Git resolves this path through `commondir`, so the
    // exclude file is shared from the common Git directory.
    Some(git_common_dir.join("info/exclude"))
}

fn read_git_info_exclude_patterns(
    fs: &dyn FsWithResolverProxy,
    path: &Utf8Path,
) -> Option<Vec<String>> {
    fs.read_file_from_path(path)
        .map(|content| content.lines().map(String::from).collect())
        .ok()
}

fn read_ignore_file_patterns(
    fs: &dyn FsWithResolverProxy,
    directory: &Utf8Path,
) -> Option<Vec<String>> {
    let mut patterns = Vec::new();
    let mut found = false;
    for file_name in VcsClientKind::Git.ignore_files() {
        if let Ok(content) = fs.read_file_from_path(&directory.join(file_name)) {
            found = true;
            patterns.extend(content.lines().map(String::from));
        }
    }
    found.then_some(patterns)
}

fn vcs_ignore_ancestor_directories(
    vcs_root: &Utf8Path,
    project_path: &Utf8Path,
) -> Vec<Utf8PathBuf> {
    let mut directories = vec![vcs_root.to_path_buf()];
    let Ok(relative_path) = project_path.strip_prefix(vcs_root) else {
        return directories;
    };

    let mut directory = vcs_root.to_path_buf();
    for component in relative_path.components() {
        directory.push(component.as_str());
        directories.push(directory.clone());
    }
    directories
}

fn read_root_git_ignore_patterns(
    fs: &dyn FsWithResolverProxy,
    directory: &Utf8Path,
) -> Vec<String> {
    let mut patterns = resolve_git_info_exclude_path(fs, directory)
        .as_deref()
        .and_then(|path| read_git_info_exclude_patterns(fs, path))
        .unwrap_or_default();
    patterns.extend(read_ignore_file_patterns(fs, directory).unwrap_or_default());
    patterns
}

/// The ignore rules of a project, read from its VCS root and from the directories between the VCS
/// root and the project folder.
struct ProjectGitIgnores {
    /// The matchers of every ignore file that was read.
    matches: VcsIgnoredPatterns,
    /// The patterns of the ignore files between the VCS root and the project folder, keyed by
    /// their directory.
    ancestor_patterns: Vec<(Utf8PathBuf, Vec<String>)>,
    /// Whether the ignore files of the VCS root contain no pattern.
    root_is_empty: bool,
}

/// Reads the ignore files of `vcs_root` and of the directories between `vcs_root` and
/// `project_path`.
///
/// The walk stops at the first directory that the patterns read so far ignore, because Git doesn't
/// read the ignore files of ignored directories.
fn read_project_git_ignores(
    fs: &dyn FsWithResolverProxy,
    vcs_root: &BiomePath,
    project_path: Option<&Utf8Path>,
) -> Result<ProjectGitIgnores, WorkspaceError> {
    let root_patterns = read_root_git_ignore_patterns(fs, vcs_root.as_path());
    let root_pattern_refs = root_patterns.iter().map(String::as_str).collect::<Vec<_>>();
    let mut matches = VcsIgnoredPatterns::Git {
        root: VcsIgnoredPatterns::git_ignore(vcs_root, root_pattern_refs.as_slice())?,
        nested: vec![],
    };

    let mut ancestor_patterns = Vec::new();
    if let Some(project_path) = project_path {
        for ignore_directory in vcs_ignore_ancestor_directories(vcs_root.as_path(), project_path)
            .into_iter()
            .skip(1)
        {
            let ignore_directory = fs.biome_path(&ignore_directory);
            if matches.is_ignored(&ignore_directory, true) {
                break;
            }
            if let Some(patterns) = read_ignore_file_patterns(fs, ignore_directory.as_path()) {
                let pattern_refs = patterns.iter().map(String::as_str).collect::<Vec<_>>();
                matches.insert_git_match(VcsIgnoredPatterns::git_ignore(
                    &ignore_directory,
                    pattern_refs.as_slice(),
                )?);
                ancestor_patterns.push((ignore_directory.to_path_buf(), patterns));
            }
        }
    }

    Ok(ProjectGitIgnores {
        matches,
        ancestor_patterns,
        root_is_empty: root_patterns.is_empty(),
    })
}

/// Replaces the Git ignore matchers of `settings` with the ignore files of the project.
///
/// Returns whether the ignore files of the VCS root contain no pattern.
fn load_project_git_ignore_patterns(
    fs: &dyn FsWithResolverProxy,
    vcs_root: &BiomePath,
    project_path: Option<&Utf8Path>,
    settings: &mut Settings,
) -> Result<bool, WorkspaceError> {
    let ignores = read_project_git_ignores(fs, vcs_root, project_path)?;
    if settings.vcs_settings.client_kind == Some(VcsClientKind::Git) {
        settings.vcs_settings.ignore_matches = Some(ignores.matches);
    }
    Ok(ignores.root_is_empty)
}

fn resolve_git_dir(fs: &dyn FsWithResolverProxy, directory: &Utf8Path) -> Option<Utf8PathBuf> {
    let dot_git = directory.join(".git");

    // Linked worktrees and submodules use a `.git` file whose `gitdir:` entry
    // points at the real Git directory. Regular repositories use `.git` as the
    // Git directory itself.
    match fs.read_file_from_path(dot_git.as_ref()) {
        Ok(content) => content.lines().find_map(|line| {
            let gitdir = line.strip_prefix("gitdir:")?.trim();
            Some(resolve_git_path(directory, gitdir))
        }),
        Err(_) => fs.path_is_dir(&dot_git).then_some(dot_git),
    }
}

fn resolve_git_common_dir(fs: &dyn FsWithResolverProxy, git_dir: &Utf8Path) -> Utf8PathBuf {
    // A linked worktree's gitdir is worktree-specific, while repository-wide
    // files such as `info/exclude` live under the common Git directory.
    // `commondir` stores that path, relative to `git_dir` when not absolute.
    let Ok(content) = fs.read_file_from_path(git_dir.join("commondir").as_ref()) else {
        return git_dir.to_path_buf();
    };

    content
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map_or_else(
            || git_dir.to_path_buf(),
            |commondir| resolve_git_path(git_dir, commondir),
        )
}

/// The `Workspace` object is long-lived, so we want it to be able to cross
/// unwind boundaries.
/// In return, we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead too hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

/// See [WorkspaceServer] for the unwind-safety rationale.
impl RefUnwindSafe for LocalWorkspace {}

/// See [WorkspaceServer] for the unwind-safety rationale.
impl RefUnwindSafe for WorkspaceServerWithDb<'_> {}

impl WorkspaceServer {
    /// Creates a new [Workspace].
    pub fn new(
        fs: Arc<dyn FsWithResolverProxy>,
        watcher_tx: Sender<WatcherInstruction>,
        notification_tx: watch::Sender<ServiceNotification>,
        search_provider: Arc<dyn SearchQuery>,
        threads: Option<usize>,
    ) -> Self {
        init_thread_pool(threads);

        Self {
            features: Features::new(),
            project_layout: Default::default(),
            #[cfg(feature = "plugins")]
            plugin_caches: Default::default(),
            documents: new_views_map(FxBuildHasher),
            search_provider,
            scanner: Scanner::new(watcher_tx),
            fs,
            notification_tx,
            closed_clients: Default::default(),
            project_clients: Default::default(),
            #[cfg(test)]
            cancel_change_file_after_document_update: AtomicBool::new(false),
            #[cfg(test)]
            close_client_files_before_change_publish: AtomicBool::new(false),
            #[cfg(test)]
            pause_close_after_document_update: AtomicBool::new(false),
            #[cfg(test)]
            close_document_updates: AtomicUsize::new(0),
            #[cfg(test)]
            scanner_test_state: ScannerTestState::default(),
        }
    }

    pub fn with_db_state<'a>(&'a self, db_state: &'a DbState) -> WorkspaceServerWithDb<'a> {
        self.with_client(db_state, ClientId::default())
    }

    /// Returns the workspace that serves the requests of `client`.
    pub fn with_client<'a>(
        &'a self,
        db_state: &'a DbState,
        client: ClientId,
    ) -> WorkspaceServerWithDb<'a> {
        WorkspaceServerWithDb {
            server: self,
            db_state,
            client,
        }
    }

    /// Forgets that the files of `client` were closed. See
    /// [WorkspaceServerWithDb::close_client_files].
    ///
    /// Call this once no request of the client can run anymore, such as when
    /// the session of a connection is dropped, so that the workspace doesn't
    /// keep a record of every client that disconnected.
    pub fn forget_closed_client(&self, client: ClientId) {
        self.closed_clients.lock().remove(&client);
    }

    /// Starts the watcher.
    ///
    /// This method will not return until the watcher stops.
    pub fn start_watcher(&self, db_state: &DbState, watcher: Watcher) {
        self.with_db_state(db_state).start_watcher(watcher);
    }
}

impl LocalWorkspace {
    /// Creates a new local [Workspace] backed by an in-process [WorkspaceServer].
    pub fn new(
        fs: Arc<dyn FsWithResolverProxy>,
        watcher_tx: Sender<WatcherInstruction>,
        notification_tx: watch::Sender<ServiceNotification>,
        search_provider: Arc<dyn SearchQuery>,
        threads: Option<usize>,
    ) -> Self {
        Self {
            db_state: DbState::new(fs.clone()),
            server: WorkspaceServer::new(fs, watcher_tx, notification_tx, search_provider, threads),
        }
    }

    fn as_workspace(&self) -> WorkspaceServerWithDb<'_> {
        self.server.with_db_state(&self.db_state)
    }

    /// Starts the watcher.
    ///
    /// This method will not return until the watcher stops.
    pub fn start_watcher(&self, watcher: Watcher) {
        self.as_workspace().start_watcher(watcher);
    }

    /// Returns the module db for use in tests.
    #[cfg(feature = "testing")]
    pub fn get_module_db_for_test(&self) -> WorkspaceDb {
        self.as_workspace().get_module_db_for_test()
    }

    /// Indexes a list of files into the module graph for test purposes.
    #[cfg(feature = "testing")]
    pub fn index_files_for_test(
        &self,
        project_key: ProjectKey,
        files: impl IntoIterator<Item = (BiomePath, DocumentFileSource)>,
    ) {
        self.as_workspace().index_files_for_test(project_key, files);
    }

    #[cfg(test)]
    fn get_db(&self) -> WorkspaceDb {
        self.as_workspace().get_db().into_untracked_db()
    }

    #[cfg(test)]
    fn open_file_internal(
        &self,
        reason: OpenFileReason,
        params: OpenFileParams,
    ) -> Result<InternalOpenFileResult, WorkspaceError> {
        self.as_workspace().open_file_internal(reason, params)
    }

    #[cfg(test)]
    fn get_parse(&self, path: &Utf8Path) -> Result<Source, WorkspaceError> {
        self.as_workspace().get_parse(&self.fs.biome_path(path))
    }

    pub fn db_get_parse_diagnostics(&self, path: &Utf8Path) -> Vec<ParseDiagnostic> {
        self.as_workspace().db_get_parse_diagnostics(path)
    }

    #[cfg(test)]
    fn get_file_source(
        &self,
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> DocumentFileSource {
        self.as_workspace()
            .get_file_source(&self.fs.biome_path(path), experimental_full_html_support)
    }

    #[cfg(test)]
    fn get_snippets(&self, path: &Utf8Path) -> Vec<SnippetRef> {
        self.as_workspace().get_snippets(path)
    }
}

#[derive(Clone, Debug)]
struct OriginalRangeMap {
    segments: Vec<OriginalRangeSegment>,
    current_len: usize,
}

#[derive(Clone, Debug)]
struct OriginalRangeSegment {
    current: Range<usize>,
    original: Range<usize>,
}

#[derive(Debug)]
struct SourceDiff {
    chunks: Vec<SourceDiffChunk>,
}

#[derive(Debug)]
enum SourceDiffChunk {
    Equal {
        old: Range<usize>,
        new: Range<usize>,
    },
    Changed {
        old: Range<usize>,
        new: Range<usize>,
    },
}

impl SourceDiffChunk {
    fn old_range(&self) -> &Range<usize> {
        match self {
            Self::Equal { old, .. } | Self::Changed { old, .. } => old,
        }
    }

    fn new_range(&self) -> &Range<usize> {
        match self {
            Self::Equal { new, .. } | Self::Changed { new, .. } => new,
        }
    }
}

impl SourceDiff {
    fn new(old: &str, new: &str) -> Self {
        let edit = TextEdit::from_unicode_words(old, new);
        let mut chunks = Vec::new();
        let mut old_offset = 0;
        let mut new_offset = 0;
        let mut changed_start = None;

        for op in &edit {
            let equal_len = match op {
                CompressedOp::DiffOp(DiffOp::Equal { range }) => Some(usize::from(range.len())),
                CompressedOp::EqualLines { line_count } => {
                    let line_count = line_count.get() as usize + 1;
                    Some(
                        old[old_offset..]
                            .split_inclusive('\n')
                            .take(line_count)
                            .map(str::len)
                            .sum(),
                    )
                }
                _ => None,
            };

            if let Some(equal_len) = equal_len {
                if let Some((old_start, new_start)) = changed_start.take() {
                    chunks.push(SourceDiffChunk::Changed {
                        old: old_start..old_offset,
                        new: new_start..new_offset,
                    });
                }
                if equal_len > 0 {
                    chunks.push(SourceDiffChunk::Equal {
                        old: old_offset..old_offset + equal_len,
                        new: new_offset..new_offset + equal_len,
                    });
                    old_offset += equal_len;
                    new_offset += equal_len;
                }
                continue;
            }

            changed_start.get_or_insert((old_offset, new_offset));
            match op {
                CompressedOp::DiffOp(DiffOp::Delete { range }) => {
                    old_offset += usize::from(range.len());
                }
                CompressedOp::DiffOp(DiffOp::Insert { range }) => {
                    new_offset += usize::from(range.len());
                }
                CompressedOp::DiffOp(DiffOp::Equal { .. }) | CompressedOp::EqualLines { .. } => {
                    unreachable!()
                }
            }
        }

        if let Some((old_start, new_start)) = changed_start {
            chunks.push(SourceDiffChunk::Changed {
                old: old_start..old_offset,
                new: new_start..new_offset,
            });
        }

        debug_assert_eq!(old_offset, old.len());
        debug_assert_eq!(new_offset, new.len());
        Self { chunks }
    }
}

impl OriginalRangeSegment {
    fn map_offset(&self, offset: usize, round_up: bool) -> usize {
        let current_len = self.current.len();
        let original_len = self.original.len();
        if current_len == 0 || original_len == 0 {
            return self.original.start;
        }
        let relative = offset.saturating_sub(self.current.start).min(current_len);
        let numerator = relative * original_len;
        let mapped = if round_up {
            numerator.div_ceil(current_len)
        } else {
            numerator / current_len
        };
        self.original.start + mapped
    }

    fn map_range(&self, range: Range<usize>) -> Range<usize> {
        self.map_offset(range.start, false)..self.map_offset(range.end, true)
    }
}

impl OriginalRangeMap {
    fn new(source_len: usize) -> Self {
        Self {
            segments: (source_len > 0)
                .then_some(OriginalRangeSegment {
                    current: 0..source_len,
                    original: 0..source_len,
                })
                .into_iter()
                .collect(),
            current_len: source_len,
        }
    }

    fn map_action_range(
        &self,
        original: &str,
        current: &str,
        fixed: &str,
        diff: &SourceDiff,
        range: TextRange,
    ) -> TextRange {
        let action_range = usize::from(range.start())..usize::from(range.end());
        if let Some(mapped) = Self::map_matching_range(original, current, action_range.clone()) {
            return Self::text_range(mapped);
        }

        let start_line_end = current[action_range.start..]
            .find('\n')
            .map_or(current.len(), |offset| action_range.start + offset + 1);
        let mapped_start =
            Self::map_matching_range(original, current, action_range.start..start_line_end)
                .map_or_else(
                    || self.map_boundary(action_range.start),
                    |range| range.start,
                );
        let end_line_start = current[..action_range.end]
            .rfind('\n')
            .map_or(0, |offset| offset + 1);
        let mapped_end =
            Self::map_matching_range(original, current, end_line_start..action_range.end)
                .map_or_else(|| self.map_boundary(action_range.end), |range| range.end);
        if mapped_start <= mapped_end {
            return Self::text_range(mapped_start..mapped_end);
        }

        let mut mapped = diff.chunks.iter().filter_map(|chunk| {
            let SourceDiffChunk::Changed { old, .. } = chunk else {
                return None;
            };
            let start = old.start.max(action_range.start);
            let end = old.end.min(action_range.end);
            if start >= end {
                return None;
            }
            let changed = &current[start..end];
            let leading = changed.len() - changed.trim_start().len();
            let trailing = changed.len() - changed.trim_end().len();
            let trimmed = start + leading..end - trailing;
            let changed_range = if trimmed.is_empty() {
                start..end
            } else {
                trimmed
            };
            Some(
                Self::map_matching_range(original, current, start..end)
                    .or_else(|| Self::map_matching_range(original, current, changed_range.clone()))
                    .or_else(|| {
                        Self::map_removed_lines(
                            original,
                            current,
                            fixed,
                            action_range.clone(),
                            changed_range.clone(),
                        )
                    })
                    .unwrap_or_else(|| self.map_range(changed_range)),
            )
        });

        let mapped_range = if let Some(first) = mapped.next() {
            mapped.fold(first, |range, next| {
                range.start.min(next.start)..range.end.max(next.end)
            })
        } else {
            self.map_range(action_range)
        };
        Self::text_range(mapped_range)
    }

    fn text_range(range: Range<usize>) -> TextRange {
        TextRange::new(
            TextSize::try_from(range.start).expect("source range exceeds 4 GiB"),
            TextSize::try_from(range.end).expect("source range exceeds 4 GiB"),
        )
    }

    fn map_removed_lines(
        original: &str,
        current: &str,
        fixed: &str,
        action_range: Range<usize>,
        changed_range: Range<usize>,
    ) -> Option<Range<usize>> {
        let mut mapped = Vec::new();
        let mut line_start = action_range.start;
        while line_start < action_range.end {
            let line_end = current[line_start..action_range.end]
                .find('\n')
                .map_or(action_range.end, |offset| line_start + offset + 1);
            let line = &current[line_start..line_end];
            let leading = line.len() - line.trim_start().len();
            let trailing = line.len() - line.trim_end().len();
            let trimmed = line_start + leading..line_end - trailing;
            if !trimmed.is_empty()
                && trimmed.start < changed_range.end
                && changed_range.start < trimmed.end
            {
                let text = &current[trimmed.clone()];
                if current.match_indices(text).count() > fixed.match_indices(text).count()
                    && let Some(range) = Self::map_matching_range(original, current, trimmed)
                {
                    mapped.push(range);
                }
            }
            line_start = line_end;
        }

        let mut mapped = mapped.into_iter();
        let first = mapped.next()?;
        Some(mapped.fold(first, |range, next| {
            range.start.min(next.start)..range.end.max(next.end)
        }))
    }

    fn map_matching_range(
        original: &str,
        current: &str,
        range: Range<usize>,
    ) -> Option<Range<usize>> {
        let text = current.get(range.clone())?;
        if text.is_empty() {
            return None;
        }
        let current_matches = current
            .match_indices(text)
            .map(|(start, _)| start)
            .collect::<Vec<_>>();
        let original_matches = original
            .match_indices(text)
            .map(|(start, _)| start)
            .collect::<Vec<_>>();
        if current_matches.len() != original_matches.len() {
            return None;
        }
        let occurrence = current_matches
            .iter()
            .position(|start| *start == range.start)?;
        let start = original_matches[occurrence];
        Some(start..start + text.len())
    }

    fn apply(&mut self, old: &str, new: &str, diff: &SourceDiff) {
        let changed = diff
            .chunks
            .iter()
            .enumerate()
            .filter(|(_, chunk)| matches!(chunk, SourceDiffChunk::Changed { .. }))
            .collect::<Vec<_>>();
        let mut used_sources = vec![false; diff.chunks.len()];
        let mut moved_from = vec![None; diff.chunks.len()];

        for (target_index, target) in &changed {
            let target_new = target.new_range();
            if target_new.is_empty() {
                continue;
            }
            let target_text = &new[target_new.clone()];
            if let Some((source_index, source)) = changed.iter().find(|(source_index, source)| {
                if used_sources[*source_index] {
                    return false;
                }
                let source_old = source.old_range();
                !source_old.is_empty() && old[source_old.clone()] == *target_text
            }) {
                used_sources[*source_index] = true;
                moved_from[*target_index] = Some(source.old_range().clone());
            }
        }

        let mut segments = Vec::new();
        for (index, chunk) in diff.chunks.iter().enumerate() {
            match chunk {
                SourceDiffChunk::Equal { old, new } => {
                    self.copy_range(old.clone(), new.start, &mut segments);
                }
                SourceDiffChunk::Changed { old, new } if !new.is_empty() => {
                    if let Some(source) = moved_from[index].clone() {
                        self.copy_range(source, new.start, &mut segments);
                    } else {
                        let original = if old.is_empty() {
                            let anchor = self.map_boundary(old.start);
                            anchor..anchor
                        } else {
                            self.map_range(old.clone())
                        };
                        segments.push(OriginalRangeSegment {
                            current: new.clone(),
                            original,
                        });
                    }
                }
                SourceDiffChunk::Changed { .. } => {}
            }
        }

        self.segments = segments;
        self.current_len = new.len();
    }

    fn copy_range(
        &self,
        range: Range<usize>,
        new_start: usize,
        output: &mut Vec<OriginalRangeSegment>,
    ) {
        for segment in &self.segments {
            let start = segment.current.start.max(range.start);
            let end = segment.current.end.min(range.end);
            if start >= end {
                continue;
            }
            output.push(OriginalRangeSegment {
                current: new_start + start - range.start..new_start + end - range.start,
                original: segment.map_range(start..end),
            });
        }
    }

    fn map_range(&self, range: Range<usize>) -> Range<usize> {
        if range.is_empty() {
            let anchor = self.map_boundary(range.start);
            return anchor..anchor;
        }

        let mut mapped = self.segments.iter().filter_map(|segment| {
            let start = segment.current.start.max(range.start);
            let end = segment.current.end.min(range.end);
            (start < end).then(|| segment.map_range(start..end))
        });
        let Some(first) = mapped.next() else {
            let anchor = self.map_boundary(range.start);
            return anchor..anchor;
        };
        mapped.fold(first, |range, next| {
            range.start.min(next.start)..range.end.max(next.end)
        })
    }

    fn map_boundary(&self, offset: usize) -> usize {
        if let Some(segment) = self
            .segments
            .iter()
            .find(|segment| segment.current.contains(&offset))
        {
            return segment.map_offset(offset, false);
        }
        if offset >= self.current_len {
            return self
                .segments
                .last()
                .map_or(0, |segment| segment.original.end);
        }
        self.segments
            .iter()
            .find(|segment| segment.current.start >= offset)
            .map_or(0, |segment| segment.original.start)
    }
}

impl WorkspaceServerWithDb<'_> {
    fn finish_change_file(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
        parsed: Source,
    ) -> Result<ChangeFileResult, WorkspaceError> {
        let mut final_diagnostics = vec![];

        if self.should_update_service_data(path) {
            let (dependencies, diagnostics) = self.update_service_data(
                path,
                UpdateKind::AddedOrChanged(OpenFileReason::ClientRequest, parsed.into()),
                project_key,
            )?;
            final_diagnostics.extend(
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new),
            );
            if !dependencies.is_empty()
                && let Some(project_path) = self.project_get_path(project_key)
            {
                let diagnostics = self.scanner.index_dependencies(
                    self,
                    project_key,
                    &project_path,
                    dependencies,
                    IndexTrigger::Update,
                )?;
                final_diagnostics.extend(diagnostics);
            }
        }

        Ok(ChangeFileResult {
            diagnostics: final_diagnostics,
        })
    }

    /// Returns a clone of the project database for passing to analyzers.
    fn module_db(&self) -> DbReadGuard {
        self.db_state.fork()
    }

    /// Returns the module db for use in tests.
    #[cfg(feature = "testing")]
    pub fn get_module_db_for_test(&self) -> WorkspaceDb {
        self.module_db().into_untracked_db()
    }

    /// Indexes a list of files into the module graph for test purposes.
    ///
    /// Unlike [`open_file`], this method uses the internal indexing path
    /// (`OpenFileReason::Index`) so that `update_service_data` is triggered,
    /// causing the module graph to be populated. It also accepts an explicit
    /// `document_file_source` per file so that framework files (`.vue`, `.astro`,
    /// `.svelte`) are correctly parsed as HTML-like regardless of the workspace
    /// settings.
    #[cfg(feature = "testing")]
    pub fn index_files_for_test(
        &self,
        project_key: ProjectKey,
        files: impl IntoIterator<Item = (BiomePath, DocumentFileSource)>,
    ) {
        for (path, document_file_source) in files {
            let _ = self.open_file_internal(
                OpenFileReason::Index(IndexTrigger::InitialScan),
                OpenFileParams {
                    project_key,
                    path,
                    content: FileContent::FromServer,
                    document_file_source: Some(document_file_source),
                    persist_node_cache: false,
                    inline_config: None,
                    editor_features: None,
                },
            );
        }
    }

    fn settings_handle_with_query<'a>(
        &self,
        settings: &'a Settings,
        editor_features: EditorFeatures,
        query: SettingsQuery,
    ) -> SettingsWithEditor<'a> {
        SettingsHandle::new(
            settings,
            SettingsEditorState::new(query).with_editor_features(editor_features),
        )
    }

    /// LSP language ids cover broad languages (`javascript`, `typescript`) but
    /// not path-specific framework variants like `.svelte.ts`.
    fn should_prefer_path_source(
        document_file_source: DocumentFileSource,
        path_source: DocumentFileSource,
    ) -> bool {
        match (document_file_source, path_source) {
            #[cfg(all(feature = "lang_js", feature = "lang_html"))]
            (DocumentFileSource::Js(_), DocumentFileSource::Html(_)) => true,
            #[cfg(feature = "lang_js")]
            (DocumentFileSource::Js(_), DocumentFileSource::Js(path_source)) => {
                !matches!(path_source.as_embedding_kind(), JsEmbeddingKind::None)
            }
            #[cfg(feature = "lang_json")]
            (DocumentFileSource::Json(_), DocumentFileSource::Json(path_source)) => {
                !path_source.kind().is_regular()
            }
            _ => false,
        }
    }

    #[cfg(all(feature = "lang_js", feature = "lang_html"))]
    fn virtual_full_html_source(source: DocumentFileSource) -> DocumentFileSource {
        let DocumentFileSource::Js(js) = source else {
            return source;
        };
        let embedding_kind = js.as_embedding_kind();
        if embedding_kind.is_astro() {
            HtmlFileSource::astro().into()
        } else if embedding_kind.is_vue() {
            HtmlFileSource::vue().into()
        } else if embedding_kind.is_svelte() {
            HtmlFileSource::svelte().into()
        } else {
            source
        }
    }

    /// Starts the watcher.
    ///
    /// This method will not return until the watcher stops.
    pub fn start_watcher(&self, mut watcher: Watcher) {
        watcher.run(&ScannerWatcherBridge::new((&self.scanner, self)));
    }

    /// Attempts to find the root of a project by searching upwards from the
    /// given `path`.
    ///
    /// The root of a project is where the top-level `biome.json` resides. So
    /// the returned path is always the given path or one of its parents.
    ///
    /// An error may be returned if no top-level `biome.json` can be found, or
    /// if there is an error opening a config file.
    fn find_project_root(&self, path: BiomePath) -> Result<Utf8PathBuf, WorkspaceError> {
        let path: Utf8PathBuf = path.into();

        for ancestor in path.ancestors() {
            let Some(config_path) = self.get_config_file(ancestor) else {
                continue;
            };

            let content = self.fs.read_file_from_path(&config_path)?;
            let extension = config_path
                .extension()
                .ok_or_else(|| BiomeDiagnostic::invalid_configuration("Missing extension"))?;
            let file_source = JsonFileSource::try_from_extension(extension)
                .map_err(|err| BiomeDiagnostic::invalid_configuration(err.to_string()))?;
            let parser_options = JsonParserOptions::from(&file_source);
            let deserialized: Deserialized<Configuration> =
                deserialize_from_json_str(&content, parser_options, "config");
            if let Some(error) = deserialized
                .diagnostics()
                .iter()
                .find(|diagnostic| diagnostic.severity() == Severity::Error)
            {
                return Err(
                    BiomeDiagnostic::invalid_configuration(print_diagnostic_to_string(error))
                        .into(),
                );
            }

            if let Some(configuration) = deserialized.into_deserialized() {
                let found = configuration.root.is_none_or(|root| root.value());
                // Found our root config!
                if found {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(WorkspaceError::not_found(path.to_string()))
    }

    /// Checks whether the directory identified by the given `path` contains a
    /// Biome configuration file, and returns its path if found.
    fn get_config_file(&self, path: &Utf8Path) -> Option<Utf8PathBuf> {
        for config_file in ConfigName::file_names() {
            let mut config_path = path.to_path_buf();
            config_path.push(config_file);
            if self.fs.path_exists(&config_path) {
                return Some(config_path);
            }
        }

        None
    }

    /// Gets the supported capabilities for a given file path.
    fn get_file_capabilities(
        &self,
        path: &BiomePath,
        experimental_full_html_support: bool,
    ) -> Capabilities {
        let language = self.get_file_source(path, experimental_full_html_support);
        self.features.get_deprecated_capabilities(language)
    }

    /// Returns an error factory function for unsupported features at a given
    /// path.
    fn build_capability_error<'a>(
        &'a self,
        path: &'a BiomePath,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            // For simplicity and avoid too many changes, we hardcode the support to false
            let file_source = self.get_file_source(path, false);

            let language = DocumentFileSource::from_path(path.as_path(), false).or(file_source);
            WorkspaceError::source_file_not_supported(
                language,
                path.to_string(),
                path.extension().map(str::to_string),
            )
        }
    }

    #[instrument(
        level = "debug",
        skip(self, params),
        fields(path = display(&params.path))
    )]
    fn open_file_internal(
        &self,
        reason: OpenFileReason,
        params: OpenFileParams,
    ) -> Result<InternalOpenFileResult, WorkspaceError> {
        if params.path.is_virtual() && matches!(&params.content, FileContent::FromServer) {
            return Err(WorkspaceError::not_found(params.path.to_string()));
        }

        #[cfg(test)]
        self.scanner_test_state.enter_file_indexing(reason);

        let OpenFileParams {
            project_key,
            path: biome_path,
            content,
            document_file_source,
            persist_node_cache,
            inline_config,
            editor_features: _,
        } = params;
        let path: Utf8PathBuf = biome_path.clone().into();

        // Detecting the source of a path is costly enough that this function
        // does it at most once without and once with full HTML support.
        let default_path_source = self.file_source_from_path(&path, false);

        if document_file_source.is_none() && !default_path_source.is_readable() {
            if reason.is_index()
                && path
                    .file_name()
                    .is_some_and(|filename| filename == "pnpm-workspace.yaml")
                && let Some(workspace_root) = path.parent()
            {
                self.refresh_pnpm_workspace_catalogs_for_scope(project_key, workspace_root);
            }
            return Ok(Default::default());
        }

        let (_, settings, query) = {
            #[cfg(test)]
            self.scanner_test_state.enter_file_settings_read(reason);
            let db = self.get_db();
            self.project_get_settings_query(&db, project_key, &biome_path, inline_config.clone())
        }
        .ok_or_else(WorkspaceError::no_project)?;
        let effective_settings = query.effective_settings(&settings);

        let html_ish_supported = effective_settings.experimental_full_html_support_enabled();
        let path_source = if html_ish_supported {
            self.file_source_from_path(&path, true)
        } else {
            default_path_source
        };

        #[cfg_attr(not(feature = "lang_js"), expect(unused_mut))]
        let mut source = if let Some(document_file_source) = document_file_source {
            #[cfg(all(feature = "lang_js", feature = "lang_html"))]
            let document_file_source =
                if html_ish_supported && self.is_virtual_document(&biome_path) {
                    Self::virtual_full_html_source(document_file_source)
                } else {
                    document_file_source
                };

            // LSP language ids cannot encode path-specific framework variants like
            // `.svelte.ts` / `.svelte.js`, and HTML full support can also upgrade
            // JS-like hints to HTML. Prefer the path-derived source only when it
            // carries richer semantics than the editor-provided JS hint.
            if Self::should_prefer_path_source(document_file_source, path_source) {
                path_source
            } else {
                document_file_source
            }
        } else {
            path_source
        };

        // The parse applies the variants that depend on the parser settings,
        // such as JSX everywhere, so a settings change parses the text again
        // with the new variants. The module kind depends on the package
        // manifest of the file, which the database doesn't hold, so it's
        // chosen here.
        #[cfg(feature = "lang_js")]
        if let DocumentFileSource::Js(js) = &mut source {
            match path.extension() {
                Some("js") if !js.is_svelte_source_module() => {
                    let manifest = self.project_layout.find_node_manifest_for_path(&path);
                    if let Some((_, manifest)) = manifest
                        && manifest.r#type == Some(PackageType::CommonJs)
                    {
                        js.set_module_kind(ModuleKind::Script);
                    }
                }
                Some("cjs") => {
                    js.set_module_kind(ModuleKind::Script);
                }
                _ => {}
            }
        }

        let (content, version) = match content {
            FileContent::FromClient { content, version } => (content, Some(version)),
            FileContent::FromServer => (self.fs.read_file_from_path(&path)?, None),
        };
        let content = Arc::<str>::from(content);
        // The content of a client goes to the editor view, and the content read
        // from the filesystem goes to the disk view, which the editor view
        // shadows.
        let view = if version.is_some() {
            ViewId::Editor(self.client)
        } else {
            ViewId::Disk
        };

        let size = content.len();
        let limit = effective_settings.get_max_file_size(&path);

        let text = if size > limit {
            Err(FileTooLarge { size, limit })
        } else if document_file_source.is_none() && !default_path_source.is_parsable() {
            Ok(None)
        } else {
            // The text is parsed the first time a query reads it, so a file
            // source that can't be parsed must be rejected here.
            self.features
                .get_deprecated_capabilities(source)
                .parser
                .parse
                .ok_or_else(self.build_capability_error(&biome_path))?;
            if persist_node_cache {
                self.db_state.retain_node_cache(&path);
            }
            Ok(Some(content.clone()))
        };

        #[cfg(test)]
        self.scanner_test_state.enter_file_commit(reason);

        let file_source_index = self.db_state.insert_file_source(source);
        let mut parsed_source = None;
        let syntax = match text {
            Err(error) => Some(Err(error)),
            Ok(None) => None,
            Ok(Some(text)) => {
                // The project layout and module resolution depend on the
                // manifests and on the JSON files that the resolver reads,
                // such as extended TypeScript configurations. The files that a
                // scan reads later need them, so a scan publishes them right
                // away instead of deferring them, except in its setup phase.
                let can_defer = self.db_state.defers_manifests()
                    || (!biome_path.is_manifest()
                        && !matches!(source, DocumentFileSource::Json(_)));
                parsed_source = if can_defer {
                    self.db_state.update_file_view_or_defer(
                        &path,
                        view,
                        text,
                        file_source_index,
                        inline_config.clone(),
                    )
                } else {
                    Some(self.db_state.update_file_view(
                        &path,
                        view,
                        text,
                        file_source_index,
                        inline_config.clone(),
                    ))
                };
                Some(Ok(()))
            }
        };

        let virtual_source = if self.is_virtual_document(&biome_path) && parsed_source.is_some() {
            self.db_state
                .fork()
                .get_client_parsed_source(path.as_path(), self.client)
        } else {
            None
        };
        if let Some(source) = virtual_source {
            self.db_state.associate_source_with_project(
                path.as_path(),
                self.client,
                source,
                project_key,
            );
        }

        let is_indexed = if
        // Dependency files can be skipped altoghether
        (biome_path.is_dependency() && !biome_path.is_manifest())
            || (
                // If the request is for indexing, we don't insert any document
                // unless the document isn't ignored.
                // That's usually the case when we want to index a manifest file that belongs to the project.
                reason.is_index()
                    && self.is_path_ignored(PathIsIgnoredParams {
                        project_key,
                        path: biome_path.clone(),
                        is_dir: false,
                        features: FeaturesBuilder::new().with_all().build(),
                        ignore_kind: IgnoreKind::Ancestors,
                    })?
            ) {
            true
        } else {
            let document_view = DocumentView {
                content: content.clone(),
                version,
                file_source_index,
                inline_config: inline_config.map(Arc::new),
                syntax: syntax.clone(),
            };
            self.documents.pin().update_or_insert_with(
                path.clone(),
                |current| current.clone().with(view, document_view.clone()),
                || Document::new(view, document_view.clone()),
            );

            // `open_file_internal` must update project data for both index
            // requests and client requests for files that are already indexed.
            reason.is_index() || self.should_update_service_data(&biome_path)
        };

        if view == ViewId::Editor(self.client) && self.is_client_closed() {
            self.close_editor_view(&path, Some(project_key))?;
            return Ok(InternalOpenFileResult::default());
        }

        // Manifest files need to update the module graph. A scan defers a file
        // when updating its inputs needs Salsa setters, and indexes the file
        // again in its next phase.
        if is_indexed {
            if let Some(root) = parsed_source {
                let (dependencies, diagnostics) = self.update_service_data(
                    &biome_path,
                    UpdateKind::AddedOrChanged(reason, root.into()),
                    project_key,
                )?;

                return Ok(InternalOpenFileResult {
                    dependencies,
                    diagnostics,
                });
            }

            // pnpm-workspace.yaml is not a parsed manifest, but updates still need to
            // re-apply catalogs to indexed package.json manifests in the same scope.
            if path
                .file_name()
                .is_some_and(|filename| filename == "pnpm-workspace.yaml")
                && let Some(workspace_root) = path.parent()
            {
                self.refresh_pnpm_workspace_catalogs_for_scope(project_key, workspace_root);
            }
        }

        // If the document was never opened by the scanner, we don't care
        // about updating service data.
        Ok(InternalOpenFileResult::default())
    }

    /// Returns whether a client update of `path` must update the service data.
    ///
    /// This is the case for indexed files, and for the documents the resolver
    /// reads, such as a TypeScript configuration referenced by an indexed one.
    fn should_update_service_data(&self, path: &BiomePath) -> bool {
        #[cfg(feature = "module_graph")]
        if self.db_state.is_resolver_manifest(path.as_path()) {
            return true;
        }
        self.is_indexed(path.as_path())
    }

    fn associate_virtual_document_with_project(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
    ) -> Result<(), WorkspaceError> {
        if !self.is_virtual_document(path) {
            return Ok(());
        }
        let source = {
            let db = self.db_state.fork();
            if db.get_project(&project_key).is_none() {
                return Err(WorkspaceError::no_project());
            }
            db.get_client_parsed_source(path.as_path(), self.client)
                .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(path)))?
        };
        self.db_state.associate_source_with_project(
            path.as_path(),
            self.client,
            source,
            project_key,
        );
        Ok(())
    }

    /// Retrieves the parser result for a given file.
    ///
    /// Returns an error if no file exists in the workspace with this path.
    fn get_parse(&self, path: &BiomePath) -> Result<Source, WorkspaceError> {
        self.assert_parse(path)?;

        let db = self.db_state.fork();
        db.get_client_parsed_source(path.as_path(), self.client)
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(path)))
    }

    fn get_parse_for_project(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
    ) -> Result<Source, WorkspaceError> {
        self.associate_virtual_document_with_project(project_key, path)?;
        self.get_parse(path)
    }

    /// Tries to resolve a binding reference at the given cursor offset.
    ///
    /// First checks if checks if the binding is inside its main parsed root, then it
    /// checks if it's inside its snippets.
    ///
    /// Returns the definition reference (if found), the effective path for the binding,
    /// the capabilities that produced it (needed for calling `resolve_definition`).
    fn resolve_binding_in_document_or_snippets(
        &self,
        path: &BiomePath,
        cursor_offset: TextSize,
        parse: &Source,
        language: DocumentFileSource,
    ) -> Result<(Option<DefinitionReference>, Capabilities), WorkspaceError> {
        let workspace_db = self.get_db();
        let capabilities = self.features.get_deprecated_capabilities(language);

        // Resolve the correct capabilities for the definition side based on
        // the definition reference kind (e.g., CssClass -> CSS handler, Local -> Same handler).
        let resolve_capabilities =
            |def_ref: &Option<DefinitionReference>, default_caps: Capabilities| -> Capabilities {
                match def_ref {
                    None => default_caps,
                    Some(def_ref) => match def_ref {
                        DefinitionReference::Local { .. }
                        | DefinitionReference::Import { .. }
                        | DefinitionReference::HtmlComponent { .. } => default_caps,
                        DefinitionReference::LocalEmbedded { to_language, .. } => match to_language
                        {
                            LocalEmbeddedLanguage::Js => {
                                #[cfg(feature = "lang_js")]
                                {
                                    self.features.get_deprecated_capabilities(
                                        DocumentFileSource::Js(JsFileSource::tsx()),
                                    )
                                }
                                #[cfg(not(feature = "lang_js"))]
                                {
                                    self.features
                                        .get_deprecated_capabilities(DocumentFileSource::Unknown)
                                }
                            }
                        },
                        // Html components are defined in JavaScript part of the file, so we need a JS handler.
                        DefinitionReference::CssClass { .. } => {
                            #[cfg(feature = "lang_css")]
                            {
                                self.features
                                    .get_deprecated_capabilities(DocumentFileSource::Css(
                                        CssFileSource::css(),
                                    ))
                            }
                            #[cfg(not(feature = "lang_css"))]
                            {
                                self.features
                                    .get_deprecated_capabilities(DocumentFileSource::Unknown)
                            }
                        }
                    },
                }
            };

        if let Some(resolve_binding) = capabilities.editors.resolve_binding {
            let result = resolve_binding(ResolveBindingParams {
                parsed_source: parse.into(),
                cursor_offset,
                path: path.to_path_buf(),
                workspace_db: workspace_db.clone_untracked_db(),
            });
            if result.is_some() {
                let capabilities = resolve_capabilities(&result, capabilities);
                return Ok((result, capabilities));
            }
        }

        // Check if cursor falls within an embedded snippet
        let snippets: Vec<_> = parse
            .snippets(&*workspace_db)
            .iter()
            .map(|snippet| snippet.to_ref(&*workspace_db))
            .collect();
        for snippet in snippets {
            let snippet_range = snippet.content_range();
            if cursor_offset < snippet_range.start() || cursor_offset >= snippet_range.end() {
                continue;
            }

            let snippet_offset = snippet.content_offset();
            let local_cursor = cursor_offset - snippet_offset;

            let Some(file_source) = workspace_db.source_from_index(snippet.document_source_index())
            else {
                continue;
            };
            let snippet_caps = self.features.get_real_capabilities(file_source);
            let Some(resolve) = snippet_caps.editors.resolve_binding else {
                continue;
            };

            let result = resolve(ResolveBindingParams {
                parsed_source: snippet.into(),
                cursor_offset: local_cursor,
                path: path.to_path_buf(),
                workspace_db: workspace_db.clone_untracked_db(),
            });

            // If binding is Local, adjust range back to parent document coordinates
            let adjusted = result.map(|b| match b {
                DefinitionReference::Local { range } => DefinitionReference::Local {
                    range: range + snippet_offset,
                },
                other => other,
            });

            if adjusted.is_some() {
                let capabilities = resolve_capabilities(&adjusted, snippet_caps);
                return Ok((adjusted, capabilities));
            }
        }

        Ok((None, capabilities))
    }

    /// Parses the language snippets if the current language implements the capability `parser.parse_embedded_nodes`
    fn parse_embedded_language_snippets(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
        any_parse: &AnyParse,
        node_cache: &mut NodeCache,
        settings: &SettingsWithEditor,
    ) -> Result<Vec<(AnyParse, EmbedContent, DocumentFileSource)>, WorkspaceError> {
        let capabilities = self.features.get_deprecated_capabilities(*file_source);
        let Some(parse_embedded_nodes) = capabilities.parser.parse_embedded_nodes else {
            return Ok(Default::default());
        };
        let result = parse_embedded_nodes(ParseEmbeddedParams {
            any_parse,
            path,
            file_source,
            settings,
            node_cache,
        });

        Ok(result.nodes)
    }

    fn process_file_state_from_server(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
    ) -> Result<ProcessFileState, WorkspaceError> {
        self.associate_virtual_document_with_project(project_key, path)?;
        self.assert_parse(path)?;
        let db = self.get_db().into_untracked_db();
        let parsed = db
            .get_client_parsed_source(path.as_path(), self.client)
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(path)))?;
        let file_source = db
            .source_from_index(parsed.document_source_index(&db))
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(path)))?;
        Ok(ProcessFileState {
            parsed: parsed.into(),
            file_source,
            db,
        })
    }

    fn parse_process_file_state(
        &self,
        path: &BiomePath,
        code: &str,
        file_source: DocumentFileSource,
        settings: &SettingsWithEditor,
        real_capabilities: bool,
    ) -> Result<ProcessFileState, WorkspaceError> {
        #[cfg(feature = "lang_js")]
        let file_source = if matches!(file_source, DocumentFileSource::Js(_))
            && matches!(path.extension(), Some("astro" | "svelte" | "vue"))
        {
            DocumentFileSource::from_path(path.as_path(), false)
        } else {
            file_source
        };
        let capabilities = if real_capabilities {
            self.features.get_real_capabilities(file_source)
        } else {
            self.features.get_deprecated_capabilities(file_source)
        };
        let parse = capabilities
            .parser
            .parse
            .ok_or_else(self.build_capability_error(path))?;
        let mut node_cache = NodeCache::default();
        let ParseResult {
            any_parse,
            language,
        } = parse(path, file_source, code, settings, &mut node_cache);
        let file_source = language.unwrap_or(file_source);

        let mut snippet_cache = NodeCache::default();
        let embedded_snippets = if file_source.may_contain_embeds() {
            self.parse_embedded_language_snippets(
                path,
                &file_source,
                &any_parse,
                &mut snippet_cache,
                settings,
            )?
        } else {
            Vec::new()
        };

        let snippets = embedded_snippets
            .into_iter()
            .map(
                |(parse, content, file_source)| ParsedSnippetOrigin::Interned {
                    parse,
                    content,
                    file_source,
                },
            )
            .collect();

        Ok(ProcessFileState {
            parsed: ParsedOrigin::interned_document(any_parse, snippets),
            file_source,
            db: self.get_db().into_untracked_db(),
        })
    }

    fn reconstruct_legacy_file(
        path: &BiomePath,
        file_source: DocumentFileSource,
        input: &str,
        output: String,
    ) -> String {
        #[cfg(feature = "lang_js")]
        {
            if !matches!(file_source, DocumentFileSource::Js(_)) {
                return output;
            }

            match path.extension() {
                Some("astro") => AstroFileHandler::output(input, &output),
                #[cfg(feature = "lang_html")]
                Some("vue") => VueFileHandler::output(input, &output),
                #[cfg(feature = "lang_html")]
                Some("svelte") => SvelteFileHandler::output(input, &output),
                _ => output,
            }
        }
        #[cfg(not(feature = "lang_js"))]
        {
            let _ = (path, file_source, input);
            output
        }
    }

    fn legacy_diagnostic_offset(
        path: &BiomePath,
        file_source: DocumentFileSource,
        source: &str,
    ) -> Option<u32> {
        #[cfg(feature = "lang_js")]
        {
            if !matches!(file_source, DocumentFileSource::Js(_)) {
                return None;
            }

            match path.extension() {
                Some("astro") => AstroFileHandler::start(source),
                #[cfg(feature = "lang_html")]
                Some("vue") => VueFileHandler::start(source),
                #[cfg(feature = "lang_html")]
                Some("svelte") => SvelteFileHandler::start(source),
                _ => None,
            }
        }
        #[cfg(not(feature = "lang_js"))]
        {
            let _ = (path, file_source, source);
            None
        }
    }

    fn fix_file_state(
        &self,
        params: FixFileParams,
        state: &mut ProcessFileState,
        #[cfg(feature = "module_graph")] module_db: Rc<dyn ModuleDb>,
        collect_final_diagnostics: bool,
    ) -> Result<Option<FixedFileResult>, WorkspaceError> {
        let refreshes_embedded_state = cfg!(feature = "html_embeds")
            && state
                .file_source
                .to_html_file_source()
                .is_some_and(|source| source.is_svelte())
            && self
                .features
                .get_deprecated_capabilities(state.file_source)
                .analyzer
                .update_snippets
                .is_some();
        if !refreshes_embedded_state {
            return self.fix_file_state_once(
                params,
                state,
                #[cfg(feature = "module_graph")]
                module_db,
                collect_final_diagnostics,
            );
        }

        let mut actions = Vec::new();
        let original = state.parsed.send_node(&state.db).into_source_text();
        let mut current = original.clone();
        let mut original_ranges = OriginalRangeMap::new(original.len());
        let mut seen = FxHashSet::default();
        seen.insert(original.clone());

        for _ in 0..100 {
            let Some(result) = self.fix_file_state_once(
                params.clone(),
                state,
                #[cfg(feature = "module_graph")]
                module_db.clone(),
                collect_final_diagnostics,
            )?
            else {
                return Ok(None);
            };
            let FixedFileResult {
                root,
                errors,
                actions: mut pass_actions,
                skipped_suggested_fixes,
            } = result;
            let applied_fixes = !pass_actions.is_empty();
            if !applied_fixes {
                return Ok(Some(FixedFileResult {
                    root,
                    errors,
                    actions,
                    skipped_suggested_fixes,
                }));
            }

            let fixed = root.clone().into_source_text();
            let diff = SourceDiff::new(&current, &fixed);
            for action in &mut pass_actions {
                action.range = original_ranges.map_action_range(
                    &original,
                    &current,
                    &fixed,
                    &diff,
                    action.range,
                );
            }
            actions.append(&mut pass_actions);
            if !seen.insert(fixed.clone()) {
                break;
            }
            original_ranges.apply(&current, &fixed, &diff);
            let (_, settings, query_context) = self
                .project_get_settings_query(
                    &state.db,
                    params.project_key,
                    &params.path,
                    params.inline_config.clone(),
                )
                .ok_or_else(WorkspaceError::no_project)?;
            let experimental_full_html_support = query_context
                .effective_settings(&settings)
                .experimental_full_html_support_enabled();
            let settings = self.settings_handle_with_query(
                &settings,
                EditorFeatures::default(),
                query_context,
            );
            let file_source = state.file_source;
            *state = self.parse_process_file_state(
                &params.path,
                &fixed,
                file_source,
                &settings,
                experimental_full_html_support,
            )?;
            current = fixed;
        }

        Err(WorkspaceError::RuleError(
            biome_analyze::RuleError::ConflictingRuleFixesError {
                rules: actions
                    .iter()
                    .rev()
                    .take(10)
                    .filter_map(|action| action.rule_name.clone())
                    .collect::<FxHashSet<_>>()
                    .into_iter()
                    .collect(),
            },
        ))
    }

    fn fix_file_state_once(
        &self,
        params: FixFileParams,
        state: &mut ProcessFileState,
        #[cfg(feature = "module_graph")] module_db: Rc<dyn ModuleDb>,
        collect_final_diagnostics: bool,
    ) -> Result<Option<FixedFileResult>, WorkspaceError> {
        let FixFileParams {
            project_key,
            path,
            fix_file_mode,
            should_format,
            only,
            skip,
            enabled_rules,
            rule_categories,
            suppression_reason,
            inline_config,
        } = params;
        #[cfg(feature = "html_embeds")]
        let mut embedded_data = state.collect_embedded_data();
        let (working_directory, settings, query_context) = self
            .project_get_settings_query(&state.db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.features.get_deprecated_capabilities(state.file_source);
        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&path))?;
        let plugins = cfg_select! {
            feature = "plugins" => {
                if rule_categories.contains(biome_analyze::RuleCategory::Lint) {
                    self.get_analyzer_plugins_for_project(
                        settings.source_path().unwrap_or_default().as_path(),
                        &settings.get_plugins_for_path(path.as_path()),
                    )
                    .map_err(WorkspaceError::plugin_errors)?
                } else {
                    Vec::new()
                }
            }
            _ => biome_analyze::AnalyzerPluginVec::new(),
        };
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        let mut errors = 0;
        let mut actions = Vec::new();
        let mut skipped_suggested_fixes = 0;

        if let Some(update_snippets) = capabilities.analyzer.update_snippets {
            // Markdown formats its snippets when it formats the whole document,
            // according to `markdown.formatter.formatEmbeds`.
            #[cfg(feature = "lang_md")]
            let should_format =
                should_format && state.file_source.to_markdown_file_source().is_none();
            let embedded_snippets: Vec<_> = state
                .iter_snippets()
                .for_analysis(
                    &state.parsed,
                    state.file_source,
                    settings.as_ref(),
                    path.as_path(),
                    &state.db,
                )
                .collect();
            let mut new_snippets = Vec::new();
            for embedded_snippet in embedded_snippets {
                let Some(document_file_source) = embedded_snippet.file_source(&state.db) else {
                    continue;
                };
                let capabilities = self
                    .features
                    .get_deprecated_capabilities(document_file_source);
                #[cfg(feature = "html_embeds")]
                if state
                    .file_source
                    .to_html_file_source()
                    .is_some_and(|source| source.is_svelte())
                    && document_file_source.to_js_file_source().is_some()
                {
                    continue;
                }
                let Some(fix_all) = capabilities.analyzer.fix_all else {
                    continue;
                };
                let Some(results) = fix_all(FixAllParams {
                    parsed_source: embedded_snippet.parsed_origin(),
                    fix_file_mode,
                    settings: &settings,
                    biome_path: &path,
                    workspace_db: state.db.clone(),
                    #[cfg(feature = "html_embeds")]
                    embedded_data: embedded_data.clone(),
                    #[cfg(feature = "module_graph")]
                    module_db: module_db.clone(),
                    project_layout: self.project_layout.clone(),
                    document_file_source,
                    only: &only,
                    skip: &skip,
                    rule_categories,
                    suppression_reason: suppression_reason.clone(),
                    enabled_rules: &enabled_rules,
                    plugins: plugins.clone(),
                    working_directory: Some(working_directory.as_path()),
                    collect_final_diagnostics,
                })?
                else {
                    continue;
                };

                let FixedFileResult {
                    root,
                    skipped_suggested_fixes: snippet_skipped_suggested_fixes,
                    actions: mut snippet_actions,
                    errors: snippet_errors,
                } = results;
                let reconstruct_snippet = should_format || !snippet_actions.is_empty();
                let snippet_offset = embedded_snippet.content_offset();
                for action in &mut snippet_actions {
                    action.range += snippet_offset;
                }
                let new_code = if reconstruct_snippet {
                    if should_format {
                        let fixed_snippet = ParsedOrigin::interned(
                            AnyParse::from(root),
                            Some(embedded_snippet.content_offset()),
                        );
                        let format = capabilities
                            .formatter
                            .format
                            .ok_or_else(self.build_capability_error(&path))?;
                        format(
                            &path,
                            &document_file_source,
                            fixed_snippet,
                            &settings,
                            state.db.clone(),
                        )?
                        .into_code()
                    } else {
                        root.into_source_text()
                    }
                } else {
                    String::new()
                };
                actions.extend(snippet_actions);
                errors += snippet_errors;
                skipped_suggested_fixes += snippet_skipped_suggested_fixes;
                if reconstruct_snippet {
                    let verbatim_ranges = cfg_select! {
                        feature = "html_embeds" => {
                            if should_format {
                                // Use the trimmed code — the same slice passed to
                                // reindent_embedded_code — so byte offsets match.
                                let trimmed = new_code.trim();
                                match document_file_source {
                                    DocumentFileSource::Js(_) => {
                                        crate::file_handlers::html::js_verbatim_ranges(trimmed)
                                    }
                                    DocumentFileSource::Css(_) => {
                                        crate::file_handlers::html::css_verbatim_ranges(trimmed)
                                    }
                                    _ => vec![],
                                }
                            } else {
                                vec![]
                            }
                        }
                        _ => {
                            vec![]
                        }
                    };
                    new_snippets.push(UpdateSnippetsNodes {
                        range: embedded_snippet.element_range(),
                        new_code,
                        needs_reindent: should_format,
                        verbatim_ranges,
                    });
                }
            }

            if !new_snippets.is_empty() {
                let new_root =
                    update_snippets(state.parsed.clone(), state.db.clone(), new_snippets)?;
                let parse = AnyParse::from(new_root);
                let snippets = self.parse_embedded_language_snippets(
                    &path,
                    &state.file_source,
                    &parse,
                    &mut NodeCache::default(),
                    &settings,
                )?;
                state.parsed = ParsedOrigin::interned_document(
                    parse,
                    snippets
                        .into_iter()
                        .map(
                            |(parse, content, file_source)| ParsedSnippetOrigin::Interned {
                                parse,
                                content,
                                file_source,
                            },
                        )
                        .collect(),
                );
                #[cfg(feature = "html_embeds")]
                {
                    embedded_data = state.collect_embedded_data();
                }
            }
        }

        let Some(fix_result) = fix_all(FixAllParams {
            parsed_source: state.parsed.clone(),
            fix_file_mode,
            settings: &settings,
            biome_path: &path,
            workspace_db: state.db.clone(),
            #[cfg(feature = "html_embeds")]
            embedded_data,
            #[cfg(feature = "module_graph")]
            module_db,
            project_layout: self.project_layout.clone(),
            document_file_source: state.file_source,
            only: &only,
            skip: &skip,
            rule_categories,
            suppression_reason,
            enabled_rules: &enabled_rules,
            plugins,
            working_directory: Some(working_directory.as_path()),
            collect_final_diagnostics,
        })?
        else {
            return Ok(None);
        };

        actions.extend(fix_result.actions);
        errors += fix_result.errors;
        skipped_suggested_fixes += fix_result.skipped_suggested_fixes;

        Ok(Some(FixedFileResult {
            root: fix_result.root,
            errors,
            actions,
            skipped_suggested_fixes,
        }))
    }

    fn format_file_state(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
        state: &ProcessFileState,
        respect_format_with_errors: bool,
    ) -> Result<Option<String>, WorkspaceError> {
        let (_, settings, query_context) = self
            .project_get_settings_query(&state.db, project_key, path, None)
            .ok_or_else(WorkspaceError::no_project)?;
        if respect_format_with_errors
            && !query_context
                .effective_settings(&settings)
                .format_with_errors_enabled_for_this_file_path(path.as_path())
            && state.has_errors_for_formatting()
        {
            return Ok(None);
        }
        let capabilities = self.features.get_deprecated_capabilities(state.file_source);
        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(path))?;
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        let embedded_nodes: Vec<_> = state.iter_snippets().collect();
        let printed = if embedded_nodes.is_empty() {
            format(
                path,
                &state.file_source,
                state.parsed.clone(),
                &settings,
                state.db.clone(),
            )?
        } else {
            let format_embedded = capabilities
                .formatter
                .format_embedded
                .ok_or_else(self.build_capability_error(path))?;
            format_embedded(
                path,
                &state.file_source,
                state.parsed.clone(),
                &settings,
                embedded_nodes,
                state.db.clone(),
            )?
        };

        Ok(Some(printed.into_code()))
    }

    fn pull_diagnostics_for_state(
        &self,
        params: PullDiagnosticsParams,
        state: &ProcessFileState,
        #[cfg(feature = "module_graph")] module_db: Rc<dyn ModuleDb>,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let PullDiagnosticsParams {
            project_key,
            path,
            categories,
            only,
            skip,
            enabled_rules,
            include_code_fix: pull_code_actions,
            inline_config,
            max_diagnostics,
            diagnostic_level,
            enforce_assist,
        } = params;
        let (working_directory, settings, query_context) = self
            .project_get_settings_query(&state.db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.features.get_deprecated_capabilities(state.file_source);
        let parse_errors = state.error_count(
            &settings,
            &path,
            categories.is_lint() || categories.is_assist(),
        );

        let (diagnostics, errors, warnings, infos, skipped_diagnostics) = if (categories.is_lint()
            || categories.is_assist())
            && let Some(lint) = capabilities.analyzer.lint
        {
            #[cfg(feature = "html_embeds")]
            let embedded_data = state.collect_embedded_data();
            let plugins = cfg_select! {
                feature = "plugins" => {
                    if categories.is_lint() {
                        self.get_analyzer_plugins_for_project(
                            settings.source_path().unwrap_or_default().as_path(),
                            &settings.get_plugins_for_path(path.as_path()),
                        )
                        .map_err(WorkspaceError::plugin_errors)?
                    } else {
                        Vec::new()
                    }
                }
                _ => Vec::new(),
            };
            let settings = self.settings_handle_with_query(
                &settings,
                EditorFeatures::default(),
                query_context,
            );
            let LintResults {
                diagnostics,
                errors,
                skipped_diagnostics,
                warnings,
                infos,
            } = lint(LintParams {
                parsed_source: state.parsed.clone(),
                settings: &settings,
                path: &path,
                only: &only,
                skip: &skip,
                language: state.file_source,
                categories,
                rule_execution: Default::default(),
                workspace_db: state.db.clone(),
                #[cfg(feature = "html_embeds")]
                embedded_data,
                #[cfg(feature = "module_graph")]
                module_db,
                project_layout: self.project_layout.clone(),
                suppression_reason: None,
                enabled_selectors: &enabled_rules,
                pull_code_actions,
                plugins,
                working_directory: Some(working_directory.as_path()),
                max_diagnostics,
                diagnostic_level,
                enforce_assist,
            });
            (diagnostics, errors, warnings, infos, skipped_diagnostics)
        } else {
            let mut diagnostics: Vec<_> = state
                .parsed
                .serde_diagnostics(&state.db)
                .into_iter()
                .filter(|diagnostic| diagnostic.severity() >= diagnostic_level)
                .collect();
            let mut errors = diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
                .count();
            for embedded_node in state.iter_snippets().blocking_formatting(state.file_source) {
                let embedded_diagnostics: Vec<_> = embedded_node
                    .serde_diagnostics()
                    .into_iter()
                    .filter(|diagnostic| diagnostic.severity() >= diagnostic_level)
                    .collect();
                errors += embedded_diagnostics
                    .iter()
                    .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
                    .count();
                diagnostics.extend(embedded_diagnostics);
            }
            (diagnostics, errors, 0, 0, 0)
        };

        Ok(PullDiagnosticsResult {
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| {
                    let diagnostic =
                        diagnostic.with_file_path(self.public_document_path_string(&path));
                    SerdeDiagnostic::new(diagnostic)
                })
                .collect(),
            errors,
            warnings,
            infos,
            parse_errors,
            skipped_diagnostics: skipped_diagnostics.into(),
        })
    }

    #[cfg(feature = "plugins")]
    fn load_plugins(
        &self,
        base_path: &Utf8Path,
        plugins: &Plugins,
    ) -> (PluginCache, Vec<PluginDiagnostic>) {
        let mut diagnostics = Vec::new();
        let plugin_cache = PluginCache::default();

        for plugin_config in plugins.iter() {
            let plugin_path = plugin_config.path();
            let includes = plugin_config.includes();
            match BiomePlugin::load_with_package_specifier(
                self.fs.clone(),
                plugin_path,
                base_path,
                includes,
                plugin_config.resolved_package_specifier(),
            ) {
                Ok((plugin, _)) => {
                    plugin_cache.insert_plugin(plugin_config, plugin);
                }
                Err(diagnostic) => diagnostics.push(diagnostic),
            }
        }

        (plugin_cache, diagnostics)
    }

    #[cfg(feature = "plugins")]
    fn get_analyzer_plugins_for_project(
        &self,
        path: &Utf8Path,
        plugins: &Plugins,
    ) -> Result<biome_analyze::AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        match self.plugin_caches.pin().get(path) {
            Some(cache) => cache.get_analyzer_plugins(plugins),
            None => PluginCache::default().get_analyzer_plugins(plugins),
        }
    }

    /// Returns whether the given `path` that falls under the project with the
    /// given `project_key` is ignored, assuming the given `scan_kind` and
    /// `reason`.
    fn is_ignored_by_scanner(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError> {
        if self.project_is_force_ignored(project_key, path) {
            return Ok(true);
        }

        // Determine the ignore kind based on the kind of request.
        let ignore_kind = match request_kind {
            IndexRequestKind::IgnoreFileDiscovery => {
                let path = self.fs.biome_path(path);
                let path_kind = if let Some(path_kind) = path_kind {
                    path_kind
                } else {
                    self.fs.symlink_path_kind(path.as_path())?
                };
                let is_ignored = match path_kind {
                    PathKind::Directory { .. } => {
                        path.is_dependency()
                            || self.project_is_ignored_by_files_config(
                                project_key,
                                path.as_path(),
                                true,
                                IgnoreKind::Ancestors,
                            )
                            || matches!(
                                scan_kind,
                                ScanKind::TargetedKnownFiles {
                                    target_paths,
                                    descend_from_targets,
                                } if !target_paths.iter().any(|target_path| {
                                    target_path.starts_with(path.as_path())
                                        || (*descend_from_targets
                                            && path.starts_with(target_path.as_path()))
                                })
                            )
                    }
                    PathKind::File { is_symlink } => is_symlink || !path.is_ignore(),
                };
                return Ok(is_ignored);
            }

            // For an initial scan, we don't descend into ignored folders.
            // Therefore, we don't need to check if any of the ancestors of the
            // given `path` are ignored. If they were, we wouldn't have been
            // scanning them in the first place. So checking whether the path
            // itself is ignored is enough.
            IndexRequestKind::Explicit(IndexTrigger::InitialScan) => IgnoreKind::Path,

            // For a (watcher) update, we can't rely on the above reasoning.
            // Even though we only install watchers on folders that contain
            // indexed files, such indexed files might've been indexed even
            // though the folder they are in is ignored, as happens when we
            // watch dependencies. Therefore, we take the following approach:
            // - If the path is already indexed, we can assume it's not ignored.
            //   This works for dependencies and non-dependencies alike.
            // - Otherwise, we verify that none of the ancestor folders are
            //   ignored, so we don't accidentally pick up on new files inside
            //   ignored folders.
            IndexRequestKind::Explicit(IndexTrigger::Update) if self.is_indexed(path) => {
                return Ok(false);
            }
            IndexRequestKind::Explicit(IndexTrigger::Update) => IgnoreKind::Ancestors,

            // If the path is a dependency of an indexed file, we accept them
            // under the following conditions:
            // - If the path is inside `node_modules`, we only care about
            //   `package.json`, type declarations, and imported stylesheets.
            //   Runtime JavaScript may be minified and isn't indexed.
            // - The path shouldn't be indexed yet, to avoid double indexing.
            IndexRequestKind::Dependency(_) => {
                let path = self.fs.biome_path(path);
                if path.is_dependency()
                    && !path.is_package_json()
                    && !path.is_type_declaration()
                    && path.extension() != Some("css")
                {
                    return Ok(true);
                }

                return Ok(self.is_indexed(path.as_path()));
            }
        };

        let path = self.fs.biome_path(path);
        let path_kind = if let Some(path_kind) = path_kind {
            path_kind
        } else {
            self.fs.symlink_path_kind(path.as_path())?
        };
        let is_ignored = match path_kind {
            PathKind::Directory { .. } => {
                if path.is_dependency() {
                    // Every mode ignores dependencies, except project mode.
                    return Ok(!scan_kind.is_project() && !scan_kind.is_type_aware());
                }

                if self.project_is_ignored_by_top_level_config(
                    project_key,
                    path.as_path(),
                    true,
                    ignore_kind,
                ) {
                    return Ok(true); // Nobody cares about ignored paths.
                }

                if let ScanKind::TargetedKnownFiles {
                    target_paths,
                    descend_from_targets,
                } = &scan_kind
                    && !target_paths.iter().any(|target_path| {
                        target_path.starts_with(path.as_path())
                            || (*descend_from_targets && path.starts_with(target_path.as_path()))
                    })
                {
                    return Ok(true); // Path is not being targeted.
                }

                false
            }
            PathKind::File { is_symlink } => {
                if is_symlink {
                    return Ok(true); // We never index symlinks.
                }

                match scan_kind {
                    ScanKind::KnownFiles | ScanKind::TargetedKnownFiles { .. } => match ignore_kind
                    {
                        IgnoreKind::Path => !path.is_required_during_scan(),
                        IgnoreKind::Ancestors => path.parent().is_none_or(|folder_path| {
                            self.project_is_ignored_by_top_level_config(
                                project_key,
                                folder_path,
                                true,
                                ignore_kind,
                            )
                        }),
                    },
                    ScanKind::Project | ScanKind::TypeAware => {
                        if path.is_dependency() {
                            // During the initial scan, we only care about
                            // `package.json` files inside `node_modules`, so that
                            // we can build the project layout and resolve
                            // dependencies that lead there. The resolved
                            // dependencies can then be scanned using
                            // `IndexReason::InitialScanDependency`.
                            //
                            // For everything else, dependencies are ignored.
                            request_kind.trigger() != IndexTrigger::InitialScan
                                || !path.is_package_json()
                        } else if path.is_required_during_scan() {
                            match ignore_kind {
                                IgnoreKind::Path => false,
                                IgnoreKind::Ancestors => path.parent().is_none_or(|folder_path| {
                                    self.project_is_ignored_by_top_level_config(
                                        project_key,
                                        folder_path,
                                        true,
                                        ignore_kind,
                                    )
                                }),
                            }
                        } else {
                            self.project_is_ignored_by_top_level_config(
                                project_key,
                                path.as_path(),
                                false,
                                ignore_kind,
                            )
                        }
                    }
                    ScanKind::NoScanner => true,
                }
            }
        };

        Ok(is_ignored)
    }

    /// Attempts to load pnpm workspace catalogs by searching for a
    /// `pnpm-workspace.yaml` starting from the given path and walking up its
    /// ancestors.
    fn load_pnpm_workspace_catalog(&self, start_dir: &Utf8Path) -> Option<Catalogs> {
        for dir in start_dir.ancestors() {
            let workspace_file = dir.join("pnpm-workspace.yaml");
            if !self.fs.path_is_file(&workspace_file) {
                continue;
            }

            if let Ok(content) = self.fs.read_file_from_path(&workspace_file)
                && let Some(catalog) = PackageJson::parse_pnpm_workspace_catalog(&content)
            {
                return Some(catalog);
            }
        }

        None
    }

    /// Applies (or clears) pnpm workspace catalogs for the `package.json`
    /// manifest stored at `package_path`, based on the current project settings.
    fn apply_pnpm_workspace_catalog_to_package(
        &self,
        project_key: ProjectKey,
        package_path: &Utf8Path,
    ) {
        let package_biome_path = self.fs.biome_path(package_path);
        let use_pnpm_workspace_catalogs = self
            .project_get_settings_for_path(project_key, &package_biome_path)
            .is_some_and(|settings| settings.use_pnpm_workspace_catalogs());

        if let Some(mut manifest) = self
            .project_layout
            .get_node_manifest_for_package(package_path)
        {
            if use_pnpm_workspace_catalogs {
                manifest.catalog = self.load_pnpm_workspace_catalog(package_path);
            } else {
                manifest.catalog = None;
            }

            self.project_layout
                .insert_node_manifest(package_path.to_path_buf(), manifest);
        }
    }

    /// Re-applies pnpm workspace catalogs to all packages under `scope_root`.
    fn refresh_pnpm_workspace_catalogs_for_scope(
        &self,
        project_key: ProjectKey,
        scope_root: &Utf8Path,
    ) {
        for package_path in self.project_layout.package_paths() {
            if package_path.starts_with(scope_root) {
                self.apply_pnpm_workspace_catalog_to_package(project_key, &package_path);
            }
        }
    }

    /// Updates the [ProjectLayout] for the given `path`.
    #[instrument(level = "debug", skip(self))]
    fn update_project_layout(
        &self,
        path: &BiomePath,
        update_kind: &UpdateKind,
        project_key: ProjectKey,
    ) -> Result<(), WorkspaceError> {
        let filename = path.file_name();
        if filename.is_some_and(|filename| filename == "package.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(|| WorkspaceError::not_found(path.to_string()))?;

            match update_kind {
                UpdateKind::AddedOrChanged(_, root) => {
                    let send_node = {
                        let db = self.get_db();
                        root.unwrap_as_send_node(&*db)
                    };
                    self.project_layout
                        .insert_serialized_node_manifest(package_path.clone(), &send_node);
                    self.apply_pnpm_workspace_catalog_to_package(project_key, &package_path);
                }
                UpdateKind::Removed => {
                    self.project_layout.remove_package(&package_path);
                }
            }
        } else if filename.is_some_and(|filename| filename == "tsconfig.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(|| WorkspaceError::not_found(path.to_string()))?;

            match update_kind {
                UpdateKind::AddedOrChanged(_, root) => {
                    let send_node = {
                        let db = self.get_db();
                        root.unwrap_as_send_node(&*db)
                    };
                    self.project_layout
                        .insert_serialized_tsconfig(package_path, &send_node);
                }
                UpdateKind::Removed => {
                    self.project_layout
                        .remove_tsconfig_from_package(&package_path);
                }
            }
        } else if let Some(turbo_filename) =
            filename.filter(|f| *f == "turbo.json" || *f == "turbo.jsonc")
        {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(|| WorkspaceError::not_found(path.to_string()))?;

            match update_kind {
                UpdateKind::AddedOrChanged(_, root) => {
                    let send_node = {
                        let db = self.get_db();
                        root.unwrap_as_send_node(&*db)
                    };
                    self.project_layout.insert_serialized_turbo_json(
                        package_path,
                        &send_node,
                        turbo_filename,
                    );
                }
                UpdateKind::Removed => {
                    self.project_layout
                        .remove_turbo_json_from_package(&package_path);
                }
            }
        } else if filename.is_some_and(|filename| filename == "pnpm-workspace.yaml") {
            let workspace_root = path.parent().map_or_default(|parent| parent.to_path_buf());

            for package_path in self.project_layout.package_paths() {
                if !package_path.starts_with(&workspace_root) {
                    continue;
                }

                self.apply_pnpm_workspace_catalog_to_package(project_key, &package_path);
            }
        }

        Ok(())
    }

    /// Updates the given `path` in the module graph.
    ///
    /// Returns the module dependencies of the `path` if `update_kind` is
    /// [`UpdateKind::AddedOrChanged`]. For other signal kinds, no dependencies
    /// are determined.
    #[tracing::instrument(level = "debug", skip(self))]
    #[cfg(feature = "module_graph")]
    fn update_module_graph_internal(
        &self,
        path: &BiomePath,
        update_kind: UpdateKind,
        infer_types: bool,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        let resolved = match update_kind {
            UpdateKind::AddedOrChanged(_, root) => {
                let db = self.get_db();

                'resolve: {
                    #[cfg(feature = "lang_js")]
                    if let Some(js_root) = root.clone().into_language_root::<AnyJsRoot>(&*db)
                        && let Some(semantic_model) = js_semantic_model(&*db, &root)
                    {
                        let semantic_model = Arc::new(semantic_model.clone());
                        let (module_info, dependencies, diagnostics) =
                            resolve_js_module(&*db, js_root, path, semantic_model, infer_types);
                        break 'resolve ResolvedModuleGraphUpdate::Upsert {
                            kind: ModuleInfoKind::Js(module_info),
                            dependencies,
                            diagnostics: diagnostics.into_iter().map(Into::into).collect(),
                        };
                    }

                    #[cfg(feature = "lang_css")]
                    if let Some(css_root) = root.clone().into_language_root::<AnyCssRoot>(&*db) {
                        let (module_info, dependencies, diagnostics) =
                            resolve_css_module(&*db, css_root, path);
                        break 'resolve ResolvedModuleGraphUpdate::Upsert {
                            kind: ModuleInfoKind::Css(module_info),
                            dependencies,
                            diagnostics: diagnostics.into_iter().map(Into::into).collect(),
                        };
                    }

                    #[cfg(feature = "lang_html")]
                    if root.clone().into_language_root::<HtmlRoot>(&*db).is_some() {
                        let (module_info, dependencies, diagnostics) =
                            resolve_html_module(&*db, path)
                                .ok_or_else(|| WorkspaceError::not_found(path.to_string()))?;
                        break 'resolve ResolvedModuleGraphUpdate::Upsert {
                            kind: ModuleInfoKind::Html(module_info),
                            dependencies,
                            diagnostics: diagnostics.into_iter().map(Into::into).collect(),
                        };
                    }

                    let _ = root;
                    ResolvedModuleGraphUpdate::Noop
                }
            }
            UpdateKind::Removed => ResolvedModuleGraphUpdate::Remove,
        };
        self.commit_module_graph_update(path, resolved)
    }

    #[cfg(feature = "module_graph")]
    /// Commits a resolved module graph update.
    ///
    /// This function and everything it calls must not create a database fork.
    fn commit_module_graph_update(
        &self,
        path: &BiomePath,
        update: ResolvedModuleGraphUpdate,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        match update {
            ResolvedModuleGraphUpdate::Upsert {
                kind,
                dependencies,
                diagnostics,
            } => {
                self.db_state.upsert_module_kind(path.to_path_buf(), kind);
                Ok((dependencies, diagnostics))
            }
            ResolvedModuleGraphUpdate::Remove => {
                self.db_state.remove_module(path.as_path());
                Ok(Default::default())
            }
            ResolvedModuleGraphUpdate::Noop => Ok(Default::default()),
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    #[cfg(not(feature = "module_graph"))]
    fn update_module_graph_internal(
        &self,
        _path: &BiomePath,
        _update_kind: UpdateKind,
        _infer_types: bool,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        Ok(Default::default())
    }

    /// Returns a clone of the database. This is usually used to **read** data from it.
    fn get_db(&self) -> DbReadGuard {
        self.db_state.fork()
    }

    /// Updates the state of any services relevant to the given `path`.
    ///
    /// Returns the module dependencies of the `path` if `update_kind` is
    /// [`UpdateKind::AddedOrChanged`]. For other signal kinds, no dependencies
    /// are determined.
    fn update_service_data(
        &self,
        path: &BiomePath,
        update_kind: UpdateKind,
        project_key: ProjectKey,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        // Index requests don't refresh dependencies: an initial scan indexes
        // them itself, and watcher updates are refreshed by the watcher
        // bridge.
        #[cfg(feature = "module_graph")]
        let refresh_dependencies = matches!(
            &update_kind,
            UpdateKind::AddedOrChanged(OpenFileReason::ClientRequest, _)
        ) && self.is_manifest_for_resolution(path);
        // The resolver prefers the parsed source of a manifest to the
        // filesystem, so a removed manifest must not outlive its file unless
        // the client still has it open.
        #[cfg(feature = "module_graph")]
        if matches!(&update_kind, UpdateKind::Removed) && self.is_manifest_for_resolution(path) {
            self.documents
                .pin()
                .compute(path.to_path_buf(), |entry| match entry {
                    Some((_, document)) => document
                        .clone()
                        .without_disk()
                        .map_or(Operation::Remove, Operation::Insert),
                    None => Operation::Abort(()),
                });
            self.db_state.remove_file_view(path.as_path(), ViewId::Disk);
        }
        if path.is_manifest() {
            self.update_project_layout(path, &update_kind, project_key)?;
        }
        let settings = self
            .project_get_settings_for_path(project_key, path)
            .ok_or_else(WorkspaceError::no_project)?;

        match &update_kind {
            UpdateKind::AddedOrChanged(OpenFileReason::Index(IndexTrigger::InitialScan), _) => {
                // We'll send a single signal at the end of the scan.
            }
            _ => {
                let _ = self.notification_tx.send(ServiceNotification::IndexUpdated);
            }
        }
        let result = self.update_module_graph_internal(
            path,
            update_kind,
            settings.module_graph_resolution_kind.is_modules_and_types(),
        )?;

        #[cfg(feature = "module_graph")]
        if refresh_dependencies {
            // Diagnostics of dependencies aren't reported to the client.
            let _diagnostics = self.index_new_dependencies(project_key)?;
        }

        Ok(result)
    }

    /// Returns whether the content of `path` can affect module resolution.
    #[cfg(feature = "module_graph")]
    fn is_manifest_for_resolution(&self, path: &BiomePath) -> bool {
        path.is_manifest() || self.db_state.is_resolver_manifest(path.as_path())
    }

    /// Refreshes the resolver path info of `path`, and of the known paths inside
    /// it, after the watcher reported a change.
    ///
    /// Returns whether the change affects a manifest: `path` itself, or a
    /// manifest inside it.
    #[cfg(feature = "module_graph")]
    fn sync_resolver_path_info(&self, path: &Utf8Path) -> bool {
        let mut changes = self.db_state.resolver_changes_within(path);
        changes.push((
            path.to_path_buf(),
            self.db_state.resolver_changes_for_modified_path(path),
        ));
        if path.file_name() == Some("package.json")
            && let Some(package_path) = path.parent()
        {
            // Installing dependencies doesn't produce watcher events for
            // `node_modules`, so the path info of the packages the manifest can
            // resolve are refreshed when it changes. Package managers may
            // hoist them to the `node_modules` of an ancestor.
            changes.extend(
                self.db_state
                    .resolver_changes_in_visible_node_modules(package_path),
            );
        }
        // A removed or created directory may contain manifests, which only
        // the paths inside it identify.
        let manifest_changed = changes
            .iter()
            .any(|(path, _)| self.is_manifest_for_resolution(&self.fs.biome_path(path)));
        self.db_state.sync_resolver_paths(changes);
        manifest_changed
    }

    /// Indexes the dependencies that became resolvable after a manifest
    /// changed.
    ///
    /// Resolution results are derived from Salsa queries, so only the imports
    /// affected by the manifest are resolved again. Each new dependency is
    /// indexed by the project that contains it, or by `project_key` if no
    /// project contains it.
    #[cfg(feature = "module_graph")]
    fn index_new_dependencies(
        &self,
        project_key: ProjectKey,
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
        let dependencies_by_project = {
            let db = self.get_db();
            let mut modules = Vec::new();
            db.for_each_module(&mut |module| modules.push(module));
            let documents = self.documents.pin();
            let mut dependencies_by_project =
                FxHashMap::<ProjectKey, ModuleDependencies>::default();
            for module in modules {
                for dependency in module_dependencies(&*db, module).iter() {
                    // Dependencies that Biome can't read, or that were already
                    // indexed without becoming modules, can't become modules.
                    if db.contains(dependency)
                        || documents.contains_key(dependency.as_path())
                        || !DocumentFileSource::can_read(dependency)
                    {
                        continue;
                    }
                    let owner = db.find_project_for_path(dependency).unwrap_or(project_key);
                    dependencies_by_project
                        .entry(owner)
                        .or_default()
                        .insert(dependency.clone());
                }
            }
            dependencies_by_project
        };

        let mut diagnostics = Vec::new();
        for (owner, dependencies) in dependencies_by_project {
            if let Some(project_path) = self.project_get_path(owner) {
                diagnostics.extend(self.scanner.index_dependencies(
                    self,
                    owner,
                    &project_path,
                    dependencies,
                    IndexTrigger::Update,
                )?);
            }
        }
        Ok(diagnostics)
    }

    // #region DATABASE OPERATIONS
    // The operations below are CRUD for the database. The goal is to create
    // small functions that we can reduce the lock contention as much as possible.
    //
    // These operations must be prefixed with `db_`.
    // Operation names:
    // - `db_add` for operations that add new inputs
    // - `db_update` for operations that update existing inputs
    // - `db_remove` for operations that remove existing inputs
    // - `db_get` for operations that read data

    /// Purges the path from the database
    fn db_unload_path(&self, path: &Utf8Path) {
        self.db_state.unload_path(path);
    }

    /// Retrieves all diagnostics that belong to a document. It contains diagnostics that belong to embedded snippets too
    pub fn db_get_parse_diagnostics(&self, path: &Utf8Path) -> Vec<ParseDiagnostic> {
        let db = self.get_db();
        db.get_client_parsed_source(path, self.client)
            .map_or_default(|parsed_source| {
                let mut diagnostics = Vec::new();
                diagnostics.extend(parsed_source.parse_diagnostics(&*db).clone());
                diagnostics.extend(parsed_source.snippets_parse_diagnostics(&*db).clone());

                diagnostics
            })
    }

    /// Makes sure that the file was parsed and it is within the configured file limit
    fn assert_parse(&self, path: &BiomePath) -> Result<(), WorkspaceError> {
        let syntax = self
            .documents
            .pin()
            .get(path.as_path())
            .and_then(|doc| doc.syntax(self.client))
            .transpose();

        match syntax {
            Ok(syntax) => match syntax {
                None => Err(WorkspaceError::not_found(
                    self.public_document_path_string(path),
                )),
                Some(_) => Ok(()),
            },
            Err(FileTooLarge { .. }) => Err(WorkspaceError::file_ignored(
                self.public_document_path_string(path),
            )),
        }
    }

    /// Returns the parsed source of `path` and references to its snippets.
    ///
    /// The references are created from `db`, so they resolve only while `db`
    /// is alive.
    fn get_parsed_snippets_and_parse_source(
        &self,
        db: &WorkspaceDb,
        path: &BiomePath,
    ) -> Result<(Source, Vec<SnippetRef>), WorkspaceError> {
        self.assert_parse(path)?;
        let parsed_source = db
            .get_client_parsed_source(path.as_path(), self.client)
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(path)))?;
        let snippets = parsed_source
            .snippets(db)
            .iter()
            .map(|snippet| snippet.to_ref(db))
            .collect();
        Ok((parsed_source, snippets))
    }

    fn file_source_from_path(
        &self,
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> DocumentFileSource {
        let source = DocumentFileSource::from_path(path, experimental_full_html_support);
        if source != DocumentFileSource::Unknown {
            return source;
        }

        resolve_symbolic_link_target(self.fs.as_ref(), path)
            .ok()
            .flatten()
            .map_or(source, |target| {
                DocumentFileSource::from_path(&target, experimental_full_html_support)
            })
    }

    /// Retrieves the supported language of a file.
    fn get_file_source(
        &self,
        path: &BiomePath,
        experimental_full_html_support: bool,
    ) -> DocumentFileSource {
        let db = self.db_state.fork();
        db.get_client_parsed_source(path.as_path(), self.client)
            .and_then(|parsed_source| {
                db.source_from_index(parsed_source.document_source_index(&*db))
            })
            .unwrap_or_else(|| {
                self.file_source_from_path(path.as_path(), experimental_full_html_support)
            })
    }

    #[cfg(test)]
    fn get_snippets(&self, path: &Utf8Path) -> Vec<SnippetRef> {
        let db = self.db_state.fork();
        db.get_client_parsed_source(path, self.client)
            .map_or_default(|source| {
                source
                    .snippets(&*db)
                    .iter()
                    .map(|snippet| snippet.to_ref(&*db))
                    .collect()
            })
    }

    // #endregion

    // #region Project methods
    pub(crate) fn project_get_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.get_project_path(project_key)
    }

    /// Returns VCS ignore files that require exact filesystem watches for a project.
    pub fn project_vcs_watch_paths(&self, project_key: ProjectKey) -> Vec<BiomePath> {
        if let Some(settings) = self.project_get_root_settings(project_key)
            && settings.is_vcs_enabled()
            && settings.vcs_settings.should_use_ignore_file()
            && let Some(client_kind) = settings.vcs_settings.client_kind
            && let Some(vcs_root) = settings
                .vcs_settings
                .ignore_matches
                .as_ref()
                .map(|matches| matches.root_path())
        {
            let project_path = self.project_get_path(project_key);
            let mut paths = project_path
                .as_deref()
                .map_or_else(
                    || vec![vcs_root.to_path_buf()],
                    |project_path| {
                        vcs_ignore_ancestor_directories(vcs_root.as_path(), project_path)
                    },
                )
                .into_iter()
                .flat_map(|directory| {
                    client_kind
                        .ignore_files()
                        .iter()
                        .map(move |file_name| self.fs.biome_path(&directory.join(file_name)))
                })
                .collect::<Vec<_>>();
            if client_kind == VcsClientKind::Git
                && let Some(path) =
                    resolve_git_info_exclude_path(self.fs.as_ref(), vcs_root.as_path())
            {
                paths.push(self.fs.biome_path(&path));
            }
            paths
        } else {
            Vec::new()
        }
    }

    /// Rereads the ignore files of the VCS root and of the directories between the VCS root and
    /// the project folder.
    ///
    /// Only the matchers are published, on top of the settings that are current at publication,
    /// so a concurrent settings update isn't overwritten.
    fn refresh_project_vcs_ignore_settings(
        &self,
        project_key: ProjectKey,
    ) -> Result<(), WorkspaceError> {
        let settings = self
            .project_get_root_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        if !settings.is_vcs_enabled()
            || !settings.vcs_settings.should_use_ignore_file()
            || settings.vcs_settings.client_kind != Some(VcsClientKind::Git)
        {
            return Ok(());
        }
        let Some(vcs_root) = settings
            .vcs_settings
            .ignore_matches
            .as_ref()
            .map(|matches| matches.root_path().clone())
        else {
            return Ok(());
        };
        let project_path = self.project_get_path(project_key);
        let ProjectGitIgnores {
            matches: VcsIgnoredPatterns::Git { root, .. },
            ancestor_patterns,
            ..
        } = read_project_git_ignores(self.fs.as_ref(), &vcs_root, project_path.as_deref())?;
        #[cfg(test)]
        self.scanner_test_state.enter_vcs_ignore_publish();
        self.store_git_ignores(
            project_key,
            Some(root),
            vcs_root.to_path_buf(),
            ancestor_patterns,
        )
    }

    pub(crate) fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.find_project_for_path(path)
    }

    pub(crate) fn project_get_root_settings(
        &self,
        project_key: ProjectKey,
    ) -> Option<Arc<Settings>> {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.get_root_settings(project_key)
    }

    fn project_get_nested_settings(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
    ) -> Option<Arc<Settings>> {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.get_nested_settings(project_key, path)
    }

    fn project_is_registered(&self, project_key: ProjectKey) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.is_project_registered(project_key)
    }

    fn project_is_ignored_by_top_level_config(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        is_dir: bool,
        ignore_kind: IgnoreKind,
    ) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.is_ignored_by_top_level_config(project_key, path, is_dir, ignore_kind)
    }

    fn project_is_ignored_by_files_config(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        is_dir: bool,
        ignore_kind: IgnoreKind,
    ) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.is_ignored_by_files_config(project_key, path, is_dir, ignore_kind)
    }

    fn project_is_ignored(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        is_dir: bool,
        features: FeatureName,
        ignore_kind: IgnoreKind,
    ) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.is_ignored(project_key, path, is_dir, features, ignore_kind)
    }

    fn project_is_force_ignored(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.is_force_ignored(project_key, path)
    }

    fn project_get_file_features(
        &self,
        db: &WorkspaceDb,
        params: GetFileFeaturesParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        db.get_file_features(params)
    }

    fn public_document_path_string(&self, path: &BiomePath) -> String {
        path.to_string()
    }

    fn document_exists(&self, path: &BiomePath) -> bool {
        self.documents
            .pin()
            .get(path.as_path())
            .is_some_and(|document| document.for_client(self.client).is_some())
    }

    fn is_virtual_document(&self, path: &BiomePath) -> bool {
        path.is_virtual()
    }

    fn project_get_settings_for_path(
        &self,
        project_key: ProjectKey,
        path: &BiomePath,
    ) -> Option<Arc<Settings>> {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        if self.is_virtual_document(path) {
            project_db
                .get_root_settings(project_key)
                .map(|settings| Arc::new(settings_without_overrides(settings.as_ref())))
        } else {
            project_db.get_settings_based_on_path(project_key, path.as_path())
        }
    }

    /// Resolves the project settings used to process `path`.
    ///
    /// The first tuple element is the directory of the selected settings: the
    /// deepest nested settings directory containing `path`, or the project root
    /// when no nested settings match. It is not the input file path. The second
    /// element is the selected project settings without inline configuration.
    /// The third is a request-scoped query with the matching override indices;
    /// when `inline_config` is present, this query selects a settings snapshot
    /// created by merging it into the project settings.
    ///
    /// Returns `None` when `project_key` does not identify a registered project.
    fn project_get_settings_query(
        &self,
        db: &WorkspaceDb,
        project_key: ProjectKey,
        path: &BiomePath,
        inline_config: Option<Configuration>,
    ) -> Option<(Utf8PathBuf, Arc<Settings>, SettingsQuery)> {
        if self.is_virtual_document(path) {
            let project = db.get_project(&project_key)?;
            let working_directory = project.path(db).to_path_buf();
            let settings = Arc::new(settings_without_overrides(
                project.root_settings(db).as_ref(),
            ));
            let inline_settings = inline_config.map(|inline_config| {
                settings_without_overrides(&settings.with_inline_configuration(inline_config))
            });
            let query = SettingsQuery::new_without_overrides(
                project,
                SettingsSelectionKey::Root,
                inline_settings,
            );
            return Some((working_directory, settings, query));
        }

        let (project, selection, working_directory, settings) =
            db.get_settings_context_for_path(project_key, path.as_path())?;
        let settings = settings.clone_arc();
        let inline_settings =
            inline_config.map(|inline_config| settings.with_inline_configuration(inline_config));
        let query = SettingsQuery::new(
            project,
            selection,
            &settings,
            inline_settings,
            path.as_path(),
        );
        Some((working_directory, settings, query))
    }

    fn path_belongs_only_to_project_with_path(
        &self,
        path: &Utf8Path,
        project_path: &Utf8Path,
    ) -> bool {
        let db = self.db_state.fork();
        let project_db: &dyn ProjectDb = db.deref();
        project_db.path_belongs_only_to_project_with_path(path, project_path)
    }

    fn insert_project(&self, path: Utf8PathBuf) -> ProjectKey {
        self.db_state.insert_project(path)
    }

    fn remove_project(&self, project_key: ProjectKey) {
        self.db_state.remove_project(project_key);
    }

    fn project_set_nested_settings(
        &self,
        project_key: ProjectKey,
        path: Utf8PathBuf,
        settings: Settings,
    ) {
        self.db_state
            .insert_nested_settings(project_key, path, settings)
    }
    fn project_set_root_settings(&self, project_key: ProjectKey, settings: Settings) {
        self.db_state.insert_root_settings(project_key, settings)
    }

    fn store_git_ignores(
        &self,
        project_key: ProjectKey,
        root_git_ignore: Option<GitIgnoreMatcher>,
        scope: Utf8PathBuf,
        payload: Vec<(Utf8PathBuf, Vec<String>)>,
    ) -> Result<(), WorkspaceError> {
        self.db_state
            .store_git_ignores(project_key, root_git_ignore, scope, payload)
    }

    // #endregion
}

macro_rules! delegate_workspace_methods {
    ($(fn $name:ident($params:ident: $params_ty:ty) -> $return_ty:ty;)*) => {
        $(
            fn $name(&self, $params: $params_ty) -> $return_ty {
                self.as_workspace().$name($params)
            }
        )*
    };
}

// Local workspace callers have no outer service that can retry when a Salsa
// write cancels an operation.
macro_rules! delegate_retrying_workspace_methods {
    ($(fn $name:ident($params:ident: $params_ty:ty) -> $return_ty:ty;)*) => {
        $(
            fn $name(&self, $params: $params_ty) -> $return_ty {
                retry_on_pending_write(|| self.as_workspace().$name($params.clone()))
            }
        )*
    };
}

impl Workspace for LocalWorkspace {
    delegate_workspace_methods! {
        fn open_project(params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError>;
        fn scan_project(params: ScanProjectParams) -> Result<ScanProjectResult, WorkspaceError>;
        fn update_settings(params: UpdateSettingsParams) -> Result<UpdateSettingsResult, WorkspaceError>;
        fn close_project(params: CloseProjectParams) -> Result<(), WorkspaceError>;
        fn open_file(params: OpenFileParams) -> Result<OpenFileResult, WorkspaceError>;
        fn file_exists(params: FileExistsParams) -> Result<bool, WorkspaceError>;
        fn is_path_ignored(params: PathIsIgnoredParams) -> Result<bool, WorkspaceError>;
        fn get_file_content(params: GetFileContentParams) -> Result<String, WorkspaceError>;
        fn check_file_size(params: CheckFileSizeParams) -> Result<CheckFileSizeResult, WorkspaceError>;
        fn format_file(params: FormatFileParams) -> Result<Printed, WorkspaceError>;
        fn format_range(params: FormatRangeParams) -> Result<Printed, WorkspaceError>;
        fn format_on_type(params: FormatOnTypeParams) -> Result<Printed, WorkspaceError>;
        fn fix_file(params: FixFileParams) -> Result<FixFileResult, WorkspaceError>;
        fn close_file(params: CloseFileParams) -> Result<(), WorkspaceError>;
        fn update_module_graph(params: UpdateModuleGraphParams) -> Result<(), WorkspaceError>;
        fn parse_pattern(params: ParsePatternParams) -> Result<ParsePatternResult, WorkspaceError>;
        fn search_pattern(params: SearchPatternParams) -> Result<SearchResults, WorkspaceError>;
        fn drop_pattern(params: DropPatternParams) -> Result<(), WorkspaceError>;
        fn rage(params: RageParams) -> Result<RageResult, WorkspaceError>;
    }

    delegate_retrying_workspace_methods! {
        fn change_file(params: ChangeFileParams) -> Result<ChangeFileResult, WorkspaceError>;
        fn process_file(params: ProcessFileParams) -> Result<ProcessFileResult, WorkspaceError>;
        fn file_features(params: SupportsFeatureParams) -> Result<FileFeaturesResult, WorkspaceError>;
        fn pull_diagnostics(params: PullDiagnosticsParams) -> Result<PullDiagnosticsResult, WorkspaceError>;
        fn pull_actions(params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError>;
        fn pull_diagnostics_and_actions(params: PullDiagnosticsAndActionsParams) -> Result<PullDiagnosticsAndActionsResult, WorkspaceError>;
        fn rename(params: RenameParams) -> Result<RenameResult, WorkspaceError>;
        fn go_to_definition(params: GoToDefinitionParams) -> Result<Option<GoToDefinitionResult>, WorkspaceError>;
        fn get_syntax_tree(params: GetSyntaxTreeParams) -> Result<GetSyntaxTreeResult, WorkspaceError>;
        fn get_control_flow_graph(params: GetControlFlowGraphParams) -> Result<String, WorkspaceError>;
        fn get_formatter_ir(params: GetFormatterIRParams) -> Result<String, WorkspaceError>;
        fn get_type_info(params: GetTypeInfoParams) -> Result<String, WorkspaceError>;
        fn get_registered_types(params: GetRegisteredTypesParams) -> Result<String, WorkspaceError>;
        fn get_semantic_model(params: GetSemanticModelParams) -> Result<String, WorkspaceError>;
        fn get_module_graph(params: GetModuleGraphParams) -> Result<GetModuleGraphResult, WorkspaceError>;
    }

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.server.fs.as_ref()
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }
}

/// The result of the setup phase of a project scan.
enum ScanSetupOutcome {
    /// The scan has no files phase: its scan kind needs only the root
    /// manifest, or the watcher already keeps the index of the project up to
    /// date.
    Done(ScanProjectResult),
    /// The files phase still has to index the files that the setup phase
    /// found.
    Files(ProjectScan),
}

impl WorkspaceServerWithDb<'_> {
    /// Returns this workspace for the same client, reading from `db_state`,
    /// such as the view of a scan.
    fn with_db_state<'a>(&'a self, db_state: &'a DbState) -> WorkspaceServerWithDb<'a> {
        WorkspaceServerWithDb {
            server: self.server,
            db_state,
            client: self.client,
        }
    }

    /// Discovers ignore files before the setup phase of a project scan.
    fn prepare_project_scan(
        &self,
        params: &ScanProjectParams,
    ) -> Result<(Vec<SerdeDiagnostic>, Duration), WorkspaceError> {
        let scan_options = ScanOptions {
            scan_kind: params.scan_kind.clone(),
            force: params.force,
            verbose: params.verbose,
            watch: params.watch,
        };
        self.scanner
            .prepare_project_ignore_files(self, params.project_key, &scan_options)
    }

    /// Runs the setup phase of a project scan: reads the configuration files,
    /// manifests and ignore files of the project.
    fn set_up_project_scan(
        &self,
        ScanProjectParams {
            project_key,
            watch,
            force,
            scan_kind,
            verbose,
        }: ScanProjectParams,
    ) -> Result<ScanSetupOutcome, WorkspaceError> {
        let mut diagnostics = Vec::new();
        if scan_kind.is_none() {
            let path = self
                .project_get_path(project_key)
                .ok_or_else(WorkspaceError::no_project)?;

            let manifest = path.join("package.json");
            if self.fs.path_exists(&manifest) {
                let trigger = IndexTrigger::InitialScan;
                let (_, manifest_diagnostics) =
                    self.index_file(project_key, self.fs.biome_path(&manifest), trigger)?;
                diagnostics.extend(
                    manifest_diagnostics
                        .into_iter()
                        .map(biome_diagnostics::serde::Diagnostic::new),
                );
            }
            return Ok(ScanSetupOutcome::Done(ScanProjectResult {
                diagnostics,
                duration: Duration::from_millis(0),
                configuration_files: vec![],
            }));
        }

        let scan_options = ScanOptions {
            scan_kind,
            force,
            verbose,
            watch,
        };

        Ok(
            match self
                .scanner
                .set_up_project_scan(self, project_key, scan_options)?
            {
                Some(scan) => ScanSetupOutcome::Files(scan),
                None => ScanSetupOutcome::Done(ScanProjectResult {
                    diagnostics,
                    duration: Duration::default(),
                    configuration_files: vec![],
                }),
            },
        )
    }
}

impl WorkspaceServerWithDb<'_> {
    /// Closes the files that the client of this workspace has open, as when it
    /// disconnects. Interrupted operations are retried. The files that the
    /// client opens or changes afterward aren't kept.
    ///
    /// Returns the first error, after attempting to close every file.
    pub fn close_client_files(&self) -> Result<(), WorkspaceError> {
        self.closed_clients.lock().insert(self.client);
        // The editor views of dependencies are in the database only.
        let mut paths: BTreeSet<_> =
            retry_on_pending_write(|| self.get_db().paths_open_in(self.client))
                .into_iter()
                .collect();
        paths.extend(
            self.documents
                .pin()
                .iter()
                .filter(|(_, document)| document.editor(self.client).is_some())
                .map(|(path, _)| path.clone()),
        );
        let mut result = Ok(());
        for path in paths {
            let closed = retry_on_pending_write(|| {
                let project_key = self.find_project_for_path(&path);
                self.close_editor_view(&path, project_key)
            });
            if result.is_ok() {
                result = closed;
            }
        }
        result
    }

    /// Returns whether [Self::close_client_files] closed the files of the
    /// client of this workspace.
    fn is_client_closed(&self) -> bool {
        self.closed_clients.lock().contains(&self.client)
    }

    /// Removes the editor view that the client of this workspace has of
    /// `path`.
    ///
    /// When no client has the file open anymore, the file is unloaded unless
    /// the scanner indexes it. `project_key` selects the project whose
    /// dependencies are indexed again when the file is a manifest.
    fn close_editor_view(
        &self,
        path: &Utf8Path,
        project_key: Option<ProjectKey>,
    ) -> Result<(), WorkspaceError> {
        let client = self.client;
        // The scanner keeps the disk view of an indexed file up to date, so
        // it becomes the content of the file again when no client has it open.
        let is_indexed = self.is_indexed(path);

        let documents = self.documents.pin();
        let result = documents.compute(path.to_path_buf(), |entry| match entry {
            Some((_, document)) => match document.clone().without_editor(client) {
                Some(document) if document.has_editors() || is_indexed => {
                    Operation::Insert(document)
                }
                _ => Operation::Remove,
            },
            None => Operation::Abort(()),
        });
        let is_open = matches!(
            result,
            Compute::Updated { new: (_, document), .. } if document.has_editors()
        );
        #[cfg(test)]
        {
            self.close_document_updates.fetch_add(1, Ordering::AcqRel);
            while self
                .pause_close_after_document_update
                .load(Ordering::Acquire)
            {
                std::thread::yield_now();
            }
        }
        // Removing this client's database view preserves views published by concurrent opens.
        self.db_state.remove_editor_view(path, client, is_indexed);
        if !is_open {
            self.db_state.remove_node_cache(path);
        }

        // Without the client's content, the resolver reads the manifest from
        // the disk view or the filesystem, which may make other dependencies
        // reachable.
        #[cfg(feature = "module_graph")]
        if let Some(project_key) = project_key
            && self.db_state.is_resolver_manifest(path)
        {
            // Diagnostics of dependencies aren't reported to the client.
            let _diagnostics = self.index_new_dependencies(project_key)?;
        }
        #[cfg(not(feature = "module_graph"))]
        let _ = project_key;

        if is_indexed {
            // The services were updated from the client's content, so the file
            // is indexed again from the filesystem.
            self.scanner.reindex_file(path.to_path_buf());
        }

        Ok(())
    }
}

impl Workspace for WorkspaceServerWithDb<'_> {
    fn open_project(&self, params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError> {
        let path = if params.open_uninitialized {
            let path = params.path.to_path_buf();
            self.find_project_root(params.path).unwrap_or(path)
        } else {
            self.find_project_root(params.path)?
        };

        let mut project_clients = self.project_clients.lock();
        let project_key = self.insert_project(path);
        project_clients
            .entry(project_key)
            .or_default()
            .insert(self.client);

        Ok(OpenProjectResult { project_key })
    }

    fn scan_project(&self, params: ScanProjectParams) -> Result<ScanProjectResult, WorkspaceError> {
        #[cfg(test)]
        self.scanner_test_state.enter_project_scan();

        let project_key = params.project_key;
        let watch = params.watch;
        let session = self.db_state.scan_session();
        let preparation = self
            .with_db_state(session.view())
            .prepare_project_scan(&params);
        let (session, _) = session.commit();
        let (preparation_diagnostics, preparation_duration) = preparation?;
        let setup = self
            .with_db_state(session.view())
            .set_up_project_scan(params);
        let (session, deferred_setup_paths) = session.commit();
        let workspace = self.with_db_state(session.view());
        let scan = match setup? {
            ScanSetupOutcome::Done(mut result) => {
                result.diagnostics.extend(preparation_diagnostics);
                result.duration += preparation_duration;
                // Without a files phase, the setup phase indexes only the root
                // manifest, so it's indexed again here if it was deferred.
                for path in deferred_setup_paths {
                    let (_, diagnostics) = workspace.index_file(
                        project_key,
                        workspace.fs.biome_path(&path),
                        IndexTrigger::InitialScan,
                    )?;
                    result.diagnostics.extend(
                        diagnostics
                            .into_iter()
                            .map(biome_diagnostics::serde::Diagnostic::new),
                    );
                }
                return Ok(result);
            }
            ScanSetupOutcome::Files(scan) => scan,
        };

        // The pnpm catalogs of a package depend on its nested settings, and
        // the nested settings are readable only after the setup commit.
        for configuration_file in scan.configuration_files() {
            if let Some(directory) = configuration_file.parent() {
                workspace.refresh_pnpm_workspace_catalogs_for_scope(project_key, directory);
            }
        }
        let mut result = self
            .scanner
            .finish_project_scan(&workspace, scan, deferred_setup_paths);
        result.diagnostics.extend(preparation_diagnostics);
        result.duration += preparation_duration;
        let (session, deferred_paths) = session.commit();
        if deferred_paths.is_empty() {
            return Ok(result);
        }

        // Indexing the deferred files again updates the services that read
        // them. If a deferred file changes again in the meantime, its next
        // indexing updates its services.
        let workspace = self.with_db_state(session.view());
        let project_path = workspace
            .project_get_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let diagnostics = self.scanner.index_files(
            &workspace,
            project_key,
            &project_path,
            deferred_paths,
            IndexTrigger::InitialScan,
            watch,
        )?;
        session.finish();
        result.diagnostics.extend(diagnostics);
        let _ = self.notification_tx.send(ServiceNotification::IndexUpdated);
        Ok(result)
    }

    /// Updates the global settings for this workspace.
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "debug", skip_all)]
    fn update_settings(
        &self,
        params: UpdateSettingsParams,
    ) -> Result<UpdateSettingsResult, WorkspaceError> {
        let UpdateSettingsParams {
            workspace_directory,
            configuration,
            project_key,
            extended_configurations,
            module_graph_resolution_kind,
        } = params;
        let mut diagnostics: Vec<biome_diagnostics::serde::Diagnostic> = vec![];
        let workspace_directory = workspace_directory.map(|p| p.to_path_buf());
        let is_root = configuration.is_root();
        let mut settings = if !is_root {
            if !self.project_is_registered(project_key) {
                return Err(WorkspaceError::no_project());
            }

            if let Some(workspace_directory) = &workspace_directory {
                self.project_get_nested_settings(project_key, workspace_directory.as_path())
                    .map_or_default(|settings| (*settings).clone())
            } else {
                return Err(WorkspaceError::no_workspace_directory());
            }
        } else {
            (*self
                .project_get_root_settings(project_key)
                .ok_or_else(WorkspaceError::no_project)?)
            .clone()
        };
        settings.module_graph_resolution_kind = module_graph_resolution_kind;

        #[cfg(feature = "plugins")]
        let configuration = {
            let mut configuration = configuration;
            let plugin_resolution_base = workspace_directory
                .clone()
                .or_else(|| self.project_get_path(project_key))
                .unwrap_or_default();
            if let Some(plugins) = configuration.plugins.as_mut() {
                plugins
                    .resolve_paths(self.fs.as_ref(), &plugin_resolution_base)
                    .map_err(|diagnostic| WorkspaceError::plugin_errors(vec![diagnostic]))?;
            }
            if let Some(overrides) = configuration.overrides.as_mut() {
                for pattern in overrides.0.iter_mut() {
                    if let Some(plugins) = pattern.plugins.as_mut() {
                        plugins
                            .resolve_paths(self.fs.as_ref(), &plugin_resolution_base)
                            .map_err(|diagnostic| {
                                WorkspaceError::plugin_errors(vec![diagnostic])
                            })?;
                    }
                }
            }
            configuration
        };

        // Resolve the stylesheet from the configuration that declares it. Nested
        // configurations that extend this one then inherit the absolute path.
        let mut configuration = configuration;
        if let Some(stylesheet) = configuration
            .tailwind
            .as_mut()
            .and_then(|tailwind| tailwind.stylesheet.as_mut())
        {
            let base = workspace_directory
                .clone()
                .or_else(|| self.project_get_path(project_key))
                .unwrap_or_default();
            *stylesheet = normalize_path(&base.join(stylesheet.as_str())).into_string();
        }

        settings.merge_with_configuration(
            configuration,
            workspace_directory.clone(),
            extended_configurations
                .into_iter()
                .map(|(path, config)| (path.into(), config))
                .collect(),
        )?;

        if let Some(stylesheet) = settings.tailwind.stylesheet()
            && !self.fs.path_is_file(stylesheet)
        {
            diagnostics.push(biome_diagnostics::serde::Diagnostic::new(
                TailwindStylesheetNotFound {
                    path: stylesheet.to_string(),
                },
            ));
        }

        #[cfg(feature = "plugins")]
        {
            let plugin_base_path = workspace_directory.clone().unwrap_or_default();
            let (plugin_cache, plugin_diagnostics) =
                self.load_plugins(&plugin_base_path, &settings.as_all_plugins());

            let has_errors = plugin_diagnostics
                .iter()
                .any(|d| d.severity() >= Severity::Error);

            if has_errors {
                return Err(WorkspaceError::plugin_errors(plugin_diagnostics));
            }
            self.plugin_caches
                .pin()
                .insert(plugin_base_path, plugin_cache);
            diagnostics.extend(
                plugin_diagnostics
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            );
        }

        if !is_root {
            let nested_workspace_directory = workspace_directory.clone().unwrap_or_default();
            self.project_set_nested_settings(
                project_key,
                nested_workspace_directory.clone(),
                settings.clone(),
            );
            self.refresh_pnpm_workspace_catalogs_for_scope(
                project_key,
                nested_workspace_directory.as_path(),
            );
        } else {
            // If the configuration is a root one, we also load the ignore files
            if settings.is_vcs_enabled() && settings.vcs_settings.should_use_ignore_file() {
                let directory = self.fs.biome_path(
                    &settings
                        .vcs_settings
                        .to_base_path(workspace_directory.as_deref())
                        .unwrap_or_default(),
                );
                match settings.vcs_settings.client_kind {
                    None => {}
                    Some(VcsClientKind::Git) => {
                        let project_path = self.project_get_path(project_key);
                        if load_project_git_ignore_patterns(
                            self.fs.as_ref(),
                            &directory,
                            project_path.as_deref(),
                            &mut settings,
                        )? {
                            diagnostics.push(biome_diagnostics::serde::Diagnostic::new(
                                VcsDiagnostic::NoIgnoreFileFound(NoIgnoreFileFound {
                                    path: directory.to_string(),
                                }),
                            ));
                        }
                    }
                }
            }

            self.project_set_root_settings(project_key, settings);
            if let Some(project_path) = self.project_get_path(project_key) {
                self.refresh_pnpm_workspace_catalogs_for_scope(project_key, project_path.as_path());
            }
        }

        Ok(UpdateSettingsResult { diagnostics })
    }

    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError> {
        let mut project_clients = self.project_clients.lock();
        let project_path = self
            .project_get_path(params.project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        if let Some(clients) = project_clients.get_mut(&params.project_key) {
            clients.remove(&self.client);
            if !clients.is_empty() {
                return Ok(());
            }
            project_clients.remove(&params.project_key);
        }

        self.remove_project(params.project_key);
        self.scanner
            .unload_project(params.project_key, project_path.clone().to_path_buf());

        // Unload all the documents within the project folder.
        let documents = self.documents.pin();
        for path in documents.keys() {
            if self.path_belongs_only_to_project_with_path(path, &project_path) {
                documents.remove(path);
                self.db_state.remove_node_cache(path);
            }
        }

        self.db_unload_path(&project_path);
        self.project_layout.unload_folder(&project_path);
        #[cfg(feature = "plugins")]
        {
            self.plugin_caches
                .pin()
                .retain(|path, _| !path.starts_with(&project_path));
        }
        Ok(())
    }

    fn open_file(&self, params: OpenFileParams) -> Result<OpenFileResult, WorkspaceError> {
        let diagnostics = self
            .open_file_internal(OpenFileReason::ClientRequest, params)
            .map(|result| {
                result
                    .diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect::<Vec<_>>()
            })?;
        Ok(OpenFileResult { diagnostics })
    }

    fn file_exists(&self, params: FileExistsParams) -> Result<bool, WorkspaceError> {
        Ok(self.document_exists(&params.file_path))
    }

    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let is_virtual = params.path.is_virtual();
        let path = params.path.clone();
        // Missing filesystem paths that skip configuration ignore checks are stdin path hints.
        // They still honor VCS ignore files.
        let check_vcs_ignore =
            params.skip_ignore_check && !is_virtual && !self.fs.path_exists(path.as_path());
        let db = self.get_db();
        let (_, settings, query) = self
            .project_get_settings_query(&db, params.project_key, &path, params.inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let language = self.get_file_source(
            &path,
            query
                .effective_settings(&settings)
                .experimental_full_html_support_enabled(),
        );
        let capabilities = self.features.get_deprecated_capabilities(language);

        let settings = self.settings_handle_with_query(&settings, EditorFeatures::default(), query);
        self.project_get_file_features(
            &db,
            GetFileFeaturesParams {
                fs: self.fs.as_ref(),
                project_key: params.project_key,
                path: path.as_path(),
                requested_features: params.features,
                language,
                capabilities: &capabilities,
                handle: &settings,
                skip_ignore_check: params.skip_ignore_check || is_virtual,
                check_vcs_ignore,
                not_requested_features: params.not_requested_features,
            },
        )
    }

    fn is_path_ignored(&self, params: PathIsIgnoredParams) -> Result<bool, WorkspaceError> {
        if self.is_virtual_document(&params.path) {
            return Ok(false);
        }

        // Never ignore Biome's top-level config file regardless of `includes`.
        if params.path.file_name().is_some_and(|file_name| {
            file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc()
        }) && params.path.parent().is_some_and(|dir_path| {
            self.project_get_path(params.project_key)
                .is_some_and(|project_path| dir_path == project_path)
        }) {
            return Ok(false);
        };

        Ok(self.project_is_ignored(
            params.project_key,
            params.path.as_path(),
            params.is_dir,
            params.features,
            params.ignore_kind,
        ))
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_syntax_tree = capabilities
            .debug
            .debug_syntax_tree
            .ok_or_else(self.build_capability_error(&path))?;

        // The feature name here can be any feature, in theory
        let parse = self.get_parse_for_project(params.project_key, &path)?;
        let printed = debug_syntax_tree(&path, parse.into(), self.get_db().into_untracked_db());

        Ok(printed)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_control_flow = capabilities
            .debug
            .debug_control_flow
            .ok_or_else(self.build_capability_error(&path))?;

        let parse = self.get_parse_for_project(params.project_key, &path)?;
        let printed = debug_control_flow(
            parse.into(),
            params.cursor,
            self.get_db().into_untracked_db(),
        );

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        self.associate_virtual_document_with_project(params.project_key, &path)?;
        let db = self.get_db();
        let (_, settings, query_context) = self
            .project_get_settings_query(&db, params.project_key, &path, None)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(params.project_key, &path)?;
        if !query_context
            .effective_settings(&settings)
            .format_with_errors_enabled_for_this_file_path(path.as_path())
            && parse.has_errors(&*db)
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        // Currently we don't inject inline configuration for debugging methods, review if we need it
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        debug_formatter_ir(
            &path,
            &document_file_source,
            parse.into(),
            &settings,
            db.into_untracked_db(),
        )
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_type_info = capabilities
            .debug
            .debug_type_info
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(params.project_key, &path)?;

        debug_type_info(parse.into(), self.module_db().into_untracked_db())
    }

    fn get_registered_types(
        &self,
        params: GetRegisteredTypesParams,
    ) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_registered_types = capabilities
            .debug
            .debug_registered_types
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(params.project_key, &path)?;

        debug_registered_types(&path, parse.into(), self.get_db().into_untracked_db())
    }

    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let debug_semantic_model = capabilities
            .debug
            .debug_semantic_model
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(params.project_key, &path)?;

        debug_semantic_model(&path, parse.into(), self.get_db().into_untracked_db())
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        let path = params.path.clone();
        self.documents
            .pin()
            .get(path.as_path())
            .and_then(|document| document.content(self.client))
            .map(ToString::to_string)
            .ok_or_else(|| WorkspaceError::not_found(params.path.to_string()))
    }

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        let path = params.path.clone();
        let (file_size, inline_config) = {
            let documents = self.documents.pin();
            let Some(view) = documents
                .get(path.as_path())
                .and_then(|document| document.for_client(self.client))
            else {
                return Err(WorkspaceError::not_found(params.path.to_string()));
            };
            (view.content.len(), view.inline_config.as_deref().cloned())
        };
        let db = self.get_db();
        let limit = self
            .project_get_settings_query(&db, params.project_key, &path, inline_config)
            .map_or_else(
                || MaxSize::default().into(),
                |(_, settings, query)| {
                    query
                        .effective_settings(&settings)
                        .get_max_file_size(path.as_path())
                },
            );
        Ok(CheckFileSizeResult { file_size, limit })
    }

    /// Changes the content of an open file.
    fn change_file(
        &self,
        ChangeFileParams {
            project_key,
            path,
            content,
            version,
            inline_config,
            editor_features: _,
        }: ChangeFileParams,
    ) -> Result<ChangeFileResult, WorkspaceError> {
        let is_virtual = path.is_virtual();
        let is_indexed = self.should_update_service_data(&path);
        let documents = self.documents.pin();
        let (existing_version, same_content, file_source_index, is_open) = documents
            .get(path.as_path())
            .map(|document| {
                (
                    document.version(self.client),
                    document
                        .content(self.client)
                        .is_some_and(|current| **current == *content),
                    document.file_source_index(self.client),
                    document.editor(self.client).is_some(),
                )
            })
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(&path)))?;

        // Once the views of a client are closed, a change it sent before must
        // not be recorded as the content on disk.
        if self.is_client_closed() {
            return Ok(ChangeFileResult {
                diagnostics: Vec::new(),
            });
        }

        if existing_version.is_some_and(|existing_version| existing_version > version)
            || existing_version == Some(version) && !same_content
        {
            warn!(%version, %path, "outdated_file_change");
            // Safely ignore older versions.
            return Ok(ChangeFileResult {
                diagnostics: Vec::new(),
            });
        }

        if existing_version == Some(version) {
            let db = self.get_db();
            let parsed = db
                .get_client_parsed_source(path.as_path(), self.client)
                .ok_or_else(|| {
                    WorkspaceError::not_found(self.public_document_path_string(&path))
                })?;
            let project_is_current =
                !is_virtual || db.project_key_for_source(parsed) == Some(project_key);
            drop(db);
            if project_is_current {
                return self.finish_change_file(project_key, &path, parsed);
            }
        }

        if self.get_db().get_project(&project_key).is_none() {
            return Err(WorkspaceError::no_project());
        }

        // The text is parsed the first time a query reads it, so a file source
        // that can't be parsed must be rejected here.
        let document_source = self
            .get_db()
            .source_from_index(file_source_index)
            .ok_or_else(self.build_capability_error(&path))?;
        self.features
            .get_deprecated_capabilities(document_source)
            .parser
            .parse
            .ok_or_else(self.build_capability_error(&path))?;

        // A client that changes a file it didn't open reports the content on
        // disk, like the CLI after it writes the fixes of a file that the
        // daemon indexes.
        let view = if is_open {
            ViewId::Editor(self.client)
        } else {
            ViewId::Disk
        };
        let content = Arc::<str>::from(content);
        #[cfg(test)]
        if self
            .close_client_files_before_change_publish
            .swap(false, Ordering::AcqRel)
        {
            self.close_client_files()?;
        }
        let parsed = self.db_state.update_file_view(
            path.as_path(),
            view,
            content.clone(),
            file_source_index,
            inline_config.clone(),
        );
        let virtual_source = if is_virtual {
            self.db_state
                .fork()
                .get_client_parsed_source(path.as_path(), self.client)
        } else {
            None
        };
        if let Some(source) = virtual_source {
            self.db_state.associate_source_with_project(
                path.as_path(),
                self.client,
                source,
                project_key,
            );
        }

        let document_view = DocumentView {
            content: content.clone(),
            version: matches!(view, ViewId::Editor(_)).then_some(version),
            file_source_index,
            inline_config: inline_config.map(Arc::new),
            syntax: Some(Ok(())),
        };
        let updated = documents.update(path.to_path_buf(), |document| {
            document.clone().with(view, document_view.clone())
        });
        // The file was closed after it was read, such as when the client
        // disconnected meanwhile, so the editor view just published must not
        // outlive it.
        if updated.is_none() {
            if view == ViewId::Editor(self.client) {
                self.close_editor_view(path.as_path(), Some(project_key))?;
            }
            return Err(WorkspaceError::not_found(
                self.public_document_path_string(&path),
            ));
        }

        #[cfg(test)]
        if self
            .cancel_change_file_after_document_update
            .swap(false, Ordering::AcqRel)
        {
            std::panic::resume_unwind(Box::new(salsa::Cancelled::PendingWrite));
        }

        if view == ViewId::Editor(self.client) && self.is_client_closed() {
            self.close_editor_view(path.as_path(), Some(project_key))?;
            return Ok(ChangeFileResult {
                diagnostics: Vec::new(),
            });
        }

        if is_indexed {
            self.finish_change_file(project_key, &path, parsed)
        } else {
            Ok(ChangeFileResult {
                diagnostics: Vec::new(),
            })
        }
    }

    fn process_file(&self, params: ProcessFileParams) -> Result<ProcessFileResult, WorkspaceError> {
        let ProcessFileParams {
            project_key,
            path,
            content,
            categories,
            only,
            skip,
            enabled_rules,
            fix_file_mode,
            suppression_reason,
            format,
            write,
            include_code_fix,
            max_diagnostics,
            diagnostic_level,
            enforce_assist,
            skip_parse_errors,
        } = params;
        let (_, settings, query) = {
            let db = self.get_db();
            self.project_get_settings_query(&db, project_key, &path, None)
        }
        .ok_or_else(WorkspaceError::no_project)?;
        let settings_handle =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query);
        let process = |source: &str,
                       mut state: ProcessFileState,
                       from_server: bool|
         -> Result<ProcessFileResult, WorkspaceError> {
            #[cfg(feature = "module_graph")]
            let module_db = state.db.rc_module_db();
            let mut applied_fixes = 0;
            let mut skipped_suggested_fixes = 0;
            let mut fixed_source = None;
            let can_transform = !(skip_parse_errors
                && state.error_count(
                    &settings,
                    &path,
                    categories.is_lint() || categories.is_assist(),
                ) > 0);
            let format_with_errors_disabled = can_transform
                && format
                && !settings.format_with_errors_enabled_for_this_file_path(path.as_path())
                && state.has_errors_for_formatting();
            let should_format = can_transform && format && !format_with_errors_disabled;

            if can_transform && let Some(fix_file_mode) = fix_file_mode {
                let fix_result = self.fix_file_state(
                    FixFileParams {
                        project_key,
                        path: path.clone(),
                        fix_file_mode,
                        should_format,
                        only: only.clone(),
                        skip: skip.clone(),
                        enabled_rules: enabled_rules.clone(),
                        rule_categories: categories,
                        suppression_reason: suppression_reason.clone(),
                        inline_config: None,
                    },
                    &mut state,
                    #[cfg(feature = "module_graph")]
                    module_db.clone(),
                    false,
                )?;
                if let Some(fix_result) = fix_result {
                    applied_fixes = fix_result.actions.len();
                    skipped_suggested_fixes = fix_result.skipped_suggested_fixes;
                    let file_source = state.file_source;
                    let fixed = Self::reconstruct_legacy_file(
                        &path,
                        file_source,
                        source,
                        fix_result.root.into_source_text(),
                    );
                    state = self.parse_process_file_state(
                        &path,
                        &fixed,
                        file_source,
                        &settings_handle,
                        settings.experimental_full_html_support_enabled(),
                    )?;
                    fixed_source = Some(fixed);
                }
            }

            let mut output = None;
            if should_format
                && write
                && let Some(formatted) = self.format_file_state(project_key, &path, &state, true)?
            {
                let file_source = state.file_source;
                let input = fixed_source.as_deref().unwrap_or(source);
                let formatted = Self::reconstruct_legacy_file(&path, file_source, input, formatted);
                // Only the analyzer reads the syntax tree of the formatted code.
                // Without it, the state only supplies parser diagnostics.
                if categories.is_lint()
                    || categories.is_assist()
                    || state.can_have_parse_diagnostics_after_formatting()
                {
                    state = self.parse_process_file_state(
                        &path,
                        &formatted,
                        file_source,
                        &settings_handle,
                        settings.experimental_full_html_support_enabled(),
                    )?;
                }
                output = Some(formatted);
            } else if applied_fixes > 0
                && let Some(fixed) = fixed_source
            {
                output = Some(fixed);
            }
            let mut output = output.filter(|output| output != source);

            let PullDiagnosticsResult {
                diagnostics,
                errors,
                warnings,
                infos,
                parse_errors,
                skipped_diagnostics,
            } = self.pull_diagnostics_for_state(
                PullDiagnosticsParams {
                    project_key,
                    path: path.clone(),
                    categories,
                    only,
                    skip,
                    enabled_rules,
                    include_code_fix,
                    inline_config: None,
                    max_diagnostics,
                    diagnostic_level,
                    enforce_assist,
                },
                &state,
                #[cfg(feature = "module_graph")]
                module_db,
            )?;
            let diagnostic_source = output.as_deref().unwrap_or(source);
            let offset =
                Self::legacy_diagnostic_offset(&path, state.file_source, diagnostic_source);
            let diagnostics = diagnostics
                .into_iter()
                .map(|mut diagnostic| {
                    if let Some(offset) = offset {
                        diagnostic.offset_by(TextSize::from(offset));
                    }
                    diagnostic
                })
                .collect();

            if should_format && !write && from_server {
                let formatted = self
                    .format_file(FormatFileParams {
                        project_key,
                        path: path.clone(),
                        inline_config: None,
                    })?
                    .into_code();
                output = Some(Self::reconstruct_legacy_file(
                    &path,
                    state.file_source,
                    source,
                    formatted,
                ))
                .filter(|output| output != source);
            }

            Ok(ProcessFileResult {
                output,
                diagnostics,
                format_with_errors_disabled,
                applied_fixes,
                errors,
                warnings,
                infos,
                parse_errors,
                skipped_diagnostics,
                skipped_suggested_fixes,
            })
        };

        match content {
            FileContent::FromServer => {
                let documents = self.documents.pin();
                let source = documents
                    .get(path.as_path())
                    .and_then(|document| document.content(self.client))
                    .ok_or_else(|| {
                        WorkspaceError::not_found(self.public_document_path_string(&path))
                    })?;
                process(
                    source,
                    self.process_file_state_from_server(project_key, &path)?,
                    true,
                )
            }
            FileContent::FromClient { content, .. } => {
                let file_source = DocumentFileSource::from_path(
                    path.as_path(),
                    settings.experimental_full_html_support_enabled(),
                );
                let state = self.parse_process_file_state(
                    &path,
                    &content,
                    file_source,
                    &settings_handle,
                    false,
                )?;
                process(&content, state, false)
            }
        }
    }

    /// Retrieves the list of diagnostics associated with a file
    #[tracing::instrument(
        level = "debug",
        skip(self, params),
        fields(
            rule_categories = display(&params.categories),
            path = display(&params.path),
            project_key = debug(&params.project_key),
            skip = debug(&params.skip),
            only = debug(&params.only),
        )
    )]
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let path = params.path.clone();
        let state = self.process_file_state_from_server(params.project_key, &path)?;
        #[cfg(feature = "module_graph")]
        let module_db = state.db.rc_module_db();
        self.pull_diagnostics_for_state(
            params,
            &state,
            #[cfg(feature = "module_graph")]
            module_db,
        )
    }

    fn pull_diagnostics_and_actions(
        &self,
        params: PullDiagnosticsAndActionsParams,
    ) -> Result<PullDiagnosticsAndActionsResult, WorkspaceError> {
        let PullDiagnosticsAndActionsParams {
            project_key,
            path,
            categories,
            only,
            skip,
            enabled_rules,
            inline_config,
        } = params;
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (working_directory, settings, query_context) = self
            .project_get_settings_query(&workspace_db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse_for_project(project_key, &path)?;
        let language =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let capabilities = self.features.get_deprecated_capabilities(language);
        let mut result = if (categories.is_lint() || categories.is_assist())
            && let Some(pull_diagnostics_and_actions) =
                capabilities.analyzer.pull_diagnostics_and_actions
        {
            let plugins = cfg_select! {
                feature = "plugins" => {
                    if categories.is_lint() {
                        self.get_analyzer_plugins_for_project(
                            settings.source_path().unwrap_or_default().as_path(),
                            &settings.get_plugins_for_path(path.as_path()),
                        )
                        .map_err(WorkspaceError::plugin_errors)?
                    } else {
                        Vec::new()
                    }
                }
                _ => Vec::new(),
            };
            let handle = self.settings_handle_with_query(
                &settings,
                EditorFeatures::default(),
                query_context,
            );
            let mut final_result = pull_diagnostics_and_actions(DiagnosticsAndActionsParams {
                parsed_source: parse.into(),
                settings: &handle,
                path: &path,
                only: &only,
                skip: &skip,
                language,
                categories,
                workspace_db: workspace_db.clone_untracked_db(),
                project_layout: self.project_layout.clone(),
                suppression_reason: None,
                enabled_selectors: &enabled_rules,
                plugins: plugins.clone(),
                working_directory: Some(working_directory.as_path()),
            });

            for embedded_node in SnippetsIterator::from_source(parse, &workspace_db).for_analysis(
                &parse.into(),
                language,
                handle.as_ref(),
                path.as_path(),
                &workspace_db,
            ) {
                let ParsedSnippetOrigin::Workspace(embedded_node) = embedded_node else {
                    continue;
                };
                let Some(file_source) =
                    workspace_db.source_from_index(embedded_node.document_source_index())
                else {
                    continue;
                };
                let capabilities = self.features.get_deprecated_capabilities(file_source);
                let Some(pull_diagnostics_and_actions) =
                    capabilities.analyzer.pull_diagnostics_and_actions
                else {
                    continue;
                };

                let snippet_result = pull_diagnostics_and_actions(DiagnosticsAndActionsParams {
                    parsed_source: embedded_node.into(),
                    settings: &handle,
                    path: &path,
                    only: &only,
                    skip: &skip,
                    language: file_source,
                    categories,
                    workspace_db: workspace_db.clone_untracked_db(),
                    project_layout: self.project_layout.clone(),
                    suppression_reason: None,
                    enabled_selectors: &enabled_rules,
                    plugins: plugins.clone(),
                    working_directory: Some(working_directory.as_path()),
                });

                final_result.diagnostics.extend(snippet_result.diagnostics);
            }

            final_result
        } else {
            // Parse diagnostics aren't fixable, so we return an empty list
            PullDiagnosticsAndActionsResult {
                diagnostics: vec![],
            }
        };

        let public_path = self.public_document_path_string(&path);
        for (diagnostic, _) in &mut result.diagnostics {
            *diagnostic =
                SerdeDiagnostic::new(diagnostic.clone().with_file_path(public_path.as_str()));
        }

        Ok(result)
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            only = debug(&params.only),
            skip = debug(&params.skip),
            range = debug(&params.range)
        )
    )]
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        let PullActionsParams {
            project_key,
            path,
            range,
            suppression_reason: _,
            only,
            skip,
            enabled_rules,
            categories,
            inline_config,
            compute_actions,
        } = params;
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (working_directory, settings, query_context) = self
            .project_get_settings_query(&workspace_db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&path))?;

        let language =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        let parsed_source = self.get_parse_for_project(project_key, &path)?;

        let plugins = cfg_select! {
            feature = "plugins" => {
                if categories.contains(biome_analyze::RuleCategory::Lint) {
                    self.get_analyzer_plugins_for_project(
                        settings
                            .as_ref()
                            .source_path()
                            .unwrap_or_default()
                            .as_path(),
                        &settings.as_ref().get_plugins_for_path(path.as_path()),
                    )
                    .map_err(WorkspaceError::plugin_errors)?
                } else {
                    Vec::new()
                }
            }
            _ => biome_analyze::AnalyzerPluginVec::new(),
        };

        let mut result = code_actions(CodeActionsParams {
            parsed_source: parsed_source.into(),
            range,
            settings: &settings,
            path: &path,
            workspace_db: workspace_db.clone_untracked_db(),
            project_layout: self.project_layout.clone(),
            language,
            only: &only,
            skip: &skip,
            suppression_reason: None,
            enabled_rules: &enabled_rules,
            plugins: plugins.clone(),
            categories,
            working_directory: Some(working_directory.as_path()),
            compute_actions,
        });

        // TODO: remove this once legacy HTML-ish support is removed
        if let Some(offset) = self
            .documents
            .pin()
            .get(path.as_path())
            .and_then(|document| document.content(self.client))
            .and_then(|content| Self::legacy_diagnostic_offset(&path, language, content))
        {
            for action in &mut result.actions {
                action.offset.get_or_insert(TextSize::from(offset));
            }
        }

        for embedded_snippet in SnippetsIterator::from_source(parsed_source, &workspace_db)
            .for_analysis(
                &parsed_source.into(),
                language,
                settings.as_ref(),
                path.as_path(),
                &workspace_db,
            )
        {
            let ParsedSnippetOrigin::Workspace(embedded_snippet) = embedded_snippet else {
                continue;
            };
            let Some(file_source) =
                workspace_db.source_from_index(embedded_snippet.document_source_index())
            else {
                continue;
            };
            let capabilities = self.features.get_deprecated_capabilities(file_source);
            let Some(code_actions) = capabilities.analyzer.code_actions else {
                continue;
            };

            // The snippet tree starts at zero, so the document range must be
            // made relative to the snippet before it filters the signals.
            let snippet_range = match range {
                Some(range) => {
                    let Some(range) = range.intersect(embedded_snippet.content_range()) else {
                        continue;
                    };
                    Some(range - embedded_snippet.content_offset())
                }
                None => None,
            };

            let embedded_actions_result = code_actions(CodeActionsParams {
                parsed_source: embedded_snippet.into(),
                range: snippet_range,
                settings: &settings,
                path: &path,
                workspace_db: workspace_db.clone_untracked_db(),
                project_layout: self.project_layout.clone(),
                language: file_source,
                only: &only,
                skip: &skip,
                suppression_reason: None,
                enabled_rules: &enabled_rules,
                plugins: plugins.clone(),
                categories,
                working_directory: Some(working_directory.as_path()),
                compute_actions,
            });

            result.actions.extend(embedded_actions_result.actions);
        }

        Ok(result)
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    #[instrument(
        level = "debug",
        skip_all,
        fields(
            path = display(&params.path),
        )
    )]
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        let FormatFileParams {
            project_key,
            path,
            inline_config,
        } = params;
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (_, settings, query_context) = self
            .project_get_settings_query(&workspace_db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let effective_settings = query_context.effective_settings(&settings);
        let capabilities = self.get_file_capabilities(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );

        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&path))?;

        let format_embedded = capabilities.formatter.format_embedded;
        let (parse, embedded_nodes) =
            self.get_parsed_snippets_and_parse_source(&workspace_db, &path)?;
        let document_file_source = self.get_file_source(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );

        if !effective_settings.format_with_errors_enabled_for_this_file_path(path.as_path())
            && (parse.parsed(&*workspace_db).has_errors()
                || SnippetsIterator::from_source(parse, &workspace_db)
                    .blocking_formatting(document_file_source)
                    .any(|snippet| snippet.has_errors()))
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);

        if !embedded_nodes.is_empty() {
            let format_embedded = format_embedded.ok_or_else(self.build_capability_error(&path))?;
            let embedded_nodes = embedded_nodes
                .into_iter()
                .map(ParsedSnippetOrigin::Workspace)
                .collect();
            return format_embedded(
                &path,
                &document_file_source,
                parse.into(),
                &settings,
                embedded_nodes,
                workspace_db.into_untracked_db(),
            );
        }
        format(
            &path,
            &document_file_source,
            parse.into(),
            &settings,
            workspace_db.into_untracked_db(),
        )
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let FormatRangeParams {
            project_key,
            path,
            range,
            inline_config,
        } = params;
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (_, settings, query_context) = self
            .project_get_settings_query(&workspace_db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let effective_settings = query_context.effective_settings(&settings);
        let capabilities = self.get_file_capabilities(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(project_key, &path)?;
        if !effective_settings.format_with_errors_enabled_for_this_file_path(path.as_path())
            && parse.has_errors(&*workspace_db)
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        format_range(
            &path,
            &document_file_source,
            parse.into(),
            &settings,
            range,
            workspace_db.into_untracked_db(),
        )
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        let FormatOnTypeParams {
            project_key,
            path,
            offset,
            inline_config,
        } = params;
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (_, settings, query_context) = self
            .project_get_settings_query(&workspace_db, project_key, &path, inline_config)
            .ok_or_else(WorkspaceError::no_project)?;
        let effective_settings = query_context.effective_settings(&settings);
        let capabilities = self.get_file_capabilities(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&path))?;

        let parse = self.get_parse_for_project(project_key, &path)?;
        if !effective_settings.format_with_errors_enabled_for_this_file_path(path.as_path())
            && parse.has_errors(&*workspace_db)
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(
            &path,
            effective_settings.experimental_full_html_support_enabled(),
        );
        let settings =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query_context);
        format_on_type(
            &path,
            &document_file_source,
            parse.into(),
            &settings,
            offset,
            workspace_db.into_untracked_db(),
        )
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            path = display(&params.path),
            rule_categories = display(&params.rule_categories),
            skip = debug(&params.skip),
            only = debug(&params.only),
            should_format = display(&params.should_format),
        )
    )]
    fn fix_file(&self, mut params: FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        let project_key = params.project_key;
        let path = params.path.clone();
        let documents = self.documents.pin();
        let source = documents
            .get(path.as_path())
            .and_then(|document| document.content(self.client))
            .ok_or_else(|| WorkspaceError::not_found(self.public_document_path_string(&path)))?;
        let mut state = self.process_file_state_from_server(project_key, &path)?;
        let (_, settings, query) = self
            .project_get_settings_query(&state.db, project_key, &path, params.inline_config.clone())
            .ok_or_else(WorkspaceError::no_project)?;
        let format_with_errors = query
            .effective_settings(&settings)
            .format_with_errors_enabled_for_this_file_path(path.as_path());
        let should_format =
            params.should_format && (format_with_errors || !state.has_errors_for_formatting());
        params.should_format = should_format;
        let settings_handle =
            self.settings_handle_with_query(&settings, EditorFeatures::default(), query);
        #[cfg(feature = "module_graph")]
        let module_db = state.db.rc_module_db();
        let fixed = self
            .fix_file_state(
                params,
                &mut state,
                #[cfg(feature = "module_graph")]
                module_db,
                true,
            )?
            .ok_or_else(self.build_capability_error(&path))?;
        let FixedFileResult {
            root,
            skipped_suggested_fixes,
            actions,
            errors,
        } = fixed;
        let file_source = state.file_source;
        let fixed_source = root.into_source_text();
        let code = if should_format {
            let fixed_file =
                Self::reconstruct_legacy_file(&path, file_source, source, fixed_source);
            state = self.parse_process_file_state(
                &path,
                &fixed_file,
                file_source,
                &settings_handle,
                settings.experimental_full_html_support_enabled(),
            )?;
            self.format_file_state(project_key, &path, &state, false)?
                .unwrap_or_else(|| state.parsed.send_node(&state.db).into_source_text())
        } else {
            fixed_source
        };

        Ok(FixFileResult {
            code,
            actions,
            errors,
            skipped_suggested_fixes,
        })
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError> {
        let path = params.path.clone();
        let settings = self
            .project_get_settings_for_path(params.project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&path))?;

        let parse = self.get_parse_for_project(params.project_key, &path)?;
        let workspace_db = self.get_db();
        let result = rename(
            &path,
            parse.into(),
            params.symbol_at,
            params.new_name,
            workspace_db.into_untracked_db(),
        )?;

        Ok(result)
    }

    fn go_to_definition(
        &self,
        params: GoToDefinitionParams,
    ) -> Result<Option<GoToDefinitionResult>, WorkspaceError> {
        let path = &params.path;
        self.associate_virtual_document_with_project(params.project_key, path)?;
        let cursor_offset = params.cursor_range.start();
        let workspace_db = self.get_db();
        let (_, settings, query) = self
            .project_get_settings_query(&workspace_db, params.project_key, path, None)
            .ok_or_else(WorkspaceError::no_project)?;

        let capability: EditorFeatures = if params.enabled {
            EditorFeatures::default().with(EditorFeature::GotoDefinition)
        } else {
            EditorFeatures::default()
        };
        let settings = self.settings_handle_with_query(&settings, capability, query);

        let has_document_services = settings.needs_document_services()
            || settings.as_ref().is_linter_enabled()
            || settings.as_ref().is_assist_enabled();
        if !has_document_services {
            return Ok(None);
        }

        let (parse, embedded_snippets) =
            self.get_parsed_snippets_and_parse_source(&workspace_db, path)?;

        let language = self.get_file_source(
            path,
            settings.as_ref().experimental_full_html_support_enabled(),
        );
        // Try to resolve the binding, checking embedded snippets first
        let (definition_ref, capabilities) =
            self.resolve_binding_in_document_or_snippets(path, cursor_offset, &parse, language)?;

        let Some(definition_ref) = definition_ref else {
            return Ok(None);
        };

        for snippet in embedded_snippets {
            if let DefinitionReference::LocalEmbedded { range, .. } = &definition_ref {
                let offset = snippet.content_offset();
                let parent_range = *range + offset;
                if !snippet.content_range().contains_range(parent_range) {
                    continue; // This snippet didn't produce the binding
                }
            }

            let Some(file_source) = workspace_db.source_from_index(snippet.document_source_index())
            else {
                continue;
            };
            let snippet_caps = self.features.get_real_capabilities(file_source);
            let Some(resolve_definition) = snippet_caps.editors.resolve_definition else {
                continue;
            };

            let result = resolve_definition(ResolveDefinitionParams {
                path,
                definition_ref: &definition_ref,
                workspace_db: workspace_db.clone_untracked_db(),
                parsed_source: snippet.into(),
            });

            if let Some(result) = result
                && !result.matches.is_empty()
            {
                return Ok(Some(result));
            }
        }

        let resolve_definition = capabilities
            .editors
            .resolve_definition
            // NOTE: here we might want to silent the error because it could be noisy
            .ok_or_else(self.build_capability_error(path))?;

        Ok(resolve_definition(ResolveDefinitionParams {
            path,
            definition_ref: &definition_ref,
            workspace_db: workspace_db.into_untracked_db(),
            parsed_source: parse.into(),
        }))
    }

    /// Closes a file opened in the workspace.
    ///
    /// This only unloads the document from the workspace if the file is NOT
    /// indexed by the scanner. If the scanner has the file indexed, it may
    /// still be required for multi-file analysis.
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.close_editor_view(params.path.as_path(), Some(params.project_key))
    }

    fn update_module_graph(&self, params: UpdateModuleGraphParams) -> Result<(), WorkspaceError> {
        #[cfg(feature = "module_graph")]
        {
            // The module graph is shared by the clients, so it reads the view
            // they share.
            self.associate_virtual_document_with_project(params.project_key, &params.path)?;
            self.assert_parse(&params.path)?;
            let parsed = self
                .get_db()
                .get_parsed_source(params.path.as_path())
                .ok_or_else(|| WorkspaceError::not_found(params.path.to_string()))?;
            let settings = self
                .project_get_settings_for_path(params.project_key, &params.path)
                .ok_or_else(WorkspaceError::no_project)?;
            let update_kind = match params.update_kind {
                super::UpdateKind::AddOrUpdate => {
                    UpdateKind::AddedOrChanged(OpenFileReason::ClientRequest, parsed.into())
                }
                super::UpdateKind::Remove => UpdateKind::Removed,
            };

            // TODO: handle diagnostics here
            self.update_module_graph_internal(
                &params.path,
                update_kind,
                settings.module_graph_resolution_kind.is_modules_and_types(),
            )?;

            Ok(())
        }
        #[cfg(not(feature = "module_graph"))]
        {
            let _ = params;
            Ok(())
        }
    }

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.fs.as_ref()
    }

    fn parse_pattern(
        &self,
        params: ParsePatternParams,
    ) -> Result<ParsePatternResult, WorkspaceError> {
        let pattern_id = self
            .search_provider
            .parse_pattern(params.pattern.as_ref(), params.default_language)?;

        Ok(ParsePatternResult { pattern_id })
    }

    fn search_pattern(
        &self,
        SearchPatternParams {
            project_key,
            path,
            pattern,
        }: SearchPatternParams,
    ) -> Result<SearchResults, WorkspaceError> {
        self.associate_virtual_document_with_project(project_key, &path)?;
        let workspace_db = self.get_db();
        let (_, settings, query) = self
            .project_get_settings_query(&workspace_db, project_key, &path, None)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let search = capabilities
            .search
            .search
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse_for_project(project_key, &path)?;

        let document_file_source =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let settings = self.settings_handle_with_query(&settings, EditorFeatures::default(), query);
        let provider = self.search_provider.clone();
        let matches = search(
            &path,
            &document_file_source,
            parse.into(),
            provider.as_ref(),
            &settings,
            pattern,
            workspace_db.into_untracked_db(),
        )?;

        Ok(SearchResults { path, matches })
    }

    fn drop_pattern(&self, params: DropPatternParams) -> Result<(), WorkspaceError> {
        self.search_provider.drop_pattern(params.pattern);
        Ok(())
    }

    fn rage(&self, _: RageParams) -> Result<RageResult, WorkspaceError> {
        let entries = vec![
            RageEntry::section("Workspace"),
            RageEntry::pair("Open Documents", &format!("{}", self.documents.len())),
        ];

        Ok(RageResult { entries })
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }

    fn get_module_graph(
        &self,
        _params: GetModuleGraphParams,
    ) -> Result<GetModuleGraphResult, WorkspaceError> {
        let db = self.get_db();
        #[cfg(feature = "module_graph")]
        let mut data = FxHashMap::default();
        #[cfg(feature = "module_graph")]
        db.for_each_module(&mut |module| {
            data.insert(module.path(&*db).as_str().to_string(), module.dump(&*db));
        });
        #[cfg(not(feature = "module_graph"))]
        let data = {
            let _ = db;
            FxHashMap::default()
        };
        Ok(GetModuleGraphResult { data })
    }
}

impl WorkspaceScannerBridge for WorkspaceServerWithDb<'_> {
    #[inline]
    fn fs(&self) -> &dyn biome_fs::FileSystem {
        self.fs.as_ref()
    }

    #[inline]
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.find_project_for_path(path)
    }

    #[inline]
    fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.project_get_path(project_key)
    }

    #[inline]
    fn get_vcs_watch_paths(&self, project_key: ProjectKey) -> Vec<BiomePath> {
        self.project_vcs_watch_paths(project_key)
    }

    #[inline]
    fn get_vcs_root(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.project_get_root_settings(project_key)?
            .vcs_settings
            .ignore_matches
            .as_ref()
            .map(|ignore_matches| ignore_matches.root_path().to_path_buf())
    }

    #[inline]
    fn refresh_project_vcs_ignore_settings(
        &self,
        project_key: ProjectKey,
    ) -> Result<(), WorkspaceError> {
        self.refresh_project_vcs_ignore_settings(project_key)
    }

    #[inline]
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError> {
        self.is_ignored_by_scanner(project_key, scan_kind, path, request_kind, path_kind)
    }

    #[inline]
    fn is_indexed(&self, path: &Utf8Path) -> bool {
        match path.file_name() {
            Some("package.json" | "tsconfig.json" | "turbo.json" | "turbo.jsonc") => {
                self.project_layout.is_indexed(path)
            }
            Some("pnpm-workspace.yaml") => path.parent().is_some_and(|workspace_root| {
                self.project_layout
                    .package_paths()
                    .into_iter()
                    .any(|package_path| package_path.starts_with(workspace_root))
            }),
            _ => {
                #[cfg(feature = "module_graph")]
                {
                    self.db_state.contains_module_untracked(path)
                }
                #[cfg(not(feature = "module_graph"))]
                {
                    false
                }
            }
        }
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: BiomePath,
        trigger: IndexTrigger,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        let open_file = |path| {
            self.open_file_internal(
                OpenFileReason::Index(trigger),
                OpenFileParams {
                    project_key,
                    path,
                    content: FileContent::FromServer,
                    document_file_source: None,
                    persist_node_cache: false,
                    // TODO: review here, it feels wrong that we can't pass the inline config
                    inline_config: None,
                    editor_features: None,
                },
            )
        };
        match trigger {
            IndexTrigger::InitialScan => open_file(path),
            IndexTrigger::Update => retry_on_pending_write(|| open_file(path.clone())),
        }
        .map(|result| (result.dependencies, result.diagnostics))
    }

    fn update_project_config_files(
        &self,
        project_key: ProjectKey,
        paths: &[BiomePath],
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
        let project_path = self
            .project_get_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let mut returned_diagnostics = Vec::new();

        let filtered_paths = paths
            .iter()
            // We remove the root configuration file from the list of paths
            // SAFETY: the paths received are files, so it's safe to assume they have a parent folder
            .filter(|config_path| project_path != config_path.parent().unwrap().as_std_path());

        for filtered_path in filtered_paths {
            let config = read_config(
                self.fs.as_ref(),
                ConfigurationPathHint::FromWorkspace(filtered_path.as_path().to_path_buf()),
                false,
            )?;
            let loaded_nested_configuration =
                LoadedConfiguration::try_from_payload(config, self.fs.as_ref())?;

            let nested_directory_path = loaded_nested_configuration
                .directory_path()
                .map(Utf8Path::to_path_buf);
            let nested_configuration = loaded_nested_configuration.resolved_configuration();
            let extended_configurations = loaded_nested_configuration.extended_configurations();
            let diagnostics = loaded_nested_configuration.diagnostics;
            let has_errors = diagnostics.iter().any(|d| d.severity() >= Severity::Error);
            returned_diagnostics.extend(
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new),
            );

            if has_errors {
                continue;
            }

            if nested_configuration.is_root() {
                returned_diagnostics.push(biome_diagnostics::serde::Diagnostic::new(
                    BiomeDiagnostic::root_in_root(
                        filtered_path.to_string(),
                        Some(project_path.to_string()),
                    ),
                ));
                continue;
            }

            let nested_configuration = if nested_configuration.extends_root() {
                let root_settings = self
                    .project_get_root_settings(project_key)
                    .ok_or_else(WorkspaceError::no_project)?;
                let mut root_configuration = root_settings
                    .source()
                    .ok_or_else(WorkspaceError::no_project)?;

                root_configuration.merge_with(nested_configuration);
                // We need to be careful that our merge doesn't leave
                // `root: true` from the root config.
                root_configuration.root = Some(Bool(false));
                root_configuration
            } else {
                nested_configuration
            };

            let scan_kind = ProjectScanComputer::new(&nested_configuration).compute();
            let params = UpdateSettingsParams {
                project_key,
                workspace_directory: nested_directory_path
                    .as_ref()
                    .map(|path| self.fs.biome_path(path)),
                configuration: nested_configuration,
                extended_configurations: extended_configurations
                    .into_iter()
                    .map(|(path, config)| (self.fs.biome_path(&path), config))
                    .collect(),
                module_graph_resolution_kind: ModuleGraphResolutionKind::from(&scan_kind),
            };
            let result = self.update_settings(params)?;

            returned_diagnostics.extend(result.diagnostics)
        }

        Ok(returned_diagnostics)
    }

    fn update_project_ignore_files(
        &self,
        project_key: ProjectKey,
        scanned_directory: &Utf8Path,
        paths: &[BiomePath],
    ) -> Result<(), WorkspaceError> {
        let settings = self
            .project_get_root_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let vcs_settings = &settings.vcs_settings;

        if !vcs_settings.is_enabled() || !vcs_settings.should_use_ignore_file() {
            return Ok(());
        }

        let Some(mut applicable_matches) = vcs_settings.ignore_matches.clone() else {
            return Ok(());
        };
        let root_ignore_path = applicable_matches.root_path().clone();
        let root_git_ignore = if vcs_settings.client_kind == Some(VcsClientKind::Git)
            && root_ignore_path.as_path() == scanned_directory
        {
            let patterns =
                read_root_git_ignore_patterns(self.fs.as_ref(), root_ignore_path.as_path());
            let patterns = patterns.iter().map(String::as_str).collect::<Vec<_>>();
            let git_ignore = VcsIgnoredPatterns::git_ignore(&root_ignore_path, &patterns)?;
            applicable_matches.replace_root_git_match(git_ignore.clone());
            Some(git_ignore)
        } else {
            None
        };

        let scope = self.fs.biome_path(scanned_directory);
        applicable_matches.replace_git_matches(&scope, &[]);
        let mut filtered_paths = paths
            .iter()
            .filter_map(|path| {
                let parent = path.parent()?;
                (path.is_ignore() && root_ignore_path.as_path() != parent).then_some((path, parent))
            })
            .collect::<Vec<_>>();
        filtered_paths.sort_unstable_by(|(left_path, left_parent), (right_path, right_parent)| {
            left_parent
                .components()
                .count()
                .cmp(&right_parent.components().count())
                .then_with(|| left_path.cmp(right_path))
        });

        let mut payload = vec![];
        for (path, dir_ignore_file) in filtered_paths {
            if !vcs_settings.is_ignore_file(path.as_path())
                || !path.starts_with(root_ignore_path.as_path())
            {
                continue;
            }

            let matcher_root = self.fs.biome_path(dir_ignore_file);
            if applicable_matches.is_ignored(&matcher_root, true) {
                continue;
            }

            let content = self.fs.read_file_from_path(path.as_path())?;
            let patterns = content.lines().map(String::from).collect::<Vec<_>>();
            let pattern_refs = patterns.iter().map(String::as_str).collect::<Vec<_>>();
            applicable_matches.insert_git_match(VcsIgnoredPatterns::git_ignore(
                &matcher_root,
                &pattern_refs,
            )?);
            payload.push((dir_ignore_file.to_path_buf(), patterns));
        }

        self.store_git_ignores(
            project_key,
            root_git_ignore,
            scanned_directory.to_path_buf(),
            payload,
        )?;

        Ok(())
    }

    fn notify(&self, notification: ServiceNotification) {
        let _ = self.notification_tx.send(notification);
    }

    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        self.update_service_data(&self.fs.biome_path(path), UpdateKind::Removed, project_key)
            .map(|(_, diagnostics)| {
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect()
            })
    }

    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        // Removed paths may have been watched folders. Existing paths are being
        // rescanned and must retain their watcher registrations.
        if !self.fs.path_exists(path) {
            self.scanner.unload_folder(path.to_path_buf());
        }

        // Unloads all descendants of the path. The files that a client has
        // open keep the content of the client.
        self.db_state.unload_disk_views(path);
        self.project_layout.unload_folder(path);

        // Finally unloads the path itself.
        self.update_service_data(&self.fs.biome_path(path), UpdateKind::Removed, project_key)
            .map(|(_, diagnostics)| {
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect()
            })
    }

    fn sync_path_info(&self, path: &Utf8Path) -> bool {
        // The watcher has no cancellation boundary of its own, and a
        // concurrent client update can cancel the database reads.
        #[cfg(feature = "module_graph")]
        {
            retry_on_pending_write(|| self.sync_resolver_path_info(path))
        }
        #[cfg(not(feature = "module_graph"))]
        {
            let _ = path;
            false
        }
    }

    fn index_new_module_dependencies(
        &self,
        project_key: ProjectKey,
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
        #[cfg(feature = "module_graph")]
        {
            retry_on_pending_write(|| self.index_new_dependencies(project_key))
        }
        #[cfg(not(feature = "module_graph"))]
        {
            let _ = project_key;
            Ok(Vec::new())
        }
    }
}

impl WorkspaceScannerBridge for LocalWorkspace {
    #[inline]
    fn fs(&self) -> &dyn biome_fs::FileSystem {
        self.server.fs.as_ref()
    }

    #[inline]
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.as_workspace().find_project_for_path(path)
    }

    #[inline]
    fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.as_workspace().project_get_path(project_key)
    }

    #[inline]
    fn get_vcs_watch_paths(&self, project_key: ProjectKey) -> Vec<BiomePath> {
        self.as_workspace().project_vcs_watch_paths(project_key)
    }

    #[inline]
    fn get_vcs_root(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.as_workspace()
            .project_get_root_settings(project_key)?
            .vcs_settings
            .ignore_matches
            .as_ref()
            .map(|ignore_matches| ignore_matches.root_path().to_path_buf())
    }

    #[inline]
    fn refresh_project_vcs_ignore_settings(
        &self,
        project_key: ProjectKey,
    ) -> Result<(), WorkspaceError> {
        self.as_workspace()
            .refresh_project_vcs_ignore_settings(project_key)
    }

    #[inline]
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
        path_kind: Option<PathKind>,
    ) -> Result<bool, WorkspaceError> {
        self.as_workspace()
            .is_ignored(project_key, scan_kind, path, request_kind, path_kind)
    }

    #[inline]
    fn is_indexed(&self, path: &Utf8Path) -> bool {
        self.as_workspace().is_indexed(path)
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: BiomePath,
        trigger: IndexTrigger,
    ) -> Result<(ModuleDependencies, Vec<Error>), WorkspaceError> {
        self.as_workspace().index_file(project_key, path, trigger)
    }

    fn update_project_config_files(
        &self,
        project_key: ProjectKey,
        files: &[BiomePath],
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
        self.as_workspace()
            .update_project_config_files(project_key, files)
    }

    fn update_project_ignore_files(
        &self,
        project_key: ProjectKey,
        scanned_directory: &Utf8Path,
        files: &[BiomePath],
    ) -> Result<(), WorkspaceError> {
        self.as_workspace()
            .update_project_ignore_files(project_key, scanned_directory, files)
    }

    #[inline]
    fn notify(&self, notification: ServiceNotification) {
        self.server.notification_tx.send(notification).ok();
    }

    fn unload_file(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        self.as_workspace().unload_file(path, project_key)
    }

    fn unload_path(
        &self,
        path: &Utf8Path,
        project_key: ProjectKey,
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        self.as_workspace().unload_path(path, project_key)
    }

    fn sync_path_info(&self, path: &Utf8Path) -> bool {
        self.as_workspace().sync_path_info(path)
    }

    fn index_new_module_dependencies(
        &self,
        project_key: ProjectKey,
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
        self.as_workspace()
            .index_new_module_dependencies(project_key)
    }
}

#[derive(Debug, Default)]
pub(super) struct InternalOpenFileResult {
    /// Dependencies we discovered of the opened file.
    pub dependencies: ModuleDependencies,

    pub diagnostics: Vec<Error>,
}

/// Reports the reason why a file is being opened/indexed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenFileReason {
    /// A workspace client has explicitly requested the file to be opened.
    ClientRequest,

    /// The file is being indexed by the scanner.
    Index(IndexTrigger),
}

impl OpenFileReason {
    pub const fn is_index(self) -> bool {
        matches!(self, Self::Index(_))
    }
}

/// Kind of update being performed.
pub enum UpdateKind {
    AddedOrChanged(OpenFileReason, AnalyzedSource),
    Removed,
}

impl Debug for UpdateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddedOrChanged(reason, _) => {
                f.debug_tuple("AddedOrChanged").field(reason).finish()
            }
            Self::Removed => write!(f, "Removed"),
        }
    }
}

/// Sets up the global Rayon thread pool the first time it's called.
///
/// This is used to assign friendly debug names to the threads of the pool.
#[cfg(not(target_family = "wasm"))]
fn init_thread_pool(threads: Option<usize>) {
    static INIT_ONCE: std::sync::Once = std::sync::Once::new();
    INIT_ONCE.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("biome::workspace_worker_{index}"))
            // When zero is passed, rayon decides the number of threads
            .num_threads(threads.unwrap_or(0))
            .build_global()
            .expect("failed to initialize the global thread pool");
    });
}

#[cfg(target_family = "wasm")]
fn init_thread_pool(_threads: Option<usize>) {}

#[cfg(test)]
#[path = "server.tests.rs"]
mod tests;

#[cfg(all(test, feature = "md_embeds"))]
#[path = "md_embeds.tests.rs"]
mod md_embeds_tests;

#[cfg(all(test, feature = "plugins"))]
#[path = "server_plugin_cache.tests.rs"]
mod plugin_cache_tests;

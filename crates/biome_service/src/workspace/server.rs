use std::panic::RefUnwindSafe;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use append_only_vec::AppendOnlyVec;
use biome_analyze::AnalyzerPluginVec;
use biome_configuration::plugins::{PluginConfiguration, Plugins};
use biome_configuration::{BiomeDiagnostic, Configuration};
use biome_deserialize::Deserialized;
use biome_deserialize::json::deserialize_from_json_str;
use biome_diagnostics::print_diagnostic_to_string;
use biome_diagnostics::{
    Diagnostic, DiagnosticExt, Severity, serde::Diagnostic as SerdeDiagnostic,
};
use biome_formatter::Printed;
use biome_fs::{BiomePath, ConfigName, FileSystem};
use biome_grit_patterns::{CompilePatternOptions, GritQuery, compile_pattern_with_options};
use biome_js_syntax::ModuleKind;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonFileSource;
use biome_module_graph::ModuleGraph;
use biome_package::PackageType;
use biome_parser::AnyParse;
use biome_plugin_loader::{BiomePlugin, PluginCache, PluginDiagnostic};
use biome_project_layout::ProjectLayout;
use biome_rowan::NodeCache;
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::Sender;
use papaya::{Compute, HashMap, HashSet, Operation};
use rustc_hash::{FxBuildHasher, FxHashMap};
use tokio::sync::watch;
use tracing::{info, instrument, warn};

use crate::diagnostics::FileTooLarge;
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DocumentFileSource, Features, FixAllParams, LintParams,
    ParseResult,
};
use crate::projects::Projects;
use crate::settings::WorkspaceSettingsHandle;
use crate::workspace::{
    FileFeaturesResult, GetFileContentParams, GetRegisteredTypesParams, GetTypeInfoParams,
    IsPathIgnoredParams, RageEntry, RageParams, RageResult, ServerInfo,
};
use crate::workspace_watcher::WatcherSignalKind;
use crate::{WatcherInstruction, Workspace, WorkspaceError, is_dir};

use super::document::Document;
use super::{
    ChangeFileParams, CheckFileSizeParams, CheckFileSizeResult, CloseFileParams,
    CloseProjectParams, FeatureName, FileContent, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSemanticModelParams, GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams,
    OpenProjectParams, ParsePatternParams, ParsePatternResult, PatternId, ProjectKey,
    PullActionsParams, PullActionsResult, PullDiagnosticsParams, PullDiagnosticsResult,
    RenameResult, ScanProjectFolderParams, ScanProjectFolderResult, SearchPatternParams,
    SearchResults, ServiceDataNotification, SupportsFeatureParams, UpdateSettingsParams,
    UpdateSettingsResult,
};

pub struct WorkspaceServer {
    /// features available throughout the application
    features: Features,

    /// Open projects, including their settings, nested packages, and other
    /// metadata.
    pub(super) projects: Projects,

    /// The layout of projects and their internal packages.
    project_layout: Arc<ProjectLayout>,

    /// Module graph tracking inferred information across modules.
    module_graph: Arc<ModuleGraph>,

    /// Keeps all loaded plugins in memory, per project.
    plugin_caches: Arc<HashMap<ProjectKey, PluginCache>>,

    /// Stores the document (text content + version number) associated with a URL
    pub(super) documents: HashMap<Utf8PathBuf, Document, FxBuildHasher>,

    /// Stores the document sources used across the workspace
    file_sources: AppendOnlyVec<DocumentFileSource>,

    /// Stores patterns to search for.
    patterns: HashMap<PatternId, GritQuery, FxBuildHasher>,

    /// Node cache for faster parsing of modified documents.
    ///
    /// ## Concurrency
    ///
    /// Because `NodeCache` cannot be cloned, and `papaya` doesn't give us owned
    /// instances of stored values, we use an `FxHashMap` here, wrapped in a
    /// `Mutex`. The node cache is only used by writers, meaning this wouldn't
    /// be a great use case for `papaya` anyway. But it does mean we need to be
    /// careful for deadlocks, and release guards to the mutex as soon as we
    /// can.
    ///
    /// Additionally, we only use the node cache for documents opened through
    /// the LSP proxy, since the editor use case is the one where we benefit
    /// most from low-latency parsing, and having a document open in an editor
    /// gives us a clear signal that edits -- and thus reparsing -- is to be
    /// anticipated. For other documents, the performance degradation due to
    /// lock contention would not be worth the potential of faster reparsing
    /// that may never actually happen.
    pub(super) node_cache: Mutex<FxHashMap<Utf8PathBuf, NodeCache>>,

    /// File system implementation.
    pub(super) fs: Box<dyn FileSystem>,

    /// Channel sender for instructions to the [crate::WorkspaceWatcher].
    watcher_tx: Sender<WatcherInstruction>,

    /// Set containing all the watched folders.
    watched_folders: HashSet<Utf8PathBuf>,

    /// Channel sender for sending notifications of service data updates.
    pub(super) notification_tx: watch::Sender<ServiceDataNotification>,
}

/// The `Workspace` object is long-lived, so we want it to be able to cross
/// unwind boundaries.
/// In return, we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead too hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

impl WorkspaceServer {
    /// Creates a new [Workspace].
    pub fn new(
        fs: Box<dyn FileSystem>,
        watcher_tx: Sender<WatcherInstruction>,
        notification_tx: watch::Sender<ServiceDataNotification>,
        threads: Option<usize>,
    ) -> Self {
        init_thread_pool(threads);

        Self {
            features: Features::new(),
            projects: Default::default(),
            project_layout: Default::default(),
            module_graph: Default::default(),
            plugin_caches: Default::default(),
            documents: Default::default(),
            file_sources: AppendOnlyVec::default(),
            patterns: Default::default(),
            node_cache: Default::default(),
            fs,
            watcher_tx,
            watched_folders: Default::default(),
            notification_tx,
        }
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

            if deserialized
                .into_deserialized()
                .and_then(|config| config.root)
                .is_none_or(|root| root.value())
            {
                // Found our root config!
                return Ok(ancestor.to_path_buf());
            }
        }

        Err(WorkspaceError::not_found())
    }

    /// Checks whether the directory identified by the given `path` contains a
    /// Biome configuration file, and returns its path if found.
    fn get_config_file(&self, path: &Utf8Path) -> Option<Utf8PathBuf> {
        for config_file in ConfigName::file_names() {
            let mut config_path = path.to_path_buf();
            config_path.push(config_file);
            if self.fs().path_exists(&config_path) {
                return Some(config_path);
            }
        }

        None
    }

    /// Gets the supported capabilities for a given file path.
    #[instrument(level = "debug", skip(self), fields(
        path = display(path.as_path())
    ))]
    fn get_file_capabilities(&self, path: &BiomePath) -> Capabilities {
        let language = self.get_file_source(path);
        self.features.get_capabilities(language)
    }

    /// Retrieves the supported language of a file.
    fn get_file_source(&self, path: &Utf8Path) -> DocumentFileSource {
        self.documents
            .pin()
            .get(path)
            .map(|doc| doc.file_source_index)
            .and_then(|index| self.get_source(index))
            .unwrap_or(DocumentFileSource::from_path(path))
    }

    /// Returns an error factory function for unsupported features at a given
    /// path.
    fn build_capability_error<'a>(
        &'a self,
        path: &'a Utf8Path,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            let file_source = self.get_file_source(path);

            let language = DocumentFileSource::from_path(path).or(file_source);
            WorkspaceError::source_file_not_supported(
                language,
                path.to_string(),
                path.extension().map(|s| s.to_string()),
            )
        }
    }

    /// Returns a previously inserted file source by index.
    ///
    /// File sources can be inserted using `insert_source()`.
    fn get_source(&self, index: usize) -> Option<DocumentFileSource> {
        if index < self.file_sources.len() {
            Some(self.file_sources[index])
        } else {
            None
        }
    }

    /// Inserts a file source so that it can be retrieved by index later.
    ///
    /// Returns the index at which the file source can be retrieved using
    /// `get_source()`.
    #[tracing::instrument(level = "debug", skip_all)]
    fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        self.file_sources
            .iter()
            .position(|file_source| *file_source == document_file_source)
            .unwrap_or_else(|| self.file_sources.push(document_file_source))
    }

    /// Opens the file and marks it as opened by the scanner.
    pub(super) fn open_file_by_scanner(
        &self,
        params: OpenFileParams,
    ) -> Result<(), WorkspaceError> {
        self.open_file_internal(true, params)
    }

    #[tracing::instrument(level = "debug", skip(self, params), fields(
        project_key = display(params.project_key),
        path = display(params.path.as_path()),
    ))]
    fn open_file_internal(
        &self,
        opened_by_scanner: bool,
        params: OpenFileParams,
    ) -> Result<(), WorkspaceError> {
        let OpenFileParams {
            project_key,
            path,
            content,
            document_file_source,
            persist_node_cache,
        } = params;
        let path: Utf8PathBuf = path.into();

        if document_file_source.is_none() && !DocumentFileSource::can_read(path.as_path()) {
            return Ok(());
        }

        let mut source = document_file_source.unwrap_or(DocumentFileSource::from_path(&path));

        if let DocumentFileSource::Js(js) = &mut source {
            if path.extension().is_some_and(|extension| extension == "js") {
                let manifest = self.project_layout.get_node_manifest_for_path(&path);
                if let Some((_, manifest)) = manifest {
                    if manifest.r#type == Some(PackageType::CommonJs) {
                        js.set_module_kind(ModuleKind::Script);
                    }
                }
            }
        }

        let (content, version) = match content {
            FileContent::FromClient { content, version } => (content, Some(version)),
            FileContent::FromServer => (self.fs.read_file_from_path(&path)?, None),
        };

        let mut index = self.insert_source(source);

        let size = content.len();
        let limit = self.projects.get_max_file_size(project_key);

        let syntax = if size > limit {
            Some(Err(FileTooLarge { size, limit }))
        } else if document_file_source.is_none() && !DocumentFileSource::can_parse(path.as_path()) {
            None
        } else {
            let mut node_cache = NodeCache::default();
            let parsed = self.parse(project_key, &path, &content, index, &mut node_cache)?;

            if let Some(language) = parsed.language {
                index = self.insert_source(language);
            }

            if persist_node_cache {
                self.node_cache
                    .lock()
                    .unwrap()
                    .insert(path.clone(), node_cache);
            }

            Some(Ok(parsed.any_parse))
        };

        let documents = self.documents.pin();
        documents.compute(path.clone(), |current| {
            match current {
                Some((_path, document)) => {
                    let version = match (document.version, version) {
                        (Some(current_version), Some(new_version)) => {
                            // This is awkward. It most likely means we have two
                            // clients independently specifying their own version,
                            // with no way for us to distinguish them. Or it is a
                            // bug.
                            // The safest thing to do seems to use the _minimum_ of
                            // the versions specified, so that updates coming from
                            // either will be accepted.
                            Some(current_version.min(new_version))
                        }
                        (Some(current_version), None) => {
                            // It appears the document is open in a client, and the
                            // scanner also wants to open/update the document. We
                            // stick with the version from the client, and ignore
                            // this request.
                            Some(current_version)
                        }
                        (None, new_version) => {
                            // The document was only opened by the scanner, so
                            // whatever's the new version will do.
                            new_version
                        }
                    };

                    // If the document already had a version, but the new
                    // content is coming from the scanner, we keep the same
                    // content that was already in the document. This means,
                    // active clients are leading over the filesystem.
                    let content = if document.version.is_some() && opened_by_scanner {
                        document.content.clone()
                    } else {
                        content.clone()
                    };

                    Operation::Insert::<Document, ()>(Document {
                        content,
                        version,
                        file_source_index: index,
                        syntax: syntax.clone(),
                        opened_by_scanner: opened_by_scanner || document.opened_by_scanner,
                    })
                }
                None => Operation::Insert(Document {
                    content: content.clone(),
                    version,
                    file_source_index: index,
                    syntax: syntax.clone(),
                    opened_by_scanner,
                }),
            }
        });

        self.update_service_data(WatcherSignalKind::AddedOrChanged, &path)
    }

    /// Retrieves the parser result for a given file.
    ///
    /// Returns an error if no file exists in the workspace with this path.
    fn get_parse(&self, path: &Utf8Path) -> Result<AnyParse, WorkspaceError> {
        let documents = self.documents.pin();
        let syntax = documents
            .get(path)
            .and_then(|doc| doc.syntax.clone())
            .transpose();

        match syntax {
            Ok(syntax) => match syntax {
                None => Err(WorkspaceError::not_found()),
                Some(syntax) => Ok(syntax.clone()),
            },
            Err(FileTooLarge { .. }) => Err(WorkspaceError::file_ignored(path.to_string())),
        }
    }

    fn parse(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        content: &str,
        file_source_index: usize,
        node_cache: &mut NodeCache,
    ) -> Result<ParseResult, WorkspaceError> {
        let file_source = self
            .get_source(file_source_index)
            .ok_or_else(WorkspaceError::not_found)?;
        let capabilities = self.features.get_capabilities(file_source);

        let parse = capabilities
            .parser
            .parse
            .ok_or_else(self.build_capability_error(path))?;

        let settings = self
            .projects
            .get_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let parsed = parse(
            &BiomePath::new(path),
            file_source,
            content,
            settings.into(),
            node_cache,
        );
        Ok(parsed)
    }

    /// Checks whether a file is ignored in the top-level config's
    /// `files.ignore`/`files.include` or in the feature's `ignore`/`include`.
    #[instrument(level = "debug", skip(self), fields(ignored))]
    fn is_ignored(&self, project_key: ProjectKey, path: &Utf8Path, features: FeatureName) -> bool {
        let file_name = path.file_name();
        let ignored_by_features = {
            let mut ignored = false;

            for feature in features.iter() {
                // a path is ignored if it's ignored by all features
                ignored &= self
                    .projects
                    .is_ignored_by_feature_config(project_key, path, feature)
            }
            ignored
        };
        // Never ignore Biome's config file regardless `include`/`ignore`
        let ignored = (file_name != Some(ConfigName::biome_json()) || file_name != Some(ConfigName::biome_jsonc())) &&
            // Apply top-level `include`/`ignore`
            (self.is_ignored_by_top_level_config(project_key, path) ||
                // Apply feature-level `include`/`ignore`
                ignored_by_features);

        tracing::Span::current().record("ignored", ignored);

        ignored
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored_by_top_level_config(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        let Some(files_settings) = self.projects.get_files_settings(project_key) else {
            return false;
        };
        let mut is_included = true;
        if !files_settings.includes.is_unset() {
            is_included = if is_dir(path) {
                files_settings
                    .includes
                    .matches_directory_with_exceptions(path)
            } else {
                files_settings.includes.matches_with_exceptions(path)
            };
        }

        let ignore_matches = self.projects.get_vcs_ignored_matches(project_key);

        !is_included
            || ignore_matches
                .as_ref()
                .is_some_and(|ignored_matches| ignored_matches.is_ignored(path, is_dir(path)))
    }

    fn load_plugins(
        &self,
        project_key: ProjectKey,
        base_path: &Utf8Path,
        plugins: &Plugins,
    ) -> Vec<PluginDiagnostic> {
        let mut diagnostics = Vec::new();
        let plugin_cache = PluginCache::default();

        for plugin_config in plugins.iter() {
            match plugin_config {
                PluginConfiguration::Path(plugin_path) => {
                    match BiomePlugin::load(self.fs.as_ref(), plugin_path, base_path) {
                        Ok(plugin) => {
                            plugin_cache.insert_plugin(plugin_path.clone().into(), plugin);
                        }
                        Err(diagnostic) => diagnostics.push(diagnostic),
                    }
                }
            }
        }

        self.plugin_caches.pin().insert(project_key, plugin_cache);

        diagnostics
    }

    fn get_analyzer_plugins_for_project(&self, project_key: ProjectKey) -> AnalyzerPluginVec {
        self.plugin_caches
            .pin()
            .get(&project_key)
            .map(|cache| cache.get_analyzer_plugins())
            .unwrap_or_default()
    }

    /// It accepts a list of ignore files. If the VCS integration is enabled, the files
    /// are read and the [Settings] are updated.
    ///
    /// ## Errors
    ///
    /// - If the project doesn't exist
    /// - If it's not possible to read the ignore file
    /// - If the ignore file contains lines that contain incorrect globs
    pub(super) fn update_project_ignore_files(
        &self,
        project_key: ProjectKey,
        paths: &[BiomePath],
    ) -> Result<(), WorkspaceError> {
        let project_path = self.projects.get_project_path(project_key);
        let mut settings = self
            .projects
            .get_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let vcs_settings = &mut settings.vcs_settings;
        if !vcs_settings.is_enabled() {
            return Ok(());
        }
        for path in paths.iter().filter(|path| path.is_ignore()) {
            let is_in_project_path = project_path
                .as_ref()
                .is_some_and(|project_path| path.starts_with(project_path));

            // We need to pass the **directory** that contains the ignore file.
            let dir_ignore_file = path.parent().unwrap_or(path);

            if vcs_settings.is_ignore_file(path) && is_in_project_path {
                let content = self.fs.read_file_from_path(path)?;
                let patterns = content.lines().collect::<Vec<_>>();
                vcs_settings.store_ignore_patterns(dir_ignore_file, patterns.as_slice())?;
            }
        }

        self.projects.set_settings(project_key, settings);

        Ok(())
    }

    /// Updates the [ProjectLayout] for the given `path`.
    #[instrument(level = "debug", skip(self))]
    pub(super) fn update_project_layout(
        &self,
        signal_kind: WatcherSignalKind,
        path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        if path
            .file_name()
            .is_some_and(|filename| filename == "package.json")
        {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(WorkspaceError::not_found)?;

            match signal_kind {
                WatcherSignalKind::AddedOrChanged => {
                    let parsed = self.get_parse(path)?;
                    self.project_layout
                        .insert_serialized_node_manifest(package_path, parsed);
                }
                WatcherSignalKind::Removed => {
                    self.project_layout.remove_package(&package_path);
                }
            }
        }

        Ok(())
    }

    /// Updates the [ModuleGraph] for the given `paths`.
    #[instrument(level = "debug", skip(self))]
    pub(super) fn update_module_graph(&self, signal_kind: WatcherSignalKind, paths: &[BiomePath]) {
        let no_paths: &[BiomePath] = &[];
        let (added_or_changed_paths, removed_paths) = match signal_kind {
            WatcherSignalKind::AddedOrChanged => {
                let documents = self.documents.pin();
                let mut added_or_changed_paths = Vec::with_capacity(paths.len());
                for path in paths {
                    let root = documents.get(path.as_path()).and_then(|doc| {
                        let file_source = self.file_sources[doc.file_source_index];
                        match file_source {
                            DocumentFileSource::Js(_) => doc
                                .syntax
                                .as_ref()
                                .and_then(|syntax| syntax.as_ref().ok())
                                .map(AnyParse::tree),
                            _ => None,
                        }
                    });
                    added_or_changed_paths.push((path, root));
                }

                (added_or_changed_paths, no_paths)
            }
            WatcherSignalKind::Removed => (Vec::new(), paths),
        };

        self.module_graph.update_graph_for_js_paths(
            self.fs.as_ref(),
            &self.project_layout,
            &added_or_changed_paths,
            removed_paths,
        );
    }

    /// Updates the state of any services relevant to the given `path`.
    pub(super) fn update_service_data(
        &self,
        signal_kind: WatcherSignalKind,
        path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        let path = BiomePath::from(path);
        if path.is_config() || path.is_manifest() {
            self.update_project_layout(signal_kind, &path)?;
        }

        self.update_module_graph(signal_kind, &[path]);

        let _ = self.notification_tx.send(ServiceDataNotification::Updated);

        Ok(())
    }
}

impl Workspace for WorkspaceServer {
    fn fs(&self) -> &dyn FileSystem {
        self.fs.as_ref()
    }

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        let documents = self.documents.pin();
        let Some(document) = documents.get(params.path.as_path()) else {
            return Err(WorkspaceError::not_found());
        };

        let file_size = document.content.len();
        let limit = self.projects.get_max_file_size(params.project_key);
        Ok(CheckFileSizeResult { file_size, limit })
    }

    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let project_key = params.project_key;
        let path = params.path.as_path();
        let language = self.get_file_source(path);
        let capabilities = self.features.get_capabilities(language);
        let handle = WorkspaceSettingsHandle::from(
            self.projects
                .get_settings(project_key)
                .ok_or_else(WorkspaceError::no_project)?,
        );
        let mut file_features = FileFeaturesResult::new();
        let file_name = path.file_name();
        file_features = file_features.with_capabilities(&capabilities);
        file_features = file_features.with_settings_and_language(&handle, path, &capabilities);

        let Some(settings) = handle.settings() else {
            return Ok(file_features);
        };
        if settings.ignore_unknown_enabled()
            && language == DocumentFileSource::Unknown
            && self.get_file_source(&params.path) == DocumentFileSource::Unknown
        {
            file_features.ignore_not_supported();
        } else if file_name == Some(ConfigName::biome_json())
            || file_name == Some(ConfigName::biome_jsonc())
        {
            // Never ignore Biome's config file
        } else if self.is_ignored_by_top_level_config(project_key, path) {
            file_features.set_ignored_for_all_features();
        } else {
            for feature in params.features.iter() {
                if self
                    .projects
                    .is_ignored_by_feature_config(project_key, path, feature)
                {
                    file_features.ignored(feature);
                }
            }
        }
        // If the file is not ignored by at least one feature, then check that the file is not protected.
        //
        // Protected files must be ignored.
        if !file_features.is_not_processed() && FileFeaturesResult::is_protected_file(path) {
            file_features.set_protected_for_all_features();
        }

        Ok(file_features)
    }

    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        Ok(self.is_ignored(params.project_key, params.path.as_path(), params.features))
    }

    /// Updates the global settings for this workspace.
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "debug", skip(self))]
    fn update_settings(
        &self,
        params: UpdateSettingsParams,
    ) -> Result<UpdateSettingsResult, WorkspaceError> {
        let mut settings = self
            .projects
            .get_settings(params.project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let workspace_directory = params.workspace_directory.map(|p| p.to_path_buf());

        settings.merge_with_configuration(params.configuration, workspace_directory.clone())?;

        let diagnostics = self.load_plugins(
            params.project_key,
            &workspace_directory.unwrap_or_default(),
            &settings.plugins,
        );
        let has_errors = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error);
        if has_errors {
            // Note we also pass non-error diagnostics here. Filtering them
            // might be cleaner, but on the other hand, including them may
            // sometimes give a hint as to why an error occurred?
            return Err(WorkspaceError::plugin_errors(diagnostics));
        }

        self.projects.set_settings(params.project_key, settings);

        Ok(UpdateSettingsResult {
            diagnostics: diagnostics.into_iter().map(Into::into).collect(),
        })
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.open_file_internal(false, params)
    }

    fn open_project(&self, params: OpenProjectParams) -> Result<ProjectKey, WorkspaceError> {
        let path = if params.open_uninitialized {
            let path = params.path.to_path_buf();
            self.find_project_root(params.path).unwrap_or(path)
        } else {
            self.find_project_root(params.path)?
        };

        Ok(self.projects.insert_project(path))
    }

    #[instrument(level = "debug", skip(self))]
    fn scan_project_folder(
        &self,
        params: ScanProjectFolderParams,
    ) -> Result<ScanProjectFolderResult, WorkspaceError> {
        let path = params
            .path
            .map(Utf8PathBuf::from)
            .or_else(|| self.projects.get_project_path(params.project_key))
            .ok_or_else(WorkspaceError::no_project)?;

        if params.scan_kind.is_none() {
            let manifest = path.join("package.json");
            if self.fs.path_exists(&manifest) {
                self.open_file_by_scanner(OpenFileParams {
                    project_key: params.project_key,
                    path: BiomePath::from(manifest.clone()),
                    content: FileContent::FromServer,
                    document_file_source: None,
                    persist_node_cache: false,
                })?;
                self.update_project_layout(WatcherSignalKind::AddedOrChanged, &manifest)?;
            }
            return Ok(ScanProjectFolderResult {
                diagnostics: Vec::new(),
                duration: Duration::from_millis(0),
            });
        }

        let should_scan = params.force
            || !self
                .watched_folders
                .pin()
                .iter()
                .any(|watched_folder| path.starts_with(watched_folder));
        if !should_scan {
            // No need to scan folders that are already being watched.
            return Ok(ScanProjectFolderResult {
                diagnostics: Vec::new(),
                duration: Duration::from_millis(0),
            });
        }

        if params.watch {
            self.watched_folders.pin().insert(path.clone());

            let _ = self
                .watcher_tx
                .try_send(WatcherInstruction::WatchFolder(path.clone()));
        }

        let result = self.scan(params.project_key, &path, params.scan_kind)?;

        Ok(ScanProjectFolderResult {
            diagnostics: result.diagnostics,
            duration: result.duration,
        })
    }

    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError> {
        let project_path = self
            .projects
            .get_project_path(params.project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        self.watched_folders.pin().retain(|watched_folder| {
            if watched_folder.starts_with(&project_path) {
                let _ = self
                    .watcher_tx
                    .try_send(WatcherInstruction::UnwatchFolder(watched_folder.clone()));
                false
            } else {
                true
            }
        });

        // Limit the scope of the pin and the lock inside.
        {
            let documents = self.documents.pin();
            let mut node_cache = self.node_cache.lock().unwrap();
            for (path, document) in documents.iter() {
                if document.opened_by_scanner
                    && self
                        .projects
                        .path_belongs_only_to_project_with_path(path, &project_path)
                {
                    documents.remove(path);
                    node_cache.remove(path.as_path());
                }
            }
        }

        self.projects.remove_project(params.project_key);

        Ok(())
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_syntax_tree = capabilities
            .debug
            .debug_syntax_tree
            .ok_or_else(self.build_capability_error(&params.path))?;

        // The feature name here can be any feature, in theory
        let parse = self.get_parse(&params.path)?;
        let printed = debug_syntax_tree(&params.path, parse);

        Ok(printed)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_control_flow = capabilities
            .debug
            .debug_control_flow
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(&params.path)?;
        let printed = debug_control_flow(parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&params.path))?;
        let handle = WorkspaceSettingsHandle::from(
            self.projects
                .get_settings(params.project_key)
                .ok_or_else(WorkspaceError::no_project)?,
        );
        let parse = self.get_parse(&params.path)?;
        if !handle.format_with_errors_enabled_for_this_file_path(&params.path) && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);

        debug_formatter_ir(&params.path, &document_file_source, parse, handle)
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_type_info = capabilities
            .debug
            .debug_type_info
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;

        debug_type_info(&params.path, parse)
    }

    fn get_registered_types(
        &self,
        params: GetRegisteredTypesParams,
    ) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_registered_types = capabilities
            .debug
            .debug_registered_types
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;

        debug_registered_types(&params.path, parse)
    }

    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_semantic_model = capabilities
            .debug
            .debug_semantic_model
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;

        debug_semantic_model(&params.path, parse)
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.documents
            .pin()
            .get(params.path.as_path())
            .map(|document| document.content.clone())
            .ok_or_else(WorkspaceError::not_found)
    }

    /// Changes the content of an open file.
    fn change_file(
        &self,
        ChangeFileParams {
            project_key,
            path,
            content,
            version,
        }: ChangeFileParams,
    ) -> Result<(), WorkspaceError> {
        let documents = self.documents.pin();
        let (index, opened_by_scanner, existing_version) = documents
            .get(path.as_path())
            .map(|document| {
                (
                    document.file_source_index,
                    document.opened_by_scanner,
                    document.version,
                )
            })
            .ok_or_else(WorkspaceError::not_found)?;

        if existing_version.is_some_and(|existing_version| existing_version >= version) {
            warn!(%version, %path, "outdated_file_change");
            return Ok(()); // Safely ignore older versions.
        }

        // We remove the node cache for the document, if it exists.
        // This is done so that we need to hold the lock as short as possible
        // (it's released directly after the statement). The potential downside
        // is that if two calls to `change_file()` happen concurrently, then the
        // second would have a cache miss, and not update the cache either.
        // This seems an unlikely scenario however, and the impact is small
        // anyway, so this seems a worthwhile tradeoff.
        let node_cache = self.node_cache.lock().unwrap().remove(path.as_path());

        let persist_node_cache = node_cache.is_some();
        let mut node_cache = node_cache.unwrap_or_default();

        let parsed = self.parse(project_key, &path, &content, index, &mut node_cache)?;

        let document = Document {
            content,
            version: Some(version),
            file_source_index: index,
            syntax: Some(Ok(parsed.any_parse)),
            opened_by_scanner,
        };

        if persist_node_cache {
            self.node_cache
                .lock()
                .unwrap()
                .insert(path.to_path_buf(), node_cache);
        }

        documents
            .insert(path.clone().into(), document)
            .ok_or_else(WorkspaceError::not_found)?;

        self.update_service_data(WatcherSignalKind::AddedOrChanged, &path)
    }

    /// Closes a file that is opened in the workspace.
    ///
    /// This only unloads the document from the workspace if the file is NOT
    /// opened by the scanner as well. If the scanner has opened the file, it
    /// may still be required for multi-file analysis.
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        let path = params.path.as_path();

        let documents = self.documents.pin();
        let result = documents.compute(path.to_path_buf(), |current| {
            match current {
                Some((_path, document)) if document.opened_by_scanner => {
                    // If the scanner is still interested in the document, we
                    // only unset the version and re-sync the content below.
                    Operation::Insert(Document {
                        version: None,
                        ..document.clone()
                    })
                }
                Some(_) => Operation::Remove,
                None => Operation::Abort(()),
            }
        });

        // The node cache can be cleared in any case.
        self.node_cache.lock().unwrap().remove(path);

        match result {
            Compute::Inserted(_, _) => unreachable!(),
            Compute::Updated { .. } => {
                // This may look counter-intuitive, but we need to consider
                // that the file may have gone out-of-sync between the client
                // and the file system. So when the client closes it, and the
                // scanner still wants it, we need to resync it to make sure
                // they're back in sync.
                let _ = self
                    .watcher_tx
                    .send(WatcherInstruction::ResyncFile(path.to_path_buf()));

                Ok(())
            }
            Compute::Removed(_, _) => self.update_service_data(WatcherSignalKind::Removed, path),
            Compute::Aborted(_) => Err(WorkspaceError::not_found()),
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
        let PullDiagnosticsParams {
            project_key,
            path,
            categories,
            only,
            skip,
            enabled_rules,
            pull_code_actions,
        } = params;
        let parse = self.get_parse(&path)?;
        let language = self.get_file_source(&path);
        let capabilities = self.features.get_capabilities(language);
        let (diagnostics, errors, skipped_diagnostics) =
            if let Some(lint) = capabilities.analyzer.lint {
                let settings = self
                    .projects
                    .get_settings(project_key)
                    .ok_or_else(WorkspaceError::no_project)?;
                let results = lint(LintParams {
                    parse,
                    workspace: &settings.into(),
                    path: &path,
                    only,
                    skip,
                    language,
                    categories,
                    module_graph: self.module_graph.clone(),
                    project_layout: self.project_layout.clone(),
                    suppression_reason: None,
                    enabled_rules,
                    pull_code_actions,
                    plugins: self.get_analyzer_plugins_for_project(project_key),
                });

                (
                    results.diagnostics,
                    results.errors,
                    results.skipped_diagnostics,
                )
            } else {
                let parse_diagnostics = parse.into_diagnostics();
                let errors = parse_diagnostics
                    .iter()
                    .filter(|diag| diag.severity() <= Severity::Error)
                    .count();

                (parse_diagnostics, errors, 0)
            };

        info!(
            "Pulled {:?} diagnostic(s), skipped {:?} diagnostic(s)",
            diagnostics.len(),
            skipped_diagnostics
        );
        Ok(PullDiagnosticsResult {
            diagnostics: diagnostics
                .into_iter()
                .map(|diag| {
                    let diag = diag.with_file_path(path.to_string());
                    SerdeDiagnostic::new(diag)
                })
                .collect(),
            errors,
            skipped_diagnostics: skipped_diagnostics.into(),
        })
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
        } = params;
        let capabilities = self.get_file_capabilities(&path);
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&path))?;

        let parse = self.get_parse(&path)?;
        let language = self.get_file_source(&path);
        let settings = self
            .projects
            .get_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        Ok(code_actions(CodeActionsParams {
            parse,
            range,
            workspace: &settings.into(),
            path: &path,
            module_graph: self.module_graph.clone(),
            project_layout: self.project_layout.clone(),
            language,
            only,
            skip,
            suppression_reason: None,
            enabled_rules,
            plugins: self.get_analyzer_plugins_for_project(project_key),
        }))
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
        let capabilities = self.get_file_capabilities(&params.path);

        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&params.path))?;
        let handle = WorkspaceSettingsHandle::from(
            self.projects
                .get_settings(params.project_key)
                .ok_or_else(WorkspaceError::no_project)?,
        );

        let parse = self.get_parse(&params.path)?;

        if !handle.format_with_errors_enabled_for_this_file_path(&params.path) && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);
        format(&params.path, &document_file_source, parse, handle)
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = WorkspaceSettingsHandle::from(
            self.projects
                .get_settings(params.project_key)
                .ok_or_else(WorkspaceError::no_project)?,
        );
        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);
        format_range(
            &params.path,
            &document_file_source,
            parse,
            settings,
            params.range,
        )
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let handle = WorkspaceSettingsHandle::from(
            self.projects
                .get_settings(params.project_key)
                .ok_or_else(WorkspaceError::no_project)?,
        );
        let parse = self.get_parse(&params.path)?;
        if !handle.format_with_errors_enabled_for_this_file_path(&params.path) && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);

        format_on_type(
            &params.path,
            &document_file_source,
            parse,
            handle,
            params.offset,
        )
    }

    #[instrument(
        level = "debug",
        skip_all,
        fields(
            rule_categories = display(&params.rule_categories),
            skip = debug(&params.skip),
            only = debug(&params.only),
            should_format = display(&params.should_format),
        )
    )]
    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError> {
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
        } = params;
        let capabilities = self.get_file_capabilities(&path);

        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&path))?;
        let parse = self.get_parse(&path)?;

        let settings = self
            .projects
            .get_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let language = self.get_file_source(&path);
        fix_all(FixAllParams {
            parse,
            fix_file_mode,
            workspace: settings.into(),
            should_format,
            biome_path: &path,
            module_graph: self.module_graph.clone(),
            project_layout: self.project_layout.clone(),
            document_file_source: language,
            only,
            skip,
            rule_categories,
            suppression_reason,
            enabled_rules,
            plugins: self.get_analyzer_plugins_for_project(project_key),
        })
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(&params.path)?;
        let result = rename(&params.path, parse, params.symbol_at, params.new_name)?;

        Ok(result)
    }

    fn rage(&self, _: RageParams) -> Result<RageResult, WorkspaceError> {
        let entries = vec![
            RageEntry::section("Workspace"),
            RageEntry::pair("Open Documents", &format!("{}", self.documents.len())),
        ];

        Ok(RageResult { entries })
    }

    fn parse_pattern(
        &self,
        params: ParsePatternParams,
    ) -> Result<ParsePatternResult, WorkspaceError> {
        let options =
            CompilePatternOptions::default().with_default_language(params.default_language);
        let pattern = compile_pattern_with_options(&params.pattern, options)?;

        let pattern_id = make_search_pattern_id();
        self.patterns.pin().insert(pattern_id.clone(), pattern);
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
        let patterns = self.patterns.pin();
        let query = patterns
            .get(&pattern)
            .ok_or_else(WorkspaceError::invalid_pattern)?;

        let capabilities = self.get_file_capabilities(&path);
        let search = capabilities
            .search
            .search
            .ok_or_else(self.build_capability_error(&path))?;
        let settings = self
            .projects
            .get_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse(&path)?;

        let document_file_source = self.get_file_source(&path);
        let matches = search(&path, &document_file_source, parse, query, settings.into())?;

        Ok(SearchResults { path, matches })
    }

    fn drop_pattern(&self, params: super::DropPatternParams) -> Result<(), WorkspaceError> {
        self.patterns.pin().remove(&params.pattern);
        Ok(())
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
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

/// Generates a pattern ID that we can use as "handle" for referencing
/// previously parsed search queries.
fn make_search_pattern_id() -> PatternId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    let counter = COUNTER.fetch_add(1, Ordering::AcqRel);
    format!("p{counter}").into()
}

#[cfg(test)]
#[path = "server.tests.rs"]
mod tests;

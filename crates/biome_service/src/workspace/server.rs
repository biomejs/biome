use super::document::Document;
use super::{
    ChangeFileParams, CheckFileSizeParams, CheckFileSizeResult, CloseFileParams,
    CloseProjectParams, FileContent, FileExitsParams, FixFileParams, FixFileResult,
    FormatFileParams, FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams,
    GetFormatterIRParams, GetSemanticModelParams, GetSyntaxTreeParams, GetSyntaxTreeResult,
    IgnoreKind, OpenFileParams, OpenProjectParams, ParsePatternParams, ParsePatternResult,
    PatternId, ProjectKey, PullActionsParams, PullActionsResult, PullDiagnosticsParams,
    PullDiagnosticsResult, RenameResult, ScanProjectFolderParams, ScanProjectFolderResult,
    SearchPatternParams, SearchResults, ServiceDataNotification, SupportsFeatureParams,
    UpdateSettingsParams, UpdateSettingsResult,
};
use crate::configuration::{LoadedConfiguration, read_config};
use crate::diagnostics::{FileTooLarge, NoIgnoreFileFound, VcsDiagnostic};
use crate::file_handlers::html::{extract_embedded_scripts, parse_embedded_styles};
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DocumentFileSource, Features, FixAllParams, LintParams,
    ParseResult,
};
use crate::projects::Projects;
use crate::workspace::scanner::ScanOptions;
use crate::workspace::{
    FileFeaturesResult, GetFileContentParams, GetRegisteredTypesParams, GetTypeInfoParams,
    IsPathIgnoredParams, OpenProjectResult, RageEntry, RageParams, RageResult, ScanKind,
    ServerInfo,
};
use crate::workspace_watcher::{OpenFileReason, WatcherSignalKind};
use crate::{WatcherInstruction, Workspace, WorkspaceError};
use append_only_vec::AppendOnlyVec;
use biome_analyze::{AnalyzerPluginVec, RuleCategory};
use biome_configuration::bool::Bool;
use biome_configuration::plugins::{PluginConfiguration, Plugins};
use biome_configuration::vcs::VcsClientKind;
use biome_configuration::{BiomeDiagnostic, Configuration, ConfigurationPathHint};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::print_diagnostic_to_string;
use biome_diagnostics::{
    Diagnostic, DiagnosticExt, Severity, serde::Diagnostic as SerdeDiagnostic,
};
use biome_formatter::Printed;
use biome_fs::{BiomePath, ConfigName};
use biome_grit_patterns::{CompilePatternOptions, GritQuery, compile_pattern_with_options};
use biome_js_syntax::{AnyJsRoot, ModuleKind};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonFileSource;
use biome_module_graph::ModuleGraph;
use biome_package::PackageType;
use biome_parser::AnyParse;
use biome_plugin_loader::{BiomePlugin, PluginCache, PluginDiagnostic};
use biome_project_layout::ProjectLayout;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AstNode, NodeCache, SendNode};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::Sender;
use papaya::{Compute, HashMap, Operation};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::panic::RefUnwindSafe;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch;
use tracing::{info, instrument, warn};

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
    plugin_caches: Arc<HashMap<Utf8PathBuf, PluginCache>>,

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
    /// careful with deadlocks and release guards to the mutex as soon as we
    /// can.
    ///
    /// Additionally, we only use the node cache for documents opened through
    /// the LSP proxy, since the editor use case is the one where we benefit
    /// most from low-latency parsing, and having a document open in an editor
    /// gives us a clear signal that edits -- and thus reparsing -- are to be
    /// expected. For other documents, the performance degradation due to
    /// lock contention would not be worth the potential of faster reparsing
    /// that may never actually happen.
    pub(super) node_cache: Mutex<FxHashMap<Utf8PathBuf, NodeCache>>,

    /// File system implementation.
    pub(super) fs: Arc<dyn FsWithResolverProxy>,

    /// Channel sender for instructions to the [crate::WorkspaceWatcher].
    pub(super) watcher_tx: Sender<WatcherInstruction>,

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
        fs: Arc<dyn FsWithResolverProxy>,
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

            if let Some(configuration) = deserialized.into_deserialized() {
                let found = configuration.root.is_none_or(|root| root.value());
                // Found our root config!
                if found {
                    return Ok(ancestor.to_path_buf());
                }
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
    fn insert_source(&self, document_file_source: DocumentFileSource) -> usize {
        self.file_sources
            .iter()
            .position(|file_source| *file_source == document_file_source)
            .unwrap_or_else(|| self.file_sources.push(document_file_source))
    }

    /// Opens the file and marks it as opened by the scanner.
    pub(super) fn open_file_during_initial_scan(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
    ) -> Result<(), WorkspaceError> {
        self.open_file_for_reason(project_key, path.into(), OpenFileReason::InitialScan)
    }

    /// Opens the file and marks it as opened by the scanner.
    #[instrument(level = "debug", skip(self, path))]
    pub(super) fn open_file_by_watcher(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: impl Into<BiomePath>,
    ) -> Result<(), WorkspaceError> {
        let path = path.into();
        let Some(project_key) = self.projects.find_project_for_path(path.as_path()) else {
            return Ok(()); // file events outside our projects can be safely ignored.
        };

        if self.is_ignored_by_scanner(project_key, scan_kind, &path, IgnoreKind::Ancestors)? {
            return Ok(());
        }

        self.open_file_for_reason(project_key, path, OpenFileReason::WatcherUpdate)
    }

    fn open_file_for_reason(
        &self,
        project_key: ProjectKey,
        path: BiomePath,
        reason: OpenFileReason,
    ) -> Result<(), WorkspaceError> {
        match self
            .module_graph
            .get_or_insert_path_info(&path, self.fs.as_ref())
        {
            Some(path_info) if path_info.is_symlink() => Ok(()),
            Some(_) => self.open_file_internal(
                reason,
                OpenFileParams {
                    project_key,
                    path,
                    content: FileContent::FromServer,
                    document_file_source: None,
                    persist_node_cache: false,
                },
            ),
            None => Err(WorkspaceError::cant_read_file(path.to_string())),
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
            match path.extension() {
                Some("js") => {
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

        let mut index = self.insert_source(source);

        let size = content.len();
        let limit = self.projects.get_max_file_size(project_key, path.as_path());

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
        let root = syntax
            .as_ref()
            .and_then(|syntax| syntax.as_ref().ok())
            .map(|parse| parse.root());

        let opened_by_scanner = reason.is_opened_by_scanner();

        // Second-pass parsing for HTML files with embedded JavaScript and CSS content
        let (embedded_scripts, embedded_styles) = if let Some(DocumentFileSource::Html(_)) =
            self.get_source(index)
            && let Some(Ok(any_parse)) = &syntax
            && let Some(html_root) = biome_html_syntax::HtmlRoot::cast(any_parse.syntax())
        {
            let mut node_cache = NodeCache::default();
            let scripts = extract_embedded_scripts(&html_root, &mut node_cache);
            let styles = parse_embedded_styles(&html_root, &mut node_cache);
            (scripts, styles)
        } else {
            (Vec::new(), Vec::new())
        };

        let documents = self.documents.pin();
        let result = documents.compute(path.clone(), |current| {
            let biome_path = BiomePath::new(&path);
            if biome_path.is_dependency() && biome_path.is_type_declaration() {
                return if current.is_some() {
                    Operation::Remove
                } else {
                    // The document isn't inside the current files, however we want
                    // signal that it's a type declaration, and we want to update the module graph
                    Operation::Abort(true)
                };
            }
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
                    if document.version.is_some() && opened_by_scanner {
                        let mut doc = document.clone();
                        doc.opened_by_scanner = true;
                        return Operation::Insert(doc);
                    };

                    Operation::Insert::<Document, bool>(Document {
                        content: content.clone(),
                        version,
                        file_source_index: index,
                        syntax: syntax.clone(),
                        opened_by_scanner: opened_by_scanner || document.opened_by_scanner,
                        _embedded_scripts: embedded_scripts.clone(),
                        _embedded_styles: embedded_styles.clone(),
                    })
                }
                None => Operation::Insert(Document {
                    content: content.clone(),
                    version,
                    file_source_index: index,
                    syntax: syntax.clone(),
                    opened_by_scanner,
                    _embedded_scripts: embedded_scripts.clone(),
                    _embedded_styles: embedded_styles.clone(),
                }),
            }
        });

        let opened_by_scanner = match result {
            Compute::Inserted(_, document)
            | Compute::Updated {
                new: (_, document), ..
            } => document.opened_by_scanner,
            Compute::Aborted(result) => result,
            _ => false,
        };

        if opened_by_scanner {
            self.update_service_data(WatcherSignalKind::AddedOrChanged(reason), &path, root)
        } else {
            // If the document was never opened by the scanner, we don't care
            // about updating service data.
            Ok(())
        }
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
            .get_settings_based_on_path(project_key, path)
            .ok_or_else(WorkspaceError::no_project)?;
        let parsed = parse(
            &BiomePath::new(path),
            file_source,
            content,
            &settings,
            node_cache,
        );
        Ok(parsed)
    }

    fn load_plugins(&self, base_path: &Utf8Path, plugins: &Plugins) -> Vec<PluginDiagnostic> {
        let mut diagnostics = Vec::new();
        let plugin_cache = PluginCache::default();

        for plugin_config in plugins.iter() {
            match plugin_config {
                PluginConfiguration::Path(plugin_path) => {
                    match BiomePlugin::load(self.fs.as_ref(), plugin_path, base_path) {
                        Ok((plugin, _)) => {
                            plugin_cache.insert_plugin(plugin_path.clone().into(), plugin);
                        }
                        Err(diagnostic) => diagnostics.push(diagnostic),
                    }
                }
            }
        }

        self.plugin_caches
            .pin()
            .insert(base_path.to_path_buf(), plugin_cache);

        diagnostics
    }

    fn get_analyzer_plugins_for_project(
        &self,
        path: &Utf8Path,
        plugins: &Plugins,
    ) -> Result<AnalyzerPluginVec, Vec<PluginDiagnostic>> {
        match self.plugin_caches.pin().get(path) {
            Some(cache) => cache.get_analyzer_plugins(plugins),
            None => Ok(Vec::new()),
        }
    }

    /// Updates the nested settings of the project assigned to the
    /// `project_key`.
    ///
    /// If a configuration file contains errors, it's not processed and the
    /// project isn't updated.
    ///
    /// It's the responsibility of the client to process the diagnostics and
    /// handle the errors emitted by the configuration.
    ///
    /// ## Errors
    ///
    /// - A nested configuration file is a root
    /// - Biome can't read the file
    pub(super) fn update_project_config_files(
        &self,
        project_key: ProjectKey,
        paths: &[BiomePath],
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        let project_path = self
            .projects
            .get_project_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let mut returned_diagnostics = Vec::new();

        let filtered_paths = paths
            .iter()
            // We remove the root configuration file from the list of paths
            // SAFETY: the paths received are files, so it's safe to assume they have a parent folder
            .filter(|config_path| project_path != config_path.parent().unwrap().as_std_path());

        for filtered_path in filtered_paths {
            let config = read_config(
                self.fs(),
                ConfigurationPathHint::FromWorkspace(filtered_path.as_path().to_path_buf()),
                false,
            )?;
            let loaded_nested_configuration =
                LoadedConfiguration::try_from_payload(config, self.fs())?;

            let LoadedConfiguration {
                directory_path: nested_directory_path,
                configuration: nested_configuration,
                diagnostics,
                ..
            } = loaded_nested_configuration;
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
                    .projects
                    .get_root_settings(project_key)
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

            let result = self.update_settings(UpdateSettingsParams {
                project_key,
                workspace_directory: nested_directory_path.map(BiomePath::from),
                configuration: nested_configuration,
            })?;

            returned_diagnostics.extend(result.diagnostics)
        }

        Ok(returned_diagnostics)
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
        let project_path = self
            .projects
            .get_project_path(project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let mut settings = self
            .projects
            .get_root_settings(project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let vcs_settings = &mut settings.vcs_settings;

        if !vcs_settings.is_enabled() {
            return Ok(());
        }

        if !vcs_settings.should_use_ignore_file() {
            return Ok(());
        }

        let filtered_paths = paths.iter().filter(|path| path.is_ignore()).filter(|path| {
            // We filter out the root ignore file, because it's stored when calling `update_settings`
            // SAFETY: the paths received are files, so it's safe to assume they have a parent folder
            project_path.as_path() != path.parent().unwrap()
        });
        for path in filtered_paths {
            let is_in_project_path = path.starts_with(&project_path);

            // We need to pass the **directory** that contains the ignore file.
            let dir_ignore_file = path.parent().unwrap_or(path);

            if vcs_settings.is_ignore_file(path) && is_in_project_path {
                let content = self.fs.read_file_from_path(path)?;
                let patterns = content.lines().collect::<Vec<_>>();
                vcs_settings.store_nested_ignore_patterns(dir_ignore_file, patterns.as_slice())?;
            }
        }

        self.projects.set_root_settings(project_key, settings);

        Ok(())
    }

    /// Updates the [ProjectLayout] for the given `path`.
    pub(super) fn update_project_layout(
        &self,
        signal_kind: WatcherSignalKind,
        path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        let filename = path.file_name();
        if filename.is_some_and(|filename| filename == "package.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(WorkspaceError::not_found)?;

            match signal_kind {
                WatcherSignalKind::AddedOrChanged(_) => {
                    let parsed = self.get_parse(path)?;
                    self.project_layout
                        .insert_serialized_node_manifest(package_path, parsed);
                }
                WatcherSignalKind::Removed => {
                    self.project_layout.remove_package(&package_path);
                }
            }
        } else if filename.is_some_and(|filename| filename == "tsconfig.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(WorkspaceError::not_found)?;

            match signal_kind {
                WatcherSignalKind::AddedOrChanged(_) => {
                    let parsed = self.get_parse(path)?;
                    self.project_layout
                        .insert_serialized_tsconfig(package_path, parsed);
                }
                WatcherSignalKind::Removed => {
                    self.project_layout
                        .remove_tsconfig_from_package(&package_path);
                }
            }
        }

        Ok(())
    }

    /// Updates the [ModuleGraph] for the given `path` with an optional `root`.
    #[tracing::instrument(level = "debug", skip(self, root))]
    fn update_module_graph(
        &self,
        signal_kind: WatcherSignalKind,
        path: &BiomePath,
        root: Option<SendNode>,
    ) {
        let (added_or_changed_paths, removed_paths) = match signal_kind {
            WatcherSignalKind::AddedOrChanged(_) => {
                let Some(root) = root.and_then(SendNode::into_node).and_then(AnyJsRoot::cast)
                else {
                    return;
                };

                (&[(path, root)] as &[_], &[] as &[_])
            }
            WatcherSignalKind::Removed => (&[] as &[_], &[path] as &[_]),
        };

        self.module_graph.update_graph_for_js_paths(
            self.fs.as_ref(),
            &self.project_layout,
            added_or_changed_paths,
            removed_paths,
        );
    }

    /// Updates the state of any services relevant to the given `path`.
    #[instrument(level = "debug", skip(self, path, root))]
    pub(super) fn update_service_data(
        &self,
        signal_kind: WatcherSignalKind,
        path: &Utf8Path,
        root: Option<SendNode>,
    ) -> Result<(), WorkspaceError> {
        let path = BiomePath::from(path);
        if path.is_config() || path.is_manifest() {
            self.update_project_layout(signal_kind, &path)?;
        }

        self.update_module_graph(signal_kind, &path, root);

        match signal_kind {
            WatcherSignalKind::AddedOrChanged(OpenFileReason::InitialScan) => {
                // We'll send a single signal at the end of the scan.
            }
            _ => {
                let _ = self.notification_tx.send(ServiceDataNotification::Updated);
            }
        }

        Ok(())
    }
}

impl Workspace for WorkspaceServer {
    fn open_project(&self, params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError> {
        let path = if params.open_uninitialized {
            let path = params.path.to_path_buf();
            self.find_project_root(params.path).unwrap_or(path)
        } else {
            self.find_project_root(params.path)?
        };

        let project_key = self.projects.insert_project(path);

        Ok(OpenProjectResult { project_key })
    }

    fn scan_project_folder(
        &self,
        ScanProjectFolderParams {
            project_key,
            path,
            watch,
            force: _, // FIXME: `force` does nothing at the moment.
            scan_kind,
            verbose,
        }: ScanProjectFolderParams,
    ) -> Result<ScanProjectFolderResult, WorkspaceError> {
        let path = path
            .map(Utf8PathBuf::from)
            .or_else(|| self.projects.get_project_path(project_key))
            .ok_or_else(WorkspaceError::no_project)?;

        if scan_kind.is_none() {
            let manifest = path.join("package.json");
            if self.fs.path_exists(&manifest) {
                self.open_file_during_initial_scan(project_key, manifest.clone())?;
                self.update_project_layout(
                    WatcherSignalKind::AddedOrChanged(OpenFileReason::InitialScan),
                    &manifest,
                )?;
            }
            return Ok(ScanProjectFolderResult {
                diagnostics: Vec::new(),
                duration: Duration::from_millis(0),
                configuration_files: vec![],
            });
        }

        let scan_options = ScanOptions {
            scan_kind,
            verbose,
            watch,
        };

        let result = self.scan(project_key, &path, scan_options)?;

        let _ = self.notification_tx.send(ServiceDataNotification::Updated);

        Ok(ScanProjectFolderResult {
            diagnostics: result.diagnostics,
            duration: result.duration,
            configuration_files: result.configuration_files,
        })
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
        } = params;
        let mut diagnostics: Vec<biome_diagnostics::serde::Diagnostic> = vec![];
        let workspace_directory = workspace_directory.map(|p| p.to_path_buf());
        let is_root = configuration.is_root();
        let extends_root = configuration.extends_root();
        let mut settings = if !is_root {
            if !self.projects.is_project_registered(project_key) {
                return Err(WorkspaceError::no_project());
            }

            if let Some(workspace_directory) = &workspace_directory {
                self.projects
                    .get_nested_settings(project_key, workspace_directory.as_path())
                    .unwrap_or_default()
            } else {
                return Err(WorkspaceError::no_workspace_directory());
            }
        } else {
            self.projects
                .get_root_settings(project_key)
                .ok_or_else(WorkspaceError::no_project)?
        };

        settings.merge_with_configuration(configuration, workspace_directory.clone())?;

        let loading_directory = if extends_root {
            self.projects.get_project_path(project_key)
        } else {
            workspace_directory.clone()
        };

        let plugin_diagnostics = self.load_plugins(
            &loading_directory.clone().unwrap_or_default(),
            &settings.as_all_plugins(),
        );

        let has_errors = plugin_diagnostics
            .iter()
            .any(|d| d.severity() >= Severity::Error);

        if has_errors {
            return Err(WorkspaceError::plugin_errors(plugin_diagnostics));
        }

        diagnostics.extend(
            plugin_diagnostics
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        );

        if !is_root {
            self.projects.set_nested_settings(
                project_key,
                workspace_directory.unwrap_or_default(),
                settings,
            );
        } else {
            // If the configuration is a root one, we also load the ignore files
            if settings.is_vcs_enabled() && settings.vcs_settings.should_use_ignore_file() {
                let directory = workspace_directory.unwrap_or_default();
                match settings.vcs_settings.client_kind {
                    None => {}
                    Some(VcsClientKind::Git) => {
                        let gitignore = directory.join(".gitignore");
                        let ignore = directory.join(".ignore");
                        let result = self
                            .fs()
                            .read_file_from_path(gitignore.as_ref())
                            .ok()
                            .or_else(|| self.fs().read_file_from_path(ignore.as_ref()).ok());
                        let content = match result {
                            Some(content) => content,
                            None => {
                                diagnostics.push(biome_diagnostics::serde::Diagnostic::new(
                                    VcsDiagnostic::NoIgnoreFileFound(NoIgnoreFileFound {
                                        path: directory.to_string(),
                                    }),
                                ));
                                return Ok(UpdateSettingsResult { diagnostics });
                            }
                        };

                        let lines: Vec<_> = content.lines().collect();
                        settings
                            .vcs_settings
                            .store_root_ignore_patterns(directory.as_ref(), lines.as_slice())?;
                    }
                }
            }

            self.projects.set_root_settings(project_key, settings);
        }

        Ok(UpdateSettingsResult { diagnostics })
    }

    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError> {
        let project_path = self
            .projects
            .get_project_path(params.project_key)
            .ok_or_else(WorkspaceError::no_project)?;

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

        let _ = self
            .watcher_tx
            .try_send(WatcherInstruction::UnwatchFolder(project_path));

        self.projects.remove_project(params.project_key);

        Ok(())
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.open_file_internal(OpenFileReason::ClientRequest, params)
    }

    fn file_exists(&self, params: FileExitsParams) -> Result<bool, WorkspaceError> {
        Ok(self
            .documents
            .pin()
            .contains_key(params.file_path.as_path()))
    }

    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let language = self.get_file_source(&params.path);
        let capabilities = self.features.get_capabilities(language);

        self.projects.get_file_features(
            params.project_key,
            &params.path,
            params.features,
            language,
            &capabilities,
        )
    }

    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        // Never ignore Biome's top-level config file regardless of `includes`.
        if params.path.file_name().is_some_and(|file_name| {
            file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc()
        }) && params.path.parent().is_some_and(|dir_path| {
            self.projects
                .get_project_path(params.project_key)
                .is_some_and(|project_path| dir_path == project_path)
        }) {
            return Ok(false);
        };

        Ok(self.projects.is_ignored(
            params.project_key,
            &params.path,
            params.features,
            params.ignore_kind,
        ))
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);

        debug_formatter_ir(&params.path, &document_file_source, parse, &settings)
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_type_info = capabilities
            .debug
            .debug_type_info
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path).ok();

        debug_type_info(&params.path, parse, self.module_graph.clone())
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

    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError> {
        let documents = self.documents.pin();
        let Some(document) = documents.get(params.path.as_path()) else {
            return Err(WorkspaceError::not_found());
        };
        let file_size = document.content.len();
        let limit = self
            .projects
            .get_max_file_size(params.project_key, params.path.as_path());
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
        let root = parsed.any_parse.root();

        // Second-pass parsing for HTML files with embedded JavaScript and CSS content
        let (embedded_scripts, embedded_styles) = if let Some(file_source) = self.get_source(index)
        {
            if matches!(file_source, DocumentFileSource::Html(_)) {
                if let Some(html_root) =
                    biome_html_syntax::HtmlRoot::cast(parsed.any_parse.syntax().clone())
                {
                    let mut embedded_node_cache = NodeCache::default();
                    let scripts = extract_embedded_scripts(&html_root, &mut embedded_node_cache);
                    let styles = parse_embedded_styles(&html_root, &mut embedded_node_cache);
                    (scripts, styles)
                } else {
                    (Vec::new(), Vec::new())
                }
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        };

        let document = Document {
            content,
            version: Some(version),
            file_source_index: index,
            syntax: Some(Ok(parsed.any_parse)),
            opened_by_scanner,
            _embedded_scripts: embedded_scripts,
            _embedded_styles: embedded_styles,
        };

        if persist_node_cache {
            self.node_cache
                .lock()
                .unwrap()
                .insert(path.to_path_buf(), node_cache);
        }

        let opened_by_scanner = document.opened_by_scanner;
        documents
            .insert(path.clone().into(), document)
            .ok_or_else(WorkspaceError::not_found)?;

        if opened_by_scanner {
            self.update_service_data(
                WatcherSignalKind::AddedOrChanged(OpenFileReason::ClientRequest),
                &path,
                Some(root),
            )
        } else {
            Ok(())
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
                    .get_settings_based_on_path(project_key, &path)
                    .ok_or_else(WorkspaceError::no_project)?;

                let plugins = self
                    .get_analyzer_plugins_for_project(
                        settings.source_path().unwrap_or_default().as_path(),
                        &settings.get_plugins_for_path(&path),
                    )
                    .map_err(WorkspaceError::plugin_errors)?;
                let results = lint(LintParams {
                    parse,
                    settings: &settings,
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
                    plugins: if categories.contains(RuleCategory::Lint) {
                        plugins
                    } else {
                        Vec::new()
                    },
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
            "Pulled {:?} diagnostic(s), skipped {:?} diagnostic(s) from {}",
            diagnostics.len(),
            skipped_diagnostics,
            path
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
            categories,
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
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        Ok(code_actions(CodeActionsParams {
            parse,
            range,
            settings: &settings,
            path: &path,
            module_graph: self.module_graph.clone(),
            project_layout: self.project_layout.clone(),
            language,
            only,
            skip,
            suppression_reason: None,
            enabled_rules,
            plugins: Vec::new(),
            categories,
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;

        let parse = self.get_parse(&params.path)?;

        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);
        format(&params.path, &document_file_source, parse, &settings)
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
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
            &settings,
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

        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(&params.path);

        format_on_type(
            &params.path,
            &document_file_source,
            parse,
            &settings,
            params.offset,
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
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let plugins = self
            .get_analyzer_plugins_for_project(
                settings.source_path().unwrap_or_default().as_path(),
                &settings.get_plugins_for_path(&path),
            )
            .map_err(WorkspaceError::plugin_errors)?;
        let language = self.get_file_source(&path);
        fix_all(FixAllParams {
            parse,
            fix_file_mode,
            settings: &settings,
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
            plugins: if rule_categories.contains(RuleCategory::Lint) {
                plugins
            } else {
                Vec::new()
            },
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
            Compute::Inserted(_, _) => Ok(()), // should be unreachable
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
            Compute::Removed(_, _) => Ok(()),
            Compute::Aborted(_) => Err(WorkspaceError::not_found()),
        }
    }

    fn fs(&self) -> &dyn FsWithResolverProxy {
        self.fs.as_ref()
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
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse(&path)?;

        let document_file_source = self.get_file_source(&path);
        let matches = search(&path, &document_file_source, parse, query, &settings)?;

        Ok(SearchResults { path, matches })
    }

    fn drop_pattern(&self, params: super::DropPatternParams) -> Result<(), WorkspaceError> {
        self.patterns.pin().remove(&params.pattern);
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

use std::panic::RefUnwindSafe;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::{document::Document, *};
use crate::Watcher;
use crate::configuration::{LoadedConfiguration, read_config};
use crate::diagnostics::{FileTooLarge, NoIgnoreFileFound, VcsDiagnostic};
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DocumentFileSource, Features, FixAllParams, FormatEmbedNode,
    LintParams, LintResults, ParseResult, UpdateSnippetsNodes,
};
use crate::projects::Projects;
use crate::scanner::{
    IndexRequestKind, IndexTrigger, ScanOptions, Scanner, ScannerWatcherBridge, WatcherInstruction,
    WorkspaceScannerBridge,
};
use crate::workspace::document::AnyEmbeddedSnippet;
use append_only_vec::AppendOnlyVec;
use biome_analyze::{AnalyzerPluginVec, RuleCategory};
use biome_configuration::bool::Bool;
use biome_configuration::max_size::MaxSize;
use biome_configuration::vcs::VcsClientKind;
use biome_configuration::{BiomeDiagnostic, Configuration, ConfigurationPathHint};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::print_diagnostic_to_string;
use biome_diagnostics::{
    Diagnostic, DiagnosticExt, Severity, serde::Diagnostic as SerdeDiagnostic,
};
use biome_formatter::Printed;
use biome_fs::{BiomePath, ConfigName, PathKind};
use biome_grit_patterns::{CompilePatternOptions, GritQuery, compile_pattern_with_options};
use biome_js_syntax::{AnyJsRoot, LanguageVariant, ModuleKind};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonFileSource;
use biome_module_graph::{ModuleDependencies, ModuleDiagnostic, ModuleGraph};
use biome_package::PackageType;
use biome_parser::AnyParse;
use biome_plugin_loader::{BiomePlugin, PluginCache, PluginDiagnostic};
use biome_plugin_loader::{PluginConfiguration, Plugins};
use biome_project_layout::ProjectLayout;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AstNode, NodeCache, SendNode};
use camino::{Utf8Path, Utf8PathBuf};
use crossbeam::channel::Sender;
use papaya::HashMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::time::Duration;
use tokio::sync::watch;
use tracing::{info, instrument, warn};

pub struct WorkspaceServer {
    /// features available throughout the application
    features: Features,

    /// Open projects, including their settings, nested packages, and other
    /// metadata.
    projects: Projects,

    /// The layout of projects and their internal packages.
    project_layout: Arc<ProjectLayout>,

    /// Module graph tracking inferred information across modules.
    module_graph: Arc<ModuleGraph>,

    /// Keeps all loaded plugins in memory, per project.
    plugin_caches: Arc<HashMap<Utf8PathBuf, PluginCache>>,

    /// Stores the document (text content + version number) associated with a URL
    documents: HashMap<Utf8PathBuf, Document, FxBuildHasher>,

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
    node_cache: Mutex<FxHashMap<Utf8PathBuf, NodeCache>>,

    /// Scanner instance.
    pub(crate) scanner: Scanner,

    /// File system implementation.
    fs: Arc<dyn FsWithResolverProxy>,

    /// Channel sender for sending notifications of service data updates.
    notification_tx: watch::Sender<ServiceNotification>,
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
        notification_tx: watch::Sender<ServiceNotification>,
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
            scanner: Scanner::new(watcher_tx),
            fs,
            notification_tx,
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

        Err(WorkspaceError::not_found())
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
        self.features.get_capabilities(language)
    }

    /// Retrieves the supported language of a file.
    fn get_file_source(
        &self,
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> DocumentFileSource {
        self.documents
            .pin()
            .get(path)
            .map(|doc| doc.file_source_index)
            .and_then(|index| self.get_source(index))
            .unwrap_or(DocumentFileSource::from_path(
                path,
                experimental_full_html_support,
            ))
    }

    /// Returns an error factory function for unsupported features at a given
    /// path.
    fn build_capability_error<'a>(
        &'a self,
        path: &'a Utf8Path,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            // For simplicity and avoid too many changes, we hardcode the support to false
            let file_source = self.get_file_source(path, false);

            let language = DocumentFileSource::from_path(path, false).or(file_source);
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
        let OpenFileParams {
            project_key,
            path: biome_path,
            content,
            document_file_source,
            persist_node_cache,
        } = params;
        let path: Utf8PathBuf = biome_path.clone().into();

        if document_file_source.is_none() && !DocumentFileSource::can_read(path.as_path()) {
            return Ok(Default::default());
        }

        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;

        let mut source = document_file_source.unwrap_or(DocumentFileSource::from_path(
            &path,
            settings.experimental_full_html_support_enabled(),
        ));

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
            if !js.is_typescript() && !js.is_jsx() {
                let jsx_everywhere = settings
                    .languages
                    .javascript
                    .parser
                    .jsx_everywhere
                    .unwrap_or_default()
                    .into();
                if jsx_everywhere {
                    js.set_variant(LanguageVariant::Jsx);
                }
            }
        }

        let (content, version) = match content {
            FileContent::FromClient { content, version } => (content, Some(version)),
            FileContent::FromServer => (self.fs.read_file_from_path(&path)?, None),
        };

        let mut file_source_index = self.insert_source(source);

        let size = content.len();
        let limit = settings.get_max_file_size(&path);

        let syntax = if size > limit {
            Some(Err(FileTooLarge { size, limit }))
        } else if document_file_source.is_none() && !DocumentFileSource::can_parse(path.as_path()) {
            None
        } else {
            let mut node_cache = NodeCache::default();
            let parsed = self.parse(
                &path,
                &content,
                &settings,
                file_source_index,
                &mut node_cache,
            )?;

            if let Some(language) = parsed.language {
                file_source_index = self.insert_source(language);
            }

            if persist_node_cache {
                self.node_cache
                    .lock()
                    .unwrap()
                    .insert(path.clone(), node_cache);
            }

            Some(Ok(parsed.any_parse))
        };
        // Second-pass parsing for HTML files with embedded JavaScript and CSS
        // content.
        let embedded_snippets = if DocumentFileSource::can_contain_embeds(
            path.as_path(),
            settings.experimental_full_html_support_enabled(),
        ) && let Some(Ok(any_parse)) = &syntax
        {
            // Second-pass parsing for HTML files with embedded JavaScript and CSS content

            let mut node_cache = NodeCache::default();
            self.parse_embedded_language_snippets(
                &biome_path,
                &source,
                any_parse,
                &mut node_cache,
                &settings,
            )?
        } else {
            Default::default()
        };

        let is_indexed = if reason.is_index() {
            // If the request is for indexing, we don't insert any document,
            // we only care about updating the module graph.
            true
        } else {
            self.documents.pin().update_or_insert_with(
                path.clone(),
                |current| {
                    let version = match (current.version, version) {
                        (Some(current_version), Some(new_version)) => {
                            // This is awkward. It most likely means we have
                            // two clients independently specifying their
                            // own version, with no way for us to
                            // distinguish them. Or it is a bug. The safest
                            // thing to do seems to use the _minimum_ of the
                            // versions specified, so that updates coming
                            // from either will be accepted.
                            Some(current_version.min(new_version))
                        }
                        (Some(current_version), None) => {
                            // It appears the document is open in a client,
                            // and the scanner also wants to open/update the
                            // document. We stick with the version from the
                            // client and ignore this request.
                            Some(current_version)
                        }
                        (None, new_version) => {
                            // The document was only opened by the scanner,
                            // so whatever's the new version will do.
                            new_version
                        }
                    };

                    Document {
                        content: content.clone(),
                        version,
                        file_source_index,
                        syntax: syntax.clone(),
                        embedded_snippets: embedded_snippets.clone(),
                    }
                },
                || Document {
                    content: content.clone(),
                    version,
                    file_source_index,
                    syntax: syntax.clone(),
                    embedded_snippets: embedded_snippets.clone(),
                },
            );

            self.is_indexed(&path)
        };

        // Manifest files need to update the module graph
        if is_indexed
            && let Some(root) = syntax
                .and_then(Result::ok)
                .map(|node| node.unwrap_as_send_node())
        {
            let (dependencies, diagnostics) =
                self.update_service_data(&path, UpdateKind::AddedOrChanged(reason, root))?;

            Ok(InternalOpenFileResult {
                dependencies,
                diagnostics,
            })
        } else {
            // If the document was never opened by the scanner, we don't care
            // about updating service data.
            Ok(InternalOpenFileResult::default())
        }
    }

    /// Retrieves the parser result for a given file.
    ///
    /// Returns an error if no file exists in the workspace with this path.
    fn get_parse(&self, path: &Utf8Path) -> Result<AnyParse, WorkspaceError> {
        let syntax = self
            .documents
            .pin()
            .get(path)
            .and_then(|doc| doc.syntax.clone())
            .transpose();

        match syntax {
            Ok(syntax) => match syntax {
                None => Err(WorkspaceError::not_found()),
                Some(syntax) => Ok(syntax),
            },
            Err(FileTooLarge { .. }) => Err(WorkspaceError::file_ignored(path.to_string())),
        }
    }

    fn get_parse_with_snippets(
        &self,
        path: &Utf8Path,
    ) -> Result<(AnyParse, Vec<AnyEmbeddedSnippet>), WorkspaceError> {
        self.documents
            .pin()
            .get(path)
            .ok_or_else(WorkspaceError::not_found)
            .and_then(|doc| match &doc.syntax {
                Some(syntax) => match syntax {
                    Ok(syntax) => Ok((syntax.clone(), doc.embedded_snippets.clone())),
                    Err(FileTooLarge { .. }) => Err(WorkspaceError::file_ignored(path.to_string())),
                },
                None => Err(WorkspaceError::not_found()),
            })
    }

    fn get_parse_with_embedded_format_nodes(
        &self,
        path: &Utf8Path,
    ) -> Result<(AnyParse, Vec<FormatEmbedNode>), WorkspaceError> {
        self.documents
            .pin()
            .get(path)
            .ok_or_else(WorkspaceError::not_found)
            .and_then(|doc| match &doc.syntax {
                Some(syntax) => match syntax {
                    Ok(syntax) => Ok((
                        syntax.clone(),
                        doc.get_embedded_snippets_format_nodes(|file_source_index| {
                            self.get_source(file_source_index)
                                .expect("Document source must exist")
                        }),
                    )),
                    Err(FileTooLarge { .. }) => Err(WorkspaceError::file_ignored(path.to_string())),
                },
                None => Err(WorkspaceError::not_found()),
            })
    }

    /// Parses the language snippets if the current language implements the capability `parser.parse_embedded_nodes`
    fn parse_embedded_language_snippets(
        &self,
        path: &BiomePath,
        source: &DocumentFileSource,
        root: &AnyParse,
        cache: &mut NodeCache,
        settings: &Settings,
    ) -> Result<Vec<AnyEmbeddedSnippet>, WorkspaceError> {
        let mut embedded_nodes = Vec::new();
        let capabilities =
            self.get_file_capabilities(path, settings.experimental_full_html_support_enabled());
        let Some(parse_embedded) = capabilities.parser.parse_embedded_nodes else {
            return Ok(Default::default());
        };
        let result = parse_embedded(root, path, source, settings, cache);

        for (mut content, file_source) in result.nodes {
            let index = self.insert_source(file_source);
            content.set_file_source_index(index);
            embedded_nodes.push(content);
        }

        Ok(embedded_nodes)
    }

    fn parse(
        &self,
        path: &Utf8Path,
        content: &str,
        settings: &Settings,
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

        let parsed = parse(
            &BiomePath::new(path),
            file_source,
            content,
            settings,
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
                    match BiomePlugin::load(self.fs.clone(), plugin_path, base_path) {
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

    /// Returns whether the given `path` that falls under the project with the
    /// given `project_key` is ignored, assuming the given `scan_kind` and
    /// `reason`.
    fn is_ignored_by_scanner(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
    ) -> Result<bool, WorkspaceError> {
        if self.projects.is_force_ignored(project_key, path) {
            return Ok(true);
        }

        // Determine the ignore kind based on the kind of request.
        let ignore_kind = match request_kind {
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
            //   `package.json` and type declarations, to avoid accidentally
            //   indexing minified files.
            // - The path shouldn't be indexed yet, to avoid double indexing.
            IndexRequestKind::Dependency(_) => {
                let path = BiomePath::new(path);
                if path.is_dependency() && !path.is_package_json() && !path.is_type_declaration() {
                    return Ok(true);
                }

                return Ok(self.is_indexed(&path));
            }
        };

        let path = BiomePath::new(path);
        let is_ignored = match self.fs.symlink_path_kind(&path)? {
            PathKind::Directory { .. } => {
                if path.is_dependency() {
                    // Every mode ignores dependencies, except project mode.
                    return Ok(!scan_kind.is_project());
                }

                if self.projects.is_ignored_by_top_level_config(
                    self.fs.as_ref(),
                    project_key,
                    &path,
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
                            self.projects.is_ignored_by_top_level_config(
                                self.fs.as_ref(),
                                project_key,
                                folder_path,
                                ignore_kind,
                            )
                        }),
                    },
                    ScanKind::Project => {
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
                                    self.projects.is_ignored_by_top_level_config(
                                        self.fs.as_ref(),
                                        project_key,
                                        folder_path,
                                        ignore_kind,
                                    )
                                }),
                            }
                        } else {
                            self.projects.is_ignored_by_top_level_config(
                                self.fs.as_ref(),
                                project_key,
                                &path,
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

    /// Updates the [ProjectLayout] for the given `path`.
    #[instrument(level = "debug", skip(self))]
    fn update_project_layout(
        &self,
        path: &Utf8Path,
        update_kind: &UpdateKind,
    ) -> Result<(), WorkspaceError> {
        let filename = path.file_name();
        if filename.is_some_and(|filename| filename == "package.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(WorkspaceError::not_found)?;

            match update_kind {
                UpdateKind::AddedOrChanged(_, root) => {
                    self.project_layout
                        .insert_serialized_node_manifest(package_path, root);
                }
                UpdateKind::Removed => {
                    self.project_layout.remove_package(&package_path);
                }
            }
        } else if filename.is_some_and(|filename| filename == "tsconfig.json") {
            let package_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(WorkspaceError::not_found)?;

            match update_kind {
                UpdateKind::AddedOrChanged(_, root) => {
                    self.project_layout
                        .insert_serialized_tsconfig(package_path, root);
                }
                UpdateKind::Removed => {
                    self.project_layout
                        .remove_tsconfig_from_package(&package_path);
                }
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
    fn update_module_graph_internal(
        &self,
        path: &BiomePath,
        update_kind: &UpdateKind,
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        let (added_or_changed_paths, removed_paths) = match update_kind {
            UpdateKind::AddedOrChanged(_, root) => {
                let Some(root) = SendNode::into_node(root.clone()).and_then(AnyJsRoot::cast) else {
                    return Default::default();
                };

                (&[(path, root)] as &[_], &[] as &[_])
            }
            UpdateKind::Removed => (&[] as &[_], &[path] as &[_]),
        };

        self.module_graph.update_graph_for_js_paths(
            self.fs.as_ref(),
            &self.project_layout,
            added_or_changed_paths,
            removed_paths,
        )
    }

    /// Updates the state of any services relevant to the given `path`.
    ///
    /// Returns the module dependencies of the `path` if `update_kind` is
    /// [`UpdateKind::AddedOrChanged`]. For other signal kinds, no dependencies
    /// are determined.
    fn update_service_data(
        &self,
        path: &Utf8Path,
        update_kind: UpdateKind,
    ) -> Result<(ModuleDependencies, Vec<ModuleDiagnostic>), WorkspaceError> {
        let path = BiomePath::from(path);
        if path.is_manifest() {
            self.update_project_layout(&path, &update_kind)?;
        }

        let result = self.update_module_graph_internal(&path, &update_kind);

        match update_kind {
            UpdateKind::AddedOrChanged(OpenFileReason::Index(IndexTrigger::InitialScan), _) => {
                // We'll send a single signal at the end of the scan.
            }
            _ => {
                let _ = self.notification_tx.send(ServiceNotification::IndexUpdated);
            }
        }

        Ok(result)
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

    fn scan_project(
        &self,
        ScanProjectParams {
            project_key,
            watch,
            force,
            scan_kind,
            verbose,
        }: ScanProjectParams,
    ) -> Result<ScanProjectResult, WorkspaceError> {
        let mut diagnostics = Vec::new();
        if scan_kind.is_none() {
            let path = self
                .projects
                .get_project_path(project_key)
                .ok_or_else(WorkspaceError::no_project)?;

            let manifest = path.join("package.json");
            if self.fs.path_exists(&manifest) {
                let trigger = IndexTrigger::InitialScan;
                let (_, _diagnostics) = self.index_file(project_key, manifest.clone(), trigger)?;
                diagnostics.extend(
                    _diagnostics
                        .into_iter()
                        .map(biome_diagnostics::serde::Diagnostic::new)
                        .collect::<Vec<_>>(),
                );
            }
            return Ok(ScanProjectResult {
                diagnostics,
                duration: Duration::from_millis(0),
                configuration_files: vec![],
            });
        }

        let scan_options = ScanOptions {
            scan_kind,
            force,
            verbose,
            watch,
        };

        let mut result = self
            .scanner
            .index_project(self, project_key, scan_options)?;

        result.diagnostics.extend(diagnostics);

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
                            .fs
                            .read_file_from_path(gitignore.as_ref())
                            .ok()
                            .or_else(|| self.fs.read_file_from_path(ignore.as_ref()).ok());
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

        self.projects.remove_project(params.project_key);
        self.scanner
            .unload_project(params.project_key, project_path.clone());

        // Unload all the documents within the project folder.
        let documents = self.documents.pin();
        let mut node_cache = self.node_cache.lock().unwrap();
        for path in documents.keys() {
            if self
                .projects
                .path_belongs_only_to_project_with_path(path, &project_path)
            {
                documents.remove(path);
                node_cache.remove(path.as_path());
            }
        }

        self.module_graph.unload_path(&project_path);
        self.project_layout.unload_folder(&project_path);
        self.plugin_caches.pin().remove(&project_path);

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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let language = self.get_file_source(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let capabilities = self.features.get_capabilities(language);

        self.projects.get_file_features(
            self.fs.as_ref(),
            params.project_key,
            &params.path,
            params.features,
            language,
            &capabilities,
        )
    }

    fn is_path_ignored(&self, params: PathIsIgnoredParams) -> Result<bool, WorkspaceError> {
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
            self.fs.as_ref(),
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let debug_control_flow = capabilities
            .debug
            .debug_control_flow
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(&params.path)?;
        let printed = debug_control_flow(parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );

        debug_formatter_ir(&params.path, &document_file_source, parse, &settings)
    }

    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError> {
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let debug_registered_types = capabilities
            .debug
            .debug_registered_types
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;

        debug_registered_types(&params.path, parse)
    }

    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError> {
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
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
            .get_settings_based_on_path(params.project_key, &params.path)
            .map_or_else(
                || MaxSize::default().into(),
                |settings| settings.get_max_file_size(&params.path),
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
        }: ChangeFileParams,
    ) -> Result<ChangeFileResult, WorkspaceError> {
        let documents = self.documents.pin();
        let (index, existing_version) = documents
            .get(path.as_path())
            .map(|document| (document.file_source_index, document.version))
            .ok_or_else(WorkspaceError::not_found)?;

        if existing_version.is_some_and(|existing_version| existing_version >= version) {
            warn!(%version, %path, "outdated_file_change");
            // Safely ignore older versions.
            return Ok(ChangeFileResult {
                diagnostics: Vec::new(),
            });
        }

        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;

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

        let parsed = self.parse(&path, &content, &settings, index, &mut node_cache)?;
        let root = parsed.any_parse.unwrap_as_send_node();
        let document_source =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());

        // Second-pass parsing for HTML files with embedded JavaScript and CSS content
        let embedded_snippets = if DocumentFileSource::can_contain_embeds(
            path.as_path(),
            settings.experimental_full_html_support_enabled(),
        ) {
            // Second-pass parsing for HTML files with embedded JavaScript and CSS content
            let mut node_cache = NodeCache::default();
            self.parse_embedded_language_snippets(
                &path,
                &document_source,
                &parsed.any_parse,
                &mut node_cache,
                &settings,
            )?
        } else {
            vec![]
        };

        let document = Document {
            content,
            version: Some(version),
            file_source_index: index,
            syntax: Some(Ok(parsed.any_parse)),
            embedded_snippets,
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

        let mut final_diagnostics = vec![];

        if self.is_indexed(&path) {
            let (dependencies, diagnostics) = self.update_service_data(
                &path,
                UpdateKind::AddedOrChanged(OpenFileReason::ClientRequest, root),
            )?;
            final_diagnostics.extend(
                diagnostics
                    .into_iter()
                    .map(biome_diagnostics::serde::Diagnostic::new)
                    .collect::<Vec<_>>(),
            );
            if !dependencies.is_empty()
                && let Some(project_path) = self.projects.get_project_path(project_key)
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
        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let (parse, embedded_snippets) = self.get_parse_with_snippets(&path)?;
        let language =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let capabilities = self.features.get_capabilities(language);
        let (diagnostics, errors, skipped_diagnostics) = if (categories.is_lint()
            || categories.is_assist())
            && let Some(lint) = capabilities.analyzer.lint
        {
            let plugins = if categories.is_lint() {
                self.get_analyzer_plugins_for_project(
                    settings.source_path().unwrap_or_default().as_path(),
                    &settings.get_plugins_for_path(&path),
                )
                .map_err(WorkspaceError::plugin_errors)?
            } else {
                Vec::new()
            };
            let results = lint(LintParams {
                parse,
                settings: &settings,
                path: &path,
                only: &only,
                skip: &skip,
                language,
                categories,
                module_graph: self.module_graph.clone(),
                project_layout: self.project_layout.clone(),
                suppression_reason: None,
                enabled_selectors: &enabled_rules,
                pull_code_actions,
                plugins: plugins.clone(),
                diagnostic_offset: None,
            });

            let LintResults {
                mut diagnostics,
                mut errors,
                mut skipped_diagnostics,
            } = results;
            for embedded_node in embedded_snippets {
                let Some(file_source) = self.get_source(embedded_node.file_source_index()) else {
                    continue;
                };
                let capabilities = self.features.get_capabilities(file_source);
                let Some(lint) = capabilities.analyzer.lint else {
                    continue;
                };

                let results = lint(LintParams {
                    parse: embedded_node.parse().clone(),
                    settings: &settings,
                    path: &path,
                    only: &only,
                    skip: &skip,
                    language: file_source,
                    categories,
                    module_graph: self.module_graph.clone(),
                    project_layout: self.project_layout.clone(),
                    suppression_reason: None,
                    enabled_selectors: &enabled_rules,
                    pull_code_actions,
                    plugins: plugins.clone(),
                    diagnostic_offset: Some(embedded_node.content_offset()),
                });

                diagnostics.extend(results.diagnostics);
                skipped_diagnostics += results.skipped_diagnostics;
                errors += results.errors;
            }

            (diagnostics, errors, skipped_diagnostics)
        } else {
            let mut parse_diagnostics = parse.into_serde_diagnostics(None);
            let mut errors = parse_diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            for embedded_node in embedded_snippets {
                let diagnostics = embedded_node.into_serde_diagnostics();
                errors += diagnostics
                    .iter()
                    .filter(|diag| diag.severity() <= Severity::Error)
                    .count();
                parse_diagnostics.extend(diagnostics);
            }

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
        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&path))?;

        let (parse, embedded_snippets) = self.get_parse_with_snippets(&path)?;
        let language =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let mut result = code_actions(CodeActionsParams {
            parse,
            range,
            settings: &settings,
            path: &path,
            module_graph: self.module_graph.clone(),
            project_layout: self.project_layout.clone(),
            language,
            only: &only,
            skip: &skip,
            suppression_reason: None,
            enabled_rules: &enabled_rules,
            plugins: Vec::new(),
            categories,
            action_offset: None,
        });

        for embedded_snippet in embedded_snippets {
            let Some(file_source) = self.get_source(embedded_snippet.file_source_index()) else {
                continue;
            };
            let capabilities = self.features.get_capabilities(file_source);
            let Some(code_actions) = capabilities.analyzer.code_actions else {
                continue;
            };

            let embedded_actions_result = code_actions(CodeActionsParams {
                parse: embedded_snippet.parse(),
                range,
                settings: &settings,
                path: &path,
                module_graph: self.module_graph.clone(),
                project_layout: self.project_layout.clone(),
                language: file_source,
                only: &only,
                skip: &skip,
                suppression_reason: None,
                enabled_rules: &enabled_rules,
                plugins: Vec::new(),
                categories,
                action_offset: Some(embedded_snippet.content_offset()),
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );

        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&params.path))?;

        let format_embedded = capabilities.formatter.format_embedded;

        let (parse, embedded_nodes) = self.get_parse_with_embedded_format_nodes(&params.path)?;

        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        let document_file_source = self.get_file_source(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        if !embedded_nodes.is_empty() {
            let format_embedded =
                format_embedded.ok_or_else(self.build_capability_error(&params.path))?;
            return format_embedded(
                &params.path,
                &document_file_source,
                parse,
                &settings,
                embedded_nodes,
            );
        }
        format(&params.path, &document_file_source, parse, &settings)
    }

    #[instrument(level = "debug", skip(self, params))]
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
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
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(&params.path)?;
        if !settings.format_with_errors_enabled_for_this_file_path(&params.path)
            && parse.has_errors()
        {
            return Err(WorkspaceError::format_with_errors_disabled());
        }
        let document_file_source = self.get_file_source(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );

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

        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());

        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&path))?;

        let (mut parse, embedded_snippets) = self.get_parse_with_snippets(&path)?;

        let plugins = self
            .get_analyzer_plugins_for_project(
                settings.source_path().unwrap_or_default().as_path(),
                &settings.get_plugins_for_path(&path),
            )
            .map_err(WorkspaceError::plugin_errors)?;
        let language =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let plugins = if rule_categories.contains(RuleCategory::Lint) {
            plugins
        } else {
            Vec::new()
        };

        let mut errors = 0;
        let mut actions = vec![];
        let mut skipped_suggested_fixes = 0;

        if let Some(update_snippets) = capabilities.analyzer.update_snippets {
            let mut new_snippets = vec![];
            for embedded_snippet in embedded_snippets {
                let Some(document_file_source) =
                    self.get_source(embedded_snippet.file_source_index())
                else {
                    continue;
                };
                let capabilities = self.features.get_capabilities(document_file_source);
                let Some(fix_all) = capabilities.analyzer.fix_all else {
                    continue;
                };

                let results = fix_all(FixAllParams {
                    parse: embedded_snippet.parse(),
                    fix_file_mode,
                    settings: &settings,
                    should_format,
                    biome_path: &path,
                    module_graph: self.module_graph.clone(),
                    project_layout: self.project_layout.clone(),
                    document_file_source,
                    only: &only,
                    skip: &skip,
                    rule_categories,
                    suppression_reason: suppression_reason.clone(),
                    enabled_rules: &enabled_rules,
                    plugins: plugins.clone(),
                })?;

                actions.extend(results.actions);
                errors += results.errors;
                skipped_suggested_fixes += results.skipped_suggested_fixes;

                new_snippets.push(UpdateSnippetsNodes {
                    range: embedded_snippet.element_range(),
                    new_code: results.code,
                });
            }

            let new_root = update_snippets(parse.clone(), new_snippets)?;
            parse.set_new_root(new_root);
        }

        let fix_result = fix_all(FixAllParams {
            parse,
            fix_file_mode,
            settings: &settings,
            should_format,
            biome_path: &path,
            module_graph: self.module_graph.clone(),
            project_layout: self.project_layout.clone(),
            document_file_source: language,
            only: &only,
            skip: &skip,
            rule_categories,
            suppression_reason: suppression_reason.clone(),
            enabled_rules: &enabled_rules,
            plugins: plugins.clone(),
        })?;

        actions.extend(fix_result.actions);
        errors += fix_result.errors;
        skipped_suggested_fixes += fix_result.skipped_suggested_fixes;

        Ok(FixFileResult {
            errors,
            code: fix_result.code,
            actions,
            skipped_suggested_fixes,
        })
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError> {
        let settings = self
            .projects
            .get_settings_based_on_path(params.project_key, &params.path)
            .ok_or_else(WorkspaceError::no_project)?;
        let capabilities = self.get_file_capabilities(
            &params.path,
            settings.experimental_full_html_support_enabled(),
        );
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
    /// indexed by the scanner. If the scanner has the file indexed, it may
    /// still be required for multi-file analysis.
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        let path = params.path.as_path();

        self.documents.pin().remove(path);
        self.node_cache.lock().unwrap().remove(path);

        if self.is_indexed(path) {
            // This may look counter-intuitive, but we need to consider that the
            // file may have gone out-of-sync between the client and the
            // filesystem. So when the client closes it, and the scanner still
            // wants to index it, we need to re-index it to make sure they're
            // back in sync.
            self.scanner.reindex_file(path.to_path_buf());
        }

        Ok(())
    }

    fn update_module_graph(&self, params: UpdateModuleGraphParams) -> Result<(), WorkspaceError> {
        let parsed = self.get_parse(params.path.as_path())?;
        let update_kind = match params.update_kind {
            super::UpdateKind::AddOrUpdate => UpdateKind::AddedOrChanged(
                OpenFileReason::ClientRequest,
                parsed.unwrap_into_send_node(),
            ),
            super::UpdateKind::Remove => UpdateKind::Removed,
        };

        self.update_module_graph_internal(&params.path, &update_kind);
        Ok(())
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
        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let patterns = self.patterns.pin();
        let query = patterns
            .get(&pattern)
            .ok_or_else(WorkspaceError::invalid_pattern)?;

        let capabilities =
            self.get_file_capabilities(&path, settings.experimental_full_html_support_enabled());
        let search = capabilities
            .search
            .search
            .ok_or_else(self.build_capability_error(&path))?;
        let settings = self
            .projects
            .get_settings_based_on_path(project_key, &path)
            .ok_or_else(WorkspaceError::no_project)?;
        let parse = self.get_parse(&path)?;

        let document_file_source =
            self.get_file_source(&path, settings.experimental_full_html_support_enabled());
        let matches = search(&path, &document_file_source, parse, query, &settings)?;

        Ok(SearchResults { path, matches })
    }

    fn drop_pattern(&self, params: DropPatternParams) -> Result<(), WorkspaceError> {
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

    fn get_module_graph(
        &self,
        _params: GetModuleGraphParams,
    ) -> Result<GetModuleGraphResult, WorkspaceError> {
        let module_graph = self.module_graph.data();
        let mut data = FxHashMap::default();

        for (path, info) in module_graph.iter() {
            data.insert(path.as_str().to_string(), info.dump());
        }

        Ok(GetModuleGraphResult { data })
    }
}

impl WorkspaceScannerBridge for WorkspaceServer {
    #[inline]
    fn fs(&self) -> &dyn biome_fs::FileSystem {
        self.fs.as_ref()
    }

    #[inline]
    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.projects.find_project_for_path(path)
    }

    #[inline]
    fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.projects.get_project_path(project_key)
    }

    #[inline]
    fn is_ignored(
        &self,
        project_key: ProjectKey,
        scan_kind: &ScanKind,
        path: &Utf8Path,
        request_kind: IndexRequestKind,
    ) -> Result<bool, WorkspaceError> {
        self.is_ignored_by_scanner(project_key, scan_kind, path, request_kind)
    }

    #[inline]
    fn is_indexed(&self, path: &Utf8Path) -> bool {
        match path.file_name() {
            Some("package.json" | "tsconfig.json") => self.project_layout.is_indexed(path),
            _ => self.module_graph.contains(path),
        }
    }

    fn index_file(
        &self,
        project_key: ProjectKey,
        path: impl Into<BiomePath>,
        trigger: IndexTrigger,
    ) -> Result<(ModuleDependencies, Vec<ModuleDiagnostic>), WorkspaceError> {
        self.open_file_internal(
            OpenFileReason::Index(trigger),
            OpenFileParams {
                project_key,
                path: path.into(),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .map(|result| (result.dependencies, result.diagnostics))
    }

    fn update_project_config_files(
        &self,
        project_key: ProjectKey,
        paths: &[BiomePath],
    ) -> Result<Vec<SerdeDiagnostic>, WorkspaceError> {
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
                self.fs.as_ref(),
                ConfigurationPathHint::FromWorkspace(filtered_path.as_path().to_path_buf()),
                false,
            )?;
            let loaded_nested_configuration =
                LoadedConfiguration::try_from_payload(config, self.fs.as_ref())?;

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

    fn update_project_ignore_files(
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

        if !vcs_settings.is_enabled() || !vcs_settings.should_use_ignore_file() {
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

    fn notify(&self, notification: ServiceNotification) {
        let _ = self.notification_tx.send(notification);
    }

    fn unload_file(
        &self,
        path: &Utf8Path,
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        self.update_service_data(path, UpdateKind::Removed)
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
    ) -> Result<Vec<biome_diagnostics::serde::Diagnostic>, WorkspaceError> {
        // Note that we cannot check the kind of the path, because the watcher
        // would only attempt to unload a file or folder after it has been
        // removed. So asking the filesystem wouldn't work anymore. So we just
        // pretend it's a folder and if it turns out the path didn't belong to a
        // folder, the scanner will ignore it.
        self.scanner.unload_folder(path.to_path_buf());

        // Unloads all descendants of the path.
        self.module_graph.unload_path(path);
        self.project_layout.unload_folder(path);

        // Finally unloads the path itself.
        self.unload_file(path)
    }
}

#[derive(Debug, Default)]
pub(super) struct InternalOpenFileResult {
    /// Dependencies we discovered of the opened file.
    pub dependencies: ModuleDependencies,

    ///
    pub diagnostics: Vec<ModuleDiagnostic>,
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
    AddedOrChanged(OpenFileReason, SendNode),
    Removed,
}

impl Debug for UpdateKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

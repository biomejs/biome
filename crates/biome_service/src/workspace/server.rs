use super::{
    ChangeFileParams, CloseFileParams, FeatureKind, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, ParsePatternParams,
    ParsePatternResult, PatternId, ProjectKey, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RegisterProjectFolderParams, RenameResult,
    SearchPatternParams, SearchResults, SetManifestForProjectParams, SupportsFeatureParams,
    UnregisterProjectFolderParams, UpdateSettingsParams,
};
use crate::diagnostics::{InvalidPattern, SearchError};
use crate::file_handlers::{
    Capabilities, CodeActionsParams, DocumentFileSource, FixAllParams, LintParams, ParseResult,
};
use crate::settings::{WorkspaceSettings, WorkspaceSettingsHandleMut};
use crate::workspace::{
    FileFeaturesResult, GetFileContentParams, IsPathIgnoredParams, OrganizeImportsParams,
    OrganizeImportsResult, RageEntry, RageParams, RageResult, ServerInfo,
};
use crate::{
    file_handlers::Features, settings::WorkspaceSettingsHandle, Workspace, WorkspaceError,
};
use biome_configuration::DEFAULT_FILE_SIZE_LIMIT;
use biome_diagnostics::{
    serde::Diagnostic as SerdeDiagnostic, Diagnostic, DiagnosticExt, Severity,
};
use biome_formatter::Printed;
use biome_fs::{BiomePath, ConfigName};
use biome_grit_patterns::GritQuery;
use biome_js_syntax::ModuleKind;
use biome_json_parser::{parse_json_with_cache, JsonParserOptions};
use biome_json_syntax::JsonFileSource;
use biome_parser::AnyParse;
use biome_project::{NodeJsProject, PackageJson, PackageType, Project};
use biome_rowan::NodeCache;
use dashmap::{mapref::entry::Entry, DashMap};
use indexmap::IndexSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{panic::RefUnwindSafe, sync::RwLock};
use tracing::{debug, info, info_span};

pub(super) struct WorkspaceServer {
    /// features available throughout the application
    features: Features,
    /// global settings object for this workspace
    settings: RwLock<WorkspaceSettings>,
    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<BiomePath, Document>,
    /// Stores the result of the parser (syntax tree + diagnostics) for a given URL
    syntax: DashMap<BiomePath, AnyParse>,
    /// The current focused project
    current_project_path: RwLock<Option<BiomePath>>,
    /// Stores the document sources used across the workspace
    file_sources: RwLock<IndexSet<DocumentFileSource>>,
    /// Stores patterns to search for.
    patterns: DashMap<PatternId, GritQuery>,
}

/// The `Workspace` object is long-lived, so we want it to be able to cross
/// unwind boundaries.
/// In return, we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead too hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

#[derive(Debug)]
pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,
    /// The index of where the original file source is saved
    /// Use `WorkspaceServer#file_sources` to retrieve the file source that belongs to the document.
    pub(crate) file_source_index: usize,
    node_cache: NodeCache,
}

impl WorkspaceServer {
    /// Create a new [Workspace]
    ///
    /// This is implemented as a crate-private method instead of using
    /// [Default] to disallow instances of [Workspace] from being created
    /// outside a [crate::App]
    pub(crate) fn new() -> Self {
        Self {
            features: Features::new(),
            settings: RwLock::default(),
            documents: DashMap::default(),
            syntax: DashMap::default(),
            current_project_path: RwLock::default(),
            file_sources: RwLock::default(),
            patterns: Default::default(),
        }
    }

    /// Provides a reference to the current settings
    fn workspace(&self) -> WorkspaceSettingsHandle {
        WorkspaceSettingsHandle::new(&self.settings)
    }

    fn workspaces_mut(&self) -> WorkspaceSettingsHandleMut {
        WorkspaceSettingsHandleMut::new(&self.settings)
    }

    /// Get the supported capabilities for a given file path
    fn get_file_capabilities(&self, path: &BiomePath) -> Capabilities {
        let language = self.get_file_source(path);

        debug!("File capabilities: {:?} {:?}", &language, &path);
        self.features.get_capabilities(path, language)
    }

    /// Retrieves the supported language of a file
    fn get_file_source(&self, path: &BiomePath) -> DocumentFileSource {
        self.documents
            .get(path)
            .map(|doc| doc.file_source_index)
            .and_then(|index| self.get_source(index))
            .unwrap_or(DocumentFileSource::from_path(path))
    }

    /// Return an error factory function for unsupported features at a given path
    fn build_capability_error<'a>(
        &'a self,
        path: &'a BiomePath,
        // feature_name: &'a str,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            let file_source = self.get_file_source(path);

            let language = DocumentFileSource::from_path(path).or(file_source);
            WorkspaceError::source_file_not_supported(
                language,
                path.display().to_string(),
                path.extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_string()),
            )
        }
    }

    /// Returns the current project. The information of this project depend on path set by [WorkspaceServer::set_current_project]
    ///
    /// ## Errors
    ///
    /// - If no document is found in the workspace. Usually, you'll have to call [WorkspaceServer::set_manifest_for_project] to store said document.
    #[tracing::instrument(level = "trace", skip(self))]
    fn get_current_manifest(&self) -> Result<Option<PackageJson>, WorkspaceError> {
        let workspace = self.workspace();
        Ok(workspace.as_ref().get_current_manifest().cloned())
    }

    #[tracing::instrument(level = "trace", skip(self), fields(return))]
    fn get_source(&self, index: usize) -> Option<DocumentFileSource> {
        let file_sources = self.file_sources.read().unwrap();
        file_sources.get_index(index).copied()
    }

    #[tracing::instrument(level = "trace", skip(self), fields(return))]
    fn set_source(&self, document_file_source: DocumentFileSource) -> usize {
        let mut file_sources = self.file_sources.write().unwrap();
        let (index, _) = file_sources.insert_full(document_file_source);
        index
    }

    /// Retrieves the current project path
    fn get_current_project_path(&self) -> Option<BiomePath> {
        self.current_project_path.read().unwrap().as_ref().cloned()
    }

    /// Updates the current project path
    fn set_current_project_path(&self, path: BiomePath) {
        let mut current_project_path = self.current_project_path.write().unwrap();
        let _ = current_project_path.insert(path);
    }

    /// Register a new project in the current workspace
    fn register_project(&self, path: PathBuf) -> ProjectKey {
        let mut workspace = self.workspaces_mut();
        let workspace_mut = workspace.as_mut();
        workspace_mut.insert_project(path.clone())
    }

    /// Updates the manifest for the current project. Given the manifest path, the function will try to parse the manifest and update the current project.
    fn register_manifest_for_project(&self, manifest_path: BiomePath) {
        let mut workspace = self.workspaces_mut();
        let workspace_mut = workspace.as_mut();
        let mut document = self.documents.get_mut(&manifest_path);
        if let Some(document) = document.as_deref_mut() {
            let parsed = parse_json_with_cache(
                document.content.as_str(),
                &mut document.node_cache,
                JsonParserOptions::default(),
            );

            let mut node_js_project = NodeJsProject::default();
            node_js_project.deserialize_manifest(&parsed.tree());
            workspace_mut.insert_manifest(node_js_project);
        }
    }

    /// Sets the current project of the current workspace
    fn set_current_project(&self, project_key: ProjectKey) {
        let mut workspace = self.workspaces_mut();
        let workspace_mut = workspace.as_mut();
        workspace_mut.set_current_project(project_key);
    }

    /// Checks whether, if the current path belongs to the current project.
    ///
    /// If there's a match, and the match **isn't** the current project, it returns the new key.
    fn path_belongs_to_current_workspace(&self, path: &BiomePath) -> Option<ProjectKey> {
        let workspace = self.workspace();
        workspace.as_ref().path_belongs_to_current_workspace(path)
    }

    /// Get the parser result for a given file
    ///
    /// Returns and error if no file exists in the workspace with this path or
    /// if the language associated with the file has no parser capability
    fn get_parse(&self, biome_path: BiomePath) -> Result<AnyParse, WorkspaceError> {
        match self.syntax.entry(biome_path) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let biome_path = entry.key();
                let capabilities = self.get_file_capabilities(biome_path);

                let mut document = self
                    .documents
                    .get_mut(biome_path)
                    .ok_or_else(WorkspaceError::not_found)?;

                let parse = capabilities
                    .parser
                    .parse
                    .ok_or_else(self.build_capability_error(biome_path))?;

                let size_limit = {
                    let workspace = self.workspace();
                    let settings = workspace.settings();
                    let limit =
                        settings.map_or(DEFAULT_FILE_SIZE_LIMIT.get(), |s| s.files.max_size.get());
                    usize::try_from(limit).unwrap_or(usize::MAX)
                };

                let document = &mut *document;
                let size = document.content.as_bytes().len();
                if size >= size_limit {
                    return Err(WorkspaceError::file_too_large(
                        biome_path.to_path_buf().display().to_string(),
                        size,
                        size_limit,
                    ));
                }

                let workspace = self.workspace();
                let Some(file_source) = self.get_source(document.file_source_index) else {
                    return Err(WorkspaceError::not_found());
                };
                let settings = workspace.settings();
                let parsed = parse(
                    biome_path,
                    file_source,
                    document.content.as_str(),
                    settings,
                    &mut document.node_cache,
                );
                let ParseResult {
                    language,
                    any_parse,
                } = parsed;
                if let Some(language) = language {
                    document.file_source_index = self.set_source(language);
                }
                Ok(entry.insert(any_parse).clone())
            }
        }
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    /// or in the feature `ignore`/`include`
    fn is_ignored(&self, path: &Path, features: FeatureName) -> bool {
        let file_name = path.file_name().and_then(|s| s.to_str());
        let ignored_by_features = {
            let mut ignored = false;

            for feature in features.iter() {
                // a path is ignored if it's ignored by all features
                ignored &= self.is_ignored_by_feature_config(path, feature)
            }
            ignored
        };
        // Never ignore Biome's config file regardless `include`/`ignore`
        (file_name != Some(ConfigName::biome_json()) || file_name != Some(ConfigName::biome_jsonc())) &&
            // Apply top-level `include`/`ignore`
            (self.is_ignored_by_top_level_config(path) ||
                // Apply feature-level `include`/`ignore`
                ignored_by_features)
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored_by_top_level_config(&self, path: &Path) -> bool {
        let settings = self.workspace();
        let settings = settings.settings();
        let Some(settings) = settings else {
            return false;
        };
        let is_included = settings.files.included_files.is_empty()
            || is_dir(path)
            || settings.files.included_files.matches_path(path);
        !is_included
            || settings.files.ignored_files.matches_path(path)
            || settings.files.git_ignore.as_ref().is_some_and(|ignore| {
                // `matched_path_or_any_parents` panics if `source` is not under the gitignore root.
                // This checks excludes absolute paths that are not a prefix of the base root.
                if !path.has_root() || path.starts_with(ignore.path()) {
                    // Because Biome passes a list of paths,
                    // we use `matched_path_or_any_parents` instead of `matched`.
                    ignore
                        .matched_path_or_any_parents(path, path.is_dir())
                        .is_ignore()
                } else {
                    false
                }
            })
    }

    /// Check whether a file is ignored in the feature `ignore`/`include`
    fn is_ignored_by_feature_config(&self, path: &Path, feature: FeatureKind) -> bool {
        let settings = self.workspace();
        let settings = settings.settings();
        let Some(settings) = settings else {
            return false;
        };
        let (feature_included_files, feature_ignored_files) = match feature {
            FeatureKind::Format => {
                let formatter = &settings.formatter;
                (&formatter.included_files, &formatter.ignored_files)
            }
            FeatureKind::Lint => {
                let linter = &settings.linter;
                (&linter.included_files, &linter.ignored_files)
            }
            FeatureKind::OrganizeImports => {
                let organize_imports = &settings.organize_imports;
                (
                    &organize_imports.included_files,
                    &organize_imports.ignored_files,
                )
            }
            FeatureKind::Assists => {
                let assists = &settings.assists;
                (&assists.included_files, &assists.ignored_files)
            }
            // TODO: enable once the configuration is available
            FeatureKind::Search => return false, // There is no search-specific config.
            FeatureKind::Debug => return false,
        };
        let is_feature_included = feature_included_files.is_empty()
            || is_dir(path)
            || feature_included_files.matches_path(path);
        !is_feature_included || feature_ignored_files.matches_path(path)
    }
}

impl Workspace for WorkspaceServer {
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let language = DocumentFileSource::from_path(&params.path);
        let path = params.path.as_path();
        let settings = self.workspace();
        let settings = settings.settings();
        let mut file_features = FileFeaturesResult::new();

        let file_name = path.file_name().and_then(|s| s.to_str());
        file_features = file_features.with_capabilities(&capabilities);
        let Some(settings) = settings else {
            return Ok(file_features);
        };
        file_features = file_features.with_settings_and_language(settings, &language, path);

        if settings.files.ignore_unknown
            && language == DocumentFileSource::Unknown
            && self.get_file_source(&params.path) == DocumentFileSource::Unknown
        {
            file_features.ignore_not_supported();
        } else if file_name == Some(ConfigName::biome_json())
            || file_name == Some(ConfigName::biome_jsonc())
        {
            // Never ignore Biome's config file
        } else if self.is_ignored_by_top_level_config(path) {
            file_features.set_ignored_for_all_features();
        } else {
            for feature in params.features.iter() {
                if self.is_ignored_by_feature_config(path, feature) {
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
        Ok(self.is_ignored(params.biome_path.as_path(), params.features))
    }
    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "trace", skip(self))]
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        let mut workspace = self.workspaces_mut();
        workspace
            .as_mut()
            .get_current_settings_mut()
            .merge_with_configuration(
                params.configuration,
                params.workspace_directory,
                params.vcs_base_path,
                params.gitignore_matches.as_slice(),
            )?;

        Ok(())
    }
    /// Add a new file to the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        let mut source = params
            .document_file_source
            .unwrap_or(DocumentFileSource::from_path(&params.path));
        let manifest = self.get_current_manifest()?;

        if let DocumentFileSource::Js(js) = &mut source {
            if let Some(manifest) = manifest {
                if manifest.r#type == Some(PackageType::Commonjs) && js.file_extension() == "js" {
                    js.set_module_kind(ModuleKind::Script);
                }
            }
        }

        let index = self.set_source(source);
        self.syntax.remove(&params.path);
        self.documents.insert(
            params.path.clone(),
            Document {
                content: params.content,
                version: params.version,
                node_cache: NodeCache::default(),
                file_source_index: index,
            },
        );
        if let Some(project_key) = self.path_belongs_to_current_workspace(&params.path) {
            self.set_current_project(project_key);
        }

        Ok(())
    }
    fn set_manifest_for_project(
        &self,
        params: SetManifestForProjectParams,
    ) -> Result<(), WorkspaceError> {
        let index = self.set_source(JsonFileSource::json().into());
        self.syntax.remove(&params.manifest_path);
        self.documents.insert(
            params.manifest_path.clone(),
            Document {
                content: params.content,
                version: params.version,
                file_source_index: index,
                node_cache: NodeCache::default(),
            },
        );
        self.register_manifest_for_project(params.manifest_path);
        Ok(())
    }

    fn register_project_folder(
        &self,
        params: RegisterProjectFolderParams,
    ) -> Result<ProjectKey, WorkspaceError> {
        let current_project_path = self.get_current_project_path();
        debug!(
            "Compare the current project with the new one {:?} {:?} {:?}",
            current_project_path.as_deref(),
            params.path.as_ref(),
            current_project_path.as_deref() != params.path.as_ref()
        );

        let is_new_path = match (current_project_path.as_deref(), params.path.as_ref()) {
            (Some(current_project_path), Some(params_path)) => current_project_path != params_path,
            _ => true,
        };

        if is_new_path {
            let path = params.path.unwrap_or_default();
            let key = self.register_project(path.clone());
            if params.set_as_current_workspace {
                self.set_current_project(key);
                self.set_current_project_path(BiomePath::new(path));
            }
            Ok(key)
        } else {
            Ok(self.workspace().as_ref().get_current_project_key())
        }
    }

    fn unregister_project_folder(
        &self,
        params: UnregisterProjectFolderParams,
    ) -> Result<(), WorkspaceError> {
        let mut workspace = self.workspaces_mut();
        workspace.as_mut().remove_project(params.path.as_path());
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
        let parse = self.get_parse(params.path.clone())?;
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

        let parse = self.get_parse(params.path.clone())?;
        let printed = debug_control_flow(parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&params.path))?;
        let workspace = self.workspace();
        let settings = workspace.settings();
        let parse = self.get_parse(params.path.clone())?;

        if let Some(settings) = settings {
            if !settings.formatter().format_with_errors && parse.has_errors() {
                return Err(WorkspaceError::format_with_errors_disabled());
            }
        }
        let document_file_source = self.get_file_source(&params.path);

        debug_formatter_ir(&params.path, &document_file_source, parse, workspace)
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        let document = self
            .documents
            .get(&params.path)
            .ok_or(WorkspaceError::not_found())?;
        Ok(document.content.clone())
    }

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError> {
        {
            let mut document = self
                .documents
                .get_mut(&params.path)
                .ok_or_else(WorkspaceError::not_found)?;

            debug_assert!(params.version > document.version);
            document.version = params.version;
            document.content = params.content;
        }

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.documents
            .remove(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Retrieves the list of diagnostics associated with a file
    #[tracing::instrument(level = "trace", skip(self))]
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let parse = self.get_parse(params.path.clone())?;
        let manifest = self.get_current_manifest()?;
        let (diagnostics, errors, skipped_diagnostics) =
            if let Some(lint) = self.get_file_capabilities(&params.path).analyzer.lint {
                info_span!("Pulling diagnostics", categories =? params.categories).in_scope(|| {
                    let results = lint(LintParams {
                        parse,
                        workspace: &self.workspace(),
                        max_diagnostics: params.max_diagnostics as u32,
                        path: &params.path,
                        only: params.only,
                        skip: params.skip,
                        language: self.get_file_source(&params.path),
                        categories: params.categories,
                        manifest,
                    });

                    (
                        results.diagnostics,
                        results.errors,
                        results.skipped_diagnostics,
                    )
                })
            } else {
                let parse_diagnostics = parse.into_diagnostics();
                let errors = parse_diagnostics
                    .iter()
                    .filter(|diag| diag.severity() <= Severity::Error)
                    .count();

                (parse_diagnostics, errors, 0)
            };

        info!("Pulled {:?} diagnostic(s)", diagnostics.len());
        Ok(PullDiagnosticsResult {
            diagnostics: diagnostics
                .into_iter()
                .map(|diag| {
                    let diag = diag.with_file_path(params.path.as_path().display().to_string());
                    SerdeDiagnostic::new(diag)
                })
                .collect(),
            errors,
            skipped_diagnostics: skipped_diagnostics.into(),
        })
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    #[tracing::instrument(level = "trace", skip(self))]
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone())?;
        let workspace = self.workspace();
        let manifest = self.get_current_manifest()?;
        let language = self.get_file_source(&params.path);
        Ok(code_actions(CodeActionsParams {
            parse,
            range: params.range,
            workspace: &workspace,
            path: &params.path,
            manifest,
            language,
            only: params.only,
            skip: params.skip,
        }))
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&params.path))?;
        let workspace = self.workspace();
        let settings = workspace.settings();
        let parse = self.get_parse(params.path.clone())?;

        if let Some(settings) = settings {
            if !settings.formatter().format_with_errors && parse.has_errors() {
                return Err(WorkspaceError::format_with_errors_disabled());
            }
        }
        let document_file_source = self.get_file_source(&params.path);
        format(&params.path, &document_file_source, parse, workspace)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let workspace = self.workspace();
        let settings = workspace.settings();
        let parse = self.get_parse(params.path.clone())?;

        if let Some(settings) = settings {
            if !settings.formatter().format_with_errors && parse.has_errors() {
                return Err(WorkspaceError::format_with_errors_disabled());
            }
        }
        let document_file_source = self.get_file_source(&params.path);
        format_range(
            &params.path,
            &document_file_source,
            parse,
            workspace,
            params.range,
        )
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let workspace = self.workspace();
        let settings = workspace.settings();
        let parse = self.get_parse(params.path.clone())?;
        if let Some(settings) = settings {
            if !settings.formatter().format_with_errors && parse.has_errors() {
                return Err(WorkspaceError::format_with_errors_disabled());
            }
        }
        let document_file_source = self.get_file_source(&params.path);

        format_on_type(
            &params.path,
            &document_file_source,
            parse,
            workspace,
            params.offset,
        )
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);

        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&params.path))?;
        let parse = self.get_parse(params.path.clone())?;

        let manifest = self.get_current_manifest()?;
        let language = self.get_file_source(&params.path);
        fix_all(FixAllParams {
            parse,
            // rules: rules.as_ref().map(|x| x.borrow()),
            fix_file_mode: params.fix_file_mode,
            // filter,
            workspace: self.workspace(),
            should_format: params.should_format,
            biome_path: &params.path,
            manifest,
            document_file_source: language,
            only: params.only,
            skip: params.skip,
            rule_categories: params.rule_categories,
        })
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone())?;
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
        let pattern = biome_grit_patterns::compile_pattern(
            &params.pattern,
            None,
            biome_grit_patterns::JsTargetLanguage.into(),
        )?;

        let pattern_id = make_search_pattern_id();
        self.patterns.insert(pattern_id.clone(), pattern);
        Ok(ParsePatternResult { pattern_id })
    }

    fn search_pattern(&self, params: SearchPatternParams) -> Result<SearchResults, WorkspaceError> {
        let Some(query) = self.patterns.get(&params.pattern) else {
            return Err(WorkspaceError::SearchError(SearchError::InvalidPattern(
                InvalidPattern,
            )));
        };

        let capabilities = self.get_file_capabilities(&params.path);
        let search = capabilities
            .search
            .search
            .ok_or_else(self.build_capability_error(&params.path))?;
        let workspace = self.workspace();
        let parse = self.get_parse(params.path.clone())?;

        let document_file_source = self.get_file_source(&params.path);
        let matches = search(
            &params.path,
            &document_file_source,
            parse,
            &query,
            workspace,
        )?;

        Ok(SearchResults {
            file: params.path,
            matches,
        })
    }

    fn drop_pattern(&self, params: super::DropPatternParams) -> Result<(), WorkspaceError> {
        self.patterns.remove(&params.pattern);
        Ok(())
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }

    fn organize_imports(
        &self,
        params: OrganizeImportsParams,
    ) -> Result<OrganizeImportsResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let organize_imports = capabilities
            .analyzer
            .organize_imports
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path)?;
        let result = organize_imports(parse)?;

        Ok(result)
    }
}

/// Returns `true` if `path` is a directory or
/// if it is a symlink that resolves to a directory.
fn is_dir(path: &Path) -> bool {
    path.is_dir() || (path.is_symlink() && fs::read_link(path).is_ok_and(|path| path.is_dir()))
}

/// Generates a pattern ID that we can use as "handle" for referencing
/// previously parsed search queries.
fn make_search_pattern_id() -> PatternId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    let counter = COUNTER.fetch_add(1, Ordering::AcqRel);
    format!("p{counter}").into()
}

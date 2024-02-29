use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, OpenProjectParams, PullActionsParams,
    PullActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RenameResult,
    SupportsFeatureParams, UpdateProjectParams, UpdateSettingsParams,
};
use crate::file_handlers::{
    Capabilities, CodeActionsParams, FixAllParams, Language, LintParams, ParseResult,
};
use crate::workspace::{
    FileFeaturesResult, GetFileContentParams, IsPathIgnoredParams, OrganizeImportsParams,
    OrganizeImportsResult, RageEntry, RageParams, RageResult, ServerInfo,
};
use crate::{
    file_handlers::Features,
    settings::{SettingsHandle, WorkspaceSettings},
    Workspace, WorkspaceError,
};
use biome_analyze::AnalysisFilter;
use biome_diagnostics::{
    serde::Diagnostic as SerdeDiagnostic, Diagnostic, DiagnosticExt, Severity,
};
use biome_formatter::Printed;
use biome_fs::{BiomePath, ConfigName};
use biome_json_parser::{parse_json_with_cache, JsonParserOptions};
use biome_parser::AnyParse;
use biome_project::NodeJsProject;
use biome_rowan::NodeCache;
use dashmap::{mapref::entry::Entry, DashMap};
use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::Path;
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
    /// Stores the features supported for each file
    file_features: DashMap<BiomePath, FileFeaturesResult>,
    /// Stores the parsed manifests
    manifests: DashMap<BiomePath, NodeJsProject>,
    /// The current focused project
    current_project_path: RwLock<Option<BiomePath>>,
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
    pub(crate) language_hint: Language,
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
            file_features: DashMap::default(),
            manifests: DashMap::default(),
            current_project_path: RwLock::default(),
        }
    }

    fn settings(&self) -> SettingsHandle {
        SettingsHandle::new(&self.settings)
    }

    /// Get the supported capabilities for a given file path
    fn get_file_capabilities(&self, path: &BiomePath) -> Capabilities {
        let language = self.get_language(path);

        debug!("File capabilities: {:?} {:?}", &language, &path);
        self.features.get_capabilities(path, language)
    }

    /// Retrieves the supported language of a file
    fn get_language(&self, path: &BiomePath) -> Language {
        self.documents
            .get(path)
            .map(|doc| doc.language_hint)
            .unwrap_or_default()
    }

    /// Return an error factory function for unsupported features at a given path
    fn build_capability_error<'a>(
        &'a self,
        path: &'a BiomePath,
        // feature_name: &'a str,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            let language_hint = self
                .documents
                .get(path)
                .map(|doc| doc.language_hint)
                .unwrap_or_default();

            let language = Language::from_path_and_known_filename(path).or(language_hint);
            WorkspaceError::source_file_not_supported(
                language,
                path.clone().display().to_string(),
                path.clone()
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_string()),
            )
        }
    }

    /// Returns the current project. The information of this project depend on path set by [WorkspaceServer::update_current_project]
    ///
    /// ## Errors
    ///
    /// - If no document is found in the workspace. Usually, you'll have to call [WorkspaceServer::open_project] to store said document.
    fn get_current_project(&self) -> Result<Option<NodeJsProject>, WorkspaceError> {
        let path = self.current_project_path.read().unwrap();
        if let Some(path) = path.as_ref() {
            match self.manifests.entry(path.clone()) {
                Entry::Occupied(entry) => Ok(Some(entry.get().clone())),
                Entry::Vacant(entry) => {
                    let path = entry.key();
                    let mut document = self
                        .documents
                        .get_mut(path)
                        .ok_or_else(WorkspaceError::not_found)?;
                    let document = &mut *document;
                    let parsed = parse_json_with_cache(
                        document.content.as_str(),
                        &mut document.node_cache,
                        JsonParserOptions::default(),
                    );

                    let mut node_js_project = NodeJsProject::default();
                    node_js_project.from_root(&parsed.tree());

                    Ok(Some(entry.insert(node_js_project).clone()))
                }
            }
        } else {
            Ok(None)
        }
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
                    let settings = self.settings();
                    let settings = settings.as_ref();
                    let limit = settings.files.max_size.get();
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

                let settings = self.settings();
                let parsed = parse(
                    biome_path,
                    document.language_hint,
                    document.content.as_str(),
                    settings,
                    &mut document.node_cache,
                );
                let ParseResult {
                    language,
                    any_parse,
                } = parsed;
                if let Some(language) = language {
                    document.language_hint = language
                }
                Ok(entry.insert(any_parse).clone())
            }
        }
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    /// or in the feature `ignore`/`include`
    fn is_ignored(&self, path: &Path, feature: FeatureName) -> bool {
        let file_name = path.file_name().and_then(|s| s.to_str());
        // Never ignore Biome's config file regardless `include`/`ignore`
        (file_name != Some(ConfigName::biome_json()) || file_name != Some(ConfigName::biome_jsonc())) &&
            // Apply top-level `include`/`ignore`
            (self.is_ignored_by_top_level_config(path) ||
                // Apply feature-level `include`/`ignore`
                self.is_ignored_by_feature_config(path, feature))
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored_by_top_level_config(&self, path: &Path) -> bool {
        let settings = self.settings();
        let is_included = settings.as_ref().files.included_files.is_empty()
            || settings.as_ref().files.included_files.matches_path(path);
        !is_included
            || settings.as_ref().files.ignored_files.matches_path(path)
            || settings
                .as_ref()
                .files
                .git_ignore
                .as_ref()
                .map(|ignore| {
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
                .unwrap_or_default()
    }

    /// Check whether a file is ignored in the feature `ignore`/`include`
    fn is_ignored_by_feature_config(&self, path: &Path, feature: FeatureName) -> bool {
        let settings = self.settings();
        let (feature_included_files, feature_ignored_files) = match feature {
            FeatureName::Format => {
                let formatter = &settings.as_ref().formatter;
                (&formatter.included_files, &formatter.ignored_files)
            }
            FeatureName::Lint => {
                let linter = &settings.as_ref().linter;
                (&linter.included_files, &linter.ignored_files)
            }
            FeatureName::OrganizeImports => {
                let organize_imports = &settings.as_ref().organize_imports;
                (
                    &organize_imports.included_files,
                    &organize_imports.ignored_files,
                )
            }
        };
        let is_feature_included =
            feature_included_files.is_empty() || feature_included_files.matches_path(path);
        !is_feature_included || feature_ignored_files.matches_path(path)
    }
}

impl Workspace for WorkspaceServer {
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let file_features_result = self.file_features.entry(params.path.clone());
        match file_features_result {
            Entry::Occupied(entry) => {
                let result = entry.get();
                Ok(result.clone())
            }
            Entry::Vacant(entry) => {
                let capabilities = self.get_file_capabilities(&params.path);
                let language = Language::from_path_and_known_filename(&params.path);
                let path = params.path.as_path();
                let settings = self.settings.read().unwrap();
                let mut file_features = FileFeaturesResult::new();
                let file_name = path.file_name().and_then(|s| s.to_str());
                file_features = file_features
                    .with_capabilities(&capabilities)
                    .with_settings_and_language(&settings, &language, path);

                if settings.files.ignore_unknown
                    && language == Language::Unknown
                    && self.get_language(&params.path) == Language::Unknown
                {
                    file_features.ignore_not_supported();
                } else if file_name == Some(ConfigName::biome_json())
                    || file_name == Some(ConfigName::biome_jsonc())
                {
                    // Never ignore Biome's config file
                } else if self.is_ignored_by_top_level_config(path) {
                    file_features.set_ignored_for_all_features();
                } else {
                    for feature in params.feature {
                        if self.is_ignored_by_feature_config(path, feature) {
                            file_features.ignored(feature);
                        }
                    }
                }

                // If the file is not ignored by at least one feature,
                // then check that the file is not protected.
                // Protected files must be ignored.
                if !file_features.is_not_processed() && FileFeaturesResult::is_protected_file(path)
                {
                    file_features.set_protected_for_all_features();
                }

                Ok(entry.insert(file_features).clone())
            }
        }
    }
    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        Ok(self.is_ignored(params.biome_path.as_path(), params.feature))
    }
    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "trace", skip(self))]
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        let mut settings = self.settings.write().unwrap();

        settings.merge_with_configuration(
            params.configuration,
            params.working_directory,
            params.vcs_base_path,
            params.gitignore_matches.as_slice(),
        )?;

        // settings changed, hence everything that is computed from the settings needs to be purged
        self.file_features.clear();
        Ok(())
    }

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.syntax.remove(&params.path);
        self.documents.insert(
            params.path,
            Document {
                content: params.content,
                version: params.version,
                language_hint: params.language_hint,
                node_cache: NodeCache::default(),
            },
        );
        Ok(())
    }

    fn open_project(&self, params: OpenProjectParams) -> Result<(), WorkspaceError> {
        self.syntax.remove(&params.path);
        self.documents.insert(
            params.path,
            Document {
                content: params.content,
                version: params.version,
                language_hint: Language::Json,
                node_cache: NodeCache::default(),
            },
        );
        Ok(())
    }

    fn update_current_project(&self, params: UpdateProjectParams) -> Result<(), WorkspaceError> {
        let mut current_project_path = self.current_project_path.write().unwrap();
        let _ = current_project_path.insert(params.path);
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
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone())?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        debug_formatter_ir(&params.path, parse, settings)
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
        let mut document = self
            .documents
            .get_mut(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        debug_assert!(params.version > document.version);
        document.version = params.version;
        document.content = params.content;

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
        let manifest = self.get_current_project()?.map(|pr| pr.manifest);
        let (diagnostics, errors, skipped_diagnostics) =
            if let Some(lint) = self.get_file_capabilities(&params.path).analyzer.lint {
                info_span!("Pulling diagnostics", categories =? params.categories).in_scope(|| {
                    let results = lint(LintParams {
                        parse,
                        settings: self.settings(),
                        max_diagnostics: params.max_diagnostics as u32,
                        path: &params.path,
                        language: self.get_language(&params.path),
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
        let settings = self.settings.read().unwrap();
        let rules = settings.linter().rules.as_ref();
        let manifest = self.get_current_project()?.map(|pr| pr.manifest);
        let language = self.get_language(&params.path);
        Ok(code_actions(CodeActionsParams {
            parse,
            range: params.range,
            rules,
            settings: self.settings(),
            path: &params.path,
            manifest,
            language,
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
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone())?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format(&params.path, parse, settings)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone())?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format_range(&params.path, parse, settings, params.range)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let settings = self.settings();
        let parse = self.get_parse(params.path.clone())?;
        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format_on_type(&params.path, parse, settings, params.offset)
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);

        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings.read().unwrap();
        let parse = self.get_parse(params.path.clone())?;
        // Compute final rules (taking `overrides` into account)
        let rules = settings.as_rules(params.path.as_path());
        let rule_filter_list = rules
            .as_ref()
            .map(|rules| rules.as_enabled_rules())
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();
        let filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
        let manifest = self.get_current_project()?.map(|pr| pr.manifest);
        let language = self.get_language(&params.path);
        fix_all(FixAllParams {
            parse,
            rules: rules.as_ref().map(|x| x.borrow()),
            fix_file_mode: params.fix_file_mode,
            filter,
            settings: self.settings(),
            should_format: params.should_format,
            biome_path: &params.path,
            manifest,
            language,
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

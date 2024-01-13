use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameResult, SupportsFeatureParams,
    UpdateSettingsParams,
};
use crate::file_handlers::{Capabilities, FixAllParams, Language, LintParams};
use crate::project_handlers::{ProjectCapabilities, ProjectHandlers};
use crate::settings::OverrideSettings;
use crate::workspace::{
    FileFeaturesResult, GetFileContentParams, IsPathIgnoredParams, OrganizeImportsParams,
    OrganizeImportsResult, RageEntry, RageParams, RageResult, ServerInfo,
};
use crate::{
    file_handlers::Features,
    settings::{SettingsHandle, WorkspaceSettings},
    Rules, Workspace, WorkspaceError,
};
use biome_analyze::{AnalysisFilter, RuleFilter};
use biome_diagnostics::{
    serde::Diagnostic as SerdeDiagnostic, Diagnostic, DiagnosticExt, Severity,
};
use biome_formatter::Printed;
use biome_fs::{RomePath, BIOME_JSON};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use dashmap::{mapref::entry::Entry, DashMap};
use std::ffi::OsStr;
use std::path::Path;
use std::{panic::RefUnwindSafe, sync::RwLock};
use tracing::{debug, info, info_span, trace};

pub(super) struct WorkspaceServer {
    /// features available throughout the application
    features: Features,
    /// global settings object for this workspace
    settings: RwLock<WorkspaceSettings>,
    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<RomePath, Document>,
    /// Stores the result of the parser (syntax tree + diagnostics) for a given URL
    syntax: DashMap<RomePath, AnyParse>,
    /// Stores the features supported for each file
    file_features: DashMap<RomePath, FileFeaturesResult>,
    /// Handlers that know how to handle a specific project
    project_handlers: ProjectHandlers,
}

/// The `Workspace` object is long lived, so we want it to be able to cross
/// unwind boundaries.
/// In return we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead to hard to debug issues)
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
    /// outside of a [crate::App]
    pub(crate) fn new() -> Self {
        Self {
            features: Features::new(),
            settings: RwLock::default(),
            documents: DashMap::default(),
            syntax: DashMap::default(),
            file_features: DashMap::default(),
            project_handlers: ProjectHandlers::new(),
        }
    }

    fn settings(&self) -> SettingsHandle {
        SettingsHandle::new(&self.settings)
    }

    /// Get the supported capabilities for a given file path
    fn get_file_capabilities(&self, path: &RomePath) -> Capabilities {
        let language = self.get_language(path);

        debug!("File capabilities: {:?} {:?}", &language, &path);
        self.features.get_capabilities(path, language)
    }

    /// Get the supported manifest capabilities for a given file path
    #[allow(unused)]
    fn get_project_capabilities(&self, path: &RomePath) -> ProjectCapabilities {
        self.project_handlers
            .get_capabilities(path, ProjectHandlers::get_manifest(path))
    }

    /// Retrieves the supported language of a file
    fn get_language(&self, path: &RomePath) -> Language {
        self.documents
            .get(path)
            .map(|doc| doc.language_hint)
            .unwrap_or_default()
    }

    /// Return an error factory function for unsupported features at a given path
    fn build_capability_error<'a>(
        &'a self,
        path: &'a RomePath,
        // feature_name: &'a str,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            let language_hint = self
                .documents
                .get(path)
                .map(|doc| doc.language_hint)
                .unwrap_or_default();

            let language = Features::get_language(path).or(language_hint);
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

    fn build_rule_filter_list<'a>(
        &'a self,
        rules: Option<&'a Rules>,
        overrides: &'a OverrideSettings,
        path: &'a Path,
    ) -> Vec<RuleFilter> {
        let enabled_rules =
            rules.map(|rules| overrides.overrides_enabled_rules(path, rules.as_enabled_rules()));

        if let Some(enabled_rules) = enabled_rules {
            enabled_rules.into_iter().collect::<Vec<RuleFilter>>()
        } else {
            vec![]
        }
    }

    /// Get the parser result for a given file
    ///
    /// Returns and error if no file exists in the workspace with this path or
    /// if the language associated with the file has no parser capability
    fn get_parse(
        &self,
        rome_path: RomePath,
        _feature: Option<FeatureName>,
    ) -> Result<AnyParse, WorkspaceError> {
        match self.syntax.entry(rome_path) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let rome_path = entry.key();
                let capabilities = self.get_file_capabilities(rome_path);

                let mut document = self
                    .documents
                    .get_mut(rome_path)
                    .ok_or_else(WorkspaceError::not_found)?;

                let parse = capabilities
                    .parser
                    .parse
                    .ok_or_else(self.build_capability_error(rome_path))?;

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
                        rome_path.to_path_buf().display().to_string(),
                        size,
                        size_limit,
                    ));
                }

                let settings = self.settings();
                let parsed = parse(
                    rome_path,
                    document.language_hint,
                    document.content.as_str(),
                    settings,
                    &mut document.node_cache,
                );

                Ok(entry.insert(parsed).clone())
            }
        }
    }

    /// Check whether a file is ignored in the top-level config `files.ignore`/`files.include`
    fn is_ignored_by_top_level_config(&self, path: &Path) -> bool {
        let settings = self.settings();
        let is_included = settings.as_ref().files.included_files.is_empty()
            || settings.as_ref().files.included_files.matches_path(path);
        !is_included || settings.as_ref().files.ignored_files.matches_path(path)
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
                let language = Language::from_path(&params.path);
                let path = params.path.as_path();
                let settings = self.settings.read().unwrap();
                let mut file_features = FileFeaturesResult::new();

                file_features = file_features
                    .with_capabilities(&capabilities)
                    .with_settings_and_language(&settings, &language, path);

                if settings.files.ignore_unknown
                    && language == Language::Unknown
                    && self.get_language(&params.path) == Language::Unknown
                {
                    file_features.ignore_not_supported();
                } else {
                    for feature in params.feature {
                        let is_ignored = self.is_path_ignored(IsPathIgnoredParams {
                            rome_path: params.path.clone(),
                            feature: feature.clone(),
                        })?;
                        if is_ignored {
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
        let settings = self.settings();
        let path = params.rome_path.as_path();
        if path.file_name().and_then(|s| s.to_str()) == Some(BIOME_JSON) {
            return Ok(false);
        }

        // Overrides have top priority
        let excluded_by_override = settings.as_ref().override_settings.is_path_excluded(path);
        if excluded_by_override.unwrap_or_default() {
            return Ok(true);
        }
        let included_by_override = settings.as_ref().override_settings.is_path_included(path);
        if included_by_override.unwrap_or_default() {
            return Ok(false);
        }

        let (ignored_files, included_files) = match params.feature {
            FeatureName::Format => {
                let formatter = &settings.as_ref().formatter;
                (&formatter.ignored_files, &formatter.included_files)
            }
            FeatureName::Lint => {
                let linter = &settings.as_ref().linter;
                (&linter.ignored_files, &linter.included_files)
            }
            FeatureName::OrganizeImports => {
                let organize_imports = &settings.as_ref().organize_imports;
                (
                    &organize_imports.ignored_files,
                    &organize_imports.included_files,
                )
            }
        };

        // Tool include/ignore have priority over (global) files include/ignore
        if ignored_files.matches_path(path) {
            return Ok(true);
        }
        if !included_files.is_empty() && included_files.matches_path(path) {
            return Ok(false);
        }

        Ok(self.is_ignored_by_top_level_config(path))
    }

    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "debug", skip(self))]
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
        let parse = self.get_parse(params.path.clone(), None)?;
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

        let parse = self.get_parse(params.path.clone(), None)?;
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
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

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
    #[tracing::instrument(level = "debug", skip(self))]
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let feature = if params.categories.is_syntax() {
            FeatureName::Format
        } else {
            FeatureName::Lint
        };

        let parse = self.get_parse(params.path.clone(), Some(feature))?;
        let settings = self.settings.read().unwrap();

        let (diagnostics, errors, skipped_diagnostics) = if let Some(lint) =
            self.get_file_capabilities(&params.path).analyzer.lint
        {
            let rules = settings.linter().rules.as_ref();
            let overrides = &settings.override_settings;
            let mut rule_filter_list =
                self.build_rule_filter_list(rules, overrides, params.path.as_path());
            if settings.organize_imports.enabled && !params.categories.is_syntax() {
                rule_filter_list.push(RuleFilter::Rule("correctness", "organizeImports"));
            }
            let mut filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
            filter.categories = params.categories;

            info_span!("Pulling diagnostics", categories =? params.categories).in_scope(|| {
                trace!("Analyzer filter to apply to lint: {:?}", &filter);

                let results = lint(LintParams {
                    parse,
                    filter,
                    rules,
                    settings: self.settings(),
                    max_diagnostics: params.max_diagnostics,
                    path: &params.path,
                    language: self.get_language(&params.path),
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
            skipped_diagnostics,
        })
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    #[tracing::instrument(level = "debug", skip(self))]
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Lint))?;
        let settings = self.settings.read().unwrap();
        let rules = settings.linter().rules.as_ref();
        Ok(code_actions(
            parse,
            params.range,
            rules,
            self.settings(),
            &params.path,
        ))
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
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

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
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

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
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;
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
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Lint))?;

        let rules = settings.as_rules(params.path.as_path());
        let overrides = &settings.override_settings;
        let rule_filter_list =
            self.build_rule_filter_list(rules.as_ref(), overrides, params.path.as_path());
        let filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
        fix_all(FixAllParams {
            parse,
            rules: rules.as_ref(),
            fix_file_mode: params.fix_file_mode,
            filter,
            settings: self.settings(),
            should_format: params.should_format,
            rome_path: &params.path,
        })
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, WorkspaceError> {
        let capabilities = self.get_file_capabilities(&params.path);
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), None)?;
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

        let parse = self.get_parse(params.path, None)?;
        let result = organize_imports(parse)?;

        Ok(result)
    }
}

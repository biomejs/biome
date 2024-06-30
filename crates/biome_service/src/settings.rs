use crate::file_handlers::{
    CssOrganizeImportsEnabled, JsOrganizeImportsEnabled, JsonOrganizeImportsEnabled,
};
use crate::workspace::{DocumentFileSource, ProjectKey, WorkspaceData};
use crate::{Matcher, WorkspaceError};
use biome_analyze::{AnalyzerOptions, AnalyzerRules};
use biome_configuration::bool::Bool;
use biome_configuration::css::{CssFormatterEnabled, CssLinterEnabled};
use biome_configuration::diagnostics::InvalidIgnorePattern;
use biome_configuration::file_size::FileSize;
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::javascript::{JsFormatterEnabled, JsLinterEnabled, JsxRuntime};
use biome_configuration::json::{JsonFormatterEnabled, JsonLinterEnabled};
use biome_configuration::linter::LinterEnabled;
use biome_configuration::organize_imports::{OrganizeImports, OrganizeImportsEnabled};
use biome_configuration::{
    push_to_analyzer_rules, BiomeDiagnostic, Configuration, CssConfiguration, FilesConfiguration,
    FormatterConfiguration, GraphqlConfiguration, IgnoreUnknownEnabled, JsConfiguration,
    JsonConfiguration, LinterConfiguration, OverrideFormatterConfiguration,
    OverrideLinterConfiguration, OverrideOrganizeImportsConfiguration, Overrides, Rules,
};
use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParseOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::{Merge, StringSet};
use biome_diagnostics::Category;
use biome_formatter::{
    AttributePosition, BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use biome_fs::BiomePath;
use biome_graphql_formatter::context::GraphqlFormatOptions;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_analyze::metadata;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::JsParseOptions;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::JsonParseOptions;
use biome_json_syntax::JsonLanguage;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use indexmap::IndexSet;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::RwLockWriteGuard;
use std::sync::{RwLock, RwLockReadGuard};
use tracing::trace;

#[derive(Debug, Default)]
/// The information tracked for each project
pub struct ProjectData {
    /// The root path of the project. This path should be **absolute**.
    path: BiomePath,
    /// The settings of the project, usually inferred from the configuration file e.g. `biome.json`.
    settings: Settings,
}

#[derive(Debug, Default)]
/// Type that manages different projects inside the workspace.
pub struct WorkspaceSettings {
    /// The data of the projects
    data: WorkspaceData<ProjectData>,
    /// The ID of the current project.
    current_project: ProjectKey,
}

impl WorkspaceSettings {
    /// Retrieves the settings of the current workspace folder
    pub fn get_current_settings(&self) -> Option<&Settings> {
        trace!("Current key {:?}", self.current_project);
        let data = self.data.get(self.current_project);
        if let Some(data) = data {
            Some(&data.settings)
        } else {
            None
        }
    }

    /// Retrieves a mutable reference of the settings of the current project
    pub fn get_current_settings_mut(&mut self) -> &mut Settings {
        &mut self
            .data
            .get_mut(self.current_project)
            .expect("You must have at least one workspace.")
            .settings
    }

    /// Register the current project using its unique key
    pub fn register_current_project(&mut self, key: ProjectKey) {
        self.current_project = key;
    }

    /// Insert a new project using its folder. Use [WorkspaceSettings::get_current_settings_mut] to retrieve
    /// a mutable reference to its [Settings] and manipulate them.
    pub fn insert_project(&mut self, workspace_path: impl Into<PathBuf>) -> ProjectKey {
        let path = BiomePath::new(workspace_path.into());
        trace!("Insert workspace folder: {:?}", path);
        self.data.insert(ProjectData {
            path,
            settings: Settings::default(),
        })
    }

    /// Remove a project using its folder.
    pub fn remove_project(&mut self, workspace_path: &Path) {
        let keys_to_remove = {
            let mut data = vec![];
            let iter = self.data.iter();

            for (key, path_to_settings) in iter {
                if path_to_settings.path.as_path() == workspace_path {
                    data.push(key)
                }
            }

            data
        };

        for key in keys_to_remove {
            self.data.remove(key)
        }
    }

    /// Checks if the current path belongs to a registered project.
    ///
    /// If there's a match, and the match **isn't** the current project, it returns the new key.
    pub fn path_belongs_to_current_workspace(&self, path: &BiomePath) -> Option<ProjectKey> {
        if self.data.is_empty() {
            return None;
        }
        trace!("Current key: {:?}", self.current_project);
        let iter = self.data.iter();
        for (key, path_to_settings) in iter {
            trace!(
                "Workspace path {:?}, file path {:?}",
                path_to_settings.path,
                path
            );
            trace!("Iter key: {:?}", key);
            if key == self.current_project {
                continue;
            }
            if path.strip_prefix(path_to_settings.path.as_path()).is_ok() {
                trace!("Update workspace to {:?}", key);
                return Some(key);
            }
        }
        None
    }

    /// Checks if the current path belongs to a registered project.
    ///
    /// If there's a match, and the match **isn't** the current project, the function will mark the match as the current project.
    pub fn set_current_project(&mut self, new_key: ProjectKey) {
        self.current_project = new_key;
    }
}

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct Settings {
    /// Formatter settings applied to all files in the workspaces
    pub formatter: FormatterSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: LinterSettings,
    /// Language specific settings
    pub languages: LanguageListSettings,
    /// Filesystem settings for the workspace
    pub files: FilesSettings,
    /// Analyzer settings
    pub organize_imports: OrganizeImportsSettings,
    /// overrides
    pub override_settings: OverrideSettings,
}

impl Settings {
    /// The [Configuration] is merged into the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: Configuration,
        working_directory: Option<PathBuf>,
        vcs_path: Option<PathBuf>,
        gitignore_matches: &[String],
    ) -> Result<(), WorkspaceError> {
        // formatter settings
        if let Some(formatter) = configuration.formatter {
            self.formatter = to_formatter_settings(working_directory.clone(), formatter)?;
        }

        // linter settings
        if let Some(linter) = configuration.linter {
            self.linter = to_linter_settings(working_directory.clone(), linter)?;
        }

        // filesystem settings
        if let Some(files) = configuration.files {
            self.files = to_file_settings(
                working_directory.clone(),
                files,
                vcs_path,
                gitignore_matches,
            )?;
        }

        // organize imports settings
        if let Some(organize_imports) = configuration.organize_imports {
            self.organize_imports =
                to_organize_imports_settings(working_directory.clone(), organize_imports)?;
        }

        // javascript settings
        if let Some(javascript) = configuration.javascript {
            self.languages.javascript = javascript.into();
        }

        // json settings
        if let Some(json) = configuration.json {
            self.languages.json = json.into();
        }

        // css settings
        if let Some(css) = configuration.css {
            self.languages.css = css.into();
        }

        // graphql settings
        if let Some(graphql) = configuration.graphql {
            self.languages.graphql = graphql.into();
        }

        // overrides settings
        if let Some(overrides) = configuration.overrides {
            self.override_settings = to_override_settings(working_directory.clone(), overrides)?;
        }

        Ok(())
    }

    /// Whether the format_with_errors is enabled for this file path
    pub fn format_with_errors_enabled_for_this_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_format_with_errors_activity_for_this_file_path(path)
            .or(self.formatter.format_with_errors)
            .unwrap_or_default()
            .into()
    }

    /// Whether the files ignore_unknown is enabled
    pub fn ignore_unknown_enabled(&self) -> bool {
        self.files.ignore_unknown.unwrap_or_default().into()
    }

    /// Whether the formatter is enabled for this file path
    pub fn formatter_enabled_for_this_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_formatter_activity_for_this_file_path(path)
            .or(self.formatter.enabled)
            .unwrap_or_default()
            .into()
    }

    /// Whether the linter is enabled for this file path
    pub fn linter_enabled_for_this_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_linter_activity_for_this_file_path(path)
            .or(self.linter.enabled)
            .unwrap_or_default()
            .into()
    }

    /// Whether the organize_imports is enabled for this file path
    pub fn organize_imports_enabled_for_this_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_organize_imports_activity_for_this_file_path(path)
            .or(self.linter.enabled)
            .unwrap_or_default()
            .into()
    }

    /// Whether the formatter is enabled for this JavaScript file path
    pub fn formatter_enabled_for_this_js_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_formatter_activity_for_this_js_file_path(path)
            .or(check_feature_activity(
                self.languages.javascript.formatter.enabled,
                self.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the linter is enabled for this JavaScript file path
    pub fn linter_enabled_for_this_js_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_linter_activity_for_this_js_file_path(path)
            .or(check_feature_activity(
                self.languages.javascript.linter.enabled,
                self.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the organize_imports is enabled for this JavaScript file path
    pub fn organize_imports_enabled_for_this_js_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_organize_imports_activity_for_this_js_file_path(path)
            .or(check_feature_activity(
                self.languages.javascript.organize_imports.enabled,
                self.organize_imports.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the formatter is enabled for this JSON file path
    pub fn formatter_enabled_for_this_json_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_json_file_path_formatter_activity(path)
            .or(check_feature_activity(
                self.languages.json.formatter.enabled,
                self.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the linter is enabled for this JSON file path
    pub fn linter_enabled_for_this_json_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_json_file_path_linter_activity(path)
            .or(check_feature_activity(
                self.languages.json.linter.enabled,
                self.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the organize_imports is enabled for this JSON file path
    pub fn organize_imports_enabled_for_this_json_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_json_file_path_organize_imports_activity(path)
            .or(check_feature_activity(
                self.languages.json.organize_imports.enabled,
                self.organize_imports.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the formatter is enabled for this CSS file path
    pub fn formatter_enabled_for_this_css_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_css_file_path_formatter_activity(path)
            .or(check_feature_activity(
                self.languages.css.formatter.enabled,
                self.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the linter is enabled for this CSS file path
    pub fn linter_enabled_for_this_css_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_css_file_path_linter_activity(path)
            .or(check_feature_activity(
                self.languages.css.linter.enabled,
                self.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// Whether the organize_imports is enabled for this CSS file path
    pub fn organize_imports_enabled_for_this_css_file_path(&self, path: &Path) -> bool {
        self.override_settings
            .check_css_file_path_organize_imports_activity(path)
            .or(check_feature_activity(
                self.languages.css.organize_imports.enabled,
                self.organize_imports.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    /// It retrieves the severity based on the `code` of the rule and the current configuration.
    ///
    /// The code of the has the following pattern: `{group}/{rule_name}`.
    ///
    /// It returns [None] if the `code` doesn't match any rule.
    pub fn get_severity_from_rule_code(
        &self,
        code: &Category,
    ) -> Option<biome_diagnostics::Severity> {
        let rules = self.linter.rules.as_ref();
        if let Some(rules) = rules {
            rules.get_severity_from_code(code)
        } else {
            None
        }
    }

    /// Returns rules taking overrides into account.
    pub fn as_rules(&self, path: &Path) -> Option<Cow<Rules>> {
        let mut result = self.linter.rules.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.linter.rules.as_ref();
            if let Some(pattern_rules) = pattern_rules {
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    result = if let Some(mut result) = result.take() {
                        // Override rules
                        result.to_mut().merge_with(pattern_rules.clone());
                        Some(result)
                    } else {
                        Some(Cow::Borrowed(pattern_rules))
                    };
                }
            }
        }
        result
    }
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
#[derive(Debug)]
pub struct WorkspaceSettingsHandle<'a> {
    inner: RwLockReadGuard<'a, WorkspaceSettings>,
}

impl<'a> WorkspaceSettingsHandle<'a> {
    pub(crate) fn new(settings: &'a RwLock<WorkspaceSettings>) -> Self {
        Self {
            inner: settings.read().unwrap(),
        }
    }

    pub(crate) fn settings(&self) -> Option<&Settings> {
        self.inner.get_current_settings()
    }
}

impl<'a> AsRef<WorkspaceSettings> for WorkspaceSettingsHandle<'a> {
    fn as_ref(&self) -> &WorkspaceSettings {
        &self.inner
    }
}

impl<'a> WorkspaceSettingsHandle<'a> {
    /// Resolve the formatting context for the given language
    pub(crate) fn format_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> L::FormatOptions
    where
        L: ServiceLanguage,
    {
        let settings = self.inner.get_current_settings();
        let formatter = settings.map(|s| &s.formatter);
        let overrides = settings.map(|s| &s.override_settings);
        let editor_settings = settings
            .map(|s| L::lookup_settings(&s.languages))
            .map(|result| &result.formatter);
        L::resolve_format_options(formatter, overrides, editor_settings, path, file_source)
    }

    pub(crate) fn analyze_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> AnalyzerOptions
    where
        L: ServiceLanguage,
    {
        let settings = self.inner.get_current_settings();
        let linter = settings.map(|s| &s.linter);
        let overrides = settings.map(|s| &s.override_settings);
        let editor_settings = settings
            .map(|s| L::lookup_settings(&s.languages))
            .map(|result| &result.linter);
        L::resolve_analyze_options(
            settings,
            linter,
            overrides,
            editor_settings,
            path,
            file_source,
        )
    }
}

pub struct WorkspaceSettingsHandleMut<'a> {
    inner: RwLockWriteGuard<'a, WorkspaceSettings>,
}

impl<'a> WorkspaceSettingsHandleMut<'a> {
    pub(crate) fn new(settings: &'a RwLock<WorkspaceSettings>) -> Self {
        Self {
            inner: settings.write().unwrap(),
        }
    }
}

impl<'a> AsMut<WorkspaceSettings> for WorkspaceSettingsHandleMut<'a> {
    fn as_mut(&mut self) -> &mut WorkspaceSettings {
        &mut self.inner
    }
}

/// Creates a [Matcher] from a [StringSet]
///
/// ## Errors
///
/// It can raise an error if the patterns aren't valid
pub fn to_matcher(
    working_directory: Option<PathBuf>,
    string_set: Option<&StringSet>,
) -> Result<Matcher, WorkspaceError> {
    let mut matcher = Matcher::empty();
    if let Some(working_directory) = working_directory {
        matcher.set_root(working_directory)
    }
    if let Some(string_set) = string_set {
        for pattern in string_set.iter() {
            matcher.add_pattern(pattern).map_err(|err| {
                BiomeDiagnostic::new_invalid_ignore_pattern(
                    pattern.to_string(),
                    err.msg.to_string(),
                )
            })?;
        }
    }
    Ok(matcher)
}

fn to_git_ignore(path: PathBuf, matches: &[String]) -> Result<Gitignore, WorkspaceError> {
    let mut gitignore_builder = GitignoreBuilder::new(path.clone());

    for the_match in matches {
        gitignore_builder
            .add_line(Some(path.clone()), the_match)
            .map_err(|err| {
                BiomeDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
                    message: err.to_string(),
                    file_path: path.to_str().map(|s| s.to_string()),
                })
            })?;
    }
    let gitignore = gitignore_builder.build().map_err(|err| {
        BiomeDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
            message: err.to_string(),
            file_path: path.to_str().map(|s| s.to_string()),
        })
    })?;
    Ok(gitignore)
}

// region: Formatter settings (base)

/// Formatter settings for the entire workspace
#[derive(Debug, Default)]
pub struct FormatterSettings {
    /// Enabled by default
    pub enabled: Option<FormatterEnabled>,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: Option<FormatWithErrorsEnabled>,
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub attribute_position: Option<AttributePosition>,
    pub bracket_spacing: Option<BracketSpacing>,
    /// List of ignore paths/files
    pub ignored_files: Matcher,
    /// List of included paths/files
    pub included_files: Matcher,
}

pub fn to_formatter_settings(
    working_directory: Option<PathBuf>,
    conf: FormatterConfiguration,
) -> Result<FormatterSettings, WorkspaceError> {
    Ok(FormatterSettings {
        enabled: conf.enabled,
        format_with_errors: conf.format_with_errors,
        indent_style: conf.indent_style.map(Into::into),
        indent_width: conf.indent_width,
        line_ending: conf.line_ending,
        line_width: conf.line_width,
        attribute_position: conf.attribute_position,
        bracket_spacing: conf.bracket_spacing,
        ignored_files: to_matcher(working_directory.clone(), conf.ignore.as_ref())?,
        included_files: to_matcher(working_directory, conf.include.as_ref())?,
    })
}

// endregion

// region: Formatter settings (override)

/// Formatter settings in a override entry
#[derive(Debug, Default)]
pub struct OverrideFormatterSettings {
    pub enabled: Option<FormatterEnabled>,
    pub format_with_errors: Option<FormatWithErrorsEnabled>,
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub attribute_position: Option<AttributePosition>,
}

impl From<OverrideFormatterConfiguration> for OverrideFormatterSettings {
    fn from(conf: OverrideFormatterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            format_with_errors: conf.format_with_errors,
            indent_style: conf.indent_style.map(Into::into),
            indent_width: conf.indent_width,
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            bracket_spacing: conf.bracket_spacing,
            attribute_position: conf.attribute_position,
        }
    }
}

// endregion

// region: Linter settings (base)

/// Linter settings for the entire workspace
#[derive(Debug, Default)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of ignored paths/files to match
    pub ignored_files: Matcher,

    /// List of included paths/files to match
    pub included_files: Matcher,
}

// TODO：Rules?
pub fn to_linter_settings(
    working_directory: Option<PathBuf>,
    conf: LinterConfiguration,
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled,
        rules: Some(conf.rules.unwrap_or_default()),
        ignored_files: to_matcher(working_directory.clone(), conf.ignore.as_ref())?,
        included_files: to_matcher(working_directory, conf.include.as_ref())?,
    })
}

// endregion

// region: Linter settings (override)

/// Linter settings in an override entry
#[derive(Debug, Default)]
pub struct OverrideLinterSettings {
    pub enabled: Option<LinterEnabled>,
    pub rules: Option<Rules>,
}

impl From<OverrideLinterConfiguration> for OverrideLinterSettings {
    fn from(conf: OverrideLinterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            rules: conf.rules,
        }
    }
}

// endregion

// region: Organize import settings (base)

/// Organize imports settings for the entire workspace
#[derive(Debug, Default)]
pub struct OrganizeImportsSettings {
    /// Enabled by default
    pub enabled: Option<OrganizeImportsEnabled>,

    /// List of ignored paths/files to match
    pub ignored_files: Matcher,

    /// List of ignored paths/files to match
    pub included_files: Matcher,
}

pub fn to_organize_imports_settings(
    working_directory: Option<PathBuf>,
    organize_imports: OrganizeImports,
) -> Result<OrganizeImportsSettings, WorkspaceError> {
    Ok(OrganizeImportsSettings {
        enabled: organize_imports.enabled,
        ignored_files: to_matcher(working_directory.clone(), organize_imports.ignore.as_ref())?,
        included_files: to_matcher(working_directory, organize_imports.include.as_ref())?,
    })
}

// endregion

// region: Organize import settings (override)

/// Organize imports settings in an override entry
#[derive(Debug, Default)]
pub struct OverrideOrganizeImportsSettings {
    pub enabled: Option<OrganizeImportsEnabled>,
}

impl From<OverrideOrganizeImportsConfiguration> for OverrideOrganizeImportsSettings {
    fn from(conf: OverrideOrganizeImportsConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
        }
    }
}

// endregion

// region: File settings (base)

/// Filesystem settings for the entire workspace
#[derive(Debug, Default)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: Option<FileSize>,

    /// gitignore file patterns
    pub git_ignore: Option<Gitignore>,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,

    /// List of paths/files to matcher
    pub included_files: Matcher,

    /// Files not recognized by Biome should not emit a diagnostic
    pub ignore_unknown: Option<IgnoreUnknownEnabled>,
}

// TODO：Rethink about partial and defaults
fn to_file_settings(
    working_directory: Option<PathBuf>,
    config: FilesConfiguration,
    vcs_config_path: Option<PathBuf>,
    gitignore_matches: &[String],
) -> Result<FilesSettings, WorkspaceError> {
    let git_ignore = if let Some(vcs_config_path) = vcs_config_path {
        Some(to_git_ignore(vcs_config_path, gitignore_matches)?)
    } else {
        None
    };

    Ok(FilesSettings {
        max_size: config.max_size,
        git_ignore,
        ignore_unknown: config.ignore_unknown,
        ignored_files: to_matcher(working_directory.clone(), config.ignore.as_ref())?,
        included_files: to_matcher(working_directory, config.include.as_ref())?,
    })
}

// endregion

// region: Language Settings

#[derive(Debug, Default)]
pub struct LanguageSettings<L: ServiceLanguage> {
    /// Parser settings for this language
    pub parser: L::ParserSettings,

    /// Formatter settings for this language
    pub formatter: L::FormatterSettings,

    /// Linter settings for this language
    pub linter: L::LinterSettings,

    /// Globals variables/bindings that can be found in a file
    pub globals: Option<IndexSet<String>>,

    /// Organize imports settings for this language
    pub organize_imports: L::OrganizeImportsSettings,

    /// Environment settings for this language
    pub environment: L::EnvironmentSettings,
}

/// Static map of language names to language-specific settings
#[derive(Debug, Default)]
pub struct LanguageListSettings {
    pub javascript: LanguageSettings<JsLanguage>,
    pub json: LanguageSettings<JsonLanguage>,
    pub css: LanguageSettings<CssLanguage>,
    pub graphql: LanguageSettings<GraphqlLanguage>,
}

impl From<JsConfiguration> for LanguageSettings<JsLanguage> {
    fn from(javascript: JsConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();

        if let Some(parser) = javascript.parser {
            language_setting.parser.parse_class_parameter_decorators =
                parser.unsafe_parameter_decorators_enabled;
        }

        if let Some(formatter) = javascript.formatter {
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.quote_style = formatter.quote_style;
            language_setting.formatter.jsx_quote_style = formatter.jsx_quote_style;
            language_setting.formatter.quote_properties = formatter.quote_properties;
            language_setting.formatter.trailing_commas = formatter.trailing_commas;
            language_setting.formatter.semicolons = formatter.semicolons;
            language_setting.formatter.arrow_parentheses = formatter.arrow_parentheses;
            language_setting.formatter.bracket_same_line =
            // TODO(zzwu): implement From trait for bracket_same_line
                formatter.bracket_same_line.map(|v| bool::from(v).into());
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.bracket_spacing = formatter.bracket_spacing;
            language_setting.formatter.attribute_position = formatter.attribute_position;
            language_setting.formatter.indent_width = formatter.indent_width.map(Into::into);
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
        }

        if let Some(linter) = javascript.linter {
            language_setting.linter.enabled = linter.enabled;
        }

        if let Some(jsx_runtime) = javascript.jsx_runtime {
            language_setting.environment = jsx_runtime.into();
        }

        if let Some(globals) = javascript.globals {
            language_setting.globals = Some(globals.into_index_set());
        }

        language_setting
    }
}

impl From<JsonConfiguration> for LanguageSettings<JsonLanguage> {
    fn from(json: JsonConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();

        if let Some(parser) = json.parser {
            language_setting.parser.allow_comments = parser.allow_comments;
            language_setting.parser.allow_trailing_commas = parser.allow_trailing_commas;
        }

        if let Some(formatter) = json.formatter {
            language_setting.formatter.trailing_commas = formatter.trailing_commas;
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.indent_width = formatter.indent_width.map(Into::into);
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
        }

        if let Some(linter) = json.linter {
            language_setting.linter.enabled = linter.enabled;
        }

        language_setting
    }
}

impl From<CssConfiguration> for LanguageSettings<CssLanguage> {
    fn from(css: CssConfiguration) -> Self {
        let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();

        if let Some(parser) = css.parser {
            language_setting.parser.allow_wrong_line_comments = parser.allow_wrong_line_comments;
            language_setting.parser.css_modules = parser.css_modules;
        }

        if let Some(formatter) = css.formatter {
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.indent_width = formatter.indent_width;
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.line_ending = formatter.line_ending;
            language_setting.formatter.quote_style = formatter.quote_style;
        }

        if let Some(linter) = css.linter {
            language_setting.linter.enabled = linter.enabled;
        }

        language_setting
    }
}

impl From<GraphqlConfiguration> for LanguageSettings<GraphqlLanguage> {
    fn from(graphql: GraphqlConfiguration) -> Self {
        let mut language_setting: LanguageSettings<GraphqlLanguage> = LanguageSettings::default();

        if let Some(formatter) = graphql.formatter {
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.indent_width = formatter.indent_width;
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.line_ending = formatter.line_ending;
            language_setting.formatter.quote_style = formatter.quote_style;
            language_setting.formatter.bracket_spacing = formatter.bracket_spacing;
        }

        if let Some(linter) = graphql.linter {
            language_setting.linter.enabled = linter.enabled;
        }

        language_setting
    }
}

pub trait ServiceLanguage: biome_rowan::Language {
    /// Parser settings type for this language
    type ParserSettings: Default;

    /// Formatter settings type for this language
    type FormatterSettings: Default;

    /// Linter settings type for this language
    type LinterSettings: Default;

    /// Organize imports settings type for this language
    type OrganizeImportsSettings: Default;

    /// Settings related to the environment/runtime in which the language is used.
    type EnvironmentSettings: Default;

    /// Fully resolved formatter options type for this language
    type FormatOptions: biome_formatter::FormatOptions + Clone + std::fmt::Display;

    /// Read the settings type for this language from the [LanguageListSettings] map
    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self>;

    /// Resolve the format options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: Option<&FormatterSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::FormatOptions;

    /// Resolve the analyze options from the global (workspace level),
    /// per-language and editor provided analyzer settings
    fn resolve_analyze_options(
        global: Option<&Settings>,
        linter: Option<&LinterSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> AnalyzerOptions;
}

// endregion

// region: Overrides Settings

#[derive(Debug, Default)]
pub struct OverrideSettings {
    pub patterns: Vec<OverrideSettingPattern>,
}

pub fn to_override_settings(
    working_directory: Option<PathBuf>,
    overrides: Overrides,
) -> Result<OverrideSettings, WorkspaceError> {
    let mut override_settings = OverrideSettings::default();
    for pattern in overrides.0 {
        let formatter = pattern.formatter.map(Into::into).unwrap_or_default();

        let linter = pattern.linter.map(Into::into).unwrap_or_default();

        let organize_imports = pattern.organize_imports.map(Into::into).unwrap_or_default();

        let languages = LanguageListSettings {
            javascript: pattern.javascript.unwrap_or_default().into(),
            json: pattern.json.unwrap_or_default().into(),
            css: pattern.css.unwrap_or_default().into(),
            graphql: pattern.graphql.unwrap_or_default().into(),
        };

        let pattern_setting = OverrideSettingPattern {
            include: to_matcher(working_directory.clone(), pattern.include.as_ref())?,
            exclude: to_matcher(working_directory.clone(), pattern.ignore.as_ref())?,
            formatter,
            linter,
            organize_imports,
            languages,
            ..OverrideSettingPattern::default()
        };

        override_settings.patterns.push(pattern_setting);
    }

    Ok(override_settings)
}

impl OverrideSettings {
    // region: Common methods

    /// Retrieves the options of lint rules that have been overridden
    pub fn override_analyzer_rules(
        &self,
        path: &Path,
        mut analyzer_rules: AnalyzerRules,
    ) -> AnalyzerRules {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                if let Some(rules) = pattern.linter.rules.as_ref() {
                    push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
                }
            }
        }
        analyzer_rules
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the top-level formatter "enabled" option
    /// explicitly for this file path
    fn check_formatter_activity_for_this_file_path(&self, path: &Path) -> Option<FormatterEnabled> {
        // Reverse the traversal as only the last override takes effect
        return self.patterns.iter().rev().find_map(|pattern| {
            // Check the top-level option
            pattern.formatter.enabled.and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        });
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the top-level linter "enabled" option
    /// explicitly for this file path
    fn check_linter_activity_for_this_file_path(&self, path: &Path) -> Option<LinterEnabled> {
        // Reverse the traversal as only the last override takes effect
        return self.patterns.iter().rev().find_map(|pattern| {
            // Check the top-level option
            pattern.linter.enabled.and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        });
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the top-level organize_imports "enabled" option
    /// explicitly for this file path
    fn check_organize_imports_activity_for_this_file_path(
        &self,
        path: &Path,
    ) -> Option<OrganizeImportsEnabled> {
        // Reverse the traversal as only the last override takes effect
        return self.patterns.iter().rev().find_map(|pattern| {
            // Check the top-level option
            pattern.organize_imports.enabled.and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        });
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the top-level formatter "format_with_errors" option
    /// explicitly for this file path
    fn check_format_with_errors_activity_for_this_file_path(
        &self,
        path: &Path,
    ) -> Option<FormatWithErrorsEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            if let Some(enabled) = pattern.formatter.format_with_errors {
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    return Some(enabled);
                }
            }
            None
        })
    }
    // endregion

    // region: Javascript-specific methods

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific formatter "enabled" option
    /// explicitly for this JavaScript file path
    ///
    /// The function also takes the top-level formatter "enabled"
    /// option into consideration
    fn check_formatter_activity_for_this_js_file_path(
        &self,
        path: &Path,
    ) -> Option<JsFormatterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.javascript.formatter.enabled,
                pattern.formatter.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific linter "enabled" option
    /// explicitly for this JavaScript file path
    ///
    /// The function also takes the top-level linter "enabled"
    /// option into consideration
    fn check_linter_activity_for_this_js_file_path(&self, path: &Path) -> Option<JsLinterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.javascript.linter.enabled,
                pattern.linter.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific organize_imports "enabled" option
    /// explicitly for this JavaScript file path
    ///
    /// The function also takes the top-level organize_imports "enabled"
    /// option into consideration
    fn check_organize_imports_activity_for_this_js_file_path(
        &self,
        path: &Path,
    ) -> Option<JsOrganizeImportsEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.javascript.organize_imports.enabled,
                pattern.organize_imports.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans and aggregates all the overrides into a single `JsParseOptions`
    pub fn to_override_js_parse_options(
        &self,
        path: &Path,
        mut options: JsParseOptions,
    ) -> JsParseOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_js_parse_options(&mut options);
            }
        }
        options
    }

    /// Scans and aggregates all the overrides into a single `JsFormatOptions`
    pub fn to_override_js_format_options(
        &self,
        path: &Path,
        mut options: JsFormatOptions,
    ) -> JsFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_js_format_options(&mut options);
            }
        }
        options
    }

    /// Scans and uses the last override to override the `globals`
    pub fn to_override_js_globals(
        &self,
        path: &BiomePath,
        base_globals: &Option<IndexSet<String>>,
    ) -> IndexSet<String> {
        self.patterns
            .iter()
            // Reverse the traversal as only the last override takes effect
            .rev()
            .find_map(|pattern| {
                let globals = pattern.languages.javascript.globals.as_ref()?;

                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(globals.clone())
                } else {
                    None
                }
            })
            .or(base_globals.clone())
            .unwrap_or_default()
    }

    /// Scans and uses the last override to override the `JsxRuntime`
    pub fn to_override_jsx_runtime(
        &self,
        path: &BiomePath,
        base_jsx_runtime: Option<JsxRuntime>,
    ) -> JsxRuntime {
        self.patterns
            .iter()
            // Reverse the traversal as only the last override takes effect
            .rev()
            .find_map(|pattern| {
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    pattern.languages.javascript.environment.jsx_runtime
                } else {
                    None
                }
            })
            .or(base_jsx_runtime)
            .unwrap_or_default()
    }

    // endregion

    // region: JSON-specific methods

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific formatter "enabled" option
    /// explicitly for this JSON file path
    ///
    /// The function also takes the top-level formatter "enabled"
    /// option into consideration
    fn check_json_file_path_formatter_activity(&self, path: &Path) -> Option<JsonFormatterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.json.formatter.enabled,
                pattern.formatter.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific linter "enabled" option
    /// explicitly for this JSON file path
    ///
    /// The function also takes the top-level linter "enabled"
    /// option into consideration
    fn check_json_file_path_linter_activity(&self, path: &Path) -> Option<JsonLinterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.json.linter.enabled,
                pattern.linter.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific organize_imports "enabled" option
    /// explicitly for this JSON file path
    ///
    /// The function also takes the top-level organize_imports "enabled"
    /// option into consideration
    fn check_json_file_path_organize_imports_activity(
        &self,
        path: &Path,
    ) -> Option<JsonOrganizeImportsEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.json.organize_imports.enabled,
                pattern.organize_imports.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans and aggregates all the overrides into a single `JsonParseOptions`
    pub fn to_override_json_parse_options(
        &self,
        path: &Path,
        mut options: JsonParseOptions,
    ) -> JsonParseOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_json_parse_options(&mut options);
            }
        }
        options
    }

    /// Scans and aggregates all the overrides into a single `JsonFormatOptions`
    pub fn to_override_json_format_options(
        &self,
        path: &Path,
        mut options: JsonFormatOptions,
    ) -> JsonFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_json_format_options(&mut options);
            }
        }
        options
    }

    // endregion

    // region: CSS-specific methods

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific formatter "enabled" option
    /// explicitly for this CSS file path
    ///
    /// The function also takes the top-level formatter "enabled"
    /// option into consideration
    fn check_css_file_path_formatter_activity(&self, path: &Path) -> Option<CssFormatterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.css.formatter.enabled,
                pattern.formatter.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific linter "enabled" option
    /// explicitly for this CSS file path
    ///
    /// The function also takes the top-level linter "enabled"
    /// option into consideration
    fn check_css_file_path_linter_activity(&self, path: &Path) -> Option<CssLinterEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(pattern.languages.css.linter.enabled, pattern.linter.enabled)
                .and_then(|enabled| {
                    // Then check whether the path satisfies
                    if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                        Some(enabled)
                    } else {
                        None
                    }
                })
        })
    }

    /// Scans the overrides and checks if there's an override
    /// that sets the language-specific organize_imports "enabled" option
    /// explicitly for this CSS file path
    ///
    /// The function also takes the top-level organize_imports "enabled"
    /// option into consideration
    fn check_css_file_path_organize_imports_activity(
        &self,
        path: &Path,
    ) -> Option<CssOrganizeImportsEnabled> {
        // Reverse the traversal as only the last override takes effect
        self.patterns.iter().rev().find_map(|pattern| {
            check_feature_activity(
                pattern.languages.css.organize_imports.enabled,
                pattern.organize_imports.enabled,
            )
            .and_then(|enabled| {
                // Then check whether the path satisfies
                if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                    Some(enabled)
                } else {
                    None
                }
            })
        })
    }

    /// Scans and aggregates all the overrides into a single `CssParseOptions`
    pub fn to_override_css_parse_options(
        &self,
        path: &Path,
        mut options: CssParseOptions,
    ) -> CssParseOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_css_parse_options(&mut options);
            }
        }
        options
    }

    /// Scans and aggregates all the overrides into a single `CssFormatOptions`
    pub fn to_override_css_format_options(
        &self,
        path: &Path,
        mut options: CssFormatOptions,
    ) -> CssFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_css_format_options(&mut options);
            }
        }
        options
    }

    // endregion

    // region: GraphQL-specific methods

    /// Scans and aggregates all the overrides into a single `GraphqlFormatOptions`
    pub fn to_override_graphql_format_options(
        &self,
        path: &Path,
        mut options: GraphqlFormatOptions,
    ) -> GraphqlFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.include.matches_path(path) && !pattern.exclude.matches_path(path) {
                pattern.apply_overrides_to_graphql_format_options(&mut options);
            }
        }
        options
    }

    // endregion
}

#[derive(Debug, Default)]
pub struct OverrideSettingPattern {
    pub exclude: Matcher,
    pub include: Matcher,
    /// Formatter settings applied to all files in the workspaces
    pub formatter: OverrideFormatterSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: OverrideLinterSettings,
    /// Linter settings applied to all files in the workspace
    pub organize_imports: OverrideOrganizeImportsSettings,
    /// Language specific settings
    pub languages: LanguageListSettings,

    // Cache
    // For js format options, we use the file source as the cache key because
    // the file source params will affect how tokens are treated during formatting.
    // So we cannot reuse the same format options for all js-family files.
    pub(crate) cached_js_format_options: RwLock<FxHashMap<JsFileSource, JsFormatOptions>>,
    pub(crate) cached_json_format_options: RwLock<Option<JsonFormatOptions>>,
    pub(crate) cached_css_format_options: RwLock<Option<CssFormatOptions>>,
    pub(crate) cached_graphql_format_options: RwLock<Option<GraphqlFormatOptions>>,
    pub(crate) cached_js_parse_options: RwLock<Option<JsParseOptions>>,
    pub(crate) _cached_json_parse_options: RwLock<Option<JsonParseOptions>>,
    pub(crate) cached_css_parse_options: RwLock<Option<CssParseOptions>>,
}

impl OverrideSettingPattern {
    // region: JavaScript

    fn apply_overrides_to_js_parse_options(&self, options: &mut JsParseOptions) {
        if let Ok(readonly_cache) = self.cached_js_parse_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = cached_options.clone();
                return;
            }
        }

        let js_parser = &self.languages.javascript.parser;

        if let Some(parse_class_parameter_decorators) = js_parser.parse_class_parameter_decorators {
            options.parse_class_parameter_decorators = parse_class_parameter_decorators.into()
        }

        if let Ok(mut writeonly_cache) = self.cached_js_parse_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    fn apply_overrides_to_js_format_options(&self, options: &mut JsFormatOptions) {
        if let Ok(readonly_cache) = self.cached_js_format_options.read() {
            if let Some(cached_options) = readonly_cache.get(&options.source_type()) {
                *options = cached_options.clone();
                return;
            }
        }

        let js_formatter = &self.languages.javascript.formatter;
        let formatter = &self.formatter;

        // Formatter settings which are also in top-level

        if let Some(indent_style) = js_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = js_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width);
        }
        if let Some(line_ending) = js_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = js_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }
        if let Some(bracket_spacing) = js_formatter.bracket_spacing.or(formatter.bracket_spacing) {
            options.set_bracket_spacing(bracket_spacing);
        }
        if let Some(attribute_position) = js_formatter
            .attribute_position
            .or(formatter.attribute_position)
        {
            options.set_attribute_position(attribute_position);
        }

        // Formatter settings which are language-specific

        if let Some(quote_style) = js_formatter.quote_style {
            options.set_quote_style(quote_style);
        }
        if let Some(jsx_quote_style) = js_formatter.jsx_quote_style {
            options.set_jsx_quote_style(jsx_quote_style);
        }
        if let Some(quote_properties) = js_formatter.quote_properties {
            options.set_quote_properties(quote_properties);
        }
        if let Some(trailing_commas) = js_formatter.trailing_commas {
            options.set_trailing_commas(trailing_commas);
        }
        if let Some(semicolons) = js_formatter.semicolons {
            options.set_semicolons(semicolons);
        }
        if let Some(arrow_parentheses) = js_formatter.arrow_parentheses {
            options.set_arrow_parentheses(arrow_parentheses);
        }
        if let Some(bracket_same_line) = js_formatter.bracket_same_line {
            options.set_bracket_same_line(bracket_same_line);
        }

        if let Ok(mut writeonly_cache) = self.cached_js_format_options.write() {
            let options = options.clone();
            writeonly_cache.insert(options.source_type(), options);
        }
    }

    // endregion

    // region: JSON

    fn apply_overrides_to_json_parse_options(&self, options: &mut JsonParseOptions) {
        // these options are no longer cached because it was causing incorrect override behavior, see #3260
        // if let Ok(readonly_cache) = self.cached_json_parse_options.read() {
        //     if let Some(cached_options) = readonly_cache.as_ref() {
        //         *options = *cached_options;
        //         return;
        //     }
        // }

        let json_parser = &self.languages.json.parser;

        if let Some(allow_comments) = json_parser.allow_comments {
            options.allow_comments = allow_comments.into();
        }
        if let Some(allow_trailing_commas) = json_parser.allow_trailing_commas {
            options.allow_trailing_commas = allow_trailing_commas.into();
        }

        // if let Ok(mut writeonly_cache) = self.cached_json_parse_options.write() {
        //     let options = *options;
        //     let _ = writeonly_cache.insert(options);
        // }
    }

    fn apply_overrides_to_json_format_options(&self, options: &mut JsonFormatOptions) {
        if let Ok(readonly_cache) = self.cached_json_format_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = cached_options.clone();
                return;
            }
        }

        let json_formatter = &self.languages.json.formatter;
        let formatter = &self.formatter;

        // Formatter settings which are also in top-level

        if let Some(indent_style) = json_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = json_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width)
        }
        if let Some(line_ending) = json_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = json_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }

        // Formatter settings which are language-specific

        if let Some(trailing_commas) = json_formatter.trailing_commas {
            options.set_trailing_commas(trailing_commas);
        }

        if let Ok(mut writeonly_cache) = self.cached_json_format_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    // endregion

    // region: CSS

    fn apply_overrides_to_css_parse_options(&self, options: &mut CssParseOptions) {
        if let Ok(readonly_cache) = self.cached_css_parse_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = *cached_options;
                return;
            }
        }

        let css_parser = &self.languages.css.parser;

        if let Some(allow_wrong_line_comments) = css_parser.allow_wrong_line_comments {
            options.allow_wrong_line_comments = allow_wrong_line_comments.into();
        }
        if let Some(css_modules) = css_parser.css_modules {
            options.css_modules = css_modules.into();
        }

        if let Ok(mut writeonly_cache) = self.cached_css_parse_options.write() {
            let options = *options;
            let _ = writeonly_cache.insert(options);
        }
    }

    fn apply_overrides_to_css_format_options(&self, options: &mut CssFormatOptions) {
        if let Ok(readonly_cache) = self.cached_css_format_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = cached_options.clone();
                return;
            }
        }

        let css_formatter = &self.languages.css.formatter;
        let formatter = &self.formatter;

        // Formatter settings which are also in top-level

        if let Some(indent_style) = css_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = css_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width)
        }
        if let Some(line_ending) = css_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = css_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }

        // Formatter settings which are language-specific

        if let Some(quote_style) = css_formatter.quote_style {
            options.set_quote_style(quote_style);
        }

        if let Ok(mut writeonly_cache) = self.cached_css_format_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    // endregion

    // region: GraphQL

    fn apply_overrides_to_graphql_format_options(&self, options: &mut GraphqlFormatOptions) {
        if let Ok(readonly_cache) = self.cached_graphql_format_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = cached_options.clone();
                return;
            }
        }

        let graphql_formatter = &self.languages.graphql.formatter;
        let formatter = &self.formatter;

        // Formatter settings which are also in top-level

        if let Some(indent_style) = graphql_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = graphql_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width)
        }
        if let Some(line_ending) = graphql_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = graphql_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }
        if let Some(bracket_spacing) = graphql_formatter
            .bracket_spacing
            .or(formatter.bracket_spacing)
        {
            options.set_bracket_spacing(bracket_spacing);
        }

        // Formatter settings which are language-specific

        if let Some(quote_style) = graphql_formatter.quote_style {
            options.set_quote_style(quote_style);
        }

        if let Ok(mut writeonly_cache) = self.cached_graphql_format_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    // endregion

    #[allow(dead_code)]
    // NOTE: Currently not used because the rule options are typed using TypeId and Any, which isn't thread safe.
    // TODO: Find a way to cache this
    fn analyzer_rules_mut(&self, _analyzer_rules: &mut AnalyzerRules) {}
}

// endregion

/// Checks the feature activity according to language-specific
/// and top level feature activities.
///
/// ```markdown
/// | Top \ Lang | Some(T) | Some(F) |    N    |
/// | :--------: | :-----: | :-----: | :-----: |
/// |  Some(T)   | Some(T) | Some(F) |    N    | <-- notice: not Some(T)
/// |  Some(F)   | Some(T) | Some(F) | Some(F) |
/// |     N      | Some(T) | Some(F) |    N    |
/// ```
///
/// The reason for the notice is that we don't want a top level
/// feature to override the language-specific feature whose default
/// value is false but in an "unset" state (`None`). So that we can
/// still use `.unwrap_or_default()` to retrieve the correct
/// fallback value. This happens when we want to mark the features
/// of some languages as opt-in.
///
fn check_feature_activity<const LANG: bool, const TOP: bool>(
    language_specific_feature_activity: Option<Bool<LANG>>,
    top_level_feature_activity: Option<Bool<TOP>>,
) -> Option<Bool<LANG>> {
    // Check the language-specific feature first
    language_specific_feature_activity
        // Then check the top level feature
        .or(top_level_feature_activity.and_then(|v| {
            if v.into() {
                None
            } else {
                Some(v.value().into())
            }
        }))
}

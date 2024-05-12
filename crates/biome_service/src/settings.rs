use crate::workspace::{DocumentFileSource, ProjectKey, WorkspaceData};
use crate::{Matcher, WorkspaceError};
use biome_analyze::AnalyzerRules;
use biome_configuration::diagnostics::InvalidIgnorePattern;
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::organize_imports::OrganizeImports;
use biome_configuration::{
    push_to_analyzer_rules, ConfigurationDiagnostic, CssConfiguration, FilesConfiguration,
    FormatterConfiguration, JavascriptConfiguration, JsonConfiguration, LinterConfiguration,
    OverrideFormatterConfiguration, OverrideLinterConfiguration,
    OverrideOrganizeImportsConfiguration, Overrides, PartialConfiguration, PartialCssConfiguration,
    PartialJavascriptConfiguration, PartialJsonConfiguration, PlainIndentStyle, Rules,
};
use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::{Merge, StringSet};
use biome_diagnostics::Category;
use biome_formatter::{AttributePosition, IndentStyle, IndentWidth, LineEnding, LineWidth};
use biome_fs::BiomePath;
use biome_js_analyze::metadata;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use indexmap::IndexSet;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::RwLockWriteGuard;
use std::{
    num::NonZeroU64,
    sync::{RwLock, RwLockReadGuard},
};
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
    pub fn get_current_settings(&self) -> &Settings {
        trace!("Current key {:?}", self.current_project);
        let data = self
            .data
            .get(self.current_project)
            .expect("You must have at least one workspace.");
        &data.settings
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
        debug_assert!(
            !self.data.is_empty(),
            "You must have at least one workspace."
        );
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
    pub formatter: FormatSettings,
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
    /// The [PartialConfiguration] is merged into the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: PartialConfiguration,
        working_directory: Option<PathBuf>,
        vcs_path: Option<PathBuf>,
        gitignore_matches: &[String],
    ) -> Result<(), WorkspaceError> {
        // formatter part
        if let Some(formatter) = configuration.formatter {
            self.formatter = to_format_settings(
                working_directory.clone(),
                FormatterConfiguration::from(formatter),
            )?;
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter =
                to_linter_settings(working_directory.clone(), LinterConfiguration::from(linter))?;
        }

        // Filesystem settings
        if let Some(files) = to_file_settings(
            working_directory.clone(),
            configuration.files.map(FilesConfiguration::from),
            vcs_path,
            gitignore_matches,
        )? {
            self.files = files;
        }

        if let Some(organize_imports) = configuration.organize_imports {
            self.organize_imports = to_organize_imports_settings(
                working_directory.clone(),
                OrganizeImports::from(organize_imports),
            )?;
        }

        // javascript settings
        if let Some(javascript) = configuration.javascript {
            self.languages.javascript = JavascriptConfiguration::from(javascript).into();
        }
        // json settings
        if let Some(json) = configuration.json {
            self.languages.json = JsonConfiguration::from(json).into();
        }
        // css settings
        if let Some(css) = configuration.css {
            self.languages.css = CssConfiguration::from(css).into();
        }

        // NOTE: keep this last. Computing the overrides require reading the settings computed by the parent settings.
        if let Some(overrides) = configuration.overrides {
            self.override_settings =
                to_override_settings(working_directory.clone(), overrides, self)?;
        }

        Ok(())
    }

    /// Retrieves the settings of the formatter
    pub fn formatter(&self) -> &FormatSettings {
        &self.formatter
    }

    /// Whether the formatter is disabled for JavaScript files
    pub fn javascript_formatter_disabled(&self) -> bool {
        let enabled = self.languages.javascript.formatter.enabled.as_ref();
        enabled == Some(&false)
    }

    /// Whether the formatter is disabled for JSON files
    pub fn json_formatter_disabled(&self) -> bool {
        let enabled = self.languages.json.formatter.enabled.as_ref();
        enabled == Some(&false)
    }

    /// Whether the formatter is disabled for CSS files
    pub fn css_formatter_disabled(&self) -> bool {
        let enabled = self.languages.css.formatter.enabled.as_ref();
        enabled == Some(&false)
    }

    /// Retrieves the settings of the linter
    pub fn linter(&self) -> &LinterSettings {
        &self.linter
    }

    /// Retrieves the settings of the organize imports
    pub fn organize_imports(&self) -> &OrganizeImportsSettings {
        &self.organize_imports
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
            let excluded = pattern.exclude.matches_path(path);
            if !excluded && !pattern.include.is_empty() && pattern.include.matches_path(path) {
                let pattern_rules = pattern.linter.rules.as_ref();
                if let Some(pattern_rules) = pattern_rules {
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

/// Formatter settings for the entire workspace
#[derive(Debug)]
pub struct FormatSettings {
    /// Enabled by default
    pub enabled: bool,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub attribute_position: Option<AttributePosition>,
    /// List of ignore paths/files
    pub ignored_files: Matcher,
    /// List of included paths/files
    pub included_files: Matcher,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_style: Some(IndentStyle::default()),
            indent_width: Some(IndentWidth::default()),
            line_ending: Some(LineEnding::default()),
            line_width: Some(LineWidth::default()),
            attribute_position: Some(AttributePosition::default()),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        }
    }
}

/// Formatter settings for the entire workspace
#[derive(Debug, Default)]
pub struct OverrideFormatSettings {
    /// Enabled by default
    pub enabled: Option<bool>,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of ignored paths/files to match
    pub ignored_files: Matcher,

    /// List of included paths/files to match
    pub included_files: Matcher,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug, Default)]
pub struct OverrideLinterSettings {
    /// Enabled by default
    pub enabled: Option<bool>,

    /// List of rules
    pub rules: Option<Rules>,
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct OrganizeImportsSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of ignored paths/files to match
    pub ignored_files: Matcher,

    /// List of ignored paths/files to match
    pub included_files: Matcher,
}

impl Default for OrganizeImportsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug, Default)]
pub struct OverrideOrganizeImportsSettings {
    /// Enabled by default
    pub enabled: Option<bool>,
}

/// Static map of language names to language-specific settings
#[derive(Debug, Default)]
pub struct LanguageListSettings {
    pub javascript: LanguageSettings<JsLanguage>,
    pub json: LanguageSettings<JsonLanguage>,
    pub css: LanguageSettings<CssLanguage>,
}

impl From<JavascriptConfiguration> for LanguageSettings<JsLanguage> {
    fn from(javascript: JavascriptConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();

        let formatter = javascript.formatter;
        language_setting.formatter.quote_style = Some(formatter.quote_style);
        language_setting.formatter.jsx_quote_style = Some(formatter.jsx_quote_style);
        language_setting.formatter.quote_properties = Some(formatter.quote_properties);
        language_setting.formatter.trailing_comma = Some(formatter.trailing_comma);
        language_setting.formatter.semicolons = Some(formatter.semicolons);
        language_setting.formatter.arrow_parentheses = Some(formatter.arrow_parentheses);
        language_setting.formatter.bracket_spacing = Some(formatter.bracket_spacing.into());
        language_setting.formatter.bracket_same_line = Some(formatter.bracket_same_line.into());
        language_setting.formatter.enabled = Some(formatter.enabled);
        language_setting.formatter.line_width = formatter.line_width;
        language_setting.formatter.indent_width = formatter.indent_width.map(Into::into);
        language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
        language_setting.parser.parse_class_parameter_decorators =
            javascript.parser.unsafe_parameter_decorators_enabled;

        language_setting.globals = Some(javascript.globals.into_index_set());
        language_setting.environment = javascript.jsx_runtime.into();

        language_setting
    }
}

impl From<JsonConfiguration> for LanguageSettings<JsonLanguage> {
    fn from(json: JsonConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();

        language_setting.parser.allow_comments = json.parser.allow_comments;
        language_setting.parser.allow_trailing_commas = json.parser.allow_trailing_commas;
        language_setting.formatter.trailing_commas = json.formatter.trailing_commas;
        language_setting.formatter.enabled = Some(json.formatter.enabled);
        language_setting.formatter.line_width = json.formatter.line_width;
        language_setting.formatter.indent_width = json.formatter.indent_width.map(Into::into);
        language_setting.formatter.indent_style = json.formatter.indent_style.map(Into::into);

        language_setting
    }
}

impl From<CssConfiguration> for LanguageSettings<CssLanguage> {
    fn from(css: CssConfiguration) -> Self {
        let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();

        language_setting.formatter.enabled = Some(css.formatter.enabled);
        language_setting.formatter.line_width = css.formatter.line_width;
        language_setting.formatter.indent_width = css.formatter.indent_width.map(Into::into);
        language_setting.formatter.indent_style = css.formatter.indent_style.map(Into::into);
        language_setting.formatter.quote_style = Some(css.formatter.quote_style);

        language_setting
    }
}

pub trait ServiceLanguage: biome_rowan::Language {
    /// Formatter settings type for this language
    type FormatterSettings: Default;

    type LinterSettings: Default;

    /// Organize imports settings type for this language
    type OrganizeImportsSettings: Default;

    /// Fully resolved formatter options type for this language
    type FormatOptions: biome_formatter::FormatOptions + Clone + std::fmt::Display;

    /// Settings that belong to the parser
    type ParserSettings: Default;

    /// Settings related to the environment/runtime in which the language is used.
    type EnvironmentSettings: Default;

    /// Read the settings type for this language from the [LanguageListSettings] map
    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::FormatOptions;
}

#[derive(Debug, Default)]
pub struct LanguageSettings<L: ServiceLanguage> {
    /// Formatter settings for this language
    pub formatter: L::FormatterSettings,

    /// Linter settings for this language
    pub linter: L::LinterSettings,

    /// Globals variables/bindings that can be found in a file
    pub globals: Option<IndexSet<String>>,

    /// Organize imports settings for this language
    pub organize_imports: L::OrganizeImportsSettings,

    /// Parser settings for this language
    pub parser: L::ParserSettings,

    /// Environment settings for this language
    pub environment: L::EnvironmentSettings,
}

/// Filesystem settings for the entire workspace
#[derive(Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: NonZeroU64,

    /// gitignore file patterns
    pub git_ignore: Option<Gitignore>,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,

    /// List of paths/files to matcher
    pub included_files: Matcher,

    /// Files not recognized by Biome should not emit a diagnostic
    pub ignore_unknown: bool,
}

/// Limit the size of files to 1.0 MiB by default
pub(crate) const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
            git_ignore: None,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
            ignore_unknown: false,
        }
    }
}

fn to_file_settings(
    working_directory: Option<PathBuf>,
    config: Option<FilesConfiguration>,
    vcs_config_path: Option<PathBuf>,
    gitignore_matches: &[String],
) -> Result<Option<FilesSettings>, WorkspaceError> {
    let config = if let Some(config) = config {
        Some(config)
    } else if vcs_config_path.is_some() {
        Some(FilesConfiguration::default())
    } else {
        None
    };
    let git_ignore = if let Some(vcs_config_path) = vcs_config_path {
        Some(to_git_ignore(vcs_config_path, gitignore_matches)?)
    } else {
        None
    };
    Ok(if let Some(config) = config {
        Some(FilesSettings {
            max_size: config.max_size,
            git_ignore,
            ignored_files: to_matcher(working_directory.clone(), Some(&config.ignore))?,
            included_files: to_matcher(working_directory, Some(&config.include))?,
            ignore_unknown: config.ignore_unknown,
        })
    } else {
        None
    })
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

    pub(crate) fn settings(&self) -> &Settings {
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
        self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> L::FormatOptions
    where
        L: ServiceLanguage,
    {
        let settings = self.inner.get_current_settings();
        L::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            &L::lookup_settings(&settings.languages).formatter,
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

#[derive(Debug, Default)]
pub struct OverrideSettings {
    pub patterns: Vec<OverrideSettingPattern>,
}

impl OverrideSettings {
    /// Checks whether at least one override excludes the provided `path`
    pub fn is_path_excluded(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if pattern.exclude.matches_path(path) {
                return Some(true);
            }
        }
        None
    }
    /// Checks whether at least one override include the provided `path`
    pub fn is_path_included(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if pattern.include.matches_path(path) {
                return Some(true);
            }
        }
        None
    }

    /// It scans the current override rules and return the formatting options that of the first override is matched
    pub fn override_js_format_options(
        &self,
        path: &Path,
        options: JsFormatOptions,
    ) -> JsFormatOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = pattern.include.matches_path(path);
            let excluded = pattern.exclude.matches_path(path);

            if excluded {
                return options;
            }

            if included {
                pattern.apply_overrides_to_js_format_options(&mut options);
            }

            options
        })
    }

    pub fn override_js_globals(
        &self,
        path: &BiomePath,
        base_set: &Option<IndexSet<String>>,
    ) -> IndexSet<String> {
        self.patterns
            .iter()
            .fold(base_set.as_ref(), |globals, pattern| {
                let included = pattern.include.matches_path(path);
                let excluded = pattern.exclude.matches_path(path);

                if included && !excluded {
                    pattern.languages.javascript.globals.as_ref()
                } else {
                    globals
                }
            })
            .cloned()
            .unwrap_or_default()
    }

    pub fn override_jsx_runtime(&self, path: &BiomePath, base_setting: JsxRuntime) -> JsxRuntime {
        self.patterns
            .iter()
            .fold(base_setting, |jsx_runtime, pattern| {
                let included = pattern.include.matches_path(path);
                let excluded = pattern.exclude.matches_path(path);

                if included && !excluded {
                    pattern.languages.javascript.environment.jsx_runtime
                } else {
                    jsx_runtime
                }
            })
    }

    /// It scans the current override rules and return the formatting options that of the first override is matched
    pub fn override_json_format_options(
        &self,
        path: &Path,
        options: JsonFormatOptions,
    ) -> JsonFormatOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = !pattern.include.is_empty() && pattern.include.matches_path(path);
            let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);
            if excluded {
                return options;
            }
            if included {
                pattern.apply_overrides_to_json_format_options(&mut options);
            }

            options
        })
    }

    /// It scans the current override rules and return the formatting options that of the first override is matched
    pub fn override_css_format_options(
        &self,
        path: &Path,
        options: CssFormatOptions,
    ) -> CssFormatOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = !pattern.include.is_empty() && pattern.include.matches_path(path);
            let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);
            if excluded {
                return options;
            }
            if included {
                pattern.apply_overrides_to_css_format_options(&mut options);
            }

            options
        })
    }

    pub fn override_js_parser_options(
        &self,
        path: &Path,
        options: JsParserOptions,
    ) -> JsParserOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = !pattern.include.is_empty() && pattern.include.matches_path(path);
            let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);
            if excluded {
                return options;
            }
            if included {
                pattern.apply_overrides_to_js_parser_options(&mut options)
            }
            options
        })
    }

    pub fn override_json_parser_options(
        &self,
        path: &Path,
        options: JsonParserOptions,
    ) -> JsonParserOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = !pattern.include.is_empty() && pattern.include.matches_path(path);
            let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);
            if excluded {
                return options;
            }
            if included {
                pattern.apply_overrides_to_json_parser_options(&mut options);
            }
            options
        })
    }

    pub fn as_css_parser_options(&self, path: &Path) -> Option<CssParserOptions> {
        for pattern in &self.patterns {
            let included = !pattern.include.is_empty() && pattern.include.matches_path(path);
            let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);

            if included || !excluded {
                let css_parser = &pattern.languages.css.parser;

                return Some(CssParserOptions {
                    allow_wrong_line_comments: css_parser.allow_wrong_line_comments,
                });
            }
        }

        None
    }

    /// Retrieves the options of lint rules that have been overridden
    pub fn override_analyzer_rules(
        &self,
        path: &Path,
        analyzer_rules: AnalyzerRules,
    ) -> AnalyzerRules {
        self.patterns
            .iter()
            .fold(analyzer_rules, |mut analyzer_rules, pattern| {
                let excluded = !pattern.exclude.is_empty() && pattern.exclude.matches_path(path);
                if !excluded && !pattern.include.is_empty() && pattern.include.matches_path(path) {
                    if let Some(rules) = pattern.linter.rules.as_ref() {
                        push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
                    }
                }

                analyzer_rules
            })
    }

    /// Scans the overrides and checks if there's an override that disable the formatter for `path`
    pub fn formatter_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if pattern.exclude.matches_path(path) {
                continue;
            }
            if !pattern.include.is_empty() && pattern.include.matches_path(path) {
                if let Some(enabled) = pattern.formatter.enabled {
                    return Some(!enabled);
                }
                continue;
            }
        }
        None
    }

    /// Scans the overrides and checks if there's an override that disable the linter for `path`
    pub fn linter_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if pattern.exclude.matches_path(path) {
                continue;
            }
            if !pattern.include.is_empty() && pattern.include.matches_path(path) {
                if let Some(enabled) = pattern.linter.enabled {
                    return Some(!enabled);
                }
                continue;
            }
        }
        None
    }

    /// Scans the overrides and checks if there's an override that disable the organize imports for `path`
    pub fn organize_imports_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if pattern.exclude.matches_path(path) {
                continue;
            }
            if !pattern.include.is_empty() && pattern.include.matches_path(path) {
                if let Some(enabled) = pattern.organize_imports.enabled {
                    return Some(!enabled);
                }
                continue;
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct OverrideSettingPattern {
    pub exclude: Matcher,
    pub include: Matcher,
    /// Formatter settings applied to all files in the workspaces
    pub formatter: OverrideFormatSettings,
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
    pub(crate) cached_js_parser_options: RwLock<Option<JsParserOptions>>,
    pub(crate) cached_json_parser_options: RwLock<Option<JsonParserOptions>>,
}
impl OverrideSettingPattern {
    fn apply_overrides_to_js_format_options(&self, options: &mut JsFormatOptions) {
        if let Ok(readonly_cache) = self.cached_js_format_options.read() {
            if let Some(cached_options) = readonly_cache.get(&options.source_type()) {
                *options = cached_options.clone();
                return;
            }
        }

        let js_formatter = &self.languages.javascript.formatter;
        let formatter = &self.formatter;
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
        if let Some(quote_style) = js_formatter.quote_style {
            options.set_quote_style(quote_style);
        }
        if let Some(jsx_quote_style) = js_formatter.jsx_quote_style {
            options.set_jsx_quote_style(jsx_quote_style);
        }
        if let Some(quote_properties) = js_formatter.quote_properties {
            options.set_quote_properties(quote_properties);
        }
        if let Some(trailing_comma) = js_formatter.trailing_comma {
            options.set_trailing_comma(trailing_comma);
        }
        if let Some(semicolons) = js_formatter.semicolons {
            options.set_semicolons(semicolons);
        }
        if let Some(arrow_parentheses) = js_formatter.arrow_parentheses {
            options.set_arrow_parentheses(arrow_parentheses);
        }
        if let Some(bracket_spacing) = js_formatter.bracket_spacing {
            options.set_bracket_spacing(bracket_spacing);
        }
        if let Some(bracket_same_line) = js_formatter.bracket_same_line {
            options.set_bracket_same_line(bracket_same_line);
        }
        if let Some(attribute_position) = js_formatter.attribute_position {
            options.set_attribute_position(attribute_position);
        }

        if let Ok(mut writeonly_cache) = self.cached_js_format_options.write() {
            let options = options.clone();
            writeonly_cache.insert(options.source_type(), options);
        }
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
        if let Some(trailing_commas) = json_formatter.trailing_commas {
            options.set_trailing_commas(trailing_commas);
        }

        if let Ok(mut writeonly_cache) = self.cached_json_format_options.write() {
            let options = options.clone();
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
        if let Some(quote_style) = css_formatter.quote_style {
            options.set_quote_style(quote_style);
        }

        if let Ok(mut writeonly_cache) = self.cached_css_format_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    fn apply_overrides_to_js_parser_options(&self, options: &mut JsParserOptions) {
        if let Ok(readonly_cache) = self.cached_js_parser_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = cached_options.clone();
                return;
            }
        }

        let js_parser = &self.languages.javascript.parser;

        options.parse_class_parameter_decorators = js_parser.parse_class_parameter_decorators;

        if let Ok(mut writeonly_cache) = self.cached_js_parser_options.write() {
            let options = options.clone();
            let _ = writeonly_cache.insert(options);
        }
    }

    fn apply_overrides_to_json_parser_options(&self, options: &mut JsonParserOptions) {
        if let Ok(readonly_cache) = self.cached_json_parser_options.read() {
            if let Some(cached_options) = readonly_cache.as_ref() {
                *options = *cached_options;
                return;
            }
        }

        let json_parser = &self.languages.json.parser;

        options.allow_trailing_commas = json_parser.allow_trailing_commas;
        options.allow_comments = json_parser.allow_comments;

        if let Ok(mut writeonly_cache) = self.cached_json_parser_options.write() {
            let options = *options;
            let _ = writeonly_cache.insert(options);
        }
    }

    #[allow(dead_code)]
    // NOTE: Currently not used because the rule options are typed using TypeId and Any, which isn't thread safe.
    // TODO: Find a way to cache this
    fn analyzer_rules_mut(&self, _analyzer_rules: &mut AnalyzerRules) {}
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
                WorkspaceError::Configuration(ConfigurationDiagnostic::new_invalid_ignore_pattern(
                    pattern.to_string(),
                    err.msg.to_string(),
                ))
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
                WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
                    InvalidIgnorePattern {
                        message: err.to_string(),
                    },
                ))
            })?;
    }
    let gitignore = gitignore_builder.build().map_err(|err| {
        WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
            InvalidIgnorePattern {
                message: err.to_string(),
            },
        ))
    })?;
    Ok(gitignore)
}

pub fn to_organize_imports_settings(
    working_directory: Option<PathBuf>,
    organize_imports: OrganizeImports,
) -> Result<OrganizeImportsSettings, WorkspaceError> {
    Ok(OrganizeImportsSettings {
        enabled: organize_imports.enabled,
        ignored_files: to_matcher(working_directory.clone(), Some(&organize_imports.ignore))?,
        included_files: to_matcher(working_directory, Some(&organize_imports.include))?,
    })
}

impl TryFrom<OverrideOrganizeImportsConfiguration> for OrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(
        organize_imports: OverrideOrganizeImportsConfiguration,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: organize_imports.enabled.unwrap_or_default(),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}

pub fn to_override_settings(
    working_directory: Option<PathBuf>,
    overrides: Overrides,
    current_settings: &Settings,
) -> Result<OverrideSettings, WorkspaceError> {
    let mut override_settings = OverrideSettings::default();
    for mut pattern in overrides.0 {
        let formatter = pattern.formatter.take().unwrap_or_default();
        let formatter = to_override_format_settings(formatter, &current_settings.formatter);

        let linter = pattern.linter.take().unwrap_or_default();
        let linter = to_override_linter_settings(linter, &current_settings.linter);

        let organize_imports = pattern.organize_imports.take().unwrap_or_default();
        let organize_imports = to_override_organize_imports_settings(
            organize_imports,
            &current_settings.organize_imports,
        );

        let mut languages = LanguageListSettings::default();
        let javascript = pattern.javascript.take().unwrap_or_default();
        let json = pattern.json.take().unwrap_or_default();
        let css = pattern.css.take().unwrap_or_default();
        languages.javascript =
            to_javascript_language_settings(javascript, &current_settings.languages.javascript);

        languages.json = to_json_language_settings(json, &current_settings.languages.json);
        languages.css = to_css_language_settings(css, &current_settings.languages.css);

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

pub(crate) fn to_override_format_settings(
    conf: OverrideFormatterConfiguration,
    format_settings: &FormatSettings,
) -> OverrideFormatSettings {
    let indent_style = conf
        .indent_style
        .map(Into::into)
        .or(format_settings.indent_style);
    let indent_width = conf
        .indent_width
        .map(Into::into)
        .or(conf.indent_size.map(Into::into))
        .or(format_settings.indent_width);

    let line_ending = conf.line_ending.or(format_settings.line_ending);
    let line_width = conf.line_width.or(format_settings.line_width);
    let format_with_errors = conf
        .format_with_errors
        .unwrap_or(format_settings.format_with_errors);

    OverrideFormatSettings {
        enabled: conf.enabled.or(Some(format_settings.enabled)),
        indent_style,
        indent_width,
        line_ending,
        line_width,
        format_with_errors,
    }
}

fn to_override_linter_settings(
    conf: OverrideLinterConfiguration,
    lint_settings: &LinterSettings,
) -> OverrideLinterSettings {
    OverrideLinterSettings {
        enabled: conf.enabled.or(Some(lint_settings.enabled)),
        rules: conf.rules.or(lint_settings.rules.clone()),
    }
}

fn to_javascript_language_settings(
    mut conf: PartialJavascriptConfiguration,
    parent_settings: &LanguageSettings<JsLanguage>,
) -> LanguageSettings<JsLanguage> {
    let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;
    language_setting.formatter.quote_style = formatter.quote_style.or(parent_formatter.quote_style);
    language_setting.formatter.jsx_quote_style = formatter
        .jsx_quote_style
        .or(parent_formatter.jsx_quote_style);
    language_setting.formatter.quote_properties = formatter
        .quote_properties
        .or(parent_formatter.quote_properties);
    language_setting.formatter.trailing_comma =
        formatter.trailing_comma.or(parent_formatter.trailing_comma);
    language_setting.formatter.semicolons = formatter.semicolons.or(parent_formatter.semicolons);
    language_setting.formatter.arrow_parentheses = formatter
        .arrow_parentheses
        .or(parent_formatter.arrow_parentheses);
    language_setting.formatter.bracket_spacing = formatter
        .bracket_spacing
        .map(Into::into)
        .or(parent_formatter.bracket_spacing);
    language_setting.formatter.bracket_same_line = formatter
        .bracket_same_line
        .map(Into::into)
        .or(parent_formatter.bracket_same_line);
    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.parse_class_parameter_decorators = parser
        .unsafe_parameter_decorators_enabled
        .unwrap_or(parent_parser.parse_class_parameter_decorators);

    let organize_imports = conf.organize_imports;
    if let Some(_organize_imports) = organize_imports {}

    language_setting.globals = conf
        .globals
        .map(StringSet::into_index_set)
        .or_else(|| parent_settings.globals.clone());

    language_setting.environment.jsx_runtime = conf
        .jsx_runtime
        .unwrap_or(parent_settings.environment.jsx_runtime);

    language_setting
}

fn to_json_language_settings(
    mut conf: PartialJsonConfiguration,
    parent_settings: &LanguageSettings<JsonLanguage>,
) -> LanguageSettings<JsonLanguage> {
    let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;

    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);
    language_setting.formatter.trailing_commas = formatter
        .trailing_commas
        .or(parent_formatter.trailing_commas);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_comments = parser
        .allow_comments
        .unwrap_or(parent_parser.allow_comments);

    language_setting.parser.allow_trailing_commas = parser
        .allow_trailing_commas
        .unwrap_or(parent_parser.allow_trailing_commas);

    language_setting
}

fn to_css_language_settings(
    mut conf: PartialCssConfiguration,
    parent_settings: &LanguageSettings<CssLanguage>,
) -> LanguageSettings<CssLanguage> {
    let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;

    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);
    language_setting.formatter.quote_style = formatter.quote_style.or(parent_formatter.quote_style);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_wrong_line_comments = parser
        .allow_wrong_line_comments
        .unwrap_or(parent_parser.allow_wrong_line_comments);

    language_setting
}

fn to_override_organize_imports_settings(
    conf: OverrideOrganizeImportsConfiguration,
    settings: &OrganizeImportsSettings,
) -> OverrideOrganizeImportsSettings {
    OverrideOrganizeImportsSettings {
        enabled: conf.enabled.or(Some(settings.enabled)),
    }
}

pub fn to_format_settings(
    working_directory: Option<PathBuf>,
    conf: FormatterConfiguration,
) -> Result<FormatSettings, WorkspaceError> {
    let indent_style = match conf.indent_style {
        PlainIndentStyle::Tab => IndentStyle::Tab,
        PlainIndentStyle::Space => IndentStyle::Space,
    };
    let indent_width = conf.indent_width.into();

    Ok(FormatSettings {
        enabled: conf.enabled,
        indent_style: Some(indent_style),
        indent_width: Some(indent_width),
        line_ending: Some(conf.line_ending),
        line_width: Some(conf.line_width),
        format_with_errors: conf.format_with_errors,
        attribute_position: Some(conf.attribute_position),
        ignored_files: to_matcher(working_directory.clone(), Some(&conf.ignore))?,
        included_files: to_matcher(working_directory, Some(&conf.include))?,
    })
}

impl TryFrom<OverrideFormatterConfiguration> for FormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideFormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            Some(PlainIndentStyle::Tab) => IndentStyle::Tab,
            Some(PlainIndentStyle::Space) => IndentStyle::Space,
            None => IndentStyle::default(),
        };
        let indent_width = conf
            .indent_width
            .map(Into::into)
            .or(conf.indent_size.map(Into::into))
            .unwrap_or_default();

        Ok(Self {
            enabled: conf.enabled.unwrap_or_default(),
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            attribute_position: Some(AttributePosition::default()),
            format_with_errors: conf.format_with_errors.unwrap_or_default(),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}

pub fn to_linter_settings(
    working_directory: Option<PathBuf>,
    conf: LinterConfiguration,
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled,
        rules: Some(conf.rules),
        ignored_files: to_matcher(working_directory.clone(), Some(&conf.ignore))?,
        included_files: to_matcher(working_directory.clone(), Some(&conf.include))?,
    })
}

impl TryFrom<OverrideLinterConfiguration> for LinterSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideLinterConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled.unwrap_or_default(),
            rules: conf.rules,
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}

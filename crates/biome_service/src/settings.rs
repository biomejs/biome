use crate::workspace::DocumentFileSource;
use crate::WorkspaceError;
use biome_analyze::{AnalyzerOptions, AnalyzerRules, RuleDomain};
use biome_configuration::analyzer::assist::{Actions, AssistConfiguration, AssistEnabled};
use biome_configuration::analyzer::{LinterEnabled, RuleDomainValue};
use biome_configuration::bool::Bool;
use biome_configuration::diagnostics::InvalidIgnorePattern;
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::html::HtmlConfiguration;
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::max_size::MaxSize;
use biome_configuration::plugins::Plugins;
use biome_configuration::{
    push_to_analyzer_assist, push_to_analyzer_rules, BiomeDiagnostic, Configuration,
    CssConfiguration, FilesConfiguration, FilesIgnoreUnknownEnabled, FormatterConfiguration,
    GraphqlConfiguration, GritConfiguration, JsConfiguration, JsonConfiguration,
    LinterConfiguration, OverrideAssistConfiguration, OverrideFormatterConfiguration,
    OverrideGlobs, OverrideLinterConfiguration, Overrides, Rules,
};
use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::Merge;
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, IndentStyle, IndentWidth, LineEnding,
    LineWidth, ObjectWrap,
};
use biome_fs::BiomePath;
use biome_graphql_formatter::context::GraphqlFormatOptions;
use biome_graphql_syntax::GraphqlLanguage;
use biome_grit_formatter::context::GritFormatOptions;
use biome_grit_syntax::GritLanguage;
use biome_html_formatter::HtmlFormatOptions;
use biome_html_syntax::HtmlLanguage;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsLanguage;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use camino::{Utf8Path, Utf8PathBuf};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::ops::Deref;
use tracing::instrument;

/// Global settings for the entire project.
#[derive(Clone, Debug, Default)]
pub struct Settings {
    /// Formatter settings applied to all files in the project.
    pub formatter: FormatSettings,
    /// Linter settings applied to all files in the project.
    pub linter: LinterSettings,
    /// Language specific settings
    pub languages: LanguageListSettings,
    /// Filesystem settings for the project.
    pub files: FilesSettings,
    /// Assist settings
    pub assist: AssistSettings,
    /// Plugin settings.
    pub plugins: Plugins,
    /// overrides
    pub override_settings: OverrideSettings,
}

impl Settings {
    /// Merges the [Configuration] into the settings.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: Configuration,
        working_directory: Option<Utf8PathBuf>,
        vcs_path: Option<Utf8PathBuf>,
        gitignore_matches: &[String],
    ) -> Result<(), WorkspaceError> {
        // formatter part
        if let Some(formatter) = configuration.formatter {
            self.formatter = to_format_settings(working_directory.clone(), formatter)?;
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter = to_linter_settings(working_directory.clone(), linter)?;
        }

        // assist part
        if let Some(assist) = configuration.assist {
            self.assist = to_assist_settings(working_directory.clone(), assist)?;
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

        // javascript settings
        if let Some(javascript) = configuration.javascript {
            self.languages.javascript = javascript.into()
        }
        // json settings
        if let Some(json) = configuration.json {
            self.languages.json = json.into()
        }
        // css settings
        if let Some(css) = configuration.css {
            self.languages.css = css.into()
        }
        // graphql settings
        if let Some(graphql) = configuration.graphql {
            self.languages.graphql = graphql.into()
        }
        // html settings
        if let Some(html) = configuration.html {
            self.languages.html = html.into()
        }

        // plugin settings
        if let Some(plugins) = configuration.plugins {
            self.plugins = plugins;
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

    /// Whether the files ignore_unknown is enabled
    pub fn ignore_unknown_enabled(&self) -> bool {
        self.files.ignore_unknown.unwrap_or_default().into()
    }

    /// Retrieves the settings of the linter
    pub fn linter(&self) -> &LinterSettings {
        &self.linter
    }

    /// Retrieves the settings of the organize imports
    pub fn assist(&self) -> &AssistSettings {
        &self.assist
    }

    /// Returns linter rules taking overrides into account.
    pub fn as_linter_rules(&self, path: &Utf8Path) -> Option<Cow<Rules>> {
        let mut result = self.linter.rules.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.linter.rules.as_ref();
            if let Some(pattern_rules) = pattern_rules {
                if pattern.is_file_included(path) {
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

    /// Extract the domains applied to the given `path`, by looking that the base `domains`, and the once applied by `overrides`
    pub fn as_linter_domains(
        &self,
        path: &Utf8Path,
    ) -> Option<Cow<FxHashMap<RuleDomain, RuleDomainValue>>> {
        let mut result = self.linter.domains.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.linter.domains.as_ref();
            if let Some(pattern_rules) = pattern_rules {
                if pattern.is_file_included(path) {
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

    /// Returns assists rules taking overrides into account.
    pub fn as_assist_actions(&self, path: &Utf8Path) -> Option<Cow<Actions>> {
        let mut result = self.assist.actions.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.assist.actions.as_ref();
            if let Some(pattern_rules) = pattern_rules {
                if pattern.is_file_included(path) {
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

    pub fn is_formatter_enabled(&self) -> bool {
        self.formatter.is_enabled()
    }

    pub fn is_linter_enabled(&self) -> bool {
        self.linter.is_enabled()
    }

    pub fn is_assist_enabled(&self) -> bool {
        self.assist.is_enabled()
    }
}

/// Formatter settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct FormatSettings {
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
    pub bracket_same_line: Option<BracketSameLine>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub object_wrap: Option<ObjectWrap>,
    /// List of included paths/files
    pub includes: Includes,
}

impl FormatSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

/// Formatter settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct OverrideFormatSettings {
    /// Enabled by default
    pub enabled: Option<FormatterEnabled>,
    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: Option<FormatWithErrorsEnabled>,
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub bracket_spacing: Option<BracketSpacing>,
    pub bracket_same_line: Option<BracketSameLine>,
    pub attribute_position: Option<AttributePosition>,
    pub object_wrap: Option<ObjectWrap>,
}

impl From<OverrideFormatterConfiguration> for OverrideFormatSettings {
    fn from(conf: OverrideFormatterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            format_with_errors: conf.format_with_errors,
            indent_style: conf.indent_style.map(Into::into),
            indent_width: conf.indent_width,
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            bracket_spacing: conf.bracket_spacing,
            bracket_same_line: conf.bracket_same_line,
            attribute_position: conf.attribute_position,
            object_wrap: conf.object_wrap,
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of included paths/files
    pub includes: Includes,

    /// Rule domains
    pub domains: Option<FxHashMap<RuleDomain, RuleDomainValue>>,
}

impl LinterSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

/// Linter settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct OverrideLinterSettings {
    /// Enabled by default
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of domains
    pub domains: Option<FxHashMap<RuleDomain, RuleDomainValue>>,
}

/// Linter settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct AssistSettings {
    /// Enabled by default
    pub enabled: Option<AssistEnabled>,

    /// List of rules
    pub actions: Option<Actions>,

    /// List of included paths/files
    pub includes: Includes,
}

impl AssistSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

/// Assist settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct OverrideAssistSettings {
    /// Enabled by default
    pub enabled: Option<AssistEnabled>,

    /// List of rules
    pub actions: Option<Actions>,
}

/// Static map of language names to language-specific settings
#[derive(Clone, Debug, Default)]
pub struct LanguageListSettings {
    pub javascript: LanguageSettings<JsLanguage>,
    pub json: LanguageSettings<JsonLanguage>,
    pub css: LanguageSettings<CssLanguage>,
    pub graphql: LanguageSettings<GraphqlLanguage>,
    pub html: LanguageSettings<HtmlLanguage>,
    pub grit: LanguageSettings<GritLanguage>,
}

impl From<JsConfiguration> for LanguageSettings<JsLanguage> {
    fn from(javascript: JsConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();

        if let Some(formatter) = javascript.formatter {
            language_setting.formatter = formatter.into();
        }

        if let Some(parser) = javascript.parser {
            language_setting.parser = parser.into();
        }

        if let Some(linter) = javascript.linter {
            language_setting.linter = linter.into();
        }

        if let Some(assist) = javascript.assist {
            language_setting.assist = assist.into();
        }

        if let Some(jsx_runtime) = javascript.jsx_runtime {
            language_setting.environment = jsx_runtime.into();
        }

        if let Some(globals) = javascript.globals {
            language_setting.globals = Some(globals);
        }

        language_setting
    }
}

impl From<JsonConfiguration> for LanguageSettings<JsonLanguage> {
    fn from(json: JsonConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();

        if let Some(parser) = json.parser {
            language_setting.parser = parser.into();
        }
        if let Some(formatter) = json.formatter {
            language_setting.formatter = formatter.into();
        }
        if let Some(linter) = json.linter {
            language_setting.linter = linter.into();
        }
        if let Some(assist) = json.assist {
            language_setting.assist = assist.into()
        }

        language_setting
    }
}

impl From<CssConfiguration> for LanguageSettings<CssLanguage> {
    fn from(css: CssConfiguration) -> Self {
        let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();

        if let Some(parser) = css.parser {
            language_setting.parser = parser.into();
        }
        if let Some(formatter) = css.formatter {
            language_setting.formatter = formatter.into();
        }
        if let Some(linter) = css.linter {
            language_setting.linter = linter.into();
        }

        if let Some(assist) = css.assist {
            language_setting.assist = assist.into();
        }

        if let Some(globals) = css.globals {
            language_setting.globals = Some(globals);
        }

        language_setting
    }
}

impl From<GraphqlConfiguration> for LanguageSettings<GraphqlLanguage> {
    fn from(graphql: GraphqlConfiguration) -> Self {
        let mut language_setting: LanguageSettings<GraphqlLanguage> = LanguageSettings::default();

        if let Some(formatter) = graphql.formatter {
            language_setting.formatter = formatter.into();
        }

        if let Some(linter) = graphql.linter {
            language_setting.linter = linter.into();
        }

        if let Some(assist) = graphql.assist {
            language_setting.assist = assist.into();
        }

        language_setting
    }
}

impl From<GritConfiguration> for LanguageSettings<GritLanguage> {
    fn from(grit: GritConfiguration) -> Self {
        let mut language_setting: LanguageSettings<GritLanguage> = LanguageSettings::default();
        if let Some(formatter) = grit.formatter {
            language_setting.formatter = formatter.into();
        }

        if let Some(linter) = grit.linter {
            language_setting.linter = linter.into();
        }

        if let Some(assist) = grit.assist {
            language_setting.assist = assist.into();
        }

        language_setting
    }
}

impl From<HtmlConfiguration> for LanguageSettings<HtmlLanguage> {
    fn from(html: HtmlConfiguration) -> Self {
        let mut language_setting: LanguageSettings<HtmlLanguage> = LanguageSettings::default();
        if let Some(formatter) = html.formatter {
            language_setting.formatter = formatter.into();
        }

        // NOTE: uncomment once ready
        // if let Some(linter) = html.linter {
        //     language_setting.linter = linter.into();
        // }
        //
        // if let Some(assist) = html.assist {
        //     language_setting.assist = assist.into();
        // }

        language_setting
    }
}

pub trait ServiceLanguage: biome_rowan::Language {
    /// Formatter settings type for this language
    type FormatterSettings: Default;

    type LinterSettings: Default;

    type AssistSettings: Default;

    /// Fully resolved formatter options type for this language
    type FormatOptions: biome_formatter::FormatOptions + Clone + std::fmt::Display + Default;

    /// Settings that belong to the parser
    type ParserSettings: Default;

    /// Settings related to the environment/runtime in which the language is used.
    type EnvironmentSettings: Default;

    /// Read the settings type for this language from the [LanguageListSettings] map
    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: Option<&FormatSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::FormatOptions;

    /// Resolve the linter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_analyzer_options(
        global: Option<&Settings>,
        linter: Option<&LinterSettings>,
        overrides: Option<&OverrideSettings>,
        language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions;

    /// Checks whether this file has the linter enabled.
    ///
    /// The language is responsible for checking this.
    fn linter_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool;

    /// Responsible to check whether this file has formatter enabled. The language is responsible to check this
    fn formatter_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool;

    /// Responsible to check whether this file has assist enabled. The language is responsible to check this
    fn assist_enabled_for_file_path(settings: Option<&Settings>, path: &Utf8Path) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct LanguageSettings<L: ServiceLanguage> {
    /// Formatter settings for this language
    pub formatter: L::FormatterSettings,

    /// Linter settings for this language
    pub linter: L::LinterSettings,

    /// Linter settings for this language
    pub assist: L::AssistSettings,

    /// Globals variables/bindings that can be found in a file
    pub globals: Option<rustc_hash::FxHashSet<Box<str>>>,

    /// Parser settings for this language
    pub parser: L::ParserSettings,

    /// Environment settings for this language
    pub environment: L::EnvironmentSettings,
}

/// Filesystem settings for the entire workspace
#[derive(Clone, Default, Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: Option<MaxSize>,

    /// gitignore file patterns
    pub git_ignore: Option<Gitignore>,

    /// List of included paths/files
    pub includes: Includes,

    /// Files not recognized by Biome should not emit a diagnostic
    pub ignore_unknown: Option<FilesIgnoreUnknownEnabled>,
}

/// An optional list of globs with exceptions that first normalizes the tested paths before matching them against the globs.
#[derive(Clone, Default, Debug)]
pub struct Includes {
    /// This path is used to normalize the tested paths against [Self::globs].
    working_directory: Option<Utf8PathBuf>,
    /// If `None`, then all files are included
    /// Otherwise this filtered out all files that doesn't match.
    globs: Option<Box<[biome_glob::Glob]>>,
}
impl Includes {
    fn new(
        working_directory: Option<Utf8PathBuf>,
        globs: Option<impl Into<Box<[biome_glob::Glob]>>>,
    ) -> Self {
        Self {
            working_directory,
            globs: globs.map(|globs| globs.into()),
        }
    }

    /// Returns `true` is no globs are set.
    pub fn is_unset(&self) -> bool {
        self.globs.is_none()
    }

    /// Normalize `path` and match it against the list of globs.
    pub fn matches_with_exceptions(&self, path: &Utf8Path) -> bool {
        let Some(globs) = self.globs.as_ref() else {
            return true;
        };
        let path = if let Some(working_directory) = &self.working_directory {
            path.strip_prefix(working_directory).unwrap_or(path)
        } else {
            path
        };
        let candidate_path = biome_glob::CandidatePath::new(path);
        candidate_path.matches_with_exceptions(globs)
    }

    /// Normalize `path` and match it against the list of globs.
    pub fn matches_directory_with_exceptions(&self, path: &Utf8Path) -> bool {
        let Some(globs) = self.globs.as_ref() else {
            return true;
        };
        let path = if let Some(working_directory) = &self.working_directory {
            path.strip_prefix(working_directory).unwrap_or(path)
        } else {
            path
        };
        let candidate_path = biome_glob::CandidatePath::new(path);
        candidate_path.matches_directory_with_exceptions(globs)
    }
}

#[derive(Clone, Default, Debug)]
pub struct OverrideIncludes {
    /// This path is used to normalize the tested paths against [Self::globs].
    working_directory: Option<Utf8PathBuf>,
    /// If `None`, then all files are included
    /// Otherwise this filtered out all files that doesn't match.
    globs: Option<OverrideGlobs>,
}
impl OverrideIncludes {
    pub fn new(working_directory: Option<Utf8PathBuf>, globs: Option<OverrideGlobs>) -> Self {
        Self {
            working_directory,
            globs,
        }
    }

    /// Returns `true` is no globs are set.
    pub fn is_unset(&self) -> bool {
        self.globs.is_none()
    }

    /// Normalize `path` and match it against the list of globs.
    pub fn matches(&self, path: &Utf8Path) -> bool {
        let Some(globs) = self.globs.as_ref() else {
            return true;
        };
        let path = if let Some(working_directory) = &self.working_directory {
            path.strip_prefix(working_directory).unwrap_or(path)
        } else {
            path
        };
        let candidate_path = biome_glob::CandidatePath::new(path);
        globs.is_match_candidate(&candidate_path)
    }
}

fn to_file_settings(
    working_directory: Option<Utf8PathBuf>,
    config: Option<FilesConfiguration>,
    vcs_config_path: Option<Utf8PathBuf>,
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
            includes: Includes::new(working_directory, config.includes),
            ignore_unknown: config.ignore_unknown,
        })
    } else {
        None
    })
}

/// Handle object holding a pin of the workspace settings until the deferred
/// language-specific options resolution is called.
#[derive(Debug)]
pub struct WorkspaceSettingsHandle {
    settings: Option<Settings>,
}

impl From<Option<Settings>> for WorkspaceSettingsHandle {
    fn from(settings: Option<Settings>) -> Self {
        Self { settings }
    }
}

impl From<Settings> for WorkspaceSettingsHandle {
    fn from(settings: Settings) -> Self {
        Self {
            settings: Some(settings),
        }
    }
}

impl WorkspaceSettingsHandle {
    pub fn settings(&self) -> Option<&Settings> {
        self.settings.as_ref()
    }

    /// Resolve the formatting context for the given language
    #[instrument(level = "debug", skip(file_source))]
    pub fn format_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> L::FormatOptions
    where
        L: ServiceLanguage,
    {
        let settings = self.settings();
        let formatter = settings.map(|s| &s.formatter);
        let overrides = settings.map(|s| &s.override_settings);
        let editor_settings = settings
            .map(|s| L::lookup_settings(&s.languages))
            .map(|result| &result.formatter);
        L::resolve_format_options(formatter, overrides, editor_settings, path, file_source)
    }

    pub fn analyzer_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions
    where
        L: ServiceLanguage,
    {
        let settings = self.settings();
        let linter_settings = settings.map(|s| &s.linter);
        let overrides = settings.map(|s| &s.override_settings);
        let editor_settings = settings
            .map(|s| L::lookup_settings(&s.languages))
            .map(|result| &result.linter);
        L::resolve_analyzer_options(
            settings,
            linter_settings,
            overrides,
            editor_settings,
            path,
            file_source,
            suppression_reason,
        )
    }

    /// Whether the linter is enabled for this file path
    pub fn linter_enabled_for_file_path<L>(&self, path: &Utf8Path) -> bool
    where
        L: ServiceLanguage,
    {
        let settings = self.settings();

        L::linter_enabled_for_file_path(settings, path)
    }

    /// Whether the formatter is enabled for this file path
    pub fn formatter_enabled_for_file_path<L>(&self, path: &Utf8Path) -> bool
    where
        L: ServiceLanguage,
    {
        let settings = self.settings();

        L::formatter_enabled_for_file_path(settings, path)
    }

    /// Whether the assist is enabled for this file path
    pub fn assist_enabled_for_file_path<L>(&self, path: &Utf8Path) -> bool
    where
        L: ServiceLanguage,
    {
        let settings = self.settings();

        L::assist_enabled_for_file_path(settings, path)
    }

    /// Whether the formatter should format with parsing errors, for this file path
    pub fn format_with_errors_enabled_for_this_file_path(&self, path: &Utf8Path) -> bool {
        let settings = self.settings();

        settings
            .and_then(|settings| {
                settings
                    .override_settings
                    .patterns
                    .iter()
                    .rev()
                    .find_map(|pattern| {
                        if let Some(enabled) = pattern.formatter.format_with_errors {
                            if pattern.is_file_included(path) {
                                return Some(enabled);
                            }
                        }
                        None
                    })
                    .or(settings.formatter.format_with_errors)
            })
            .unwrap_or_default()
            .into()
    }
}

#[derive(Clone, Debug, Default)]
pub struct OverrideSettings {
    pub patterns: Vec<OverrideSettingPattern>,
}

impl OverrideSettings {
    /// It scans the current override rules and return the formatting options that of the first override is matched
    pub fn override_js_format_options(
        &self,
        path: &Utf8Path,
        mut options: JsFormatOptions,
    ) -> JsFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_js_format_options(&mut options);
            }
        }
        options
    }

    pub fn override_js_globals(
        &self,
        path: &BiomePath,
        base_set: &Option<rustc_hash::FxHashSet<Box<str>>>,
    ) -> rustc_hash::FxHashSet<Box<str>> {
        self.patterns
            .iter()
            // Reverse the traversal as only the last override takes effect
            .rev()
            .find_map(|pattern| {
                if pattern.languages.javascript.globals.is_some() && pattern.is_file_included(path)
                {
                    pattern.languages.javascript.globals.clone()
                } else {
                    None
                }
            })
            .or_else(|| base_set.clone())
            .unwrap_or_default()
    }

    pub fn override_jsx_runtime(&self, path: &BiomePath, base_setting: JsxRuntime) -> JsxRuntime {
        self.patterns
            .iter()
            // Reverse the traversal as only the last override takes effect
            .rev()
            .find_map(|pattern| {
                if pattern.is_file_included(path) {
                    pattern.languages.javascript.environment.jsx_runtime
                } else {
                    None
                }
            })
            .unwrap_or(base_setting)
    }

    pub fn to_override_grit_format_options(
        &self,
        path: &Utf8Path,
        mut options: GritFormatOptions,
    ) -> GritFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_grit_format_options(&mut options);
            }
        }
        options
    }

    pub fn to_override_html_format_options(
        &self,
        path: &Utf8Path,
        mut options: HtmlFormatOptions,
    ) -> HtmlFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_html_format_options(&mut options);
            }
        }
        options
    }

    pub fn to_override_js_parser_options(
        &self,
        path: &Utf8Path,
        mut options: JsParserOptions,
    ) -> JsParserOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_js_parser_options(&mut options);
            }
        }
        options
    }

    pub fn to_override_json_parser_options(
        &self,
        path: &Utf8Path,
        mut options: JsonParserOptions,
    ) -> JsonParserOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_parser_options(&mut options);
            }
        }
        options
    }

    /// Scans the override rules and returns the parser options of the first matching override.
    pub fn to_override_css_parser_options(
        &self,
        path: &Utf8Path,
        mut options: CssParserOptions,
    ) -> CssParserOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_css_parser_options(&mut options);
            }
        }
        options
    }

    // #region: CSS-specific methods

    /// Scans and aggregates all the overrides into a single [CssFormatOptions]
    pub fn to_override_css_format_options(
        &self,
        path: &Utf8Path,
        mut options: CssFormatOptions,
    ) -> CssFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_css_format_options(&mut options);
            }
        }
        options
    }

    /// Scans and aggregates all the overrides into a single [JsonParserOptions]
    pub fn to_override_json_parse_options(
        &self,
        path: &Utf8Path,
        mut options: JsonParserOptions,
    ) -> JsonParserOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_parser_options(&mut options);
            }
        }
        options
    }

    /// Scans and aggregates all the overrides into a single `JsonFormatOptions`
    pub fn to_override_json_format_options(
        &self,
        path: &Utf8Path,
        mut options: JsonFormatOptions,
    ) -> JsonFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_format_options(&mut options);
            }
        }
        options
    }

    // #endregion

    // #region: GraphQL  methods

    /// Scans and aggregates all the overrides into a single [GraphqlFormatOptions]
    pub fn to_override_graphql_format_options(
        &self,
        path: &Utf8Path,
        mut options: GraphqlFormatOptions,
    ) -> GraphqlFormatOptions {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_graphql_format_options(&mut options);
            }
        }
        options
    }
    // #endregion

    /// Retrieves the options of lint rules that have been overridden
    pub fn override_analyzer_rules(
        &self,
        path: &Utf8Path,
        mut analyzer_rules: AnalyzerRules,
    ) -> AnalyzerRules {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                if let Some(rules) = pattern.linter.rules.as_ref() {
                    push_to_analyzer_rules(
                        rules,
                        biome_js_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_rules(
                        rules,
                        biome_json_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_rules(
                        rules,
                        biome_css_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_rules(
                        rules,
                        biome_graphql_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                }

                if let Some(actions) = pattern.assist.actions.as_ref() {
                    push_to_analyzer_assist(
                        actions,
                        biome_js_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_assist(
                        actions,
                        biome_json_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_assist(
                        actions,
                        biome_css_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                    push_to_analyzer_assist(
                        actions,
                        biome_graphql_analyze::METADATA.deref(),
                        &mut analyzer_rules,
                    );
                }
            }
        }
        analyzer_rules
    }
}

#[derive(Clone, Debug, Default)]
pub struct OverrideSettingPattern {
    includes: OverrideIncludes,
    /// Formatter settings applied to all files in the workspaces
    pub formatter: OverrideFormatSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: OverrideLinterSettings,
    /// Assist settings applied to all files in the workspace
    pub assist: OverrideAssistSettings,
    /// Language specific settings
    pub languages: LanguageListSettings,
}

impl OverrideSettingPattern {
    /// Returns `true` if this override settings concerns `file_path`.
    ///
    /// Note that only path to regular files should be passed.
    /// This function doesn't take directories into account.
    pub fn is_file_included(&self, file_path: &Utf8Path) -> bool {
        !self.includes.is_unset() && self.includes.matches(file_path)
    }

    fn apply_overrides_to_js_format_options(&self, options: &mut JsFormatOptions) {
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
        if let Some(trailing_commas) = js_formatter.trailing_commas {
            options.set_trailing_commas(trailing_commas);
        }
        if let Some(semicolons) = js_formatter.semicolons {
            options.set_semicolons(semicolons);
        }
        if let Some(arrow_parentheses) = js_formatter.arrow_parentheses {
            options.set_arrow_parentheses(arrow_parentheses);
        }
        if let Some(bracket_spacing) = js_formatter.bracket_spacing.or(formatter.bracket_spacing) {
            options.set_bracket_spacing(bracket_spacing);
        }
        if let Some(bracket_same_line) = js_formatter.bracket_same_line {
            options.set_bracket_same_line(bracket_same_line);
        }
        if let Some(attribute_position) = js_formatter
            .attribute_position
            .or(formatter.attribute_position)
        {
            options.set_attribute_position(attribute_position);
        }
    }

    fn apply_overrides_to_json_format_options(&self, options: &mut JsonFormatOptions) {
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
        if let Some(expand_lists) = json_formatter.expand {
            options.set_expand(expand_lists);
        }
    }

    fn apply_overrides_to_css_format_options(&self, options: &mut CssFormatOptions) {
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
    }

    fn apply_overrides_to_graphql_format_options(&self, options: &mut GraphqlFormatOptions) {
        let graphql_formatter = &self.languages.graphql.formatter;
        let formatter = &self.formatter;

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
        if let Some(quote_style) = graphql_formatter.quote_style {
            options.set_quote_style(quote_style);
        }
    }

    fn apply_overrides_to_grit_format_options(&self, options: &mut GritFormatOptions) {
        let grit_formatter = &self.languages.grit.formatter;
        let formatter = &self.formatter;

        if let Some(indent_style) = grit_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = grit_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width)
        }
        if let Some(line_ending) = grit_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = grit_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }
    }

    fn apply_overrides_to_html_format_options(&self, options: &mut HtmlFormatOptions) {
        let html_formatter = &self.languages.html.formatter;
        let formatter = &self.formatter;

        if let Some(indent_style) = html_formatter.indent_style.or(formatter.indent_style) {
            options.set_indent_style(indent_style);
        }
        if let Some(indent_width) = html_formatter.indent_width.or(formatter.indent_width) {
            options.set_indent_width(indent_width)
        }
        if let Some(line_ending) = html_formatter.line_ending.or(formatter.line_ending) {
            options.set_line_ending(line_ending);
        }
        if let Some(line_width) = html_formatter.line_width.or(formatter.line_width) {
            options.set_line_width(line_width);
        }
    }

    fn apply_overrides_to_js_parser_options(&self, options: &mut JsParserOptions) {
        let js_parser = &self.languages.javascript.parser;

        options.parse_class_parameter_decorators = js_parser
            .parse_class_parameter_decorators
            .unwrap_or_default()
            .into();
    }

    fn apply_overrides_to_json_parser_options(&self, options: &mut JsonParserOptions) {
        // these options are no longer cached because it was causing incorrect override behavior, see #3260
        let json_parser = &self.languages.json.parser;

        if let Some(allow_comments) = json_parser.allow_comments {
            options.allow_comments = allow_comments.value();
        }
        if let Some(allow_trailing_commas) = json_parser.allow_trailing_commas {
            options.allow_trailing_commas = allow_trailing_commas.value();
        }
    }

    fn apply_overrides_to_css_parser_options(&self, options: &mut CssParserOptions) {
        let css_parser = &self.languages.css.parser;

        if let Some(allow_wrong_line_comments) = css_parser.allow_wrong_line_comments {
            options.allow_wrong_line_comments = allow_wrong_line_comments.value();
        }
        if let Some(css_modules) = css_parser.css_modules_enabled {
            options.css_modules = css_modules.value();
        }
    }

    #[expect(dead_code)]
    // NOTE: Currently not used because the rule options are typed using TypeId and Any, which isn't thread safe.
    // TODO: Find a way to cache this
    fn analyzer_rules_mut(&self, _analyzer_rules: &mut AnalyzerRules) {}
}

fn to_git_ignore(path: Utf8PathBuf, matches: &[String]) -> Result<Gitignore, WorkspaceError> {
    let mut gitignore_builder = GitignoreBuilder::new(path.clone());

    for the_match in matches {
        gitignore_builder
            .add_line(Some(path.clone().into_std_path_buf()), the_match)
            .map_err(|err| {
                BiomeDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
                    message: err.to_string(),
                    file_path: Some(path.to_string()),
                })
            })?;
    }
    let gitignore = gitignore_builder.build().map_err(|err| {
        BiomeDiagnostic::InvalidIgnorePattern(InvalidIgnorePattern {
            message: err.to_string(),
            file_path: Some(path.to_string()),
        })
    })?;
    Ok(gitignore)
}

pub fn to_override_settings(
    working_directory: Option<Utf8PathBuf>,
    overrides: Overrides,
    current_settings: &Settings,
) -> Result<OverrideSettings, WorkspaceError> {
    let mut override_settings = OverrideSettings::default();
    for mut pattern in overrides.0 {
        let formatter = pattern
            .formatter
            .map(|formatter| OverrideFormatSettings {
                enabled: formatter.enabled,
                format_with_errors: formatter
                    .format_with_errors
                    .or(current_settings.formatter.format_with_errors),
                indent_style: formatter.indent_style,
                indent_width: formatter.indent_width,
                line_ending: formatter.line_ending,
                line_width: formatter.line_width,
                bracket_spacing: formatter.bracket_spacing,
                bracket_same_line: formatter.bracket_same_line,
                attribute_position: formatter.attribute_position,
                object_wrap: formatter.object_wrap,
            })
            .unwrap_or_default();
        let linter = pattern
            .linter
            .map(|linter| OverrideLinterSettings {
                enabled: linter.enabled,
                rules: linter.rules,
                domains: linter.domains,
            })
            .unwrap_or_default();
        let assist = pattern
            .assist
            .map(|assist| OverrideAssistSettings {
                enabled: assist.enabled,
                actions: assist.actions,
            })
            .unwrap_or_default();

        let mut languages = LanguageListSettings::default();
        let javascript = pattern.javascript.take().unwrap_or_default();
        let json = pattern.json.take().unwrap_or_default();
        let css = pattern.css.take().unwrap_or_default();
        let graphql = pattern.graphql.take().unwrap_or_default();
        let grit = pattern.grit.take().unwrap_or_default();
        let html = pattern.html.take().unwrap_or_default();

        languages.javascript =
            to_javascript_language_settings(javascript, &current_settings.languages.javascript);

        languages.json = to_json_language_settings(json, &current_settings.languages.json);
        languages.css = to_css_language_settings(css, &current_settings.languages.css);
        languages.graphql =
            to_graphql_language_settings(graphql, &current_settings.languages.graphql);
        languages.grit = to_grit_language_settings(grit, &current_settings.languages.grit);
        languages.html = to_html_language_settings(html, &current_settings.languages.html);

        let pattern_setting = OverrideSettingPattern {
            includes: OverrideIncludes::new(working_directory.clone(), pattern.includes),
            formatter,
            linter,
            assist,
            languages,
        };

        override_settings.patterns.push(pattern_setting);
    }

    Ok(override_settings)
}

fn to_javascript_language_settings(
    mut conf: JsConfiguration,
    parent_settings: &LanguageSettings<JsLanguage>,
) -> LanguageSettings<JsLanguage> {
    let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    let linter = conf.linter.take().unwrap_or_default();
    language_setting.linter.enabled = linter.enabled;

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.parse_class_parameter_decorators = parser
        .unsafe_parameter_decorators_enabled
        .or(parent_parser.parse_class_parameter_decorators);

    language_setting.globals = conf.globals;
    language_setting.environment.jsx_runtime =
        conf.jsx_runtime.or(parent_settings.environment.jsx_runtime);

    language_setting
}

fn to_json_language_settings(
    mut conf: JsonConfiguration,
    parent_settings: &LanguageSettings<JsonLanguage>,
) -> LanguageSettings<JsonLanguage> {
    let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_comments = parser.allow_comments.or(parent_parser.allow_comments);

    language_setting.parser.allow_trailing_commas = parser
        .allow_trailing_commas
        .or(parent_parser.allow_trailing_commas);

    language_setting
}

fn to_css_language_settings(
    mut conf: CssConfiguration,
    parent_settings: &LanguageSettings<CssLanguage>,
) -> LanguageSettings<CssLanguage> {
    let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_wrong_line_comments = parser
        .allow_wrong_line_comments
        .or(parent_parser.allow_wrong_line_comments);
    language_setting.parser.css_modules_enabled =
        parser.css_modules.or(parent_parser.css_modules_enabled);

    language_setting
}

fn to_graphql_language_settings(
    mut conf: GraphqlConfiguration,
    _parent_settings: &LanguageSettings<GraphqlLanguage>,
) -> LanguageSettings<GraphqlLanguage> {
    let mut language_setting: LanguageSettings<GraphqlLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    language_setting
}

fn to_grit_language_settings(
    mut conf: GritConfiguration,
    _parent_settings: &LanguageSettings<GritLanguage>,
) -> LanguageSettings<GritLanguage> {
    let mut language_setting: LanguageSettings<GritLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    language_setting
}
fn to_html_language_settings(
    mut conf: HtmlConfiguration,
    _parent_settings: &LanguageSettings<HtmlLanguage>,
) -> LanguageSettings<HtmlLanguage> {
    let mut language_setting: LanguageSettings<HtmlLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    language_setting
}

pub fn to_format_settings(
    working_directory: Option<Utf8PathBuf>,
    conf: FormatterConfiguration,
) -> Result<FormatSettings, WorkspaceError> {
    Ok(FormatSettings {
        enabled: conf.enabled,
        indent_style: conf.indent_style,
        indent_width: conf.indent_width,
        line_ending: conf.line_ending,
        line_width: conf.line_width,
        format_with_errors: conf.format_with_errors,
        attribute_position: conf.attribute_position,
        bracket_same_line: conf.bracket_same_line,
        bracket_spacing: conf.bracket_spacing,
        object_wrap: conf.object_wrap,
        includes: Includes::new(working_directory, conf.includes),
    })
}

impl TryFrom<OverrideFormatterConfiguration> for FormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideFormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            Some(IndentStyle::Tab) => IndentStyle::Tab,
            Some(IndentStyle::Space) => IndentStyle::Space,
            None => IndentStyle::default(),
        };
        let indent_width = conf.indent_width.map(Into::into).unwrap_or_default();

        Ok(Self {
            enabled: conf.enabled,
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            attribute_position: Some(AttributePosition::default()),
            bracket_same_line: conf.bracket_same_line,
            bracket_spacing: Some(BracketSpacing::default()),
            object_wrap: conf.object_wrap,
            format_with_errors: conf.format_with_errors,
            includes: Default::default(),
        })
    }
}

pub fn to_linter_settings(
    working_directory: Option<Utf8PathBuf>,
    conf: LinterConfiguration,
) -> Result<LinterSettings, WorkspaceError> {
    Ok(LinterSettings {
        enabled: conf.enabled,
        rules: conf.rules,
        includes: Includes::new(working_directory, conf.includes),
        domains: conf.domains,
    })
}

impl TryFrom<OverrideLinterConfiguration> for LinterSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideLinterConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled,
            rules: conf.rules,
            includes: Default::default(),
            domains: conf.domains,
        })
    }
}

pub fn to_assist_settings(
    working_directory: Option<Utf8PathBuf>,
    conf: AssistConfiguration,
) -> Result<AssistSettings, WorkspaceError> {
    Ok(AssistSettings {
        enabled: conf.enabled,
        actions: conf.actions,
        includes: Includes::new(working_directory, conf.includes),
    })
}

impl TryFrom<OverrideAssistConfiguration> for AssistSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideAssistConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled,
            actions: conf.actions,
            includes: Default::default(),
        })
    }
}

/// Checks the feature activity according to language-specific
/// and top level feature activities.
///
/// | Top-Level \ Language | Some(true) | Some(false) | None       |
/// |:--------------------:|:----------:|:-----------:|:----------:|
/// | Some(true)           | Some(true) | Some(false) | None       |
/// | Some(false)          | Some(true) | Some(false) | Some(false)|
/// | None                 | Some(true) | Some(false) | None       |
pub(crate) fn check_feature_activity<const LANG: bool, const TOP: bool>(
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

/// Checks the feature activity according to language-specific
/// and top level feature activities for the overrides.
///
/// | Top-Level \ Language  | Some(true) | Some(false) | None       |
/// |:---------------------:|:----------:|:-----------:|:----------:|
/// | Some(true)            | Some(true) | Some(false) | Some(true) |
/// | Some(false)           | Some(true) | Some(false) | Some(false)|
/// | None                  | Some(true) | Some(false) | None       |
pub(crate) fn check_override_feature_activity<const LANG: bool, const TOP: bool>(
    language_specific_feature_activity: Option<Bool<LANG>>,
    top_level_feature_activity: Option<Bool<TOP>>,
) -> Option<Bool<LANG>> {
    // Check the language-specific feature first
    language_specific_feature_activity
        // Then check the top level feature
        .or(top_level_feature_activity.map(|v| v.value().into()))
}

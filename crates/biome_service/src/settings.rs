use crate::workspace::{DocumentFileSource, FeatureKind};
use crate::{WorkspaceError, is_dir};
use biome_analyze::{AnalyzerOptions, AnalyzerRules};
use biome_configuration::analyzer::assist::{Actions, AssistConfiguration, AssistEnabled};
use biome_configuration::analyzer::{LinterEnabled, RuleDomains};
use biome_configuration::bool::Bool;
use biome_configuration::diagnostics::InvalidIgnorePattern;
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::html::{ExperimentalFullSupportEnabled, HtmlConfiguration};
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::max_size::MaxSize;
use biome_configuration::vcs::{VcsClientKind, VcsConfiguration, VcsEnabled, VcsUseIgnoreFile};
use biome_configuration::{
    BiomeDiagnostic, Configuration, CssConfiguration, DEFAULT_SCANNER_IGNORE_ENTRIES,
    FilesConfiguration, FilesIgnoreUnknownEnabled, FormatterConfiguration, GraphqlConfiguration,
    GritConfiguration, JsConfiguration, JsonConfiguration, LinterConfiguration,
    OverrideAssistConfiguration, OverrideFormatterConfiguration, OverrideGlobs,
    OverrideLinterConfiguration, Overrides, Rules, push_to_analyzer_assist, push_to_analyzer_rules,
};
use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssLanguage;
use biome_deserialize::Merge;
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, IndentStyle, IndentWidth,
    LineEnding, LineWidth,
};
use biome_fs::BiomePath;
use biome_graphql_formatter::context::GraphqlFormatOptions;
use biome_graphql_syntax::GraphqlLanguage;
use biome_grit_formatter::context::GritFormatOptions;
use biome_grit_syntax::GritLanguage;
use biome_html_formatter::HtmlFormatOptions;
use biome_html_parser::HtmlParseOptions;
use biome_html_syntax::HtmlLanguage;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsLanguage;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_plugin_loader::Plugins;
use camino::{Utf8Path, Utf8PathBuf};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;
use tracing::instrument;

/// Settings active in a project.
///
/// These can be either root settings, or settings for a section of the project.
#[derive(Clone, Debug, Default)]
pub struct Settings {
    /// The configuration that originated this setting, if applicable.
    ///
    /// It contains [Configuration] and the folder where it was found.
    source: Option<Arc<(Configuration, Option<Utf8PathBuf>)>>,

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
    /// The VCS settings of the project
    pub vcs_settings: VcsSettings,

    // TODO: remove once HTML full support is stable
    pub experimental_full_html_support: Option<ExperimentalFullSupportEnabled>,
}

impl Settings {
    pub fn experimental_full_html_support_enabled(&self) -> bool {
        self.experimental_full_html_support
            .unwrap_or_default()
            .value()
    }

    pub fn source(&self) -> Option<Configuration> {
        self.source.as_ref().map(|source| {
            let (config, _) = source.deref().clone();
            config
        })
    }

    pub fn source_path(&self) -> Option<Utf8PathBuf> {
        self.source.as_ref().and_then(|source| {
            let (_, path) = source.deref().clone();
            path
        })
    }

    /// Merges the [Configuration] into the settings.
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn merge_with_configuration(
        &mut self,
        configuration: Configuration,
        working_directory: Option<Utf8PathBuf>,
    ) -> Result<(), WorkspaceError> {
        self.source = Some(Arc::new((configuration.clone(), working_directory.clone())));

        // formatter part§
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
        if let Some(files) = configuration.files {
            self.files = to_file_settings(working_directory.clone(), files)?;
        }

        // VCS settings
        if let Some(vcs) = configuration.vcs {
            self.vcs_settings = to_vcs_settings(vcs)?;
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
            self.experimental_full_html_support = html.experimental_full_support_enabled;
            self.languages.html = html.into();
        }

        // plugin settings
        if let Some(plugins) = configuration.plugins {
            self.plugins = plugins;
        }

        // NOTE: keep this last. Computing the overrides require reading the settings computed by the parent settings.
        if let Some(overrides) = configuration.overrides {
            self.override_settings = to_override_settings(working_directory, overrides, self)?;
        }

        Ok(())
    }

    /// Retrieves the settings of the formatter
    pub fn formatter(&self) -> &FormatSettings {
        &self.formatter
    }

    /// Whether the files ignore_unknown is enabled
    #[inline]
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
    pub fn as_linter_rules(&self, path: &Utf8Path) -> Option<Cow<'_, Rules>> {
        let mut result = self.linter.rules.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.linter.rules.as_ref();
            if let Some(pattern_rules) = pattern_rules
                && pattern.is_file_included(path)
            {
                result = if let Some(mut result) = result.take() {
                    // Override rules
                    result.to_mut().merge_with(pattern_rules.clone());
                    Some(result)
                } else {
                    Some(Cow::Borrowed(pattern_rules))
                };
            }
        }
        result
    }

    /// Extract the domains applied to the given `path`, by looking that the base `domains`, and the once applied by `overrides`
    pub fn as_linter_domains(&self, path: &Utf8Path) -> Option<Cow<'_, RuleDomains>> {
        let mut result = self.linter.domains.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.linter.domains.as_ref();
            if let Some(pattern_rules) = pattern_rules
                && pattern.is_file_included(path)
            {
                result = if let Some(mut result) = result.take() {
                    // Override rules
                    result.to_mut().merge_with(pattern_rules.clone());
                    Some(result)
                } else {
                    Some(Cow::Borrowed(pattern_rules))
                };
            }
        }

        result
    }

    /// Returns assists rules taking overrides into account.
    pub fn as_assist_actions(&self, path: &Utf8Path) -> Option<Cow<'_, Actions>> {
        let mut result = self.assist.actions.as_ref().map(Cow::Borrowed);
        let overrides = &self.override_settings;
        for pattern in overrides.patterns.iter() {
            let pattern_rules = pattern.assist.actions.as_ref();
            if let Some(pattern_rules) = pattern_rules
                && pattern.is_file_included(path)
            {
                result = if let Some(mut result) = result.take() {
                    // Override rules
                    result.to_mut().merge_with(pattern_rules.clone());
                    Some(result)
                } else {
                    Some(Cow::Borrowed(pattern_rules))
                };
            }
        }
        result
    }

    /// Returns the plugins that should be enabled for the given `path`, taking overrides into account.
    pub fn get_plugins_for_path(&self, path: &Utf8Path) -> Cow<'_, Plugins> {
        let mut result = Cow::Borrowed(&self.plugins);

        for pattern in &self.override_settings.patterns {
            if pattern.is_file_included(path) {
                result.to_mut().extend_from_slice(&pattern.plugins);
            }
        }

        result
    }

    /// Return all plugins configured in setting
    pub fn as_all_plugins(&self) -> Cow<'_, Plugins> {
        let mut result = Cow::Borrowed(&self.plugins);

        let all_override_plugins = self
            .override_settings
            .patterns
            .iter()
            .flat_map(|pattern| pattern.plugins.iter().cloned())
            .collect::<Vec<_>>();

        if !all_override_plugins.is_empty() {
            result.to_mut().0.extend(all_override_plugins);
        }

        result
    }

    pub fn is_formatter_enabled(&self) -> bool {
        self.formatter.is_enabled()
    }

    pub fn is_linter_enabled(&self) -> bool {
        self.linter.is_enabled()
    }

    pub fn is_vcs_enabled(&self) -> bool {
        self.vcs_settings.is_enabled()
    }

    pub fn linter_recommended_enabled(&self) -> bool {
        self.linter.recommended_enabled()
    }

    pub fn is_assist_enabled(&self) -> bool {
        self.assist.is_enabled()
    }

    /// Returns whether the given `path` is ignored for the given `feature`,
    /// based on the current settings.
    ///
    /// `path` is expected to point to a file and not a directory.
    #[inline]
    pub fn is_path_ignored_for_feature(&self, path: &Utf8Path, feature: FeatureKind) -> bool {
        let feature_includes_files = match feature {
            FeatureKind::Format => &self.formatter.includes,
            FeatureKind::Lint => &self.linter.includes,
            FeatureKind::Assist => &self.assist.includes,
            FeatureKind::HtmlFullSupport => return false,
            FeatureKind::Search => return false, // There is no search-specific config.
            FeatureKind::Debug => return false,
        };

        if is_dir(path) {
            !feature_includes_files.is_dir_included(path)
        } else {
            !feature_includes_files.is_file_included(path)
        }
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
    pub expand: Option<Expand>,
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
    pub expand: Option<Expand>,
}

impl From<OverrideFormatterConfiguration> for OverrideFormatSettings {
    fn from(conf: OverrideFormatterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            format_with_errors: conf.format_with_errors,
            indent_style: conf.indent_style,
            indent_width: conf.indent_width,
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            bracket_spacing: conf.bracket_spacing,
            bracket_same_line: conf.bracket_same_line,
            attribute_position: conf.attribute_position,
            expand: conf.expand,
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
    pub domains: Option<RuleDomains>,
}

impl LinterSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn recommended_enabled(&self) -> bool {
        self.rules
            .as_ref()
            .and_then(|rules| rules.recommended)
            // If there isn't a clear value, we default to true
            .unwrap_or(true)
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
    pub domains: Option<RuleDomains>,
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

/// Files settings for the entire workspace
#[derive(Clone, Debug, Default)]
pub struct OverrideFilesSettings {
    pub max_size: Option<MaxSize>,
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
        let mut language_setting: Self = Self::default();

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
        let mut language_setting: Self = Self::default();

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
        let mut language_setting: Self = Self::default();

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
        let mut language_setting: Self = Self::default();

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
        let mut language_setting: Self = Self::default();
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
        let mut language_setting: Self = Self::default();
        if let Some(formatter) = html.formatter {
            language_setting.formatter = formatter.into();
        }

        if let Some(parser) = html.parser {
            language_setting.parser = parser.into();
        }

        if let Some(linter) = html.linter {
            language_setting.linter = linter.into();
        }

        if let Some(assist) = html.assist {
            language_setting.assist = assist.into();
        }

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

    type ParserOptions: Default;

    /// Settings related to the environment/runtime in which the language is used.
    type EnvironmentSettings: Default;

    /// Read the settings type for this language from the [LanguageListSettings] map
    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self>;

    /// Retrieve the environment settings of the current language
    fn resolve_environment(settings: &Settings) -> Option<&Self::EnvironmentSettings>;

    /// Retrieve the parser options that belong to this language
    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::ParserOptions;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::FormatOptions;

    /// Resolve the linter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_analyzer_options(
        global: &Settings,
        language: &Self::LinterSettings,
        environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions;

    /// Checks whether this file has the linter enabled.
    ///
    /// The language is responsible for checking this.
    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool;

    /// Responsible to check whether this file has formatter enabled. The language is responsible to check this
    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool;

    /// Responsible to check whether this file has assist enabled. The language is responsible to check this
    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool;
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

    /// List of included paths/files
    pub includes: Includes,

    /// Files not recognized by Biome should not emit a diagnostic
    pub ignore_unknown: Option<FilesIgnoreUnknownEnabled>,

    /// List of file and folder names that should be unconditionally ignored by
    /// the scanner.
    pub scanner_ignore_entries: Vec<Vec<u8>>,
}

#[derive(Clone, Default, Debug)]
pub struct VcsSettings {
    pub client_kind: Option<VcsClientKind>,
    pub root: Option<Utf8PathBuf>,
    pub use_ignore_file: Option<VcsUseIgnoreFile>,
    pub enabled: Option<VcsEnabled>,

    pub ignore_matches: Option<VcsIgnoredPatterns>,
}

impl VcsSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    /// Returns whether the given `path` should be ignored per the VCS settings.
    #[inline]
    pub fn is_ignored(&self, path: &Utf8Path, root_path: Option<&Utf8Path>) -> bool {
        self.should_use_ignore_file()
            && self.ignore_matches.as_ref().is_some_and(|ignored_matches| {
                ignored_matches.is_ignored(path, is_dir(path), root_path)
            })
    }

    #[inline]
    pub fn should_use_ignore_file(&self) -> bool {
        self.use_ignore_file.unwrap_or_default().into()
    }

    pub fn to_base_path(&self, base_path: Option<&Utf8Path>) -> Option<Utf8PathBuf> {
        Some(match (base_path, &self.root) {
            (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
            (None, Some(root)) => Utf8PathBuf::from(root),
            (Some(vcs_base_path), None) => Utf8PathBuf::from(vcs_base_path),
            (None, None) => return None,
        })
    }

    /// Checks where if the current file is a recognised file for the current VCS client
    pub fn is_ignore_file(&self, path: &Utf8Path) -> bool {
        path.file_name().is_some_and(|file_name| {
            self.client_kind
                .is_some_and(|client_kind| client_kind.ignore_files().contains(&file_name))
        })
    }

    /// Stores the patterns of the root ignore file
    pub fn store_root_ignore_patterns(
        &mut self,
        path: &Utf8Path,
        patterns: &[&str],
    ) -> Result<(), WorkspaceError> {
        match self.client_kind {
            Some(VcsClientKind::Git) => {
                let git_ignore = VcsIgnoredPatterns::git_ignore(path, patterns)?;
                self.ignore_matches = Some(VcsIgnoredPatterns::Git {
                    root: git_ignore,
                    nested: vec![],
                });
            }
            None => {}
        };

        Ok(())
    }

    /// Stores a list of patterns inside as a nested ignore file
    pub fn store_nested_ignore_patterns(
        &mut self,
        path: &Utf8Path,
        patterns: &[&str],
    ) -> Result<(), WorkspaceError> {
        match self.client_kind {
            Some(VcsClientKind::Git) => {
                let git_ignore = VcsIgnoredPatterns::git_ignore(path, patterns)?;
                if let Some(ignore_matches) = self.ignore_matches.as_mut() {
                    ignore_matches.insert_git_match(git_ignore);
                }
            }
            None => {}
        };

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum VcsIgnoredPatterns {
    Git {
        /// Represents the `.gitignore` file at the root of the project
        root: Gitignore,
        /// The list of nested `.gitignore` files found inside the project
        nested: Vec<Gitignore>,
    },
}

impl VcsIgnoredPatterns {
    /// Checks whether the path ignored by any ignore file found inside the project
    ///
    /// The `root_path` represents the root of the project, as we want to match all ignore files untile the root.
    pub fn is_ignored(&self, path: &Utf8Path, is_dir: bool, root_path: Option<&Utf8Path>) -> bool {
        match self {
            Self::Git { root, nested, .. } => {
                match root_path {
                    None => Self::is_git_ignore(root, nested.as_slice(), path, is_dir),
                    Some(root_path) => {
                        // NOTE: this could be a bug of the library, need to explore. Let's assume it isn't
                        // When crawling the file system with the CLI, we correctly exclude ignored folders
                        // such as `dist/` or `build/`, in case the path to match is `/Users/foo/project/dist`
                        //
                        // However, the LSP sends absolute file paths, e.g. `/Users/foo/project/dist/a.min.js`,
                        // and they **don't** match globs such as `dist/`.
                        // To work around this limitation, we crawl upwards the parents of the path, until
                        // we arrive at the `root_path`.
                        let mut current_path = path;
                        loop {
                            if current_path == root_path {
                                break false;
                            }
                            if Self::is_git_ignore(
                                root,
                                nested.as_slice(),
                                current_path,
                                current_path.is_dir(),
                            ) {
                                break true;
                            }
                            if let Some(parent) = current_path.parent() {
                                current_path = parent;
                            } else {
                                break false;
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_git_ignore(
        root: &Gitignore,
        nested: &[Gitignore],
        path: &Utf8Path,
        is_dir: bool,
    ) -> bool {
        let root_ignored = {
            let path = path.strip_prefix(root.path()).unwrap_or(path);
            root.matched(path, is_dir).is_ignore()
        };

        let nested_ignored = nested.iter().any(|gitignore| {
            if let Ok(stripped_path) = path.strip_prefix(gitignore.path()) {
                gitignore.matched(stripped_path, is_dir).is_ignore()
            } else {
                false
            }
        });

        root_ignored || nested_ignored
    }

    pub fn insert_git_match(&mut self, git_ignore: Gitignore) {
        match self {
            Self::Git { nested, .. } => {
                nested.push(git_ignore);
            }
        }
    }

    /// Creates an instance of [Gitignore] for the given patterns.
    ///
    /// ## Error
    ///
    /// If the patterns are invalid
    fn git_ignore(path: &Utf8Path, patterns: &[&str]) -> Result<Gitignore, WorkspaceError> {
        let mut gitignore_builder = GitignoreBuilder::new(path.as_std_path());

        for the_match in patterns {
            gitignore_builder
                .add_line(Some(path.to_path_buf().into_std_path_buf()), the_match)
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
}

/// An optional list of globs with exceptions that first normalizes the tested paths before matching them against the globs.
#[derive(Clone, Default, Debug)]
pub struct Includes {
    /// This path is used to normalize the tested paths against [Self::globs].
    working_directory: Option<Utf8PathBuf>,
    /// If `None`, then all files are included
    /// Otherwise this filtered out all files that doesn't match.
    globs: Option<Vec<biome_glob::NormalizedGlob>>,
}
impl Includes {
    fn new(
        working_directory: Option<Utf8PathBuf>,
        globs: Option<Vec<biome_glob::NormalizedGlob>>,
    ) -> Self {
        Self {
            working_directory,
            globs,
        }
    }

    pub fn store_globs(&mut self, globs: impl Into<Box<[biome_glob::NormalizedGlob]>>) {
        let current_globs = self.globs.get_or_insert_default();
        current_globs.extend(globs.into());
    }

    /// Returns whether the given `file_path` is included.
    ///
    /// `file_path` must point to an ordinary file. If it is a directory, you
    /// should use [Self::is_dir_included()] instead.
    #[inline]
    pub fn is_file_included(&self, file_path: &Utf8Path) -> bool {
        self.is_unset() || self.matches_with_exceptions(file_path)
    }

    /// Returns whether the given `dir_path` is included.
    ///
    /// `file_path` must point to a directory. If it is a file, you should use
    /// [Self::is_file_included()] instead.
    #[inline]
    pub fn is_dir_included(&self, dir_path: &Utf8Path) -> bool {
        self.is_unset() || self.matches_directory_with_exceptions(dir_path)
    }

    /// Returns whether the given `path` is force-ignored.
    #[inline]
    pub fn is_force_ignored(&self, path: &Utf8Path) -> bool {
        let Some(globs) = self.globs.as_ref() else {
            return false;
        };
        let path = if let Some(working_directory) = &self.working_directory {
            path.strip_prefix(working_directory).unwrap_or(path)
        } else {
            path
        };
        let candidate_path = biome_glob::CandidatePath::new(path);
        candidate_path.matches_forced_negation(globs)
    }

    /// Returns `true` is no globs are set.
    #[inline]
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
    config: FilesConfiguration,
) -> Result<FilesSettings, WorkspaceError> {
    Ok(FilesSettings {
        max_size: config.max_size,
        includes: Includes::new(working_directory, config.includes),
        ignore_unknown: config.ignore_unknown,
        scanner_ignore_entries: config.experimental_scanner_ignores.map_or_else(
            || {
                DEFAULT_SCANNER_IGNORE_ENTRIES
                    .iter()
                    .map(|entry| entry.to_vec())
                    .collect()
            },
            |entries| entries.into_iter().map(String::into_bytes).collect(),
        ),
    })
}

fn to_vcs_settings(config: VcsConfiguration) -> Result<VcsSettings, WorkspaceError> {
    Ok(VcsSettings {
        client_kind: config.client_kind,
        enabled: config.enabled,
        root: config.root.map(Utf8PathBuf::from),
        use_ignore_file: config.use_ignore_file,
        ignore_matches: None,
    })
}

impl Settings {
    /// Resolve the formatting context for the given language
    #[instrument(level = "debug", skip(self, file_source))]
    pub fn format_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> L::FormatOptions
    where
        L: ServiceLanguage,
    {
        let formatter = &self.formatter;
        let overrides = &self.override_settings;
        let editor_settings = &L::lookup_settings(&self.languages).formatter;
        L::resolve_format_options(formatter, overrides, editor_settings, path, file_source)
    }

    pub fn parse_options<L>(
        &self,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> L::ParserOptions
    where
        L: ServiceLanguage,
    {
        let overrides = &self.override_settings;
        let editor_settings = &L::lookup_settings(&self.languages).parser;
        L::resolve_parse_options(overrides, editor_settings, path, file_source)
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
        let linter_settings = &L::lookup_settings(&self.languages).linter;

        let environment = L::resolve_environment(self);
        L::resolve_analyzer_options(
            self,
            linter_settings,
            environment,
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
        L::linter_enabled_for_file_path(self, path)
    }

    /// Whether the formatter is enabled for this file path
    pub fn formatter_enabled_for_file_path<L>(&self, path: &Utf8Path) -> bool
    where
        L: ServiceLanguage,
    {
        L::formatter_enabled_for_file_path(self, path)
    }

    /// Whether the assist is enabled for this file path
    pub fn assist_enabled_for_file_path<L>(&self, path: &Utf8Path) -> bool
    where
        L: ServiceLanguage,
    {
        L::assist_enabled_for_file_path(self, path)
    }

    /// Whether the formatter should format with parsing errors, for this file path
    pub fn format_with_errors_enabled_for_this_file_path(&self, path: &Utf8Path) -> bool {
        self.override_settings
            .patterns
            .iter()
            .rev()
            .find_map(|pattern| {
                if let Some(enabled) = pattern.formatter.format_with_errors
                    && pattern.is_file_included(path)
                {
                    return Some(enabled);
                }
                None
            })
            .or(self.formatter.format_with_errors)
            .unwrap_or_default()
            .into()
    }

    /// Returns the maximum file size setting for the given `file_path`.
    pub fn get_max_file_size(&self, file_path: &Utf8Path) -> usize {
        let limit = self
            .override_settings
            .patterns
            .iter()
            .rev()
            .find_map(|pattern| {
                if pattern.is_file_included(file_path) {
                    pattern.files.max_size
                } else {
                    None
                }
            })
            .or(self.files.max_size)
            .unwrap_or_default();

        usize::from(limit)
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

    pub fn apply_override_grit_format_options(
        &self,
        path: &Utf8Path,
        options: &mut GritFormatOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_grit_format_options(options);
            }
        }
    }

    pub fn apply_override_html_format_options(
        &self,
        path: &Utf8Path,
        options: &mut HtmlFormatOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_html_format_options(options);
            }
        }
    }

    pub fn apply_override_js_parser_options(&self, path: &Utf8Path, options: &mut JsParserOptions) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_js_parser_options(options);
            }
        }
    }

    pub fn apply_override_json_parser_options(
        &self,
        path: &Utf8Path,
        options: &mut JsonParserOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_parser_options(options);
            }
        }
    }

    pub(crate) fn apply_override_html_parser_options(
        &self,
        path: &Utf8Path,
        options: &mut HtmlParseOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_html_parser_options(options);
            }
        }
    }

    /// Scans the override rules and returns the parser options of the first matching override.
    pub fn apply_override_css_parser_options(
        &self,
        path: &Utf8Path,
        options: &mut CssParserOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_css_parser_options(options);
            }
        }
    }

    /// Scans and aggregates all the overrides into a single [CssFormatOptions]
    pub fn apply_override_css_format_options(
        &self,
        path: &Utf8Path,
        options: &mut CssFormatOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_css_format_options(options);
            }
        }
    }

    /// Scans and aggregates all the overrides into a single [JsonParserOptions]
    pub fn apply_override_json_parse_options(
        &self,
        path: &Utf8Path,
        options: &mut JsonParserOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_parser_options(options);
            }
        }
    }

    /// Scans and aggregates all the overrides into a single `JsonFormatOptions`
    pub fn apply_override_json_format_options(
        &self,
        path: &Utf8Path,
        options: &mut JsonFormatOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_json_format_options(options);
            }
        }
    }

    /// Scans and aggregates all the overrides into a single [GraphqlFormatOptions]
    pub fn apply_override_graphql_format_options(
        &self,
        path: &Utf8Path,
        options: &mut GraphqlFormatOptions,
    ) {
        for pattern in self.patterns.iter() {
            if pattern.is_file_included(path) {
                pattern.apply_overrides_to_graphql_format_options(options);
            }
        }
    }

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
    /// Files specific settings
    pub files: OverrideFilesSettings,
    /// Additional plugins to be applied
    pub plugins: Plugins,
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
        if let Some(expand) = js_formatter.expand.or(formatter.expand) {
            options.set_expand(expand);
        }
        if let Some(attribute_position) = js_formatter
            .attribute_position
            .or(formatter.attribute_position)
        {
            options.set_attribute_position(attribute_position);
        }
        if let Some(operator_line_break) = js_formatter.operator_linebreak {
            options.set_operator_linebreak(operator_line_break);
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
        if let Some(expand_lists) = json_formatter.expand.or(formatter.expand) {
            options.set_expand(expand_lists);
        }
        if let Some(bracket_spacing) = json_formatter.bracket_spacing.or(formatter.bracket_spacing)
        {
            options.set_bracket_spacing(bracket_spacing);
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

        // #region global formatter options
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

        if let Some(bracket_same_line) = html_formatter
            .bracket_same_line
            .or(formatter.bracket_same_line)
        {
            options.set_bracket_same_line(bracket_same_line);
        }

        if let Some(attribute_position) = html_formatter
            .attribute_position
            .or(formatter.attribute_position)
        {
            options.set_attribute_position(attribute_position);
        }

        // #endregion

        // #region HTML formatter options

        if let Some(whitespace_sensitivity) = html_formatter.whitespace_sensitivity {
            options.set_whitespace_sensitivity(whitespace_sensitivity);
        }

        if let Some(self_close_void_elements) = html_formatter.self_close_void_elements {
            options.set_self_close_void_elements(self_close_void_elements);
        }

        if let Some(indent_script_and_style) = html_formatter.indent_script_and_style {
            options.set_indent_script_and_style(indent_script_and_style);
        }

        // #endregion
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

    fn apply_overrides_to_html_parser_options(&self, options: &mut HtmlParseOptions) {
        let html_parser = &self.languages.html.parser;

        if let Some(interpolation) = html_parser.interpolation {
            options.set_double_text_expression(interpolation.value());
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
        if let Some(tailwind_directives) = css_parser.tailwind_directives {
            options.tailwind_directives = tailwind_directives.value();
        }
    }

    #[expect(dead_code)]
    // NOTE: Currently not used because the rule options are typed using TypeId and Any, which isn't thread safe.
    // TODO: Find a way to cache this
    fn analyzer_rules_mut(&self, _analyzer_rules: &mut AnalyzerRules) {}
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
                expand: formatter.expand,
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
        let plugins = pattern.plugins.unwrap_or_default();

        let files = pattern
            .files
            .map(|files| OverrideFilesSettings {
                max_size: files.max_size,
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
            files,
            plugins,
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

    let linter = conf.linter.take().unwrap_or_default();
    language_setting.linter.enabled = linter.enabled;

    let assist = conf.assist.take().unwrap_or_default();
    language_setting.assist.enabled = assist.enabled;

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
    language_setting.parser.tailwind_directives = parser
        .tailwind_directives
        .or(parent_parser.tailwind_directives);

    let linter = conf.linter.take().unwrap_or_default();
    language_setting.linter.enabled = linter.enabled;

    let assist = conf.assist.take().unwrap_or_default();
    language_setting.assist.enabled = assist.enabled;

    language_setting
}

fn to_graphql_language_settings(
    mut conf: GraphqlConfiguration,
    _parent_settings: &LanguageSettings<GraphqlLanguage>,
) -> LanguageSettings<GraphqlLanguage> {
    let mut language_setting: LanguageSettings<GraphqlLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    let linter = conf.linter.take().unwrap_or_default();
    language_setting.linter.enabled = linter.enabled;

    let assist = conf.assist.take().unwrap_or_default();
    language_setting.assist.enabled = assist.enabled;

    language_setting
}

fn to_grit_language_settings(
    mut conf: GritConfiguration,
    _parent_settings: &LanguageSettings<GritLanguage>,
) -> LanguageSettings<GritLanguage> {
    let mut language_setting: LanguageSettings<GritLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();

    language_setting.formatter = formatter.into();

    let linter = conf.linter.take().unwrap_or_default();
    language_setting.linter = linter.into();

    let assist = conf.assist.take().unwrap_or_default();
    language_setting.assist = assist.into();

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
        expand: conf.expand,
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
        let indent_width = conf.indent_width.unwrap_or_default();

        Ok(Self {
            enabled: conf.enabled,
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            attribute_position: Some(AttributePosition::default()),
            bracket_same_line: conf.bracket_same_line,
            bracket_spacing: Some(BracketSpacing::default()),
            expand: conf.expand,
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

#[cfg(test)]
#[path = "settings.tests.rs"]
mod tests;

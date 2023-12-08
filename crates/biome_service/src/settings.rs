use crate::configuration::diagnostics::InvalidIgnorePattern;
use crate::configuration::{push_to_analyzer_rules, JavascriptConfiguration, JsonConfiguration};
use crate::{
    configuration::FilesConfiguration, Configuration, ConfigurationDiagnostic, Matcher, MergeWith,
    Rules, WorkspaceError,
};
use biome_analyze::{AnalyzerRules, RuleFilter};
use biome_deserialize::StringSet;
use biome_diagnostics::Category;
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth};
use biome_fs::RomePath;
use biome_js_analyze::metadata;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsLanguage;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use globset::{Glob, GlobSetBuilder};
use indexmap::IndexSet;
use std::ops::{BitOr, Sub};
use std::path::Path;
use std::{
    num::NonZeroU64,
    sync::{RwLock, RwLockReadGuard},
};

/// Global settings for the entire workspace
#[derive(Debug, Default)]
pub struct WorkspaceSettings {
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

impl WorkspaceSettings {
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

    /// Retrieves the settings of the linter
    pub fn linter(&self) -> &LinterSettings {
        &self.linter
    }

    /// Retrieves the settings of the organize imports
    pub fn organize_imports(&self) -> &OrganizeImportsSettings {
        &self.organize_imports
    }

    /// The (configuration)[Configuration] is merged into the workspace
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn merge_with_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), WorkspaceError> {
        // formatter part
        if let Some(formatter) = configuration.formatter {
            self.formatter = FormatSettings::try_from(formatter)?;
        }

        // linter part
        if let Some(linter) = configuration.linter {
            self.linter = LinterSettings::try_from(linter)?;
        }

        // Filesystem settings
        if let Some(files) = configuration.files {
            self.files = FilesSettings::try_from(files)?;
        }

        if let Some(organize_imports) = configuration.organize_imports {
            self.organize_imports = OrganizeImportsSettings::try_from(organize_imports)?;
        }

        if let Some(overrides) = configuration.overrides {
            self.override_settings = OverrideSettings::try_from(overrides)?;
        }

        // javascript settings
        if let Some(javascript) = configuration.javascript {
            self.languages.javascript = javascript.into();
        }
        // json settings
        if let Some(json) = configuration.json {
            self.languages.json = json.into();
        }

        Ok(())
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

    /// Returns rules
    pub fn as_rules(&self, path: &Path) -> Option<Rules> {
        let overrides = &self.override_settings;
        self.linter
            .rules
            .as_ref()
            .map(|rules| overrides.override_as_rules(path, rules.clone()))
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
    /// List of ignore paths/files
    pub ignored_files: Option<Matcher>,
    /// List of included paths/files
    pub included_files: Option<Matcher>,
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
            ignored_files: None,
            included_files: None,
        }
    }
}

/// Formatter settings for the entire workspace
#[derive(Debug)]
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
    pub ignored_files: Option<Matcher>,

    /// List of included paths/files to match
    pub included_files: Option<Matcher>,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
            ignored_files: None,
            included_files: None,
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug)]
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
    pub ignored_files: Option<Matcher>,

    /// List of ignored paths/files to match
    pub included_files: Option<Matcher>,
}

impl Default for OrganizeImportsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            ignored_files: None,
            included_files: None,
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct OverrideOrganizeImportsSettings {
    /// Enabled by default
    pub enabled: Option<bool>,
}

/// Static map of language names to language-specific settings
#[derive(Debug, Default)]
pub struct LanguageListSettings {
    pub javascript: LanguageSettings<JsLanguage>,
    pub json: LanguageSettings<JsonLanguage>,
}

impl From<JavascriptConfiguration> for LanguageSettings<JsLanguage> {
    fn from(javascript: JavascriptConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();
        let formatter = javascript.formatter;
        if let Some(formatter) = formatter {
            language_setting.formatter.quote_style = formatter.quote_style;
            language_setting.formatter.jsx_quote_style = formatter.jsx_quote_style;
            language_setting.formatter.quote_properties = formatter.quote_properties;
            language_setting.formatter.trailing_comma = formatter.trailing_comma;
            language_setting.formatter.semicolons = formatter.semicolons;
            language_setting.formatter.arrow_parentheses = formatter.arrow_parentheses;
            language_setting.formatter.bracket_spacing = formatter.bracket_spacing.map(Into::into);
            language_setting.formatter.bracket_same_line =
                formatter.bracket_same_line.map(Into::into);
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.indent_width = formatter
                .indent_width
                .map(Into::into)
                .or(formatter.indent_size.map(Into::into));
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
        }

        if let Some(parser) = javascript.parser {
            language_setting.parser.parse_class_parameter_decorators = parser
                .unsafe_parameter_decorators_enabled
                .unwrap_or_default();
        }

        let organize_imports = javascript.organize_imports;
        if let Some(_organize_imports) = organize_imports {}

        language_setting.globals = javascript.globals.map(|global| global.into_index_set());

        language_setting
    }
}

impl From<JsonConfiguration> for LanguageSettings<JsonLanguage> {
    fn from(json: JsonConfiguration) -> Self {
        let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();
        if let Some(parser) = json.parser {
            language_setting.parser.allow_comments = parser.allow_comments.unwrap_or_default();
            language_setting.parser.allow_trailing_commas =
                parser.allow_trailing_commas.unwrap_or_default();
        }
        if let Some(formatter) = json.formatter {
            language_setting.formatter.enabled = formatter.enabled;
            language_setting.formatter.line_width = formatter.line_width;
            language_setting.formatter.indent_width = formatter
                .indent_width
                .map(Into::into)
                .or(formatter.indent_size.map(Into::into));
            language_setting.formatter.indent_style = formatter.indent_style.map(Into::into);
        }
        language_setting
    }
}
pub trait Language: biome_rowan::Language {
    /// Formatter settings type for this language
    type FormatterSettings: Default;

    type LinterSettings: Default;

    /// Organize imports settings type for this language
    type OrganizeImportsSettings: Default;

    /// Fully resolved formatter options type for this language
    type FormatOptions: biome_formatter::FormatOptions;

    /// Settings that belong to the parser
    type ParserSettings: Default;

    /// Read the settings type for this language from the [LanguageListSettings] map
    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &RomePath,
    ) -> Self::FormatOptions;
}

#[derive(Debug, Default)]
pub struct LanguageSettings<L: Language> {
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
}

/// Filesystem settings for the entire workspace
#[derive(Debug)]
pub struct FilesSettings {
    /// File size limit in bytes
    pub max_size: NonZeroU64,

    /// List of paths/files to matcher
    pub ignored_files: Option<Matcher>,

    /// List of paths/files to matcher
    pub included_files: Option<Matcher>,

    /// Files not recognized by Biome should not emit a diagnostic
    pub ignore_unknown: bool,
}

/// Limit the size of files to 1.0 MiB by default
const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
            ignored_files: None,
            included_files: None,
            ignore_unknown: false,
        }
    }
}

impl TryFrom<FilesConfiguration> for FilesSettings {
    type Error = WorkspaceError;

    fn try_from(config: FilesConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            max_size: config.max_size.unwrap_or(DEFAULT_FILE_SIZE_LIMIT),
            ignored_files: to_matcher(config.ignore.as_ref())?,
            included_files: to_matcher(config.include.as_ref())?,
            ignore_unknown: config.ignore_unknown.unwrap_or_default(),
        })
    }
}

/// Handle object holding a temporary lock on the workspace settings until
/// the deferred language-specific options resolution is called
#[derive(Debug)]
pub struct SettingsHandle<'a> {
    inner: RwLockReadGuard<'a, WorkspaceSettings>,
}

impl<'a> SettingsHandle<'a> {
    pub(crate) fn new(settings: &'a RwLock<WorkspaceSettings>) -> Self {
        Self {
            inner: settings.read().unwrap(),
        }
    }
}

impl<'a> AsRef<WorkspaceSettings> for SettingsHandle<'a> {
    fn as_ref(&self) -> &WorkspaceSettings {
        &self.inner
    }
}

impl<'a> SettingsHandle<'a> {
    /// Resolve the formatting context for the given language
    pub(crate) fn format_options<L>(self, path: &RomePath) -> L::FormatOptions
    where
        L: Language,
    {
        L::resolve_format_options(
            &self.inner.formatter,
            &self.inner.override_settings,
            &L::lookup_settings(&self.inner.languages).formatter,
            path,
        )
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
            if let Some(exclude) = pattern.exclude.as_ref() {
                if exclude.matches_path(path) {
                    return Some(true);
                }
            }
        }
        None
    }
    /// Checks whether at least one override include the provided `path`
    pub fn is_path_included(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            if let Some(include) = pattern.include.as_ref() {
                if include.matches_path(path) {
                    return Some(true);
                }
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
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if excluded == Some(true) {
                return options;
            }

            if included == Some(true) {
                let js_formatter = &pattern.languages.javascript.formatter;
                let formatter = &pattern.formatter;

                if let Some(indent_style) = js_formatter.indent_style.or(formatter.indent_style) {
                    options.set_indent_style(indent_style);
                }

                if let Some(indent_width) = js_formatter.indent_width.or(formatter.indent_width) {
                    options.set_indent_width(indent_width)
                }
                if let Some(line_width) = js_formatter.line_width.or(formatter.line_width) {
                    options.set_line_width(line_width);
                }
                if let Some(quote_style) = js_formatter.quote_style {
                    options.set_quote_style(quote_style);
                }
                if let Some(trailing_comma) = js_formatter.trailing_comma {
                    options.set_trailing_comma(trailing_comma);
                }
                if let Some(quote_properties) = js_formatter.quote_properties {
                    options.set_quote_properties(quote_properties);
                }
                if let Some(jsx_quote_style) = js_formatter.jsx_quote_style {
                    options.set_jsx_quote_style(jsx_quote_style);
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
            }

            options
        })
    }

    /// It scans the current override rules and return the formatting options that of the first override is matched
    pub fn override_json_format_options(
        &self,
        path: &Path,
        options: JsonFormatOptions,
    ) -> JsonFormatOptions {
        self.patterns.iter().fold(options, |mut options, pattern| {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));
            if excluded == Some(true) {
                return options;
            }
            if included == Some(true) {
                let json_formatter = &pattern.languages.json.formatter;

                if let Some(indent_style) = json_formatter
                    .indent_style
                    .or(pattern.formatter.indent_style)
                {
                    options.set_indent_style(indent_style);
                }

                if let Some(indent_width) = json_formatter
                    .indent_width
                    .or(pattern.formatter.indent_width)
                {
                    options.set_indent_width(indent_width)
                }
                if let Some(line_width) = json_formatter.line_width.or(pattern.formatter.line_width)
                {
                    options.set_line_width(line_width);
                }
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
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));
            if excluded == Some(true) {
                return options;
            }
            if included == Some(true) {
                let js_parser = &pattern.languages.javascript.parser;

                options.parse_class_parameter_decorators =
                    js_parser.parse_class_parameter_decorators;
            }
            options
        })
    }

    pub fn as_json_parser_options(&self, path: &Path) -> Option<JsonParserOptions> {
        for pattern in &self.patterns {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                let json_parser = &pattern.languages.json.parser;

                return Some(JsonParserOptions {
                    allow_comments: json_parser.allow_comments,
                    allow_trailing_commas: json_parser.allow_trailing_commas,
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
                let included = pattern.include.as_ref().map(|p| p.matches_path(path));
                let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

                if included == Some(true) || excluded == Some(false) {
                    if let Some(rules) = pattern.linter.rules.as_ref() {
                        push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
                    }
                }
                analyzer_rules
            })
    }

    /// Retrieves the enabled rules that match the given `path`
    pub fn overrides_enabled_rules<'a>(
        &'a self,
        path: &Path,
        rules: IndexSet<RuleFilter<'a>>,
    ) -> IndexSet<RuleFilter> {
        self.patterns.iter().fold(rules, move |mut rules, pattern| {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                if let Some(pattern_rules) = pattern.linter.rules.as_ref() {
                    let disabled_rules = pattern_rules.as_disabled_rules();
                    let enabled_rules = pattern_rules.as_enabled_rules();

                    rules = rules.bitor(&enabled_rules).sub(&disabled_rules);
                }
            }

            rules
        })
    }

    pub fn override_as_rules(&self, path: &Path, rules: Rules) -> Rules {
        self.patterns.iter().fold(rules, |mut rules, pattern| {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                let pattern_rules = pattern.linter.rules.as_ref();
                if let Some(patter_rules) = pattern_rules {
                    rules.merge_with(patter_rules.clone())
                }
            }

            rules
        })
    }

    /// Scans the overrides and checks if there's an override that disable the formatter for `path`
    pub fn formatter_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                if let Some(enabled) = pattern.formatter.enabled {
                    return Some(!enabled);
                }
            }
        }
        None
    }

    /// Scans the overrides and checks if there's an override that disable the linter for `path`
    pub fn linter_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                if let Some(enabled) = pattern.linter.enabled {
                    return Some(!enabled);
                }
            }
        }
        None
    }

    /// Scans the overrides and checks if there's an override that disable the organize imports for `path`
    pub fn organize_imports_disabled(&self, path: &Path) -> Option<bool> {
        for pattern in &self.patterns {
            let included = pattern.include.as_ref().map(|p| p.matches_path(path));
            let excluded = pattern.exclude.as_ref().map(|p| p.matches_path(path));

            if included == Some(true) || excluded == Some(false) {
                if let Some(enabled) = pattern.organize_imports.enabled {
                    return Some(!enabled);
                }
            }
        }
        None
    }
}
#[derive(Debug)]
pub struct OverrideSettingPattern {
    pub exclude: Option<Matcher>,
    pub include: Option<Matcher>,
    /// Formatter settings applied to all files in the workspaces
    pub formatter: OverrideFormatSettings,
    /// Linter settings applied to all files in the workspace
    pub linter: OverrideLinterSettings,
    /// Linter settings applied to all files in the workspace
    pub organize_imports: OverrideOrganizeImportsSettings,
    /// Language specific settings
    pub languages: LanguageListSettings,
}

/// Creates a [Matcher] from a [StringSet]
///
/// ## Errors
///
/// It can raise an error if the patterns aren't valid
pub fn to_matcher(string_set: Option<&StringSet>) -> Result<Option<Matcher>, WorkspaceError> {
    if let Some(string_set) = string_set {
        let mut builder = GlobSetBuilder::new();
        for pattern in string_set.iter() {
            builder.add(Glob::new(pattern).map_err(|err| {
                WorkspaceError::Configuration(ConfigurationDiagnostic::new_invalid_ignore_pattern(
                    pattern.to_string(),
                    err.to_string(),
                ))
            })?);
        }
        Ok(Some(Matcher::new(builder.build().map_err(|err| {
            WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
                InvalidIgnorePattern {
                    message: err.to_string(),
                },
            ))
        })?)))
    } else {
        Ok(None)
    }
}

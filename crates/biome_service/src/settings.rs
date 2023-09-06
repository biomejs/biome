use crate::{
    configuration::FilesConfiguration, Configuration, ConfigurationDiagnostic, MatchOptions,
    Matcher, Rules, WorkspaceError,
};
use biome_diagnostics::Category;
use biome_formatter::{IndentStyle, IndentWidth, LineWidth};
use biome_fs::RomePath;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use indexmap::IndexSet;
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
    pub languages: LanguagesSettings,
    /// Filesystem settings for the workspace
    pub files: FilesSettings,
    /// Analyzer settings
    pub organize_imports: OrganizeImportsSettings,
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
    #[tracing::instrument(level = "debug", skip(self))]
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

        // javascript settings
        let javascript = configuration.javascript;
        if let Some(javascript) = javascript {
            self.languages.javascript.globals = javascript.globals.map(|g| g.into_index_set());
            let formatter = javascript.formatter;
            if let Some(formatter) = formatter {
                self.languages.javascript.formatter.quote_style = formatter.quote_style;
                self.languages.javascript.formatter.jsx_quote_style = formatter.jsx_quote_style;
                self.languages.javascript.formatter.quote_properties = formatter.quote_properties;
                self.languages.javascript.formatter.trailing_comma = formatter.trailing_comma;
                self.languages.javascript.formatter.semicolons = formatter.semicolons;
                self.languages.javascript.formatter.arrow_parentheses = formatter.arrow_parentheses;
                self.languages.javascript.formatter.enabled = formatter.enabled;
                self.languages.javascript.formatter.line_width = formatter.line_width;
                self.languages.javascript.formatter.indent_width =
                    formatter.indent_size.map(Into::into);
                self.languages.javascript.formatter.indent_style =
                    formatter.indent_style.map(Into::into);
            }

            if let Some(parser) = javascript.parser {
                self.languages
                    .javascript
                    .parser
                    .parse_class_parameter_decorators = parser
                    .unsafe_parameter_decorators_enabled
                    .unwrap_or_default();
            }

            let organize_imports = javascript.organize_imports;
            if let Some(_organize_imports) = organize_imports {}
        }

        // json settings
        let json = configuration.json;
        if let Some(json) = json {
            if let Some(parser) = json.parser {
                self.languages.json.parser.allow_comments =
                    parser.allow_comments.unwrap_or_default();
            }
            if let Some(formatter) = json.formatter {
                self.languages.json.formatter.enabled = formatter.enabled;
                self.languages.json.formatter.line_width = formatter.line_width;
                self.languages.json.formatter.indent_width = formatter.indent_size.map(Into::into);
                self.languages.json.formatter.indent_style = formatter.indent_style.map(Into::into);
            }
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
    pub indent_size: Option<IndentWidth>,
    pub line_width: Option<LineWidth>,
    /// List of paths/files to matcher
    pub ignored_files: Matcher,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_style: Some(IndentStyle::default()),
            indent_size: Some(IndentWidth::default()),
            line_width: Some(LineWidth::default()),
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct LinterSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of rules
    pub rules: Option<Rules>,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,
}

impl Default for LinterSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
        }
    }
}

/// Linter settings for the entire workspace
#[derive(Debug)]
pub struct OrganizeImportsSettings {
    /// Enabled by default
    pub enabled: bool,

    /// List of paths/files to matcher
    pub ignored_files: Matcher,
}

impl Default for OrganizeImportsSettings {
    fn default() -> Self {
        Self {
            // currently experimental
            enabled: false,
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
        }
    }
}

/// Static map of language names to language-specific settings
#[derive(Debug, Default)]
pub struct LanguagesSettings {
    pub javascript: LanguageSettings<JsLanguage>,
    pub json: LanguageSettings<JsonLanguage>,
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

    /// Read the settings type for this language from the [LanguagesSettings] map
    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self>;

    /// Resolve the formatter options from the global (workspace level),
    /// per-language and editor provided formatter settings
    fn resolve_format_options(
        global: &FormatSettings,
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
    pub ignored_files: Matcher,

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
            ignored_files: Matcher::new(MatchOptions {
                case_sensitive: true,
                require_literal_leading_dot: false,
                require_literal_separator: false,
            }),
            ignore_unknown: false,
        }
    }
}

impl TryFrom<FilesConfiguration> for FilesSettings {
    type Error = WorkspaceError;

    fn try_from(config: FilesConfiguration) -> Result<Self, Self::Error> {
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        if let Some(ignore) = config.ignore {
            for pattern in ignore.index_set() {
                matcher.add_pattern(pattern).map_err(|err| {
                    WorkspaceError::Configuration(
                        ConfigurationDiagnostic::new_invalid_ignore_pattern(
                            pattern.to_string(),
                            err.msg.to_string(),
                        ),
                    )
                })?;
            }
        }
        Ok(Self {
            max_size: config.max_size.unwrap_or(DEFAULT_FILE_SIZE_LIMIT),
            ignored_files: matcher,
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
            &L::lookup_settings(&self.inner.languages).formatter,
            path,
        )
    }
}

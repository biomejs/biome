//! This module contains the configuration of `biome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.
pub mod css;
pub mod diagnostics;
pub mod formatter;
mod generated;
pub mod javascript;
pub mod json;
pub mod linter;
mod merge;
pub mod organize_imports;
mod overrides;
mod parse;
pub mod vcs;

use crate::configuration::diagnostics::CantLoadExtendFile;
pub use crate::configuration::diagnostics::ConfigurationDiagnostic;
pub(crate) use crate::configuration::generated::push_to_analyzer_rules;
pub use crate::configuration::merge::MergeWith;
use crate::configuration::organize_imports::{organize_imports, OrganizeImports};
use crate::configuration::overrides::Overrides;
use crate::configuration::vcs::{vcs_configuration, VcsConfiguration};
use crate::settings::WorkspaceSettings;
use crate::{DynRef, WorkspaceError, VERSION};
use biome_analyze::AnalyzerRules;
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, StringSet};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{AutoSearchResult, FileSystem, OpenOptions};
use biome_js_analyze::metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use bpaf::Bpaf;
pub use css::{css_configuration, CssConfiguration, CssFormatter};
pub use formatter::{
    deserialize_line_width, formatter_configuration, serialize_line_width, FormatterConfiguration,
    PlainIndentStyle,
};
pub use javascript::{javascript_configuration, JavascriptConfiguration, JavascriptFormatter};
pub use json::{json_configuration, JsonConfiguration, JsonFormatter};
pub use linter::{linter_configuration, LinterConfiguration, RuleConfiguration, Rules};
pub use overrides::to_override_settings;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

/// The configuration that is contained inside the file `biome.json`
#[derive(Debug, Deserialize, Serialize, Clone, Bpaf, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Configuration {
    /// Allows to pass a path to a JSON schema file.
    ///
    /// We publish a JSON schema file for the `biome.json`.
    ///
    /// You can specify a relative path to the schema of the `@biomejs/biome` npm package if `@biomejs/biome` is installed in the `node_modules` folder:
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "$schema": "./node_modules/@biomejs/biome/configuration_schema.json"
    /// }
    /// ```
    ///
    /// If you have problems with resolving the physical file, you can use the one published in this site:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "$schema": "https://biomejs.dev/schemas/1.4.0/schema.json"
    /// }
    /// ```
    #[serde(rename(serialize = "$schema", deserialize = "$schema"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub schema: Option<String>,

    /// Set of properties to integrate Biome with a VCS software.
    ///
    /// ### `vcs.enabled`
    ///
    /// Whether Biome should integrate itself with the VCS client
    ///
    /// > Default: false
    ///
    /// ### `vcs.clientKind`
    ///
    /// The kind of client.
    ///
    /// Values:
    /// - `"git"`
    ///
    /// ### `vcs.useIgnoreFile`
    ///
    /// Whether Biome should use the VCS ignore file. When `true`, Biome will ignore the files
    /// specified in the ignore file.
    ///
    /// ### `vcs.root`
    ///
    /// The folder where Biome should check for VCS files. By default, Biome will use the same
    /// folder where `biome.json` was found.
    ///
    /// If Biome can't find the configuration, it will attempt to use the current working directory.
    /// If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
    /// will be emitted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(vcs_configuration), optional, hide_usage)]
    pub vcs: Option<VcsConfiguration>,

    /// ### `files.maxSize`
    ///
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons.
    ///
    /// > Default: 1024*1024 (1MB)
    ///
    /// :::caution
    /// When both `include` and `ignore` are specified, `ignore` takes **precedence** over `include`
    /// :::
    ///
    /// Given the following example:
    ///
    /// ```json title="biome.json"
    /// {
    ///   "files": {
    ///     "include": ["scripts/**/*.js", "src/**/*.js"],
    ///     "ignore": ["scripts/**/*.js"]
    ///   }
    /// }
    /// ```
    ///
    /// Only the files that match the pattern `src/**/*.js` will be handled, while the files that match the pattern
    /// `scripts/**/*.js` will be ignored.
    ///
    ///  ### `files.ignoreUnknown`
    ///
    /// Biome won't emit diagnostics if it encounters files that can't handle.
    ///
    /// ```json title="biome.json"
    /// {
    ///   "files": {
    ///     "ignoreUnknown": true
    ///   }
    /// }
    /// ```
    ///
    /// > Default: false
    ///
    /// For advanced configuration options, see the [documentation](/referece/configuration_example/##files).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(files_configuration), optional, hide_usage)]
    pub files: Option<FilesConfiguration>,

    /// These options apply to all languages.  There are additional language-specific formatting options below.
    ///
    /// ### `formatter.enabled`
    ///
    /// Enables Biome's formatter
    ///
    /// > Default: `true`
    ///
    ///Given the following example:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "formatter": {
    ///     "include": ["scripts/**/*.js", "src/**/*.js"],
    ///     "ignore": ["scripts/**/*.js"]
    ///   }
    /// }
    /// ```
    ///
    /// Only the files that match the patter `src/**/*.js` will be formatted, while the files that match the pattern
    /// `scripts/**/*.js` will be ignored.
    ///
    /// ### `formatter.formatWithErrors`
    ///
    /// Allows to format a document that has syntax errors.
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "formatter": {
    ///     "formatWithErrors": true
    ///   }
    /// }
    /// ```
    ///
    /// > Default: `false`
    ///
    /// ### `formatter.indentStyle`
    ///
    /// The style of the indentation. It can be `"tab"` or `"space"`.
    ///
    /// > Default: `tab`
    ///
    /// ### `formatter.indentSize`
    ///
    /// How big the indentation should be.
    ///
    /// > Default: `2`
    ///
    /// ### `formatter.indentWidth`
    ///
    /// How big the indentation should be.
    ///
    /// > Default: `2`
    ///
    /// ### `formatter.lineEnding`
    ///
    /// The type of line ending.
    /// - `lf`, Line Feed only (`\n`), common on Linux and macOS as well as inside git repos
    /// - `crlf` Carriage Return + Line Feed characters (`\r\n`), common on Windows
    /// - `cr` Carriage Return character only (`\r`), used very rarely
    ///
    /// > Default: `lf`
    ///
    /// ### `formatter.lineWidth`
    ///
    /// How many characters can be written on a single line.
    ///
    /// > Default: `80`
    ///
    /// ### `formatter.bracketSameLine`
    ///
    /// Choose whether the ending `>` of a multi-line JSX element should be on the last attribute line or not
    ///
    /// > Default: false
    ///
    /// ### `formatter.bracketSpacing`
    ///
    /// Choose whether spaces should be added between brackets and inner values
    ///
    /// > Default: true
    ///
    /// For advanced configuration options, see the [documentation](/referece/configuration_example/##formatter.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(formatter_configuration), optional)]
    pub formatter: Option<FormatterConfiguration>,

    /// ### `organizeImports.enabled`
    ///
    /// Enables Biome's sort imports.
    ///
    /// > Default: `true`
    ///
    /// :::caution
    /// When both `include` and `ignore` are specified, `ignore` takes **precedence** over `include`
    /// :::
    ///
    /// Given the following example:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "organizeImports": {
    ///     "include": ["scripts/**/*.js", "src/**/*.js"],
    ///     "ignore": ["scripts/**/*.js"]
    ///   }
    /// }
    /// ```
    ///
    /// Only the files that match the patter `src/**/*.js` will have their imports sorted, while the files that match the pattern
    /// `scripts/**/*.js` will be ignored.
    ///
    /// For advanced configuration options, see the [documentation](/referece/configuration_example/##organizeImports.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub organize_imports: Option<OrganizeImports>,

    /// ### `linter.enabled`
    ///
    /// Enables Biome's linter
    ///
    /// > Default: `true`
    ///
    /// For detailed examples and advanced configuration options, see the [documentation]
    /// (/referece/configuration_example/##linter).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(linter_configuration), optional)]
    pub linter: Option<LinterConfiguration>,

    /// These options apply only to JavaScript (and TypeScript) files.
    ///
    /// ### `javascript.parser.unsafeParameterDecoratorsEnabled`
    ///
    /// Allows to support the unsafe/experimental parameter decorators.
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "javascript": {
    ///     "parser": {
    ///         "unsafeParameterDecoratorsEnabled": true
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// > Default: `false`
    ///
    /// ### `javascript.formatter.quoteStyle`
    ///
    /// The type of quote used when representing string literals. It can be `single` or `double`.
    ///
    /// > Default: `double`
    ///
    /// ### `javascript.formatter.jsxQuoteStyle`
    ///
    /// The type of quote used when representing jsx string literals. It can be `single` or `double`.
    ///
    /// > Default: `double`
    ///
    /// ### `javascript.formatter.quoteProperties`
    ///
    /// When properties inside objects should be quoted. It can be `asNeeded` or `preserve`.
    ///
    /// > Default: `asNeeded`
    ///
    /// ### `javascript.formatter.trailingComma`
    ///
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Possible values:
    /// - `all`, the trailing comma is always added;
    /// - `es5`, the trailing comma is added only in places where it's supported by older version of JavaScript;
    /// - `none`, trailing commas are never added;
    ///
    /// > Default: `all`
    ///
    /// ### `javascript.formatter.semicolons`
    ///
    /// It configures where the formatter prints semicolons:
    /// - `always`, the semicolons is always added at the end of each statement;
    /// - `asNeeded`, the semicolons are added only in places where it's needed, to protect from [ASI](https://en.wikibooks.org/wiki/JavaScript/Automatic_semicolon_insertion)
    ///
    /// > Default: `always`
    ///
    /// Example:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "javascript": {
    ///     "formatter": {
    ///       "semicolons": "asNeeded"
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### `javascript.formatter.arrowParentheses`
    ///
    /// Whether to add non-necessary parentheses to arrow functions:
    /// - `always`, the parentheses are always added;
    /// - `asNeeded`, the parentheses are added only when they are needed;
    ///
    /// > Default: `always`
    ///
    /// ### `javascript.formatter.enabled`
    ///
    /// Enables Biome's formatter for JavaScript (and its super languages) files.
    ///
    /// > Default: `true`
    ///
    /// ### `javascript.formatter.indentStyle`
    ///
    /// The style of the indentation for JavaScript (and its super languages) files. It can be `"tab"` or `"space"`.
    ///
    /// > Default: `tab`
    ///
    /// ### `javascript.formatter.indentSize`
    ///
    /// How big the indentation should be for JavaScript (and its super languages) files.
    ///
    /// > Default: `2`
    ///
    /// ### `javascript.formatter.lineEnding`
    ///
    /// The type of line ending for JavaScript (and its super languages) files:
    /// - `lf`, Line Feed only (`\n`), common on Linux and macOS as well as inside git repos;
    /// - `crlf` Carriage Return + Line Feed characters (`\r\n`), common on Windows;
    /// - `cr` Carriage Return character only (`\r`), used very rarely;
    ///
    /// > Default: `lf`
    ///
    /// ### `javascript.formatter.lineWidth`
    ///
    /// How many characters can be written on a single line in JavaScript (and its super languages) files.
    ///
    /// > Default: `80`
    ///
    ///
    /// ### `javascript.globals`
    ///
    /// A list of global names that Biome should ignore (analyzer, linter, etc.)
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "javascript": {
    ///     "globals": ["$", "_", "externalVariable"]
    ///   }
    /// }
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_configuration), optional)]
    pub javascript: Option<JavascriptConfiguration>,

    /// Options applied to the JSON files.
    ///
    /// ### `json.parser.allowComments`
    ///
    /// Enables the parsing of comments in JSON files.
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "json": {
    ///     "parser": {
    ///       "allowComments": true
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### `json.parser.allowTrailingCommas`
    ///
    /// Enables the parsing of trailing Commas in JSON files.
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "json": {
    ///     "parser": {
    ///       "allowTrailingCommas": true
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### `json.formatter.enabled`
    ///
    /// Enables Biome's formatter for JSON (and its super languages) files.
    ///
    /// > Default: `true`
    ///
    /// ### `json.formatter.indentStyle`
    ///
    ///
    /// The style of the indentation for JSON (and its super languages) files. It can be `"tab"` or `"space"`.
    ///
    /// > Default: `tab`
    ///
    ///
    /// ### `json.formatter.indentSize`
    ///
    /// How big the indentation should be for JSON (and its super languages) files.
    ///
    /// > Default: `2`
    ///
    /// ### `json.formatter.lineEnding`
    ///
    /// The type of line ending for JSON (and its super languages) files:
    /// - `lf`, Line Feed only (`\n`), common on Linux and macOS as well as inside git repos;
    /// - `crlf` Carriage Return + Line Feed characters (`\r\n`), common on Windows;
    /// - `cr` Carriage Return character only (`\r`), used very rarely;
    ///
    /// > Default: `lf`
    ///
    /// ### `json.formatter.lineWidth`
    ///
    /// How many characters can be written on a single line in JSON (and its super languages) files.
    ///
    /// > Default: `80`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_configuration), optional)]
    pub json: Option<JsonConfiguration>,

    /// Specific configuration for the Css language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_configuration), optional, hide)]
    pub css: Option<CssConfiguration>,

    /// A list of paths to other JSON files that will extend the current configuration file.
    ///
    /// The files defined in this array:
    /// - must exist in the file system;
    /// - are resolved from the path where the `biome.json` file is defined;
    /// - must be relative paths. Paths to libraries are not resolved;
    /// - must be reachable by Biome, e.g. symbolic links might not be resolved by Biome;
    /// - will be processed in order: from the first one to the last one;
    /// - can override the same properties, but ultimately only the last one will be used by Biome;
    ///
    /// For advanced configuration options, see the [documentation](/referece/configuration_example/##extends).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub extends: Option<StringSet>,

    /// A list of patterns.
    ///
    /// Use this configuration to change the behaviour of the tools for certain files.
    ///
    /// When a file is matched against an override pattern, the configuration specified in that pattern will be override the top-level configuration.
    ///
    /// The order of the patterns matter. If a file *can* match three patterns, only the first one is used.
    ///
    ///
    /// It will include the options of [top level formatter](#formatter) configuration, minus `ignore` and `include`.
    ///
    /// #### Examples
    ///
    /// For example, it's possible to modify the formatter `lineWidth`, `indentStyle` for certain files that are included in the glob path `generated/**`:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "formatter": {
    ///     "lineWidth": 100
    ///   },
    ///   "overrides": [
    ///     {
    ///       "include": ["generated/**"],
    ///       "formatter": {
    ///         "lineWidth": 160,
    ///         "indentStyle": "space"
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### `overrides.<ITEM>.linter`
    ///
    /// It will include the options of [top level linter](#linter) configuration, minus `ignore` and `include`.
    ///
    ///
    /// #### Examples
    ///
    /// You can disable certain rules for certain glob paths, and disable the linter for other glob paths:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "linter": {
    ///     "enabled": true,
    ///     "rules": {
    ///       "recommended": true
    ///     }
    ///   },
    ///   "overrides": [
    ///     {
    ///       "include": ["lib/**"],
    ///       "linter": {
    ///         "rules": {
    ///           "suspicious": {
    ///             "noDebugger": "off"
    ///           }
    ///         }
    ///       }
    ///     },
    ///     {
    ///       "include": ["shims/**"],
    ///       "linter": {
    ///         "enabled": false
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### `overrides.<ITEM>.organizeImports`
    ///
    /// It will include the options of [top level organize imports](#organizeimports), minus `ignore` and `include`.
    ///
    /// ### `overrides.<ITEM>.javascript`
    ///
    /// It will include the options of [top level javascript](#javascript) configuration.
    ///
    /// #### Examples
    ///
    /// You can change the formatting behaviour of JavaScript files in certain folders:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "formatter": {
    ///     "lineWidth": 120
    ///   },
    ///   "javascript": {
    ///     "formatter": {
    ///       "quoteStyle": "single"
    ///     }
    ///   },
    ///   "overrides": [
    ///     {
    ///       "include": ["lib/**"],
    ///       "javascript": {
    ///         "formatter": {
    ///           "quoteStyle": "double"
    ///         }
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    ///
    /// ### `overrides.<ITEM>.json`
    ///
    /// It will include the options of [top level json](#json) configuration.
    ///
    ///
    /// #### Examples
    ///
    /// You can enable parsing features for certain JSON files:
    ///
    ///
    ///
    /// ```json title="biome.json"
    /// {
    ///   "linter": {
    ///     "enabled": true,
    ///     "rules": {
    ///       "recommended": true
    ///     }
    ///   },
    ///   "overrides": [
    ///     {
    ///       "include": [".vscode/**"],
    ///       "json": {
    ///         "parser": {
    ///           "allowComments": true,
    ///           "allowTrailingComma": true
    ///         }
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// For advanced configuration options, see the [documentation]
    /// (/referece/configuration_example/##override).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub overrides: Option<Overrides>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            files: None,
            linter: Some(LinterConfiguration {
                enabled: Some(true),
                ..LinterConfiguration::default()
            }),
            organize_imports: Some(OrganizeImports::default()),
            formatter: None,
            css: None,
            javascript: None,
            json: None,
            schema: None,
            vcs: None,
            extends: None,
            overrides: None,
        }
    }
}

impl MergeWith<Configuration> for Configuration {
    fn merge_with(&mut self, other_configuration: Configuration) {
        // files
        self.merge_with(other_configuration.files);
        // formatter
        self.merge_with(other_configuration.formatter);
        // javascript
        self.merge_with(other_configuration.javascript);
        // json
        self.merge_with(other_configuration.json);
        // css
        self.merge_with(other_configuration.css);
        // linter
        self.merge_with(other_configuration.linter);
        // organize imports
        self.merge_with(other_configuration.organize_imports);
        // VCS
        self.merge_with(other_configuration.vcs);
        // overrides
        self.merge_with(other_configuration.overrides);
    }

    fn merge_with_if_not_default(&mut self, other_configuration: Configuration)
    where
        Configuration: Default,
    {
        // files
        self.merge_with_if_not_default(other_configuration.files);
        // formatter
        self.merge_with_if_not_default(other_configuration.formatter);
        // javascript
        self.merge_with_if_not_default(other_configuration.javascript);
        // json
        self.merge_with_if_not_default(other_configuration.json);
        // css
        self.merge_with_if_not_default(other_configuration.css);
        // linter
        self.merge_with_if_not_default(other_configuration.linter);
        // organize imports
        self.merge_with_if_not_default(other_configuration.organize_imports);
        // VCS
        self.merge_with_if_not_default(other_configuration.vcs);
        // overrides
        self.merge_with_if_not_default(other_configuration.overrides);
    }
}

impl Configuration {
    pub fn is_formatter_disabled(&self) -> bool {
        self.formatter
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_linter_disabled(&self) -> bool {
        self.linter
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_organize_imports_disabled(&self) -> bool {
        self.organize_imports
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_vcs_disabled(&self) -> bool {
        self.vcs.as_ref().map(|f| f.is_disabled()).unwrap_or(true)
    }

    pub fn is_vcs_enabled(&self) -> bool {
        !self.is_vcs_disabled()
    }

    /// This function checks if the VCS integration is enabled, and if so, it will attempts to resolve the
    /// VCS root directory and the `.gitignore` file.
    ///
    /// ## Returns
    ///
    /// A tuple with VCS root folder and the contents of the `.gitignore` file
    pub fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError> {
        let Some(vcs) = &self.vcs else {
            return Ok((None, vec![]));
        };
        if vcs.is_enabled() {
            let vcs_base_path = match (vcs_base_path, &vcs.root) {
                (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
                (None, Some(root)) => PathBuf::from(root),
                (Some(vcs_base_path), None) => PathBuf::from(vcs_base_path),
                (None, None) => return Err(WorkspaceError::vcs_disabled()),
            };
            if let Some(client_kind) = &vcs.client_kind {
                if !vcs.ignore_file_disabled() {
                    let result = file_system
                        .auto_search(vcs_base_path, client_kind.ignore_file(), false)
                        .map_err(WorkspaceError::from)?;

                    if let Some(result) = result {
                        return Ok((
                            Some(result.directory_path),
                            result
                                .content
                                .lines()
                                .map(String::from)
                                .collect::<Vec<String>>(),
                        ));
                    }
                }
            }
        }
        Ok((None, vec![]))
    }
}

impl MergeWith<Option<VcsConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<VcsConfiguration>) {
        if let Some(other_vcs) = other {
            let vcs = self.vcs.get_or_insert_with(VcsConfiguration::default);
            vcs.merge_with(other_vcs);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<VcsConfiguration>)
    where
        Option<VcsConfiguration>: Default,
    {
        self.merge_with(other)
    }
}

impl MergeWith<Option<OrganizeImports>> for Configuration {
    fn merge_with(&mut self, other: Option<OrganizeImports>) {
        if let Some(other_organize_imports) = other {
            let organize_imports = self
                .organize_imports
                .get_or_insert_with(OrganizeImports::default);
            organize_imports.merge_with(other_organize_imports);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<OrganizeImports>)
    where
        Option<OrganizeImports>: Default,
    {
        self.merge_with(other)
    }
}

impl MergeWith<Option<LinterConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<LinterConfiguration>) {
        if let Some(other_linter) = other {
            let linter = self.linter.get_or_insert_with(LinterConfiguration::default);
            linter.merge_with(other_linter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<LinterConfiguration>)
    where
        Option<LinterConfiguration>: Default,
    {
        if let Some(other_linter) = other {
            let linter = self.linter.get_or_insert_with(LinterConfiguration::default);
            linter.merge_with_if_not_default(other_linter);
        }
    }
}
impl MergeWith<Option<FilesConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<FilesConfiguration>) {
        if let Some(files_configuration) = other {
            let files = self.files.get_or_insert_with(FilesConfiguration::default);
            files.merge_with(files_configuration);
        };
    }

    fn merge_with_if_not_default(&mut self, other: Option<FilesConfiguration>)
    where
        Option<FilesConfiguration>: Default,
    {
        if let Some(files_configuration) = other {
            let files = self.files.get_or_insert_with(FilesConfiguration::default);
            files.merge_with_if_not_default(files_configuration);
        };
    }
}
impl MergeWith<Option<JavascriptConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<JavascriptConfiguration>) {
        if let Some(other) = other {
            let js_configuration = self
                .javascript
                .get_or_insert_with(JavascriptConfiguration::default);
            js_configuration.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<JavascriptConfiguration>)
    where
        Option<JavascriptConfiguration>: Default,
    {
        if let Some(other) = other {
            let js_configuration = self
                .javascript
                .get_or_insert_with(JavascriptConfiguration::default);
            js_configuration.merge_with_if_not_default(other);
        }
    }
}
impl MergeWith<Option<JsonConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<JsonConfiguration>) {
        if let Some(other) = other {
            let json_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
            json_configuration.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<JsonConfiguration>)
    where
        Option<JsonConfiguration>: Default,
    {
        if let Some(other) = other {
            let json_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
            json_configuration.merge_with_if_not_default(other);
        }
    }
}
impl MergeWith<Option<CssConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<CssConfiguration>) {
        if let Some(other) = other {
            let css_configuration = self.css.get_or_insert_with(CssConfiguration::default);
            css_configuration.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<CssConfiguration>)
    where
        Option<CssConfiguration>: Default,
    {
        if let Some(other) = other {
            let css_configuration = self.css.get_or_insert_with(CssConfiguration::default);
            css_configuration.merge_with_if_not_default(other);
        }
    }
}
impl MergeWith<Option<FormatterConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<FormatterConfiguration>) {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(FormatterConfiguration::default);
            formatter.merge_with(other_formatter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<FormatterConfiguration>)
    where
        Option<FormatterConfiguration>: Default,
    {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(FormatterConfiguration::default);
            formatter.merge_with_if_not_default(other_formatter);
        }
    }
}

impl MergeWith<Option<JavascriptFormatter>> for Configuration {
    fn merge_with(&mut self, other: Option<JavascriptFormatter>) {
        let javascript_configuration = self
            .javascript
            .get_or_insert_with(JavascriptConfiguration::default);
        javascript_configuration.merge_with(other);
    }

    fn merge_with_if_not_default(&mut self, other: Option<JavascriptFormatter>)
    where
        Option<JavascriptFormatter>: Default,
    {
        let javascript_configuration = self
            .javascript
            .get_or_insert_with(JavascriptConfiguration::default);
        javascript_configuration.merge_with_if_not_default(other);
    }
}

impl MergeWith<Option<JsonFormatter>> for Configuration {
    fn merge_with(&mut self, other: Option<JsonFormatter>) {
        let json_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
        json_configuration.merge_with(other);
    }

    fn merge_with_if_not_default(&mut self, other: Option<JsonFormatter>)
    where
        Option<JsonFormatter>: Default,
    {
        let json_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
        json_configuration.merge_with_if_not_default(other);
    }
}

impl MergeWith<Option<CssFormatter>> for Configuration {
    fn merge_with(&mut self, other: Option<CssFormatter>) {
        let css_configuration = self.css.get_or_insert_with(CssConfiguration::default);
        css_configuration.merge_with(other);
    }

    fn merge_with_if_not_default(&mut self, other: Option<CssFormatter>)
    where
        Option<CssFormatter>: Default,
    {
        let css_configuration = self.css.get_or_insert_with(CssConfiguration::default);
        css_configuration.merge_with_if_not_default(other);
    }
}

impl MergeWith<Option<Overrides>> for Configuration {
    fn merge_with(&mut self, other: Option<Overrides>) {
        if let Some(other) = other {
            let overrides = self.overrides.get_or_insert_with(Overrides::default);
            overrides.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<Overrides>)
    where
        Option<Overrides>: Default,
    {
        self.merge_with(other)
    }
}

/// The configuration of the filesystem
#[derive(Default, Debug, Deserialize, Serialize, Clone, Bpaf, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons. Defaults to 1 MiB
    #[bpaf(long("files-max-size"), argument("NUMBER"))]
    pub max_size: Option<NonZeroU64>,

    /// A list of Unix shell style patterns. Biome will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. Biome will handle only those files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,

    /// Tells Biome to not emit diagnostics when handling files that doesn't know
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("files-ignore-unknown"), argument("true|false"), optional)]
    pub ignore_unknown: Option<bool>,
}

impl MergeWith<FilesConfiguration> for FilesConfiguration {
    fn merge_with(&mut self, other: FilesConfiguration) {
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore)
        }
        if let Some(include) = other.include {
            self.include = Some(include)
        }
        if let Some(max_size) = other.max_size {
            self.max_size = Some(max_size)
        }
        if let Some(ignore_unknown) = other.ignore_unknown {
            self.ignore_unknown = Some(ignore_unknown)
        }
    }

    fn merge_with_if_not_default(&mut self, other: FilesConfiguration)
    where
        FilesConfiguration: Default,
    {
        if other != FilesConfiguration::default() {
            self.merge_with(other)
        }
    }
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<Configuration>,
    /// The path of where the `biome.json` file was found. This contains the `biome.json` name.
    pub configuration_file_path: PathBuf,
    /// The base path of where the `biome.json` file was found.
    /// This has to be used to resolve other configuration files.
    pub configuration_directory_path: PathBuf,
}

#[derive(Debug, Default, PartialEq)]
pub enum ConfigurationBasePath {
    /// The default mode, not having a configuration file is not an error.
    #[default]
    None,
    /// The base path provided by the LSP, not having a configuration file is not an error.
    Lsp(PathBuf),
    /// The base path provided by the user, not having a configuration file is an error.
    /// Throws any kind of I/O errors.
    FromUser(PathBuf),
}

impl ConfigurationBasePath {
    const fn is_from_user(&self) -> bool {
        matches!(self, ConfigurationBasePath::FromUser(_))
    }
}

/// Load the configuration for this session of the CLI, merging the content of
/// the `biome.json` file if it exists on disk with common command line options
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationBasePath,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    let loaded_configuration = LoadedConfiguration::from(config);
    loaded_configuration.apply_extends(fs)
}

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [base path](ConfigurationBasePath) should be provided.
///
/// The function will try to traverse upwards the file system until if finds a `biome.json` file, or there
/// aren't directories anymore.
///
/// If a the configuration base path was provided by the user, the function will error. If not, Biome will use
/// its defaults.
fn load_config(
    file_system: &DynRef<'_, dyn FileSystem>,
    base_path: ConfigurationBasePath,
) -> LoadConfig {
    let config_name = file_system.config_name();
    let deprecated_config_name = file_system.deprecated_config_name();
    let working_directory = file_system.working_directory();
    let configuration_directory = match base_path {
        ConfigurationBasePath::Lsp(ref path) | ConfigurationBasePath::FromUser(ref path) => {
            path.clone()
        }
        _ => match working_directory {
            Some(wd) => wd,
            None => PathBuf::new(),
        },
    };
    let should_error = base_path.is_from_user();

    let auto_search_result;
    let result =
        file_system.auto_search(configuration_directory.clone(), config_name, should_error);
    if let Ok(result) = result {
        if result.is_none() {
            auto_search_result = file_system.auto_search(
                configuration_directory.clone(),
                deprecated_config_name,
                should_error,
            )?;
        } else {
            auto_search_result = result;
        }
    } else {
        auto_search_result = file_system.auto_search(
            configuration_directory.clone(),
            deprecated_config_name,
            should_error,
        )?;
    }

    if let Some(auto_search_result) = auto_search_result {
        let AutoSearchResult {
            content,
            directory_path,
            file_path,
        } = auto_search_result;
        let deserialized =
            deserialize_from_json_str::<Configuration>(&content, JsonParserOptions::default());
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: file_path,
            configuration_directory_path: directory_path,
        }))
    } else {
        Ok(None)
    }
}

/// Creates a new configuration on file system
///
/// ## Errors
///
/// It fails if:
/// - the configuration file already exists
/// - the program doesn't have the write rights
pub fn create_config(
    fs: &mut DynRef<dyn FileSystem>,
    mut configuration: Configuration,
) -> Result<(), WorkspaceError> {
    let path = PathBuf::from(fs.config_name());

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            WorkspaceError::Configuration(ConfigurationDiagnostic::new_already_exists())
        } else {
            WorkspaceError::cant_read_file(format!("{}", path.display()))
        }
    })?;

    // we now check if biome is installed inside `node_modules` and if so, we
    if VERSION == "0.0.0" {
        let schema_path = Path::new("./node_modules/@biomejs/biome/configuration_schema.json");
        let options = OpenOptions::default().read(true);
        if fs.open_with_options(schema_path, options).is_ok() {
            configuration.schema = schema_path.to_str().map(String::from);
        }
    } else {
        configuration.schema = Some(format!(
            "https://biomejs.dev/schemas/{}/schema.json",
            VERSION
        ));
    }

    let contents = serde_json::to_string_pretty(&configuration).map_err(|_| {
        WorkspaceError::Configuration(ConfigurationDiagnostic::new_serialization_error())
    })?;

    let parsed = parse_json(&contents, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())?
            .print()
            .expect("valid format document");

    config_file
        .set_content(formatted.as_code().as_bytes())
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path.display())))?;

    Ok(())
}

/// Returns the rules applied to a specific [Path], given the [WorkspaceSettings]
pub fn to_analyzer_rules(settings: &WorkspaceSettings, path: &Path) -> AnalyzerRules {
    let linter_settings = &settings.linter;
    let overrides = &settings.override_settings;
    let mut analyzer_rules = AnalyzerRules::default();
    if let Some(rules) = linter_settings.rules.as_ref() {
        push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
    }

    overrides.override_analyzer_rules(path, analyzer_rules)
}

/// Yield information regarding the configuration that was found
#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    /// If present, the path of the directory where it was found
    pub directory_path: Option<PathBuf>,
    /// If present, the path of the file where it was found
    pub file_path: Option<PathBuf>,
    /// The Deserialized configuration
    pub configuration: Configuration,
    /// All diagnostics that were emitted during parsing and deserialization
    pub diagnostics: Vec<Error>,
}

impl LoadedConfiguration {
    /// Consumes itself to generate a new [LoadedConfiguration] where the new `configuration`
    /// is the result of its `extends` fields applied from left to right, and the last one element
    /// applied is itself.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    pub fn apply_extends(
        mut self,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Self, WorkspaceError> {
        let deserialized = self.deserialize_extends(fs)?;
        let (configurations, errors): (Vec<_>, Vec<_>) = deserialized
            .into_iter()
            .map(|d| d.consume())
            .map(|(config, diagnostics)| (config.unwrap_or_default(), diagnostics))
            .unzip();

        let extended_configuration = configurations.into_iter().reduce(
            |mut previous_configuration, current_configuration| {
                previous_configuration.merge_with(current_configuration);
                previous_configuration
            },
        );
        let configuration = if let Some(mut extended_configuration) = extended_configuration {
            // Here we want to keep only the values that aren't a default
            extended_configuration.merge_with_if_not_default(self.configuration);
            extended_configuration
        } else {
            self.configuration
        };
        self.diagnostics.extend(
            errors
                .into_iter()
                .flatten()
                .map(|diagnostic| {
                    diagnostic.with_file_path(
                        self.file_path
                            .as_ref()
                            .map(|path| path.display().to_string()),
                    )
                })
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            configuration,
            diagnostics: self.diagnostics,
            file_path: self.file_path,
            directory_path: self.directory_path,
        })
    }

    /// It attempts to deserialize all the configuration files that were specified in the `extends` property
    fn deserialize_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError> {
        let Some(extends) = &self.configuration.extends else {
            return Ok(vec![]);
        };

        let directory_path = self
            .directory_path
            .as_ref()
            .cloned()
            .unwrap_or(fs.working_directory().unwrap_or(PathBuf::from("./")));
        let mut deserialized_configurations = vec![];
        for path in extends.iter() {
            let config_path = directory_path.join(path);
            let mut file = fs
                .open_with_options(config_path.as_path(), OpenOptions::default().read(true))
                .map_err(|err| {
                    CantLoadExtendFile::new(config_path.display().to_string(), err.to_string()).with_verbose_advice(
                        markup!{
								"Biome tried to load the configuration file "<Emphasis>{directory_path.display().to_string()}</Emphasis>" using "<Emphasis>{config_path.display().to_string()}</Emphasis>" as base path."
							}
                    )
                })?;
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                CantLoadExtendFile::new(config_path.display().to_string(), err.to_string()).with_verbose_advice(
                    markup!{
							"It's possible that the file was created with a different user/group. Make sure you have the rights to read the file."
						}
                )

            })?;
            let deserialized = deserialize_from_json_str::<Configuration>(
                content.as_str(),
                JsonParserOptions::default(),
            );
            deserialized_configurations.push(deserialized)
        }
        Ok(deserialized_configurations)
    }

    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Path> {
        self.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Whether the are errors emitted. Error are [Severity::Error] or greater.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It return an iterator over the diagnostics emitted during the resolution of the configuration file
    pub fn as_diagnostics_iter(&self) -> ConfigurationDiagnosticsIter {
        ConfigurationDiagnosticsIter::new(self.diagnostics.as_slice())
    }
}

pub struct ConfigurationDiagnosticsIter<'a> {
    errors: &'a [Error],
    len: usize,
    index: usize,
}

impl<'a> ConfigurationDiagnosticsIter<'a> {
    fn new(errors: &'a [Error]) -> Self {
        Self {
            len: errors.len(),
            index: 0,
            errors,
        }
    }
}

impl<'a> Iterator for ConfigurationDiagnosticsIter<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == self.index {
            return None;
        }

        let item = self.errors.get(self.index);
        self.index += 1;
        item
    }
}

impl FusedIterator for ConfigurationDiagnosticsIter<'_> {}

impl From<Option<ConfigurationPayload>> for LoadedConfiguration {
    fn from(value: Option<ConfigurationPayload>) -> Self {
        if let Some(value) = value {
            let ConfigurationPayload {
                configuration_directory_path,
                configuration_file_path,
                deserialized,
            } = value;
            let (configuration, diagnostics) = deserialized.consume();
            LoadedConfiguration {
                configuration: configuration.unwrap_or_default(),
                diagnostics: diagnostics
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic.with_file_path(configuration_file_path.display().to_string())
                    })
                    .collect(),
                directory_path: Some(configuration_directory_path),
                file_path: Some(configuration_file_path),
            }
        } else {
            LoadedConfiguration::default()
        }
    }
}

//! This module contains the configuration of `biome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further option divided by tool.

#![deny(clippy::use_self)]

pub mod analyzer;
pub mod bool;
pub mod css;
pub mod diagnostics;
pub mod editorconfig;
mod extends;
pub mod formatter;
pub mod generated;
pub mod graphql;
pub mod grit;
pub mod html;
pub mod javascript;
pub mod json;
pub mod max_size;
mod overrides;
pub mod vcs;

use crate::analyzer::assist::{Actions, AssistConfiguration, Source, assist_configuration};
use crate::analyzer::{RuleAssistConfiguration, RuleDomains};
use crate::bool::Bool;
use crate::css::{CssFormatterConfiguration, CssLinterConfiguration, CssParserConfiguration};
pub use crate::diagnostics::BiomeDiagnostic;
pub use crate::diagnostics::CantLoadExtendFile;
use crate::extends::Extends;
pub use crate::generated::{push_to_analyzer_assist, push_to_analyzer_rules};
use crate::graphql::{GraphqlFormatterConfiguration, GraphqlLinterConfiguration};
pub use crate::grit::{GritConfiguration, grit_configuration};
use crate::javascript::{JsFormatterConfiguration, JsLinterConfiguration};
use crate::json::{JsonFormatterConfiguration, JsonLinterConfiguration};
use crate::max_size::MaxSize;
use crate::vcs::{VcsConfiguration, vcs_configuration};
pub use analyzer::{
    LinterConfiguration, RuleConfiguration, RuleFixConfiguration, RulePlainConfiguration,
    RuleWithFixOptions, RuleWithOptions, Rules, linter_configuration,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::{KeyValuePair, markup};
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValidator, DeserializableValue,
    DeserializationContext, DeserializationDiagnostic, DeserializationVisitor, Deserialized, Text,
    TextRange,
};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::Severity;
use biome_formatter::{IndentStyle, QuoteStyle};
use bpaf::Bpaf;
use camino::Utf8PathBuf;
pub use css::{CssConfiguration, css_configuration};
pub use formatter::{FormatterConfiguration, formatter_configuration};
pub use graphql::{GraphqlConfiguration, graphql_configuration};
pub use html::{HtmlConfiguration, html_configuration};
pub use javascript::{JsConfiguration, js_configuration};
pub use json::{JsonConfiguration, json_configuration};
pub use overrides::{
    OverrideAssistConfiguration, OverrideFilesConfiguration, OverrideFormatterConfiguration,
    OverrideGlobs, OverrideLinterConfiguration, OverridePattern, Overrides,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::LazyLock;
use vcs::VcsClientKind;

pub const DEFAULT_SCANNER_IGNORE_ENTRIES: &[&[u8]] = &[
    b".cache",
    b".git",
    b".hg",
    b".netlify",
    b".output",
    b".svn",
    b".yarn",
    b".timestamp",
    b".turbo",
    b".vercel",
    b".DS_Store",
];

pub const VERSION: &str = match option_env!("BIOME_VERSION") {
    Some(version) => version,
    None => "0.0.0",
};

pub type RootEnabled = Bool<true>;

/// The configuration that is contained inside the file `biome.json`
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, default, rename_all = "camelCase")]
#[deserializable(with_validator)]
pub struct Configuration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[serde(rename = "$schema")]
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,

    /// Indicates whether this configuration file is at the root of a Biome
    /// project. By default, this is `true`.
    #[bpaf(hide, hide_usage)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<RootEnabled>,

    /// A list of paths to other JSON files, used to extends the current configuration.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<Extends>,

    /// The configuration of the VCS integration
    #[bpaf(external(vcs_configuration), optional, hide_usage)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<VcsConfiguration>,

    /// The configuration of the filesystem
    #[bpaf(external(files_configuration), optional, hide_usage)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<FilesConfiguration>,

    /// The configuration of the formatter
    #[bpaf(external(formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<FormatterConfiguration>,

    /// The configuration for the linter
    #[bpaf(external(linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<LinterConfiguration>,

    /// Specific configuration for the JavaScript language
    #[bpaf(external(js_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript: Option<JsConfiguration>,

    /// Specific configuration for the Json language
    #[bpaf(external(json_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonConfiguration>,

    /// Specific configuration for the Css language
    #[bpaf(external(css_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<CssConfiguration>,

    /// Specific configuration for the GraphQL language
    #[bpaf(external(graphql_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql: Option<GraphqlConfiguration>,

    /// Specific configuration for the GraphQL language
    #[bpaf(external(grit_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit: Option<GritConfiguration>,

    /// Specific configuration for the HTML language
    #[bpaf(external(html_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<HtmlConfiguration>,

    /// A list of granular patterns that should be applied only to a sub set of files
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Overrides>,

    /// List of plugins to load.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<biome_plugin_loader::Plugins>,

    /// Specific configuration for assists
    #[bpaf(external(assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<AssistConfiguration>,
}

impl DeserializableValidator for Configuration {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        if self.root.is_some_and(|root| root.value())
            && self
                .extends
                .as_ref()
                .is_some_and(|extends| extends.extends_root())
        {
            ctx.report(
                DeserializationDiagnostic::new(
                    markup!(<Warn>"The configuration file cannot be both a root of a Biome project and extended from a root configuration."</Warn>),
                )
                    .with_range(range)
                    .with_custom_severity(Severity::Error)
            )
        }

        true
    }
}

impl Configuration {
    /// Returns the initial configuration as generated by `biome init`.
    pub fn init() -> Self {
        Self {
            schema: Some(format!("https://biomejs.dev/schemas/{VERSION}/schema.json").into()),
            vcs: Some(VcsConfiguration {
                enabled: Some(false.into()),
                client_kind: Some(VcsClientKind::Git),
                use_ignore_file: Some(false.into()),
                ..Default::default()
            }),
            files: Some(FilesConfiguration {
                ignore_unknown: Some(false.into()),
                ..Default::default()
            }),
            formatter: Some(FormatterConfiguration {
                enabled: Some(true.into()),
                indent_style: Some(IndentStyle::Tab),
                ..Default::default()
            }),
            linter: Some(LinterConfiguration {
                enabled: Some(true.into()),
                rules: Some(Rules {
                    recommended: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            assist: Some(AssistConfiguration {
                enabled: Some(true.into()),
                actions: Some(Actions {
                    source: Some(Source {
                        organize_imports: Some(RuleAssistConfiguration::Plain(
                            crate::analyzer::RuleAssistPlainConfiguration::On,
                        )),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            javascript: Some(JsConfiguration {
                formatter: Some(JsFormatterConfiguration {
                    quote_style: Some(QuoteStyle::Double),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// Whether these settings belong to a root.
    ///
    /// If [`Self::root`] is `None`, this uses the result of
    /// [`Self::extends_root()`] to determine an implicit default (a config that
    /// extends the root cannot be a root itself).
    pub fn is_root(&self) -> bool {
        match self.root {
            Some(root) => root.value(),
            None => !self.extends_root(),
        }
    }

    pub fn extends_root(&self) -> bool {
        self.extends.as_ref().is_some_and(Extends::extends_root)
    }

    pub fn get_formatter_configuration(&self) -> FormatterConfiguration {
        self.formatter.clone().unwrap_or_default()
    }

    pub fn get_javascript_formatter_configuration(&self) -> JsFormatterConfiguration {
        self.javascript
            .as_ref()
            .and_then(|lang| lang.formatter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_javascript_linter_configuration(&self) -> JsLinterConfiguration {
        self.javascript
            .as_ref()
            .and_then(|lang| lang.linter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_json_formatter_configuration(&self) -> JsonFormatterConfiguration {
        self.json
            .as_ref()
            .and_then(|lang| lang.formatter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn is_formatter_enabled(&self) -> bool {
        self.formatter.as_ref().is_some_and(|f| f.is_enabled())
    }

    pub fn is_linter_enabled(&self) -> bool {
        self.linter.as_ref().is_some_and(|f| f.is_enabled())
    }

    pub fn is_assist_enabled(&self) -> bool {
        self.assist.as_ref().is_some_and(|f| f.is_enabled())
    }

    pub fn get_linter_rules(&self) -> Rules {
        self.linter
            .as_ref()
            .map(|f| f.get_rules())
            .unwrap_or_default()
    }

    pub fn get_linter_domains(&self) -> Option<&RuleDomains> {
        self.linter.as_ref().and_then(|l| l.domains.as_ref())
    }

    pub fn get_assist_actions(&self) -> Actions {
        self.assist
            .as_ref()
            .map(|f| f.get_actions())
            .unwrap_or_default()
    }

    pub fn is_vcs_enabled(&self) -> bool {
        self.vcs.as_ref().is_some_and(|v| v.is_enabled())
    }

    pub fn use_ignore_file(&self) -> bool {
        self.is_vcs_enabled()
            && self
                .vcs
                .as_ref()
                .is_some_and(|vcs| vcs.use_ignore_file == Some(true.into()))
    }

    /// Whether Biome should check for `.editorconfig` file
    pub fn use_editorconfig(&self) -> bool {
        self.formatter
            .as_ref()
            .is_some_and(|c| c.use_editorconfig_resolved())
    }

    pub fn get_json_linter_configuration(&self) -> JsonLinterConfiguration {
        self.json
            .as_ref()
            .and_then(|lang| lang.linter.clone())
            .unwrap_or_default()
    }

    pub fn get_css_parser_configuration(&self) -> CssParserConfiguration {
        self.css
            .as_ref()
            .and_then(|lang| lang.parser.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_css_formatter_configuration(&self) -> CssFormatterConfiguration {
        self.css
            .as_ref()
            .and_then(|lang| lang.formatter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_css_linter_configuration(&self) -> CssLinterConfiguration {
        self.css
            .as_ref()
            .and_then(|lang| lang.linter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_graphql_formatter_configuration(&self) -> GraphqlFormatterConfiguration {
        self.graphql
            .as_ref()
            .and_then(|lang| lang.formatter.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_graphql_linter_configuration(&self) -> GraphqlLinterConfiguration {
        self.graphql
            .as_ref()
            .and_then(|lang| lang.linter.as_ref())
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Merge)]
#[serde(deny_unknown_fields, default, rename_all = "camelCase")]
pub struct Schema(String);

impl FromStr for Schema {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<String> for Schema {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Schema {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Schema {
    fn schema_name() -> String {
        "Schema".into()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(generator)
    }
}

static SCHEMA_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https://biomejs.dev/schemas/([\d.]+)/schema.json").unwrap());

impl Deserializable for Schema {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Schema;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::STR;

            fn visit_str(
                self,
                ctx: &mut impl DeserializationContext,
                value: Text,
                range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                if let Some(captures) = SCHEMA_REGEX.captures(value.text())
                    && let Some(config_version_match) = captures.get(1)
                {
                    let cli_version = Version::new(VERSION);
                    let config_version_str = Version::new(config_version_match.as_str());
                    match config_version_str.cmp(&cli_version) {
                            Ordering::Less | Ordering::Greater => {
                                ctx.report(
                                    DeserializationDiagnostic::new(
                                        markup!(<Warn>"The configuration schema version does not match the CLI version " {VERSION}</Warn>),
                                    )
                                        .with_range(range)
                                        .with_custom_severity(Severity::Information)
                                        .with_note(markup!(
                                        {KeyValuePair("Expected", markup!({VERSION}))}
                                        {KeyValuePair("Found", markup!({config_version_str}))}
                                    ))
                                        .with_note(markup!("Run the command "<Emphasis>"biome migrate"</Emphasis>" to migrate the configuration file."))
                                )


                            }
                            _ => {},
                        }
                }

                Some(Schema(value.text().into()))
            }
        }

        value.deserialize(ctx, Visitor, name)
    }
}

#[derive(PartialEq, Eq)]
pub struct Version<'a>(&'a str);

impl<'a> Version<'a> {
    pub fn new(version: &'a str) -> Self {
        Version(version)
    }

    fn parse_version(&self) -> Vec<u32> {
        self.0
            .split('.')
            .filter_map(|part| part.parse::<u32>().ok())
            .collect()
    }
}

impl PartialOrd for Version<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_parts = self.parse_version();
        let other_parts = other.parse_version();

        for (a, b) in self_parts.iter().zip(other_parts.iter()) {
            match a.cmp(b) {
                Ordering::Equal => {}
                non_eq => return non_eq,
            }
        }

        self_parts.len().cmp(&other_parts.len())
    }
}

impl Display for Version<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::io::Error> {
        write!(f, "{}", self.0)
    }
}

pub type FilesIgnoreUnknownEnabled = Bool<false>;

/// The configuration of the filesystem
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons. Defaults to 1 MiB
    #[bpaf(long("files-max-size"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<MaxSize>,

    /// Tells Biome to not emit diagnostics when handling files that it doesn't know
    #[bpaf(long("files-ignore-unknown"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unknown: Option<FilesIgnoreUnknownEnabled>,

    /// A list of glob patterns. Biome will handle only those files/folders that will
    /// match these patterns.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::NormalizedGlob>>,

    /// **Deprecated:** Please use _force-ignore syntax_ in `files.includes`
    /// instead: https://biomejs.dev/reference/configuration/#filesincludes
    ///
    /// Set of file and folder names that should be unconditionally ignored by
    /// Biome's scanner.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_scanner_ignores: Option<Vec<String>>,
}

impl FilesConfiguration {
    fn deserialize_field(
        &mut self,
        ctx: &mut impl DeserializationContext,
        name: &str,
        value: &impl DeserializableValue,
        range: TextRange,
    ) {
        match name {
            "maxSize" => {
                if let Some(value) = Deserializable::deserialize(ctx, value, name) {
                    self.max_size = value;
                }
            }
            "ignoreUnknown" => {
                if let Some(value) = Deserializable::deserialize(ctx, value, name) {
                    self.ignore_unknown = value;
                }
            }
            "includes" => {
                if let Some(value) = Deserializable::deserialize(ctx, value, name) {
                    self.includes = value;
                }
            }
            "experimentalScannerIgnores" => {
                if let Some(value) = Deserializable::deserialize(ctx, value, name) {
                    self.experimental_scanner_ignores = value;

                    let mut diagnostic = DeserializationDiagnostic::new_deprecated(name, range);
                    if let Some(ignores) = &self.experimental_scanner_ignores {
                        let suggestions: Vec<String> = ignores
                            .iter()
                            .filter(|entry| {
                                !DEFAULT_SCANNER_IGNORE_ENTRIES.contains(&entry.as_bytes())
                            })
                            .map(|entry| format!("\"!!**/{entry}\""))
                            .collect();
                        diagnostic = diagnostic.note_with_list(
                            markup! {
                                "You may want to add the following entries to "
                                <Emphasis>"files.includes"</Emphasis>" instead:"
                            },
                            &suggestions,
                        );
                    }
                    let link = "https://biomejs.dev/reference/configuration/#filesincludes";
                    ctx.report(diagnostic.with_note(markup! {
                        "See "<Hyperlink href={link}>"the files.includes documentation"</Hyperlink>
                        " for more information."
                    }));
                }
            }
            unknown_key => {
                const ALLOWED_KEYS: &[&str] = &[
                    "maxSize",
                    "ignoreUnknown",
                    "includes",
                    "experimentalScannerIgnores",
                ];
                ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    range,
                    ALLOWED_KEYS,
                ))
            }
        }
    }
}

impl biome_deserialize::Deserializable for FilesConfiguration {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = FilesConfiguration;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut result = FilesConfiguration::default();
                for (key, value) in members.flatten() {
                    if let Some(key_text) = Text::deserialize(ctx, &key, "") {
                        result.deserialize_field(ctx, key_text.text(), &value, key.range());
                    }
                }
                Some(result)
            }
        }

        value.deserialize(ctx, Visitor, name)
    }
}

#[derive(Debug)]
pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<Configuration>,
    /// The path of where the `biome.json` or `biome.jsonc` file was found. This contains the file name.
    pub configuration_file_path: Utf8PathBuf,
    /// The base path where the external configuration in a package should be resolved from
    pub external_resolution_base_path: Utf8PathBuf,
}

#[derive(Debug, Default, PartialEq, Clone, Eq, Hash)]
pub enum ConfigurationPathHint {
    /// The default mode, not having a configuration file is not an error.
    /// The path will be filled with the working directory if it is not filled at the time of usage.
    #[default]
    None,

    /// Very similar to [ConfigurationPathHint::None]. However, the path provided by this variant
    /// will be used as **working directory**, which means that all globs defined in the configuration
    /// will use **this path** as base path.
    FromWorkspace(Utf8PathBuf),

    /// The configuration path provided by the LSP, not having a configuration file is not an error.
    /// The path will always be a directory path.
    FromLsp(Utf8PathBuf),

    /// The configuration path provided by the user, not having a configuration file is an error.
    /// The path can either be a directory path or a file path.
    /// Throws any kind of I/O errors.
    FromUser(Utf8PathBuf),
}

impl Display for ConfigurationPathHint {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::None => write!(fmt, "Configuration file not provided.",),
            Self::FromWorkspace(path) => {
                write!(fmt, "Configuration path provided from a workspace: {path}",)
            }
            Self::FromLsp(path) => {
                write!(fmt, "Configuration path provided from the LSP: {path}",)
            }
            Self::FromUser(path) => {
                write!(fmt, "Configuration path provided by the user: {path}",)
            }
        }
    }
}

impl ConfigurationPathHint {
    pub const fn is_from_user(&self) -> bool {
        matches!(self, Self::FromUser(_))
    }
    pub const fn is_from_lsp(&self) -> bool {
        matches!(self, Self::FromLsp(_))
    }

    pub fn to_path_buf(&self) -> Option<Utf8PathBuf> {
        match self {
            Self::None => None,
            Self::FromWorkspace(path) | Self::FromLsp(path) | Self::FromUser(path) => {
                Some(path.to_path_buf())
            }
        }
    }
}

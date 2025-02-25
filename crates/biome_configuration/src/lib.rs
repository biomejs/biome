//! This module contains the configuration of `biome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further option divided by tool.
pub mod analyzer;
pub mod bool;
pub mod css;
pub mod diagnostics;
pub mod editorconfig;
pub mod formatter;
pub mod generated;
pub mod graphql;
pub mod grit;
pub mod html;
pub mod javascript;
pub mod json;
pub mod max_size;
mod overrides;
pub mod plugins;
pub mod vcs;

use crate::analyzer::assist::{assist_configuration, Actions, AssistConfiguration, Source};
use crate::analyzer::RuleAssistConfiguration;
use crate::bool::Bool;
use crate::css::{CssFormatterConfiguration, CssLinterConfiguration, CssParserConfiguration};
pub use crate::diagnostics::BiomeDiagnostic;
pub use crate::diagnostics::CantLoadExtendFile;
pub use crate::generated::{push_to_analyzer_assist, push_to_analyzer_rules};
use crate::graphql::{GraphqlFormatterConfiguration, GraphqlLinterConfiguration};
pub use crate::grit::{grit_configuration, GritConfiguration};
use crate::javascript::{JsFormatterConfiguration, JsLinterConfiguration};
use crate::json::{JsonFormatterConfiguration, JsonLinterConfiguration};
use crate::max_size::MaxSize;
use crate::vcs::{vcs_configuration, VcsConfiguration};
pub use analyzer::{
    linter_configuration, LinterConfiguration, RuleConfiguration, RuleFixConfiguration,
    RulePlainConfiguration, RuleWithFixOptions, RuleWithOptions, Rules,
};
use biome_console::fmt::{Display, Formatter};
use biome_deserialize::Deserialized;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, QuoteStyle};
use bpaf::Bpaf;
use camino::Utf8PathBuf;
pub use css::{css_configuration, CssConfiguration};
pub use formatter::{formatter_configuration, FormatterConfiguration};
pub use graphql::{graphql_configuration, GraphqlConfiguration};
pub use html::{html_configuration, HtmlConfiguration};
pub use javascript::{js_configuration, JsConfiguration};
pub use json::{json_configuration, JsonConfiguration};
pub use overrides::{
    OverrideAssistConfiguration, OverrideFormatterConfiguration, OverrideGlobs,
    OverrideLinterConfiguration, OverridePattern, Overrides,
};
use plugins::Plugins;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::num::NonZeroU64;
use vcs::VcsClientKind;

pub const VERSION: &str = match option_env!("BIOME_VERSION") {
    Some(version) => version,
    None => "0.0.0",
};

/// Limit the size of files to 1.0 MiB by default
pub const DEFAULT_FILE_SIZE_LIMIT: NonZeroU64 =
    // SAFETY: This constant is initialized with a non-zero value
    unsafe { NonZeroU64::new_unchecked(1024 * 1024) };

/// The configuration that is contained inside the file `biome.json`
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, default, rename_all = "camelCase")]
pub struct Configuration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[serde(rename = "$schema")]
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<str>>,

    /// Indicates whether this configuration file is at the root of a Biome
    /// project. By default, this is `true`.
    #[bpaf(hide, hide_usage)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Bool<false>>,

    /// A list of paths to other JSON files, used to extends the current configuration.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<Vec<Box<str>>>,

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
    pub plugins: Option<Plugins>,

    /// Specific configuration for assists
    #[bpaf(external(assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<AssistConfiguration>,
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

    pub fn get_assist_actions(&self) -> Actions {
        self.assist
            .as_ref()
            .map(|f| f.get_actions())
            .unwrap_or_default()
    }

    pub fn is_vcs_enabled(&self) -> bool {
        self.assist.as_ref().is_some_and(|f| f.is_enabled())
    }

    /// Whether Biome should check for `.editorconfig` file
    pub fn use_editorconfig(&self) -> bool {
        self.formatter
            .as_ref()
            .and_then(|f| f.use_editorconfig)
            .is_some_and(|editorconfig| editorconfig.value())
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

pub type FilesIgnoreUnknownEnabled = Bool<false>;

/// The configuration of the filesystem
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons. Defaults to 1 MiB
    #[bpaf(long("files-max-size"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<MaxSize>,

    /// Tells Biome to not emit diagnostics when handling files that doesn't know
    #[bpaf(long("files-ignore-unknown"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unknown: Option<FilesIgnoreUnknownEnabled>,

    /// A list of glob patterns. Biome will handle only those files/folders that will
    /// match these patterns.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::Glob>>,
}

#[derive(Debug)]
pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<Configuration>,
    /// The path of where the `biome.json` or `biome.jsonc` file was found. This contains the file name.
    pub configuration_file_path: Utf8PathBuf,
    /// The base path where the external configuration in a package should be resolved from
    pub external_resolution_base_path: Utf8PathBuf,
    /// Whether `biome.json` and `biome.jsonc` were found in the same folder
    pub double_configuration_found: bool,
}

#[derive(Debug, Default, PartialEq, Clone)]
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
            ConfigurationPathHint::None => write!(fmt, "Configuration file not provided.",),
            ConfigurationPathHint::FromWorkspace(path) => write!(
                fmt,
                "Configuration path provided from a workspace: {}",
                path
            ),
            ConfigurationPathHint::FromLsp(path) => {
                write!(fmt, "Configuration path provided from the LSP: {}", path,)
            }
            ConfigurationPathHint::FromUser(path) => {
                write!(fmt, "Configuration path provided by the user: {}", path,)
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
}

#[cfg(test)]
mod test {
    use oxc_resolver::{FileMetadata, FsCache, ResolveOptions, ResolverGeneric};
    use std::env;
    use std::fs::read_link;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    #[test]
    fn resolver_test() {
        #[derive(Debug, Default)]
        struct Test;

        impl oxc_resolver::FileSystem for Test {
            fn read_to_string(&self, _path: &Path) -> std::io::Result<String> {
                Ok(String::from(
                    r#"{ "name": "example", "exports": { "./biome": "./biome.json" }}"#,
                ))
            }

            fn metadata(&self, _path: &Path) -> std::io::Result<FileMetadata> {
                Ok(FileMetadata::new(true, false, false))
            }

            fn symlink_metadata(&self, _path: &Path) -> std::io::Result<FileMetadata> {
                Ok(FileMetadata::new(true, false, false))
            }

            fn read_link(&self, path: &Path) -> std::io::Result<PathBuf> {
                read_link(path)
            }
        }

        let resolver = ResolverGeneric::new_with_cache(
            Arc::new(FsCache::new(Test {})),
            ResolveOptions {
                condition_names: vec!["node".to_string(), "import".to_string()],
                extensions: vec![".json".to_string()],
                ..ResolveOptions::default()
            },
        );

        let result = resolver
            .resolve(
                env::current_dir()
                    .unwrap()
                    .canonicalize()
                    .unwrap()
                    .display()
                    .to_string(),
                "example/biome",
            )
            .unwrap();

        dbg!(&result);
    }
}

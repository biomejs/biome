//! This module contains the configuration of `biome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.
pub mod analyzer;
pub mod css;
pub mod diagnostics;
pub mod editorconfig;
pub mod formatter;
pub mod generated;
pub mod graphql;
pub mod javascript;
pub mod json;
pub mod organize_imports;
mod overrides;
pub mod vcs;

use crate::analyzer::assists::{
    partial_assists_configuration, AssistsConfiguration, PartialAssistsConfiguration,
};
use crate::css::CssLinter;
pub use crate::diagnostics::BiomeDiagnostic;
pub use crate::diagnostics::CantLoadExtendFile;
pub use crate::generated::{push_to_analyzer_assists, push_to_analyzer_rules};
use crate::javascript::JavascriptLinter;
use crate::json::JsonLinter;
use crate::organize_imports::{partial_organize_imports, OrganizeImports, PartialOrganizeImports};
use crate::vcs::{partial_vcs_configuration, PartialVcsConfiguration, VcsConfiguration};
pub use analyzer::{
    partial_linter_configuration, LinterConfiguration, PartialLinterConfiguration,
    RuleConfiguration, RuleFixConfiguration, RulePlainConfiguration, RuleWithFixOptions,
    RuleWithOptions, Rules,
};
use biome_deserialize::{Deserialized, StringSet};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{IndentStyle, QuoteStyle};
use bpaf::Bpaf;
pub use css::{
    partial_css_configuration, CssConfiguration, CssFormatter, PartialCssConfiguration,
    PartialCssFormatter,
};
pub use formatter::{
    partial_formatter_configuration, FormatterConfiguration, PartialFormatterConfiguration,
};
pub use graphql::{
    partial_graphql_configuration, GraphqlConfiguration, GraphqlFormatter, GraphqlLinter,
    PartialGraphqlConfiguration, PartialGraphqlFormatter, PartialGraphqlLinter,
};
pub use javascript::{
    partial_javascript_configuration, JavascriptConfiguration, JavascriptFormatter,
    PartialJavascriptConfiguration, PartialJavascriptFormatter,
};
pub use json::{
    partial_json_configuration, JsonConfiguration, JsonFormatter, PartialJsonConfiguration,
    PartialJsonFormatter,
};
pub use overrides::{
    OverrideAssistsConfiguration, OverrideFormatterConfiguration, OverrideLinterConfiguration,
    OverrideOrganizeImportsConfiguration, OverridePattern, Overrides,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::num::NonZeroU64;
use std::path::PathBuf;
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
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct Configuration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[partial(serde(rename = "$schema"))]
    #[partial(bpaf(hide))]
    pub schema: String,

    /// A list of paths to other JSON files, used to extends the current configuration.
    #[partial(bpaf(hide))]
    pub extends: StringSet,

    /// The configuration of the VCS integration
    #[partial(type, bpaf(external(partial_vcs_configuration), optional, hide_usage))]
    pub vcs: VcsConfiguration,

    /// The configuration of the filesystem
    #[partial(
        type,
        bpaf(external(partial_files_configuration), optional, hide_usage)
    )]
    pub files: FilesConfiguration,

    /// The configuration of the formatter
    #[partial(type, bpaf(external(partial_formatter_configuration), optional))]
    pub formatter: FormatterConfiguration,

    /// The configuration of the import sorting
    #[partial(type, bpaf(external(partial_organize_imports), optional))]
    pub organize_imports: OrganizeImports,

    /// The configuration for the linter
    #[partial(type, bpaf(external(partial_linter_configuration), optional))]
    pub linter: LinterConfiguration,

    /// Specific configuration for the JavaScript language
    #[partial(type, bpaf(external(partial_javascript_configuration), optional))]
    pub javascript: JavascriptConfiguration,

    /// Specific configuration for the Json language
    #[partial(type, bpaf(external(partial_json_configuration), optional))]
    pub json: JsonConfiguration,

    /// Specific configuration for the Css language
    #[partial(type, bpaf(external(partial_css_configuration), optional))]
    pub css: CssConfiguration,

    /// Specific configuration for the GraphQL language
    #[partial(type, bpaf(external(partial_graphql_configuration), optional))]
    pub graphql: GraphqlConfiguration,

    /// A list of granular patterns that should be applied only to a sub set of files
    #[partial(bpaf(hide))]
    pub overrides: Overrides,

    /// Specific configuration for assists
    #[partial(type, bpaf(external(partial_assists_configuration), optional))]
    pub assists: AssistsConfiguration,
}

impl PartialConfiguration {
    /// Returns the initial configuration as generated by `biome init`.
    pub fn init() -> Self {
        Self {
            schema: Some(format!("https://biomejs.dev/schemas/{VERSION}/schema.json")),
            vcs: Some(PartialVcsConfiguration {
                enabled: Some(false),
                client_kind: Some(VcsClientKind::Git),
                use_ignore_file: Some(false),
                ..Default::default()
            }),
            files: Some(PartialFilesConfiguration {
                ignore_unknown: Some(false),
                ignore: Some(Default::default()),
                ..Default::default()
            }),
            formatter: Some(PartialFormatterConfiguration {
                enabled: Some(true),
                indent_style: Some(IndentStyle::Tab),
                ..Default::default()
            }),
            organize_imports: Some(PartialOrganizeImports {
                enabled: Some(true),
                ..Default::default()
            }),
            linter: Some(PartialLinterConfiguration {
                enabled: Some(true),
                rules: Some(Rules {
                    recommended: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            javascript: Some(PartialJavascriptConfiguration {
                formatter: Some(PartialJavascriptFormatter {
                    quote_style: Some(QuoteStyle::Double),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn is_formatter_disabled(&self) -> bool {
        self.formatter.as_ref().map_or(false, |f| f.is_disabled())
    }

    pub fn get_formatter_configuration(&self) -> FormatterConfiguration {
        self.formatter
            .as_ref()
            .map(|f| f.get_formatter_configuration())
            .unwrap_or_default()
    }

    pub fn get_javascript_formatter_configuration(&self) -> JavascriptFormatter {
        self.javascript
            .as_ref()
            .map(|f| {
                f.formatter
                    .as_ref()
                    .map(|f| f.get_formatter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_javascript_linter_configuration(&self) -> JavascriptLinter {
        self.javascript
            .as_ref()
            .map(|f| {
                f.linter
                    .as_ref()
                    .map(|f| f.get_linter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_json_formatter_configuration(&self) -> JsonFormatter {
        self.json
            .as_ref()
            .map(|f| {
                f.formatter
                    .as_ref()
                    .map(|f| f.get_formatter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_json_linter_configuration(&self) -> JsonLinter {
        self.json
            .as_ref()
            .map(|f| {
                f.linter
                    .as_ref()
                    .map(|f| f.get_linter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_css_formatter_configuration(&self) -> CssFormatter {
        self.css
            .as_ref()
            .map(|f| {
                f.formatter
                    .as_ref()
                    .map(|f| f.get_formatter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_css_linter_configuration(&self) -> CssLinter {
        self.css
            .as_ref()
            .map(|f| {
                f.linter
                    .as_ref()
                    .map(|f| f.get_linter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
    pub fn get_graphql_linter_configuration(&self) -> GraphqlLinter {
        self.graphql
            .as_ref()
            .map(|f| {
                f.linter
                    .as_ref()
                    .map(|f| f.get_linter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn get_graphql_formatter_configuration(&self) -> GraphqlFormatter {
        self.graphql
            .as_ref()
            .map(|f| {
                f.formatter
                    .as_ref()
                    .map(|f| f.get_formatter_configuration())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn is_linter_disabled(&self) -> bool {
        self.linter.as_ref().map_or(false, |f| f.is_disabled())
    }

    pub fn get_linter_rules(&self) -> Rules {
        self.linter
            .as_ref()
            .map(|f| f.get_rules())
            .unwrap_or_default()
    }

    pub fn is_organize_imports_disabled(&self) -> bool {
        self.organize_imports
            .as_ref()
            .map_or(false, |f| f.is_disabled())
    }

    pub fn is_vcs_disabled(&self) -> bool {
        self.vcs.as_ref().map_or(true, |f| f.is_disabled())
    }

    pub fn is_vcs_enabled(&self) -> bool {
        !self.is_vcs_disabled()
    }

    /// Whether Biome should check for `.editorconfig` file
    pub fn use_editorconfig(&self) -> Option<bool> {
        self.formatter.as_ref().and_then(|f| f.use_editorconfig)
    }
}

/// The configuration of the filesystem
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons. Defaults to 1 MiB
    #[partial(bpaf(long("files-max-size"), argument("NUMBER")))]
    pub max_size: NonZeroU64,

    /// Tells Biome to not emit diagnostics when handling files that doesn't know
    #[partial(bpaf(long("files-ignore-unknown"), argument("true|false"), optional))]
    pub ignore_unknown: bool,

    /// A list of Unix shell style patterns. Biome will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. Biome will handle only those files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl Default for FilesConfiguration {
    fn default() -> Self {
        Self {
            max_size: DEFAULT_FILE_SIZE_LIMIT,
            ignore: Default::default(),
            include: Default::default(),
            ignore_unknown: false,
        }
    }
}

pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<PartialConfiguration>,
    /// The path of where the `biome.json` or `biome.jsonc` file was found. This contains the file name.
    pub configuration_file_path: PathBuf,
    /// The base path where the external configuration in a package should be resolved from
    pub external_resolution_base_path: PathBuf,
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
    FromWorkspace(PathBuf),

    /// The configuration path provided by the LSP, not having a configuration file is not an error.
    /// The path will always be a directory path.
    FromLsp(PathBuf),
    /// The configuration path provided by the user, not having a configuration file is an error.
    /// The path can either be a directory path or a file path.
    /// Throws any kind of I/O errors.
    FromUser(PathBuf),
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
    use oxc_resolver::{FileMetadata, ResolveOptions, ResolverGeneric};
    use std::env;
    use std::path::{Path, PathBuf};

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

            fn canonicalize(&self, _path: &Path) -> std::io::Result<PathBuf> {
                env::current_dir().unwrap().canonicalize()
            }
        }

        let resolver = ResolverGeneric::new_with_file_system(
            Test {},
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

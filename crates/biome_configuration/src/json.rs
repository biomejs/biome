use crate::{bool::Bool, PlainIndentStyle};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentWidth, LineEnding, LineWidth};
use biome_json_formatter::context::TrailingCommas;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    /// Parsing options
    #[bpaf(external(json_parser_configuration), optional)]
    pub parser: Option<JsonParserConfiguration>,

    /// Formatting options
    #[bpaf(external(json_formatter_configuration), optional)]
    pub formatter: Option<JsonFormatterConfiguration>,

    /// Linting options
    #[bpaf(external(json_linter_configuration), optional)]
    pub linter: Option<JsonLinterConfiguration>,
}

pub type AllowCommentsEnabled = Bool<false>;
pub type AllowTrailingCommasEnabled = Bool<false>;

/// Options that changes how the JSON parser behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParserConfiguration {
    #[bpaf(hide)]
    /// Allow parsing comments in `.json` files
    pub allow_comments: Option<AllowCommentsEnabled>,

    #[bpaf(hide)]
    /// Allow parsing trailing commas in `.json` files
    pub allow_trailing_commas: Option<AllowTrailingCommasEnabled>,
}

pub type JsonFormatterEnabled = Bool<true>;

#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonFormatterConfiguration {
    /// Control the formatter for JSON (and its super languages) files.
    #[bpaf(long("json-formatter-enabled"), argument("true|false"))]
    pub enabled: Option<JsonFormatterEnabled>,

    /// The indent style applied to JSON (and its super languages) files.
    #[bpaf(long("json-formatter-indent-style"), argument("tab|space"))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[bpaf(long("json-formatter-indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[bpaf(long("json-formatter-indent-size"), argument("NUMBER"))]
    #[deserializable(deprecated(use_instead = "json.formatter.indentWidth"))]
    pub indent_size: Option<IndentWidth>,

    /// The type of line ending applied to JSON (and its super languages) files.
    #[bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[bpaf(long("json-formatter-line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    #[bpaf(long("json-formatter-trailing-commas"), argument("none|all"))]
    pub trailing_commas: Option<TrailingCommas>,
}

impl JsonFormatterConfiguration {
    pub fn enabled_resolved(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn trailing_commas_resolved(&self) -> TrailingCommas {
        self.trailing_commas.unwrap_or_default()
    }
}

pub type JsonLinterEnabled = Bool<true>;

/// Linter options specific to the JSON linter
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonLinterConfiguration {
    /// Control the linter for JSON (and its super languages) files.
    #[bpaf(long("json-linter-enabled"), argument("true|false"))]
    pub enabled: Option<JsonLinterEnabled>,
}

impl JsonLinterConfiguration {
    pub fn enabled_resolved(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{BracketSpacing, Expand, IndentStyle, IndentWidth, LineEnding, LineWidth};
use biome_json_formatter::context::TrailingCommas;
use clap::Args;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    /// Parsing options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<JsonParserConfiguration>,

    /// Formatting options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JsonFormatterConfiguration>,

    /// Linting options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<JsonLinterConfiguration>,

    /// Assist options
    #[clap(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<JsonAssistConfiguration>,
}

pub type JsonAllowCommentsEnabled = Bool<false>;
pub type JsonAllowTrailingCommasEnabled = Bool<false>;

/// Options that changes how the JSON parser behaves
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParserConfiguration {
    #[arg(long = "json-parse-allow-comments", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing comments in `.json` files
    pub allow_comments: Option<JsonAllowCommentsEnabled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long = "json-parse-allow-trailing-commas", value_name = "true|false")]
    /// Allow parsing trailing commas in `.json` files
    pub allow_trailing_commas: Option<JsonAllowTrailingCommasEnabled>,
}

pub type JsonFormatterEnabled = Bool<true>;

#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(skip)]
pub struct JsonFormatterConfiguration {
    /// Control the formatter for JSON (and its super languages) files.
    #[arg(long = "json-formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonFormatterEnabled>,

    /// The indent style applied to JSON (and its super languages) files.
    #[arg(long = "json-formatter-indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[arg(long = "json-formatter-indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to JSON (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[arg(long = "json-formatter-line-ending", value_name = "lf|crlf|cr|auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[arg(long = "json-formatter-line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    #[arg(long = "json-formatter-trailing-commas", value_name = "none|all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether to expand arrays and objects on multiple lines.
    /// When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
    /// and array literals are formatted on a single line if it fits in the line.
    /// When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
    /// When set to `never`, these literals are formatted on a single line if it fits in the line.
    /// When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
    #[arg(long = "json-formatter-expand", value_name = "auto|always|never")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[arg(long = "json-formatter-bracket-spacing", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,
}

impl JsonFormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn trailing_commas_resolved(&self) -> TrailingCommas {
        self.trailing_commas.unwrap_or_default()
    }
}

pub type JsonLinterEnabled = Bool<true>;

/// Linter options specific to the JSON linter
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonLinterConfiguration {
    /// Control the linter for JSON (and its super languages) files.
    #[arg(long = "json-linter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonLinterEnabled>,
}

pub type JsonAssistEnabled = Bool<true>;
/// Assist options specific to the JSON linter
#[derive(
    Args, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonAssistConfiguration {
    /// Control the assist for JSON (and its super languages) files.
    #[arg(long = "json-assist-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonAssistEnabled>,
}

impl JsonLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

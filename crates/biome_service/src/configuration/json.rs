use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_deserialize_macros::{Mergeable, NoneState};
use biome_formatter::{LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Eq, Mergeable, NoneState, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_parser), optional)]
    pub parser: Option<JsonParser>,

    /// Formatting options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_formatter), optional)]
    pub formatter: Option<JsonFormatter>,
}

/// Options that changes how the JSON parser behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Eq, Mergeable, NoneState, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing comments in `.json` files
    pub allow_comments: Option<bool>,
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing trailing commas in `.json` files
    pub allow_trailing_commas: Option<bool>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Eq, Mergeable, NoneState, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonFormatter {
    /// Control the formatter for JSON (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-enabled"), argument("true|false"), optional)]
    pub enabled: Option<bool>,

    /// The indent style applied to JSON (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<u8>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// The type of line ending applied to JSON (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,
}

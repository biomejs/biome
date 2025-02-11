use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, ObjectWrap,
};
use biome_json_formatter::context::{Expand, TrailingCommas};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<JsonParserConfiguration>,

    /// Formatting options
    #[bpaf(external(json_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JsonFormatterConfiguration>,

    /// Linting options
    #[bpaf(external(json_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<JsonLinterConfiguration>,

    /// Assist options
    #[bpaf(external(json_assist_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<JsonAssistConfiguration>,
}

pub type JsonAllowCommentsEnabled = Bool<false>;
pub type JsonAllowTrailingCommasEnabled = Bool<false>;

/// Options that changes how the JSON parser behaves
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParserConfiguration {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing comments in `.json` files
    pub allow_comments: Option<JsonAllowCommentsEnabled>,

    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow parsing trailing commas in `.json` files
    pub allow_trailing_commas: Option<JsonAllowTrailingCommasEnabled>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonFormatterEnabled>,

    /// The indent style applied to JSON (and its super languages) files.
    #[bpaf(long("json-formatter-indent-style"), argument("tab|space"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[bpaf(long("json-formatter-indent-width"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to JSON (and its super languages) files.
    #[bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[bpaf(long("json-formatter-line-width"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    #[bpaf(long("json-formatter-trailing-commas"), argument("none|all"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether to expand arrays and objects on multiple lines. When set to `always`, these literals are formatted on multiple lines,
    /// regardless of length of the list. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "followSource".
    #[bpaf(
        long("json-formatter-expand"),
        argument("always|follow-source"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("json-formatter-bracket-spacing"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to enforce collapsing object literals when possible. Defaults to preserve.
    #[bpaf(long("json-formatter-object-wrap"), argument("preserve|collapse"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_wrap: Option<ObjectWrap>,
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
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonLinterConfiguration {
    /// Control the linter for JSON (and its super languages) files.
    #[bpaf(long("json-linter-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonLinterEnabled>,
}

pub type JsonAssistEnabled = Bool<true>;
/// Linter options specific to the JSON linter
#[derive(
    Bpaf, Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonAssistConfiguration {
    /// Control the assist for JSON (and its super languages) files.
    #[bpaf(long("json-assist-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonAssistEnabled>,
}

impl JsonLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

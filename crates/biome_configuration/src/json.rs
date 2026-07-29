use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    BracketSpacing, DelimiterSpacing, Expand, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TrailingNewline,
};
use biome_json_formatter::context::TrailingCommas;
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    /// Parsing options
    #[cfg_attr(feature = "cli", bpaf(external(json_parser_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<JsonParserConfiguration>,

    /// Formatting options
    #[cfg_attr(
        feature = "cli",
        bpaf(external(json_formatter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JsonFormatterConfiguration>,

    /// Linting options
    #[cfg_attr(feature = "cli", bpaf(external(json_linter_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<JsonLinterConfiguration>,

    /// Assist options
    #[cfg_attr(feature = "cli", bpaf(external(json_assist_configuration), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<JsonAssistConfiguration>,
}

pub type JsonAllowCommentsEnabled = Bool<false>;
pub type JsonAllowTrailingCommasEnabled = Bool<false>;

/// Options that change how the JSON parser behaves.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonParserConfiguration {
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-parse-allow-comments"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allows parsing comments in `.json` files.
    pub allow_comments: Option<JsonAllowCommentsEnabled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-parse-allow-trailing-commas"), argument("true|false"))
    )]
    /// Allows parsing trailing commas in `.json` files.
    pub allow_trailing_commas: Option<JsonAllowTrailingCommasEnabled>,
}

pub type JsonFormatterEnabled = Bool<true>;

#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonFormatterConfiguration {
    /// Controls the formatter for JSON and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonFormatterEnabled>,

    /// The indent style applied to JSON and languages that extend it. If unset, inherits the global
    /// indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-indent-style"), argument("tab|space"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to JSON and languages that extend it. If unset, inherits the
    /// global indentation width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-indent-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to JSON and languages that extend it. If unset, inherits the global
    /// line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr|auto"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// The maximum line width applied to JSON and languages that extend it. If unset, inherits the
    /// global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-line-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Prints trailing commas wherever possible in multiline comma-separated structures. Defaults
    /// to `none`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-trailing-commas"), argument("none|all"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Uses the same `auto`, `always`, and `never` behavior as the global expansion setting.
    ///
    /// If unset, inherits the global expansion setting.
    ///
    /// When formatting `package.json`, Biome uses `always` unless configured otherwise.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-expand"), argument("auto|always|never"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// Whether to insert spaces inside braces in object literals. If unset, inherits the global
    /// bracket spacing setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-bracket-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Controls spaces inside JSON square brackets when their content fits on one line. When
    /// enabled, `[1, 2, 3]` becomes `[ 1, 2, 3 ]`. Empty brackets are unchanged.
    ///
    /// If unset, inherits the global delimiter spacing setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-delimiter-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Whether to add a trailing newline at the end of the file. If unset, inherits the global
    /// trailing newline setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-formatter-trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
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
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonLinterConfiguration {
    /// Controls the linter for JSON and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-linter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonLinterEnabled>,
}

pub type JsonAssistEnabled = Bool<true>;
/// Assist options specific to the JSON linter
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsonAssistConfiguration {
    /// Controls assist actions for JSON and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("json-assist-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsonAssistEnabled>,
}

impl JsonLinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

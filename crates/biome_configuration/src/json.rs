use crate::PlainIndentStyle;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{IndentWidth, LineEnding, LineWidth};
use biome_json_formatter::context::TrailingCommas;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(default, deny_unknown_fields))]
pub struct JsonConfiguration {
    /// Parsing options
    #[partial(type, bpaf(external(partial_json_parser), optional))]
    pub parser: JsonParser,

    /// Formatting options
    #[partial(type, bpaf(external(partial_json_formatter), optional))]
    pub formatter: JsonFormatter,

    /// Linting options
    #[partial(type, bpaf(external(partial_json_linter), optional))]
    pub linter: JsonLinter,

    /// Assists options
    #[partial(type, bpaf(external(partial_json_assists), optional))]
    pub assists: JsonAssists,
}

/// Options that changes how the JSON parser behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JsonParser {
    #[partial(bpaf(hide))]
    /// Allow parsing comments in `.json` files
    pub allow_comments: bool,

    #[partial(bpaf(hide))]
    /// Allow parsing trailing commas in `.json` files
    pub allow_trailing_commas: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JsonFormatter {
    /// Control the formatter for JSON (and its super languages) files.
    #[partial(bpaf(long("json-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to JSON (and its super languages) files.
    #[partial(bpaf(long("json-formatter-indent-style"), argument("tab|space"), optional))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[partial(bpaf(long("json-formatter-indent-width"), argument("NUMBER"), optional))]
    pub indent_width: Option<IndentWidth>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[partial(bpaf(long("json-formatter-indent-size"), argument("NUMBER"), optional))]
    #[partial(deserializable(deprecated(use_instead = "json.formatter.indentWidth")))]
    pub indent_size: Option<IndentWidth>,

    /// The type of line ending applied to JSON (and its super languages) files.
    #[partial(bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[partial(bpaf(long("json-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
    #[partial(bpaf(long("json-formatter-trailing-commas"), argument("none|all"), optional))]
    pub trailing_commas: Option<TrailingCommas>,
}

impl PartialJsonFormatter {
    pub fn get_formatter_configuration(&self) -> JsonFormatter {
        JsonFormatter {
            enabled: self.enabled.unwrap_or_default(),
            indent_style: self.indent_style,
            indent_width: self.indent_width,
            indent_size: self.indent_size,
            line_ending: self.line_ending,
            line_width: self.line_width,
            trailing_commas: self.trailing_commas,
        }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self {
            enabled: true,
            indent_style: Default::default(),
            indent_width: Default::default(),
            indent_size: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            trailing_commas: Default::default(),
        }
    }
}

impl PartialJsonFormatter {
    pub fn get_linter_configuration(&self) -> JsonLinter {
        JsonLinter {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

/// Linter options specific to the JSON linter
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JsonLinter {
    /// Control the linter for JSON (and its super languages) files.
    #[partial(bpaf(long("json-linter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

impl Default for JsonLinter {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl PartialJsonLinter {
    pub fn get_linter_configuration(&self) -> JsonLinter {
        JsonLinter {
            enabled: self.enabled.unwrap_or_default(),
        }
    }
}

/// Linter options specific to the JSON linter
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JsonAssists {
    /// Control the linter for JSON (and its super languages) files.
    #[partial(bpaf(long("json-assists-enabled"), argument("true|false"), optional))]
    pub enabled: bool,
}

impl Default for JsonAssists {
    fn default() -> Self {
        Self { enabled: true }
    }
}

use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{LineEnding, LineWidth};
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
    pub indent_width: Option<u8>,

    /// The size of the indentation applied to JSON (and its super languages) files. Default to 2.
    #[partial(bpaf(long("json-formatter-indent-size"), argument("NUMBER"), optional))]
    #[partial(deserializable(deprecated(use_instead = "json.formatter.indentWidth")))]
    pub indent_size: Option<u8>,

    /// The type of line ending applied to JSON (and its super languages) files.
    #[partial(bpaf(long("json-formatter-line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
    #[partial(serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    ))]
    #[partial(bpaf(long("json-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,
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
        }
    }
}

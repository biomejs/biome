use crate::configuration::merge::MergeWith;
use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_formatter::LineWidth;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to JSON files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
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

impl JsonConfiguration {
    pub const KNOWN_KEYS: &'static [&'static str] = &["parser", "formatter"];
}

impl MergeWith<JsonConfiguration> for JsonConfiguration {
    fn merge_with(&mut self, other: JsonConfiguration) {
        if let Some(other_parser) = other.parser {
            let parser = self.parser.get_or_insert_with(JsonParser::default);
            parser.merge_with(other_parser);
        }
        if let Some(other_formatter) = other.formatter {
            let formatter = self.formatter.get_or_insert_with(JsonFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

/// Options that changes how the JSON parser behaves
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
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

impl JsonParser {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] =
        &["allowComments", "allowTrailingCommas"];
}

impl MergeWith<JsonParser> for JsonParser {
    fn merge_with(&mut self, other: JsonParser) {
        if let Some(allow_comments) = other.allow_comments {
            self.allow_comments = Some(allow_comments);
        }
        if let Some(allow_trailing_commas) = other.allow_trailing_commas {
            self.allow_trailing_commas = Some(allow_trailing_commas);
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
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
    #[bpaf(long("json-formatter-indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// What's the max width of a line, applied to JSON (and its super languages) files. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("json-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,
}

impl JsonFormatter {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] =
        &["enabled", "indentStyle", "indentSize", "lineWidth"];
}

impl MergeWith<JsonFormatter> for JsonFormatter {
    fn merge_with(&mut self, other: JsonFormatter) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
        if let Some(indent_size) = other.indent_size {
            self.indent_size = Some(indent_size);
        }
        if let Some(indent_style) = other.indent_style {
            self.indent_style = Some(indent_style);
        }
        if let Some(line_width) = other.line_width {
            self.line_width = Some(line_width);
        }
    }
}

impl MergeWith<Option<JsonFormatter>> for JsonConfiguration {
    fn merge_with(&mut self, other: Option<JsonFormatter>) {
        if let Some(other_formatter) = other {
            let formatter = self.formatter.get_or_insert_with(JsonFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

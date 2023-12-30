use crate::configuration::merge::MergeWith;
use crate::configuration::{deserialize_line_width, serialize_line_width, PlainIndentStyle};
use biome_formatter::{LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to CSS files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct CssConfiguration {
    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_parser), optional)]
    pub parser: Option<CssParser>,

    /// Formatting options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_formatter), optional)]
    pub formatter: Option<CssFormatter>,
}

impl MergeWith<CssConfiguration> for CssConfiguration {
    fn merge_with(&mut self, other: CssConfiguration) {
        if let Some(other_parser) = other.parser {
            let parser = self.parser.get_or_insert_with(CssParser::default);
            parser.merge_with(other_parser);
        }
        if let Some(other_formatter) = other.formatter {
            let formatter = self.formatter.get_or_insert_with(CssFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: CssConfiguration)
    where
        CssConfiguration: Default,
    {
        if other != CssConfiguration::default() {
            self.merge_with(other)
        }
    }
}

/// Options that changes how the CSS parser behaves
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allow comments to appear on incorrect lines in `.css` files
    pub allow_wrong_line_comments: Option<bool>,
}

impl MergeWith<CssParser> for CssParser {
    fn merge_with(&mut self, other: CssParser) {
        if let Some(allow_wrong_line_comments) = other.allow_wrong_line_comments {
            self.allow_wrong_line_comments = Some(allow_wrong_line_comments);
        }
    }

    fn merge_with_if_not_default(&mut self, other: CssParser)
    where
        CssParser: Default,
    {
        if other != CssParser::default() {
            self.merge_with(other)
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CssFormatter {
    /// Control the formatter for CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-enabled"), argument("true|false"), optional)]
    pub enabled: Option<bool>,

    /// The indent style applied to CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<u8>,

    /// The size of the indentation applied to CSS (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// The type of line ending applied to CSS (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("css-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,
}

impl MergeWith<CssFormatter> for CssFormatter {
    fn merge_with(&mut self, other: CssFormatter) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
        if let Some(indent_size) = other.indent_size {
            self.indent_width = Some(indent_size);
        }
        if let Some(indent_style) = other.indent_style {
            self.indent_style = Some(indent_style);
        }
        if let Some(line_width) = other.line_width {
            self.line_width = Some(line_width);
        }
    }

    fn merge_with_if_not_default(&mut self, other: CssFormatter)
    where
        CssFormatter: Default,
    {
        if other != CssFormatter::default() {
            self.merge_with(other)
        }
    }
}

impl MergeWith<Option<CssFormatter>> for CssConfiguration {
    fn merge_with(&mut self, other: Option<CssFormatter>) {
        if let Some(other_formatter) = other {
            let formatter = self.formatter.get_or_insert_with(CssFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<CssFormatter>)
    where
        Option<CssFormatter>: Default,
    {
        if let Some(other_formatter) = other {
            if other_formatter != CssFormatter::default() {
                let formatter = self.formatter.get_or_insert_with(CssFormatter::default);
                formatter.merge_with(other_formatter);
            }
        }
    }
}

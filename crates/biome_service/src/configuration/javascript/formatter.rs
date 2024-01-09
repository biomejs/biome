use crate::configuration::PlainIndentStyle;
use crate::configuration::{deserialize_line_width, serialize_line_width};
use crate::MergeWith;
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use biome_js_formatter::context::trailing_comma::TrailingComma;
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Formatting options specific to the JavaScript files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// The type of quotes used in JSX. Defaults to double.
    #[bpaf(long("jsx-quote-style"), argument("double|single"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_quote_style: Option<QuoteStyle>,
    /// When properties in objects are quoted. Defaults to asNeeded.
    #[bpaf(long("quote-properties"), argument("preserve|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_properties: Option<QuoteProperties>,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[bpaf(long("trailing-comma"), argument("all|es5|none"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_comma: Option<TrailingComma>,
    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[bpaf(long("semicolons"), argument("always|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolons: Option<Semicolons>,
    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[bpaf(long("arrow-parentheses"), argument("always|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_parentheses: Option<ArrowParentheses>,
    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("bracket-spacing"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<bool>,
    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[bpaf(long("bracket-same-line"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<bool>,

    /// Control the formatter for JavaScript (and its super languages) files.
    #[bpaf(long("javascript-formatter-enabled"), argument("true|false"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The indent style applied to JavaScript (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(
        long("javascript-formatter-indent-style"),
        argument("tab|space"),
        optional
    )]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("javascript-formatter-indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(
        long("javascript-formatter-indent-width"),
        argument("NUMBER"),
        optional
    )]
    pub indent_width: Option<u8>,

    /// The type of line ending applied to JavaScript (and its super languages) files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(
        long("javascript-formatter-line-ending"),
        argument("lf|crlf|cr"),
        optional
    )]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("javascript-formatter-line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,

    // TODO: Rename the argument to `javascript-formatter-quote-style` once
    // it's also a top-level configurable property.
    /// The type of quotes used in JavaScript code. Defaults to double.
    #[bpaf(long("quote-style"), argument("double|single"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,
}

impl MergeWith<JavascriptFormatter> for JavascriptFormatter {
    fn merge_with(&mut self, other: JavascriptFormatter) {
        if let Some(arrow_parentheses) = other.arrow_parentheses {
            self.arrow_parentheses = Some(arrow_parentheses);
        }
        if let Some(bracket_spacing) = other.bracket_spacing {
            self.bracket_spacing = Some(bracket_spacing);
        }
        if let Some(bracket_same_line) = other.bracket_same_line {
            self.bracket_same_line = Some(bracket_same_line);
        }
        if let Some(quote_properties) = other.quote_properties {
            self.quote_properties = Some(quote_properties);
        }
        if let Some(quote_style) = other.quote_style {
            self.quote_style = Some(quote_style);
        }
        if let Some(jsx_quote_style) = other.jsx_quote_style {
            self.jsx_quote_style = Some(jsx_quote_style);
        }
        if let Some(semicolons) = other.semicolons {
            self.semicolons = Some(semicolons);
        }
        if let Some(trailing_comma) = other.trailing_comma {
            self.trailing_comma = Some(trailing_comma);
        }
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

    fn merge_with_if_not_default(&mut self, other: JavascriptFormatter)
    where
        JavascriptFormatter: Default,
    {
        if other != JavascriptFormatter::default() {
            self.merge_with_if_not_default(other)
        }
    }
}

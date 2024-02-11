use biome_formatter::{
    AttributePosition, FormatContext, FormatResult, Formatted, IndentStyle, LineEnding, LineWidth,
    Printed, QuoteStyle,
};
use biome_formatter_test::TestFormatLanguage;
use biome_js_formatter::context::trailing_comma::TrailingComma;
use biome_js_formatter::context::{
    ArrowParentheses, BracketSameLine, BracketSpacing, JsFormatContext, JsFormatOptions,
    QuoteProperties, Semicolons,
};
use biome_js_formatter::{format_node, format_range, JsFormatLanguage};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use biome_rowan::{FileSource, SyntaxNode};
use biome_text_size::TextRange;
use serde::{Deserialize, Serialize};

pub struct JsTestFormatLanguage {
    source_type: JsFileSource,
}

impl JsTestFormatLanguage {
    pub fn new(source_type: JsFileSource) -> Self {
        JsTestFormatLanguage { source_type }
    }
}

impl TestFormatLanguage for JsTestFormatLanguage {
    type SyntaxLanguage = JsLanguage;
    type Options = JsFormatOptions;
    type Context = JsFormatContext;
    type FormatLanguage = JsFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let parse = parse(
            text,
            self.source_type,
            JsParserOptions::default().with_parse_class_parameter_decorators(),
        );

        AnyParse::new(
            parse.syntax().as_send().unwrap(),
            parse.into_diagnostics(),
            self.source_type.as_any_file_source(),
        )
    }

    fn deserialize_format_options(
        &self,
        options: &str,
    ) -> Vec<<Self::Context as FormatContext>::Options> {
        let test_options: TestOptions = serde_json::from_str(options).unwrap();

        test_options
            .cases
            .into_iter()
            .map(|case| case.into_format_options(self.source_type))
            .collect()
    }

    fn format_node(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        format_node(options, node)
    }

    fn format_range(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        format_range(options, node, range)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize, Default)]
pub enum JsSerializableIndentStyle {
    /// Tab
    #[default]
    Tab,
    /// Space
    Space,
}

impl From<JsSerializableIndentStyle> for IndentStyle {
    fn from(test: JsSerializableIndentStyle) -> Self {
        match test {
            JsSerializableIndentStyle::Tab => IndentStyle::Tab,
            JsSerializableIndentStyle::Space => IndentStyle::Space,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize, Default)]
pub enum JsSerializableLineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    #[default]
    Lf,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    Crlf,

    /// Carriage Return character only (\r), used very rarely
    Cr,
}

impl From<JsSerializableLineEnding> for LineEnding {
    fn from(test: JsSerializableLineEnding) -> Self {
        match test {
            JsSerializableLineEnding::Lf => LineEnding::Lf,
            JsSerializableLineEnding::Crlf => LineEnding::Crlf,
            JsSerializableLineEnding::Cr => LineEnding::Cr,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableQuoteStyle {
    Double,
    Single,
}

impl From<JsSerializableQuoteStyle> for QuoteStyle {
    fn from(test: JsSerializableQuoteStyle) -> Self {
        match test {
            JsSerializableQuoteStyle::Double => QuoteStyle::Double,
            JsSerializableQuoteStyle::Single => QuoteStyle::Single,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableAttributePosition {
    Auto,
    Multiline,
}

impl From<JsSerializableAttributePosition> for AttributePosition {
    fn from(test: JsSerializableAttributePosition) -> Self {
        match test {
            JsSerializableAttributePosition::Auto => AttributePosition::Auto,
            JsSerializableAttributePosition::Multiline => AttributePosition::Multiline,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableQuoteProperties {
    AsNeeded,
    Preserve,
}

impl From<JsSerializableQuoteProperties> for QuoteProperties {
    fn from(test: JsSerializableQuoteProperties) -> Self {
        match test {
            JsSerializableQuoteProperties::AsNeeded => QuoteProperties::AsNeeded,
            JsSerializableQuoteProperties::Preserve => QuoteProperties::Preserve,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableTrailingComma {
    All,
    ES5,
    None,
}

impl From<JsSerializableTrailingComma> for TrailingComma {
    fn from(test: JsSerializableTrailingComma) -> Self {
        match test {
            JsSerializableTrailingComma::All => TrailingComma::All,
            JsSerializableTrailingComma::ES5 => TrailingComma::Es5,
            JsSerializableTrailingComma::None => TrailingComma::None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableSemicolons {
    Always,
    AsNeeded,
}

impl From<JsSerializableSemicolons> for Semicolons {
    fn from(test: JsSerializableSemicolons) -> Self {
        match test {
            JsSerializableSemicolons::Always => Semicolons::Always,
            JsSerializableSemicolons::AsNeeded => Semicolons::AsNeeded,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsSerializableArrowParentheses {
    Always,
    AsNeeded,
}

impl From<JsSerializableArrowParentheses> for ArrowParentheses {
    fn from(test: JsSerializableArrowParentheses) -> Self {
        match test {
            JsSerializableArrowParentheses::Always => ArrowParentheses::Always,
            JsSerializableArrowParentheses::AsNeeded => ArrowParentheses::AsNeeded,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JsSerializableFormatOptions {
    /// The indent style.
    pub indent_style: Option<JsSerializableIndentStyle>,

    /// The indent width.
    pub indent_width: Option<u8>,

    /// The type of line ending.
    pub line_ending: Option<JsSerializableLineEnding>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,

    /// The style for quotes. Defaults to double.
    pub quote_style: Option<JsSerializableQuoteStyle>,

    /// The style for JSX quotes. Defaults to double.
    pub jsx_quote_style: Option<JsSerializableQuoteStyle>,

    /// When properties in objects are quoted. Defaults to as-needed.
    pub quote_properties: Option<JsSerializableQuoteProperties>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    pub trailing_comma: Option<JsSerializableTrailingComma>,

    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    pub semicolons: Option<JsSerializableSemicolons>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    pub arrow_parentheses: Option<JsSerializableArrowParentheses>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    pub bracket_spacing: Option<bool>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    pub bracket_same_line: Option<bool>,

    /// Attribute position style. Defaults to auto
    pub attribute_position: Option<JsSerializableAttributePosition>,
}

impl JsSerializableFormatOptions {
    fn into_format_options(self, source_type: JsFileSource) -> JsFormatOptions {
        JsFormatOptions::new(source_type)
            .with_indent_style(self.indent_style.unwrap_or_default().into())
            .with_indent_width(
                self.indent_width
                    .map(|value| value.into())
                    .unwrap_or_default(),
            )
            .with_line_ending(self.line_ending.unwrap_or_default().into())
            .with_line_width(
                self.line_width
                    .and_then(|width| LineWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
            .with_jsx_quote_style(
                self.jsx_quote_style
                    .map_or_else(|| QuoteStyle::Double, |value| value.into()),
            )
            .with_quote_style(
                self.quote_style
                    .map_or_else(|| QuoteStyle::Double, |value| value.into()),
            )
            .with_quote_properties(
                self.quote_properties
                    .map_or_else(|| QuoteProperties::AsNeeded, |value| value.into()),
            )
            .with_trailing_comma(
                self.trailing_comma
                    .map_or_else(|| TrailingComma::All, |value| value.into()),
            )
            .with_semicolons(
                self.semicolons
                    .map_or_else(|| Semicolons::Always, |value| value.into()),
            )
            .with_arrow_parentheses(
                self.arrow_parentheses
                    .map_or_else(|| ArrowParentheses::Always, |value| value.into()),
            )
            .with_bracket_spacing(
                self.bracket_spacing
                    .map_or_else(BracketSpacing::default, |value| value.into()),
            )
            .with_bracket_same_line(
                self.bracket_same_line
                    .map_or_else(BracketSameLine::default, |value| value.into()),
            )
            .with_attribute_position(
                self.attribute_position
                    .map_or_else(AttributePosition::default, |value| value.into()),
            )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestOptions {
    cases: Vec<JsSerializableFormatOptions>,
}

use biome_formatter::{FormatResult, Formatted, IndentStyle, LineEnding, LineWidth, Printed};
use biome_formatter_test::TestFormatLanguage;
use biome_json_formatter::context::{JsonFormatContext, JsonFormatOptions};
use biome_json_formatter::{format_node, format_range, JsonFormatLanguage};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNode, TextRange};
use biome_service::settings::{ServiceLanguage, Settings};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct JsonTestFormatLanguage {
    _source_type: JsonFileSource,
}

impl TestFormatLanguage for JsonTestFormatLanguage {
    type ServiceLanguage = JsonLanguage;
    type Context = JsonFormatContext;
    type FormatLanguage = JsonFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let parse = parse_json(text, JsonParserOptions::default().with_allow_comments());

        AnyParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics())
    }

    fn to_language_settings<'a>(
        &self,
        settings: &'a Settings,
    ) -> &'a <Self::ServiceLanguage as ServiceLanguage>::FormatterSettings {
        &settings.languages.json.formatter
    }

    fn format_node(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        format_node(options, node)
    }

    fn format_range(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        format_range(options, node, range)
    }

    fn default_options(&self) -> <Self::ServiceLanguage as ServiceLanguage>::FormatOptions {
        JsonFormatOptions::default()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsonSerializableIndentStyle {
    /// Tab
    Tab,
    /// Space
    Space,
}

impl From<JsonSerializableIndentStyle> for IndentStyle {
    fn from(test: JsonSerializableIndentStyle) -> Self {
        match test {
            JsonSerializableIndentStyle::Tab => IndentStyle::Tab,
            JsonSerializableIndentStyle::Space => IndentStyle::Space,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsonSerializableLineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    Lf,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    Crlf,

    /// Carriage Return character only (\r), used very rarely
    Cr,
}

impl From<JsonSerializableLineEnding> for LineEnding {
    fn from(test: JsonSerializableLineEnding) -> Self {
        match test {
            JsonSerializableLineEnding::Lf => LineEnding::Lf,
            JsonSerializableLineEnding::Crlf => LineEnding::Crlf,
            JsonSerializableLineEnding::Cr => LineEnding::Cr,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JsonSerializableFormatOptions {
    /// The indent style.
    pub indent_style: Option<JsonSerializableIndentStyle>,

    /// The indent width.
    pub indent_width: Option<u8>,

    /// The type of line ending.
    pub line_ending: Option<JsonSerializableLineEnding>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,
}

impl From<JsonSerializableFormatOptions> for JsonFormatOptions {
    fn from(test: JsonSerializableFormatOptions) -> Self {
        JsonFormatOptions::default()
            .with_indent_style(test.indent_style.map(Into::into).unwrap_or_default())
            .with_indent_width(test.indent_width.map(Into::into).unwrap_or_default())
            .with_line_width(
                test.line_width
                    .and_then(|width| LineWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestOptions {
    cases: Vec<JsonSerializableFormatOptions>,
}

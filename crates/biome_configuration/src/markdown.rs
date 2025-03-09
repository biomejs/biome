use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    BracketSpacing, IndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Configuration for the Markdown language
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    /// Configuration for the Markdown formatter
    #[bpaf(external(markdown_formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<MarkdownFormatterConfiguration>,

    /// Configuration for the Markdown linter
    #[bpaf(external(markdown_linter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<MarkdownLinterConfiguration>,

    /// Configuration for the Markdown parser
    #[bpaf(external(markdown_parser_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<MarkdownParserConfiguration>,
}

pub type MarkdownFormatterEnabled = Bool<true>;

/// Configuration for the Markdown formatter
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownFormatterConfiguration {
    /// Markdown formatter options
    #[bpaf(long("markdown-formatter-enabled"), argument("true|false"))]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to Markdown files.
    #[bpaf(long("markdown-formatter-indent-style"), argument("tab|space"))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Markdown files. Default to 2.
    #[bpaf(long("markdown-formatter-indent-width"), argument("NUMBER"))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to Markdown files.
    #[bpaf(long("markdown-formatter-line-ending"), argument("lf|crlf|cr"))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to Markdown files. Defaults to 80.
    #[bpaf(long("markdown-formatter-line-width"), argument("NUMBER"))]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in Markdown code. Defaults to double.
    #[bpaf(long("markdown-formatter-quote-style"), argument("double|single"))]
    pub quote_style: Option<QuoteStyle>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("bracket-spacing"), argument("true|false"))]
    pub bracket_spacing: Option<BracketSpacing>,
}

impl MarkdownFormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

pub type MarkdownLinterEnabled = Bool<true>;

/// Configuration for the Markdown linter
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownLinterConfiguration {
    /// Control the linter for Markdown files.
    #[bpaf(long("markdown-linter-enabled"), argument("true|false"))]
    pub enabled: Option<MarkdownLinterEnabled>,
}

/// Configuration for the Markdown parser
#[derive(
    Bpaf, Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownParserConfiguration {
    /// Whether to parse GitHub Flavored Markdown extensions
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gfm: Option<Bool<true>>,
}

#[test]
fn default_markdown_formatter() {
    let markdown_configuration = MarkdownFormatterConfiguration::default();

    assert!(markdown_configuration.is_enabled());
}

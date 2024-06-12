use biome_css_formatter::context::{CssFormatContext, CssFormatOptions};
use biome_css_formatter::{format_node, format_range, CssFormatLanguage};
use biome_css_parser::{parse_css, CssParserOptions};
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_formatter::{FormatResult, Formatted, Printed};
use biome_formatter_test::TestFormatLanguage;
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNode, TextRange};
use biome_service::settings::{ServiceLanguage, Settings};

#[derive(Default)]
pub struct CssTestFormatLanguage {
    _source_type: CssFileSource,
}

impl TestFormatLanguage for CssTestFormatLanguage {
    type ServiceLanguage = CssLanguage;
    type Context = CssFormatContext;
    type FormatLanguage = CssFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let parse = parse_css(
            text,
            CssParserOptions::default()
                .allow_wrong_line_comments()
                .allow_css_modules(),
        );

        AnyParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics())
    }

    fn to_language_settings<'a>(
        &self,
        settings: &'a Settings,
    ) -> &'a <Self::ServiceLanguage as ServiceLanguage>::FormatterSettings {
        &settings.languages.css.formatter
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
        CssFormatOptions::default()
    }
}

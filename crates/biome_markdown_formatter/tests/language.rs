use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_markdown_formatter::{MarkdownFormatLanguage, context::MarkdownFormatContext};
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownLanguage;
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

#[derive(Default)]
pub struct MarkdownTestFormatLanguage {}

impl TestFormatLanguage for MarkdownTestFormatLanguage {
    type ServiceLanguage = MarkdownLanguage;
    type Context = MarkdownFormatContext;
    type FormatLanguage = MarkdownFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_markdown(text).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.markdown.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &BiomePath::new(""),
            file_source,
        );
        MarkdownFormatLanguage::new(options)
    }
}

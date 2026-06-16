use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_markdown_formatter::{MdFormatLanguage, context::MarkdownFormatContext};
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownLanguage;
use biome_parser::AnyParsedSource;
use biome_service::settings::{ServiceLanguage, Settings};

#[derive(Default)]
pub struct MarkdownTestFormatLanguage {}

impl TestFormatLanguage for MarkdownTestFormatLanguage {
    type ServiceLanguage = MarkdownLanguage;
    type Context = MarkdownFormatContext;
    type FormatLanguage = MdFormatLanguage;

    fn parse(&self, text: &str) -> AnyParsedSource {
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
        MdFormatLanguage::new(options)
    }
}

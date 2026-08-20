use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_markdown_formatter::{MdFormatLanguage, context::MarkdownFormatContext};
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownLanguage;
use biome_parser::AnyParse;
use biome_service::settings::Settings;

#[derive(Default)]
pub struct MarkdownTestFormatLanguage {}

impl TestFormatLanguage for MarkdownTestFormatLanguage {
    type ServiceLanguage = MarkdownLanguage;
    type Context = MarkdownFormatContext;
    type FormatLanguage = MdFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_markdown(text).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let path = BiomePath::new("");
        let options = settings.format_options::<Self::ServiceLanguage>(
            &settings.matching_override_indices(path.as_path()),
            file_source,
        );
        MdFormatLanguage::new(options)
    }
}

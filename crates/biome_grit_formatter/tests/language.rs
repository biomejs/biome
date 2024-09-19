use biome_formatter_test::TestFormatLanguage;
use biome_grit_formatter::{context::GritFormatContext, GritFormatLanguage};
use biome_grit_syntax::GritLanguage;

#[derive(Default)]
pub struct GritTestFormatLanguage;

impl TestFormatLanguage for GritTestFormatLanguage {
    type ServiceLanguage = GritLanguage;
    type Context = GritFormatContext;
    type FormatLanguage = GritFormatLanguage;

    fn parse(&self, _text: &str) -> biome_parser::AnyParse {
        todo!()
    }

    fn to_format_language(
        &self,
        _settings: &biome_service::settings::Settings,
        _file_source: &biome_service::workspace::DocumentFileSource,
    ) -> Self::FormatLanguage {
        todo!()
    }
}

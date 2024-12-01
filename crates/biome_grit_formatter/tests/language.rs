use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_grit_formatter::{context::GritFormatContext, GritFormatLanguage};
use biome_grit_parser::parse_grit;
use biome_grit_syntax::GritLanguage;
use biome_service::settings::ServiceLanguage;

#[derive(Default)]
pub struct GritTestFormatLanguage;

impl TestFormatLanguage for GritTestFormatLanguage {
    type ServiceLanguage = GritLanguage;
    type Context = GritFormatContext;
    type FormatLanguage = GritFormatLanguage;

    fn parse(&self, text: &str) -> biome_parser::AnyParse {
        parse_grit(text).into()
    }

    fn to_format_language(
        &self,
        settings: &biome_service::settings::Settings,
        file_source: &biome_service::workspace::DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.grit.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(language_settings),
            &BiomePath::new(""),
            file_source,
        );
        GritFormatLanguage::new(options)
    }
}

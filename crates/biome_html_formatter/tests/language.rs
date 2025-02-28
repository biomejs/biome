use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_html_formatter::context::HtmlFormatContext;
use biome_html_formatter::HtmlFormatLanguage;
use biome_html_parser::parse_html;
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

pub struct HtmlTestFormatLanguage {
    #[expect(dead_code)]
    source_type: HtmlFileSource,
}

impl HtmlTestFormatLanguage {
    pub fn new(source_type: HtmlFileSource) -> Self {
        HtmlTestFormatLanguage { source_type }
    }
}

impl TestFormatLanguage for HtmlTestFormatLanguage {
    type ServiceLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatLanguage = HtmlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_html(text).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(&settings.languages.html.formatter),
            &BiomePath::new(""),
            file_source,
        );
        HtmlFormatLanguage::new(options)
    }
}

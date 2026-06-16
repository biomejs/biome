use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_html_formatter::HtmlFormatLanguage;
use biome_html_formatter::context::HtmlFormatContext;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::HtmlLanguage;
use biome_languages::{DocumentFileSource, HtmlFileSource};
use biome_parser::AnyParsedSource;
use biome_service::settings::{ServiceLanguage, Settings};

pub struct HtmlTestFormatLanguage {
    source_type: HtmlFileSource,
}

impl HtmlTestFormatLanguage {
    pub fn new(source_type: HtmlFileSource) -> Self {
        Self { source_type }
    }
}

impl TestFormatLanguage for HtmlTestFormatLanguage {
    type ServiceLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatLanguage = HtmlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParsedSource {
        parse_html(text, HtmlParserOptions::from(&self.source_type)).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            &settings.languages.html.formatter,
            &BiomePath::new(""),
            file_source,
        );
        HtmlFormatLanguage::new(options)
    }
}

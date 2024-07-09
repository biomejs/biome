use biome_css_formatter::context::CssFormatContext;
use biome_css_formatter::CssFormatLanguage;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_css_syntax::{CssFileSource, CssLanguage};
use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

#[derive(Default)]
pub struct CssTestFormatLanguage {
    _source_type: CssFileSource,
}

impl TestFormatLanguage for CssTestFormatLanguage {
    type ServiceLanguage = CssLanguage;
    type Context = CssFormatContext;
    type FormatLanguage = CssFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let options = CssParserOptions::default()
            .allow_wrong_line_comments()
            .allow_css_modules();

        parse_css(text, options).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.css.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(language_settings),
            &BiomePath::new(""),
            file_source,
        );
        CssFormatLanguage::new(options)
    }
}

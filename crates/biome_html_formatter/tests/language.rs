use biome_configuration::{Configuration, HtmlConfiguration, html::HtmlFormatterConfiguration};
use biome_formatter::FormatLanguage;
use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_html_formatter::HtmlFormatLanguage;
use biome_html_formatter::context::HtmlFormatContext;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::HtmlLanguage;
use biome_languages::{DocumentFileSource, HtmlFileSource};
use biome_parser::AnyParse;
use biome_service::settings::Settings;

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

    fn parse(&self, text: &str) -> AnyParse {
        parse_html(text, HtmlParserOptions::from(&self.source_type)).into()
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
        HtmlFormatLanguage::new(options)
    }

    fn configure_formatter(
        &self,
        configuration: &mut Configuration,
        format_language: &Self::FormatLanguage,
    ) {
        let options = format_language.options();
        configuration.html = Some(HtmlConfiguration {
            experimental_full_support_enabled: Some(true.into()),
            formatter: Some(HtmlFormatterConfiguration {
                enabled: Some(true.into()),
                attribute_position: Some(options.attribute_position()),
                bracket_same_line: Some(options.bracket_same_line()),
                whitespace_sensitivity: Some(options.whitespace_sensitivity()),
                indent_script_and_style: Some(options.indent_script_and_style()),
                self_close_void_elements: Some(options.self_close_void_elements()),
                trailing_newline: Some(options.trailing_newline()),
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

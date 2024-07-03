use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_js_formatter::context::JsFormatContext;
use biome_js_formatter::JsFormatLanguage;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

pub struct JsTestFormatLanguage {
    source_type: JsFileSource,
}

impl JsTestFormatLanguage {
    pub fn new(source_type: JsFileSource) -> Self {
        JsTestFormatLanguage { source_type }
    }
}

impl TestFormatLanguage for JsTestFormatLanguage {
    type ServiceLanguage = JsLanguage;
    type Context = JsFormatContext;
    type FormatLanguage = JsFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let options = JsParserOptions::default().with_parse_class_parameter_decorators();

        parse(text, self.source_type, options).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.javascript.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(language_settings),
            &BiomePath::new(""),
            file_source,
        );
        JsFormatLanguage::new(options)
    }
}

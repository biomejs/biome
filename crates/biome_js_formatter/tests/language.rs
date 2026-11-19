use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_js_formatter::JsFormatLanguage;
use biome_js_formatter::context::JsFormatContext;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsLanguage;
use biome_languages::{DocumentFileSource, JsFileSource};
use biome_parser::AnyParse;
use biome_service::settings::Settings;

pub struct JsTestFormatLanguage {
    source_type: JsFileSource,
}

impl JsTestFormatLanguage {
    pub fn new(source_type: JsFileSource) -> Self {
        Self { source_type }
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
        let path = BiomePath::new("");
        let options = settings.format_options::<Self::ServiceLanguage>(
            &settings.matching_override_indices(path.as_path()),
            file_source,
        );
        JsFormatLanguage::new(options)
    }
}

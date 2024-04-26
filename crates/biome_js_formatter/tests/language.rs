use biome_formatter::{FormatResult, Formatted, Printed};
use biome_formatter_test::TestFormatLanguage;
use biome_js_formatter::context::{JsFormatContext, JsFormatOptions};
use biome_js_formatter::{format_node, format_range, JsFormatLanguage};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use biome_rowan::SyntaxNode;
use biome_service::settings::{ServiceLanguage, Settings};
use biome_text_size::TextRange;

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
        let parse = parse(
            text,
            self.source_type,
            JsParserOptions::default().with_parse_class_parameter_decorators(),
        );

        AnyParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics())
    }

    fn to_language_settings<'a>(
        &self,
        settings: &'a Settings,
    ) -> &'a <Self::ServiceLanguage as ServiceLanguage>::FormatterSettings {
        &settings.languages.javascript.formatter
    }

    fn format_node(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        format_node(options, node)
    }

    fn format_range(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        format_range(options, node, range)
    }

    fn default_options(&self) -> <Self::ServiceLanguage as ServiceLanguage>::FormatOptions {
        JsFormatOptions::new(self.source_type)
    }
}

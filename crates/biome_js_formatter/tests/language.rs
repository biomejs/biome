use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::FormatError;
use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_js_formatter::{context::JsFormatContext, JsForeignLanguageFormatter};
use biome_js_formatter::{JsForeignLanguage, JsFormatLanguage};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use biome_service::settings::{ServiceLanguage, Settings};

pub struct JsTestFormatLanguage {
    source_type: JsFileSource,
}

impl JsTestFormatLanguage {
    pub fn new(source_type: JsFileSource) -> Self {
        JsTestFormatLanguage { source_type }
    }
}

#[derive(Debug, Clone)]
struct MultiLanguageFormatter {
    css_parse_options: CssParserOptions,
    css_format_options: CssFormatOptions,
}

impl JsForeignLanguageFormatter for MultiLanguageFormatter {
    fn format(
        &self,
        language: biome_js_formatter::JsForeignLanguage,
        content: &str,
    ) -> biome_formatter::FormatResult<biome_formatter::prelude::Document> {
        match language {
            JsForeignLanguage::Css => {
                let parse = parse_css(content, self.css_parse_options);
                if parse.has_errors() {
                    return Err(FormatError::SyntaxError);
                }
                biome_css_formatter::format_node(self.css_format_options.clone(), &parse.syntax())
                    .map(|formatted| formatted.into_document())
            }
        }
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

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &biome_service::workspace::DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.javascript.formatter;
        let js_formatter_options = Self::ServiceLanguage::resolve_format_options(
            Some(&settings.formatter),
            Some(&settings.override_settings),
            Some(language_settings),
            &BiomePath::new(""),
            file_source,
        );
        let css_parse_options = CssParserOptions {
            css_modules: settings.languages.css.parser.css_modules,
            allow_wrong_line_comments: settings.languages.css.parser.allow_wrong_line_comments,
        };
        let css_format_options = CssFormatOptions::default().with_quote_style(
            settings
                .languages
                .css
                .formatter
                .quote_style
                .unwrap_or_default(),
        );
        let foreign_language_formatter = MultiLanguageFormatter {
            css_parse_options,
            css_format_options,
        };
        JsFormatLanguage::new(js_formatter_options, foreign_language_formatter)
    }
}

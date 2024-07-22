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

#[derive(Debug, Clone)]
struct MultiLanguageFormatter {
    format_with_errors: bool,
    css_parse_options: CssParserOptions,
    css_format_options: CssFormatOptions,
}

impl JsForeignLanguageFormatter for MultiLanguageFormatter {
    fn format(
        &self,
        language: biome_js_formatter::JsForeignLanguage,
        source: &str,
    ) -> biome_formatter::FormatResult<biome_formatter::prelude::Document> {
        match language {
            JsForeignLanguage::Css => {
                let parse = parse_css(source, self.css_parse_options);
                if !self.format_with_errors && parse.has_errors() {
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
        let options = JsParserOptions::default().with_parse_class_parameter_decorators();

        parse(text, self.source_type, options).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
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
            css_modules: settings
                .languages
                .css
                .parser
                .css_modules
                .unwrap_or_default(),
            allow_wrong_line_comments: settings
                .languages
                .css
                .parser
                .allow_wrong_line_comments
                .unwrap_or_default(),
            grit_metavariable: true,
        };
        let css_format_options = CssFormatOptions::default().with_quote_style(
            settings
                .languages
                .css
                .formatter
                .quote_style
                .unwrap_or_default(),
        );
        let format_with_errors = settings.formatter.format_with_errors;
        let multi_language_formatter = MultiLanguageFormatter {
            css_parse_options,
            css_format_options,
            format_with_errors,
        };
        JsFormatLanguage::new(js_formatter_options, multi_language_formatter)
    }
}

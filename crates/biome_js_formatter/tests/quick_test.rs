use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node as css_format_node;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::{AttributePosition, IndentStyle, LineWidth, QuoteStyle};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_js_formatter::context::{
    ArrowParentheses, EmbeddedLanguageFormatting, JsFormatOptions, Semicolons,
};
use biome_js_formatter::{
    format_node, JsForeignLanguage, JsForeignLanguageFormatter, JsFormatLanguage,
};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
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
                    return Err(biome_formatter::FormatError::SyntaxError);
                }
                css_format_node(self.css_format_options.clone(), &parse.syntax())
                    .map(|formatted| formatted.into_document())
            }
        }
    }
}

#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
        css`
        .foo {
          color:            ${color}
        }
        `

    "#;
    let source_type = JsFileSource::tsx();
    let tree = parse(
        src,
        source_type,
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    let options = JsFormatOptions::new(source_type)
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(80).unwrap())
        .with_semicolons(Semicolons::Always)
        .with_quote_style(QuoteStyle::Double)
        .with_jsx_quote_style(QuoteStyle::Single)
        .with_arrow_parentheses(ArrowParentheses::AsNeeded)
        .with_attribute_position(AttributePosition::Multiline)
        .with_embedded_language_formatting(EmbeddedLanguageFormatting::Auto);

    let css_parse_options = CssParserOptions::default();
    let css_format_options = CssFormatOptions::default();
    let foreign_language_formatter = MultiLanguageFormatter {
        css_parse_options,
        css_format_options,
    };

    let doc = format_node(
        options.clone(),
        foreign_language_formatter.clone(),
        &tree.syntax(),
    )
    .unwrap();
    let result = doc.print().unwrap();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::JsTestFormatLanguage::new(source_type),
        JsFormatLanguage::new(options, foreign_language_formatter),
    )
    .check_reformat();
}

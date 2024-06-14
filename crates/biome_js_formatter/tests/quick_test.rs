use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node as css_format_node;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::{AttributePosition, IndentStyle, LineWidth, QuoteStyle};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, Semicolons};
use biome_js_formatter::{
    format_node, ForeignLanguage, JsForeignLanguageFormatter, JsFormatLanguage,
};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
}

#[derive(Debug, Clone)]
struct ForeignLanguageFormatter {
    css_parse_options: CssParserOptions,
    css_format_options: CssFormatOptions,
}

impl JsForeignLanguageFormatter for ForeignLanguageFormatter {
    fn fmt(
        &self,
        language: biome_js_formatter::ForeignLanguage,
        content: &str,
    ) -> biome_formatter::FormatResult<biome_formatter::prelude::Document> {
        match language {
            ForeignLanguage::Css => {
                let parse = parse_css(content, self.css_parse_options.clone());
                css_format_node(self.css_format_options.clone(), &parse.syntax())
                    .map(|formatted| formatted.into_document())
            }
        }
    }
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
type InstanceID = string;
type MaybeCardWithAttachment = string;
function outerFunctionToForceIndent() {
    const cardWithAttachment: (id: InstanceID) => MaybeCardWithAttachment = (
        id
    ) => {
        return `${id}test`;
    };
}

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
        .with_attribute_position(AttributePosition::Multiline);

    let css_parse_options = CssParserOptions::default();
    let css_format_options = CssFormatOptions::default();
    let foreign_language_formatter = ForeignLanguageFormatter {
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

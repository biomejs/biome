use biome_formatter::{AttributePosition, IndentStyle, LineWidth, QuoteStyle, TrailingNewline};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, Semicolons};
use biome_js_formatter::{JsFormatLanguage, format_node};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
const c = [
  , /* this */
];
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

    let doc = format_node(options.clone(), &tree.syntax(), false).unwrap();
    let result = doc.print().unwrap();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::JsTestFormatLanguage::new(source_type),
        JsFormatLanguage::new(options),
    )
    .check_reformat();
}

#[test]
fn test_trailing_newline_enabled() {
    let src = r#"const a = 1;"#;
    let source_type = JsFileSource::js_module();
    let tree = parse(src, source_type, JsParserOptions::default());
    let options =
        JsFormatOptions::new(source_type).with_trailing_newline(TrailingNewline::from(true));

    let doc = format_node(options, &tree.syntax(), false).unwrap();
    let result = doc.print().unwrap();

    // With trailing newline enabled (default), should end with newline
    assert!(
        result.as_code().ends_with('\n'),
        "Expected code to end with newline"
    );
}

#[test]
fn test_trailing_newline_disabled() {
    let src = r#"const a = 1;"#;
    let source_type = JsFileSource::js_module();
    let tree = parse(src, source_type, JsParserOptions::default());
    let options =
        JsFormatOptions::new(source_type).with_trailing_newline(TrailingNewline::from(false));

    let doc = format_node(options, &tree.syntax(), false).unwrap();
    let result = doc.print().unwrap();

    // With trailing newline disabled, should NOT end with newline
    assert!(
        !result.as_code().ends_with('\n'),
        "Expected code to NOT end with newline"
    );
    assert!(
        !result.as_code().ends_with('\r'),
        "Expected code to NOT end with carriage return"
    );
}

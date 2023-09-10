use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use rome_formatter_test::check_reformat::CheckReformat;
use rome_js_formatter::context::{ArrowParentheses, JsFormatOptions, QuoteStyle, Semicolons};
use rome_js_formatter::format_node;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
        (action: h) => {}
        (action?) => {}
        (action
        // yes
        ) => {}
        ({ action }) => {}
        ([ action ]) => {}
        (...action) => {}
        (action = 1) => {}
    "#;
    let syntax = JsFileSource::tsx();
    let tree = parse(
        src,
        syntax,
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    let options = JsFormatOptions::new(syntax)
        .with_semicolons(Semicolons::Always)
        .with_quote_style(QuoteStyle::Double)
        .with_jsx_quote_style(QuoteStyle::Single)
        .with_arrow_parentheses(ArrowParentheses::AsNeeded);
    let result = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap();

    let root = &tree.syntax();
    let language = language::JsTestFormatLanguage::new(JsFileSource::tsx());
    let check_reformat =
        CheckReformat::new(root, result.as_code(), "quick_test", &language, options);
    check_reformat.check_reformat();

    // I don't know why semicolons are added there, but it's not related to my code changes so ¯\_(ツ)_/¯
    assert_eq!(
        result.as_code(),
        r#"(action: h) => {};
(action?) => {};
(
	action,
	// yes
) => {};
({ action }) => {};
([action]) => {};
(...action) => {};
(action = 1) => {};
"#
    );
}

use biome_formatter::IndentStyle;
use biome_formatter_test::check_reformat::CheckReformat;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, QuoteStyle, Semicolons};
use biome_js_formatter::format_node;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
    /**
 * Curried function that ends with a BEM CSS Selector
 *
 * @param {String} block - the BEM Block you'd like to select.
 * @returns {Function}
 */
export const bem =
	(block) =>
	/**
	 * @param {String} [element] - the BEM Element within that block; if undefined, selects the block itself.
	 * @returns {Function}
	 */
	(element) =>
	/**
	 * @param {?String} [modifier] - the BEM Modifier for the Block or Element; if undefined, selects the Block or Element unmodified.
	 * @returns {String}
	 */
	(modifier) =>
		[
			".",
			css(block),
			element ? `__${css(element)}` : "",
			modifier ? `--${css(modifier)}` : "",
		].join("");
    "#;
    let syntax = JsFileSource::tsx();
    let tree = parse(
        src,
        syntax,
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    let options = JsFormatOptions::new(syntax)
        .with_indent_style(IndentStyle::Space)
        .with_semicolons(Semicolons::Always)
        .with_quote_style(QuoteStyle::Double)
        .with_jsx_quote_style(QuoteStyle::Single)
        .with_arrow_parentheses(ArrowParentheses::Always);

    let doc = format_node(options.clone(), &tree.syntax()).unwrap();
    let result = doc.print().unwrap();
    let source_type = JsFileSource::js_module();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::JsTestFormatLanguage::new(source_type),
        options,
    )
    .check_reformat();
}

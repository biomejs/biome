use biome_formatter::IndentStyle;
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
a(
    (long1ArgumentNamfffe1AndEvongferSothat,ff) => (
      long3ArgumentName3ItafsafBreaks22  
    ) => [a],
  );
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

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());
    // I don't know why semicolons are added there, but it's not related to my code changes so ¯\_(ツ)_/¯
    assert_eq!(
        result.as_code(),
        r#"const fooo: SomeThingWithLongMappedType<{
  [P in
    | AAAAAAAAAAAAAAAAA
    | BBBBBBBBBBBB
    | CCCCCCCCCCCCCCCCCCCCC
    | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD]: number;
}> = {};
"#
    );
}

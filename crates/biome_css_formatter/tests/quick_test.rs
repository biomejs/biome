use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter_test::check_reformat::CheckReformat;

mod language {
    include!("language.rs");
}

// #[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
div.one, div.three#four { color: blue; background-color: red;}
html {}
"#;
    let parse = parse_css(src, CssParserOptions::default());

    let options = CssFormatOptions::default();
    let doc = format_node(options.clone(), &parse.syntax()).unwrap();
    let result = doc.print().unwrap();

    let root = &parse.syntax();
    let language = language::CssTestFormatLanguage::default();

    println!("{:#?}\n\n", parse);

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(root, result.as_code(), "quick_test", &language, options).check_reformat();
}

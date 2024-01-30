use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::{IndentStyle, LineWidth};
use biome_formatter_test::check_reformat::CheckReformat;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
    div {
        border: #fff    solid
        
        2px;
        border: THICK   #000;
        border: medium;
    }

"#;
    let parse = parse_css(src, CssParserOptions::default());
    println!("{:#?}", parse);

    let options = CssFormatOptions::default()
        .with_line_width(LineWidth::try_from(80).unwrap())
        .with_indent_style(IndentStyle::Space);
    let doc = format_node(options.clone(), &parse.syntax()).unwrap();
    let result = doc.print().unwrap();

    let root = &parse.syntax();
    let language = language::CssTestFormatLanguage::default();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(root, result.as_code(), "quick_test", &language, options).check_reformat();
}

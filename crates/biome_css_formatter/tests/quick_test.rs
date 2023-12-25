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
        color: rgba(255, 255, 255, 1);
        color:   rgba   (
            0,
            1,
            255,
            1
        );
        color: arbitrary(really long list, of complex parameter values, each one on its own line);
        color: more-arbitrary(just, has, lots, of, individual, parameters, breaking, over, lines);
        color: arbitrary(one really long parameter value that itself will break over multiple lines and fill together);
    }
"#;
    let parse = parse_css(src, CssParserOptions::default());
    println!("{:#?}", parse.syntax());

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

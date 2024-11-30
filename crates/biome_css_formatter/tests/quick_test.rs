use biome_css_formatter::format_node;
use biome_css_formatter::{context::CssFormatOptions, CssFormatLanguage};
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::{IndentStyle, LineWidth, QuoteStyle};
use biome_formatter_test::check_reformat::CheckReformat;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
/* 1some medium long comment */
.line1 selector,
/* 2some medium long comment */
.line1 selector,
/* 3some medium long comment */
div selector {
  background: green;
}
"#;
    let parse = parse_css(src, CssParserOptions::default());
    println!("{parse:#?}");

    let options = CssFormatOptions::default()
        .with_line_width(LineWidth::try_from(80).unwrap())
        .with_indent_style(IndentStyle::Space)
        .with_quote_style(QuoteStyle::Single);
    let doc = format_node(options.clone(), &parse.syntax()).unwrap();
    let result = doc.print().unwrap();

    let root = &parse.syntax();
    let language = language::CssTestFormatLanguage::default();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        root,
        result.as_code(),
        "quick_test",
        &language,
        CssFormatLanguage::new(options),
    )
    .check_reformat();
}

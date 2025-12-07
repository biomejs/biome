use biome_formatter::{AttributePosition, IndentStyle, LineWidth};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_html_formatter::context::HtmlFormatOptions;
use biome_html_formatter::{HtmlFormatLanguage, format_node};
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::HtmlFileSource;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    //     let src = r#"
    // <span>foo</span>
    // <!-- biome-ignore format: reason -->
    // foo bar baz boof
    // quick brown fox
    // "#;
    let src = r#"
<template>
  <template>foo</template>
</template>

<template>
  <template>foooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo</template>
</template>

"#;
    let source_type = HtmlFileSource::html();
    let tree = parse_html(src, HtmlParseOptions::from(&source_type));
    let options = HtmlFormatOptions::new(HtmlFileSource::html())
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(80).unwrap())
        .with_attribute_position(AttributePosition::Auto);

    let doc = format_node(options.clone(), &tree.syntax(), false).unwrap();
    let result = doc.print().unwrap();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::HtmlTestFormatLanguage::new(source_type),
        HtmlFormatLanguage::new(options),
    )
    .check_reformat();
}

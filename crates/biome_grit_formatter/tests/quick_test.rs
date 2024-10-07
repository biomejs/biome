use biome_formatter::{IndentStyle, LineWidth};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_grit_formatter::context::GritFormatOptions;
use biome_grit_formatter::{format_node, GritFormatLanguage};
use biome_grit_parser::parse_grit;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
`$method('$message')` where {
  if ($message <: r"Hello, .*!") {
    $method => `console.info`
  } else {
    $method => `console.warn`
  }
}
"#;
    let tree = parse_grit(src);
    let options = GritFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(80).unwrap());

    let doc = format_node(options.clone(), &tree.syntax()).unwrap();
    let result = doc.print().unwrap();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::GritTestFormatLanguage,
        GritFormatLanguage::new(options),
    )
    .check_reformat();
}

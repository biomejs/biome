use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::parse_markdown;

#[ignore]
#[test]
fn quick_test() {
    let source = r#"# Hello World

## Heading Level 2

### Heading with trailing hashes ###

# Simple
"#;
    let parse = parse_markdown(source);

    // Print CST
    eprintln!("{:#?}", parse.syntax());

    let options = MdFormatOptions::default();
    let result =
        biome_formatter::format_node(&parse.syntax(), MdFormatLanguage::new(options), false);

    // Print formatted output
    let formatted = result.unwrap();
    eprintln!("Formatted:\n{}", formatted.print().unwrap().as_code());
}

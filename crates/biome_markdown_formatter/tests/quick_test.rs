use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::parse_markdown;

#[ignore]
#[test]
fn quick_test() {
    // `- a\n   b`: 3 spaces = 2 required + 1 excess → strip to 2.
    let source = "- a\n   b\n";
    let parse = parse_markdown(source);

    let options = MdFormatOptions::default();
    let result = biome_formatter::format_node(
        &parse.syntax(),
        MdFormatLanguage::new(options.clone()),
        false,
    );

    let formatted = result.unwrap();
    let output = formatted.print().unwrap();
    eprintln!("Formatted:\n{}<END>", output.as_code());
}

use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::parse_markdown;

#[ignore]
#[test]
fn quick_test() {
    let source = r#"100. Triple-digit marker: required continuation indent is 5.
     aligned at 5 spaces
     one-extra-space continuation gets stripped
       two-extra-space preserved

     loose paragraph aligned at 5

     loose paragraph with single excess

        loose paragraph with multi excess
"#;
    let parse = parse_markdown(source);

    // Print CST
    eprintln!("{:#?}", parse.syntax());
    // print red tree
    eprintln!("{:#?}", parse.tree());

    let options = MdFormatOptions::default();
    let result = biome_formatter::format_node(
        &parse.syntax(),
        MdFormatLanguage::new(options.clone()),
        false,
    );

    let formatted = result.unwrap();
    let first_ir = formatted.document();
    let output = formatted.print().unwrap();
    eprintln!("Formatted:\n{}", output.as_code());

    // Idempotency
    // Now re-parse the formatted output and show its CST
    let reparse = parse_markdown(output.as_code());
    eprintln!("\n--- Re-parsed CST ---");
    eprintln!("{:#?}", reparse.tree());

    let result2 =
        biome_formatter::format_node(&reparse.syntax(), MdFormatLanguage::new(options), false);
    let output2 = result2.unwrap();
    let second_ir = output2.document();
    similar_asserts::assert_eq!(
        output2.print().unwrap().as_code(),
        output.as_code(),
        "left is the re-formatted"
    );
    similar_asserts::assert_eq!(
        second_ir.to_string(),
        first_ir.to_string(),
        "left is the re-formatted"
    );
}

use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::parse_markdown;

#[ignore]
#[test]
fn quick_test() {
    let source = "~~~~
aaa
~~~
~~~~

";
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

    // Print formatted output
    let formatted = result.unwrap();
    let output = formatted.print().unwrap();
    let first_ir = format!("{}", formatted.into_document());
    eprintln!("Formatted:\n{}", output.as_code());

    // Now re-parse the formatted output and show its CST
    let reparse = parse_markdown(output.as_code());
    eprintln!("\n--- Re-parsed CST ---");
    eprintln!("{:#?}", reparse.tree());

    let result2 =
        biome_formatter::format_node(&reparse.syntax(), MdFormatLanguage::new(options), false);
    let output2 = result2.unwrap();
    eprintln!("Re-formatted:\n{}", output2.print().unwrap().as_code());
    let second_ir = format!("{}", output2.into_document());

    similar_asserts::assert_eq!(second_ir, first_ir, "left is the re-formatted");
}

use biome_yaml_formatter::{YamlFormatLanguage, YamlFormatOptions};
use biome_yaml_parser::parse_yaml;

#[ignore]
#[test]
fn quick_test() {
    let source = "# top comment\na: 10\n# between\nb: 20\nc: 30 # inline\n";
    let parse = parse_yaml(source);

    let options = YamlFormatOptions::default();
    let result = biome_formatter::format_node(
        &parse.syntax(),
        YamlFormatLanguage::new(options.clone()),
        false,
    );

    let formatted = result.unwrap();
    let first_ir = formatted.document();
    let output = formatted.print().unwrap();
    eprintln!("Formatted:\n{}", output.as_code());
    eprintln!("IR:\n{}", first_ir);

    // Idempotency
    // Now re-parse the formatted output and show its CST
    let reparse = parse_yaml(output.as_code());
    eprintln!("\n--- Re-parsed CST ---");
    eprintln!("{:#?}", reparse.tree());

    let result2 =
        biome_formatter::format_node(&reparse.syntax(), YamlFormatLanguage::new(options), false);
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

use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::parse_markdown;

#[ignore]
#[test]
fn quick_test() {
    let source = "foo
baz

backslash\
form

no hard line
here

foo  
bar with empty line after  

foo  
bar without empty line after  
";
    let parse = parse_markdown(source);

    // Print CST
    eprintln!("{:#?}", parse.syntax());
    // print red tree
    eprintln!("{:#?}", parse.tree());

    let options = MdFormatOptions::default();
    let result =
        biome_formatter::format_node(&parse.syntax(), MdFormatLanguage::new(options), false);

    // Print formatted output
    let formatted = result.unwrap();
    let code = formatted.print().unwrap().as_code().to_string();
    eprintln!("=== Formatted (len={}) ===", code.len());
    eprintln!("{:?}", code);
    eprintln!("=== end ===");
}

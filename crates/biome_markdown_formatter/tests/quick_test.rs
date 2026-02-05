use biome_markdown_formatter::{MarkdownFormatLanguage, context::MarkdownFormatOptions};
use biome_markdown_parser::parse_markdown;

// The test is currently ignored because the parser isn't fully implemented
// causing an infinite loop in `parse_block_list` (syntax.rs) because it can't find EOF.
//
// Having test is still useful for making sure the type checker is happy.
//
#[test]
#[ignore]
fn format_simple_paragraph() {
    let source = "This is a simple paragraph.";

    let parse = parse_markdown(source);
    let options = MarkdownFormatOptions::default();
    let formatted =
        biome_formatter::format_node(&parse.syntax(), MarkdownFormatLanguage::new(options), false);

    match formatted {
        Ok(result) => {
            let printed = result.print().unwrap();
            println!("Formatted output: {}", printed.as_code());
            assert!(!printed.as_code().is_empty());
        }
        Err(e) => {
            panic!("Formatting failed: {:?}", e);
        }
    }
}

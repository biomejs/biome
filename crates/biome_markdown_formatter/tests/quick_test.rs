use biome_markdown_formatter::{MarkdownFormatLanguage, context::MarkdownFormatOptions};
use biome_markdown_parser::parse_markdown;

#[test]
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

#[test]
fn format_heading() {
    let source = "# Main Heading";

    let parse = parse_markdown(source);
    let options = MarkdownFormatOptions::default();
    let formatted =
        biome_formatter::format_node(&parse.syntax(), MarkdownFormatLanguage::new(options), false);

    match formatted {
        Ok(result) => {
            let printed = result.print().unwrap();
            println!("Formatted heading: {}", printed.as_code());
            assert!(!printed.as_code().is_empty());
        }
        Err(e) => {
            panic!("Formatting failed: {:?}", e);
        }
    }
}

use biome_formatter::FormatElement;
use biome_markdown_formatter::{MarkdownFormatLanguage, context::MarkdownFormatOptions};
use biome_markdown_parser::parse_markdown;
use biome_rowan::TextSize;

// This test only verifies the formatter infrastructure works end-to-end
// without actually formatting the input.
#[ignore]
#[test]
fn quick_test() {
    let source = "A simple paragraph";
    let parse = parse_markdown(source);
    let options = MarkdownFormatOptions::default();

    let result =
        biome_formatter::format_node(&parse.syntax(), MarkdownFormatLanguage::new(options), false);

    let boxed_text: Box<str> = source.into();
    assert_eq!(
        result.unwrap().document().as_elements().first().unwrap(),
        &FormatElement::Text {
            text: boxed_text,
            source_position: TextSize::default(),
        }
    );
}

use biome_formatter::format_element::document::Document;
use biome_formatter::prelude::*;
use biome_markdown_formatter::context::MdFormatOptions;
use biome_markdown_formatter::format_node;
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_rowan::TextRange;

fn parse_mdx(source: &str) -> biome_markdown_parser::MarkdownParse {
    let parsed = parse_markdown(source, MarkdownParserOptions::default().with_mdx(true));
    assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
    parsed
}

/// The ranges of the literals that hold the JavaScript of MDX constructs.
fn snippet_ranges(parsed: &biome_markdown_parser::MarkdownParse) -> Vec<TextRange> {
    parsed
        .syntax()
        .descendants_tokens(biome_rowan::Direction::Next)
        .filter(|token| {
            matches!(
                token.kind(),
                MarkdownSyntaxKind::MDX_ESM_LITERAL
                    | MarkdownSyntaxKind::MDX_EXPRESSION_LITERAL
                    | MarkdownSyntaxKind::MDX_JSX_TAG_LITERAL
            )
        })
        .map(|token| token.text_trimmed_range())
        .collect()
}

#[test]
fn mdx_constructs_keep_their_text_without_embedded_formatting() {
    let source = "import {Chart} from './chart.js'\n\n# Hello, {props.name}\n\n<Note type=\"tip\">\n\nSome text.\n\n</Note>\n\n<Card>{ props.children }</Card>\n\nA <Badge>new</Badge> tag.\n";
    let parsed = parse_mdx(source);
    let formatted = format_node(MdFormatOptions::default(), &parsed.syntax(), Vec::new()).unwrap();
    assert_eq!(formatted.print().unwrap().as_code(), source);
}

#[test]
fn embedded_snippets_are_replaced_in_place() {
    let source = "import {a} from 'a'\n\n<Note> { props.x } </Note>\n\nText {b} and <B />.\n";
    let parsed = parse_mdx(source);
    let ranges = snippet_ranges(&parsed);
    assert_eq!(ranges.len(), 6);

    let mut formatted =
        format_node(MdFormatOptions::default(), &parsed.syntax(), ranges.clone()).unwrap();
    let replacements = ["ESM", "OPEN", "EXPR", "CLOSE", "B", "TAG"];
    formatted.format_embedded(|range| {
        let index = ranges.iter().position(|candidate| *candidate == range)?;
        Some(Document::new(vec![FormatElement::Token {
            text: replacements[index],
        }]))
    });
    assert_eq!(
        formatted.print().unwrap().as_code(),
        "ESM\n\nOPEN {EXPR} CLOSE\n\nText {B} and TAG.\n"
    );
}

#[test]
fn markdown_around_mdx_constructs_is_formatted() {
    let source = "<Note>\n\nSome *text* {x}.\n\n* item <B />\n\n</Note>\n";
    let parsed = parse_mdx(source);
    let formatted = format_node(MdFormatOptions::default(), &parsed.syntax(), Vec::new()).unwrap();
    assert_eq!(
        formatted.print().unwrap().as_code(),
        "<Note>\n\nSome _text_ {x}.\n\n- item <B />\n\n</Note>\n"
    );
}

#[test]
fn spaces_between_flow_constructs_are_kept() {
    let source = "<Note> {props.x} </Note>\n";
    let parsed = parse_mdx(source);
    let formatted = format_node(MdFormatOptions::default(), &parsed.syntax(), Vec::new()).unwrap();
    assert_eq!(formatted.print().unwrap().as_code(), source);
}

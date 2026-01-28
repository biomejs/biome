use std::collections::HashSet;

use biome_markdown_syntax::{MdLinkLabel, MdLinkReferenceDefinition};
use biome_rowan::{AstNode, Direction};

use crate::MarkdownLosslessTreeSink;
use crate::MarkdownParseOptions;
use crate::parser::MarkdownParser;
use crate::syntax::parse_document;

pub(crate) fn normalize_reference_label(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    let mut saw_whitespace = false;

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                push_normalized_char(&mut out, next, &mut saw_whitespace);
            }
            continue;
        }

        if c.is_whitespace() {
            saw_whitespace = true;
            continue;
        }

        push_normalized_char(&mut out, c, &mut saw_whitespace);
    }

    out
}

fn push_normalized_char(out: &mut String, c: char, saw_whitespace: &mut bool) {
    if *saw_whitespace && !out.is_empty() {
        out.push(' ');
    }
    *saw_whitespace = false;
    for lower in c.to_lowercase() {
        out.push(lower);
    }
}

pub(crate) fn collect_link_reference_definitions(
    source: &str,
    options: MarkdownParseOptions,
) -> HashSet<String> {
    let mut parser = MarkdownParser::new(source, options);
    parse_document(&mut parser);
    let (events, diagnostics, trivia, _list_tightness, _list_item_indents, _quote_indents) =
        parser.finish();

    let mut tree_sink = MarkdownLosslessTreeSink::new(source, &trivia);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (root, _) = tree_sink.finish();

    let mut definitions = HashSet::new();

    for node in root.descendants() {
        if let Some(def) = MdLinkReferenceDefinition::cast(node)
            && let Ok(label) = def.label()
        {
            let raw = collect_label_text(label);
            let normalized = normalize_reference_label(&raw);
            if !normalized.is_empty() {
                definitions.insert(normalized);
            }
        }
    }

    definitions
}

fn collect_label_text(label: MdLinkLabel) -> String {
    let mut text = String::new();
    for token in label
        .content()
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token())
    {
        text.push_str(token.text());
    }
    text
}

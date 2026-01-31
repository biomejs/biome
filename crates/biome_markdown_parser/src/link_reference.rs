use std::collections::HashSet;

use biome_markdown_syntax::MdLinkReferenceDefinition;
use biome_rowan::AstNode;

use crate::MarkdownLosslessTreeSink;
use crate::MarkdownParseOptions;
use crate::parser::MarkdownParser;
use crate::syntax::parse_document;
use crate::syntax::reference::normalize_reference_label;

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
            let raw = label.syntax().text_trimmed().to_string();
            let normalized = normalize_reference_label(&raw);
            if !normalized.is_empty() {
                definitions.insert(normalized.into_owned());
            }
        }
    }

    definitions
}

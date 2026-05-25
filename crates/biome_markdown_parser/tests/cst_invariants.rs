use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::{
    MarkdownSyntaxKind, MdContinuationIndent, MdListMarkerPrefix, MdOrderedListItem,
};
use biome_rowan::{AstNode, AstNodeList};

fn indent_len(indent: impl AstNodeList) -> usize {
    indent.len()
}

fn continuation_indents(input: &str) -> Vec<usize> {
    let parsed = parse_markdown(input);

    parsed
        .syntax()
        .descendants()
        .filter_map(MdContinuationIndent::cast)
        .map(|indent| indent_len(indent.indent()))
        .collect()
}

fn marker_prefix(input: &str, marker_text: &str) -> MdListMarkerPrefix {
    let parsed = parse_markdown(input);

    parsed
        .syntax()
        .descendants()
        .filter_map(MdListMarkerPrefix::cast)
        .find(|prefix| {
            prefix
                .marker()
                .is_ok_and(|marker| marker.text_trimmed() == marker_text)
        })
        .unwrap_or_else(|| panic!("expected ordered marker prefix {marker_text:?} in {input:?}"))
}

fn ordered_list_item_count(input: &str) -> usize {
    let parsed = parse_markdown(input);

    parsed
        .syntax()
        .descendants()
        .filter_map(MdOrderedListItem::cast)
        .count()
}

#[test]
fn nested_ordered_marker_keeps_parent_continuation_indent() {
    let input = "+ outer\n   1. nested\n";

    assert_eq!(continuation_indents(input), [3]);
    assert_eq!(
        indent_len(marker_prefix(input, "1.").pre_marker_indent()),
        0
    );
}

#[test]
fn nested_bullet_marker_keeps_parent_continuation_indent() {
    let input = "+ outer\n   - nested\n";

    assert_eq!(continuation_indents(input), [3]);
    assert_eq!(indent_len(marker_prefix(input, "-").pre_marker_indent()), 0);
}

#[test]
fn non_one_ordered_marker_does_not_interrupt_paragraph_continuation() {
    let input = "+ outer\n  2. still paragraph\n";

    assert_eq!(ordered_list_item_count(input), 0);
}

#[test]
fn quote_prefixed_nested_ordered_marker_does_not_steal_parent_indent() {
    let input = "> + outer\n>   1. nested\n";

    assert_eq!(
        indent_len(marker_prefix(input, "1.").pre_marker_indent()),
        0
    );
}

#[test]
fn bullet_list_blank_separators_do_not_appear_as_siblings() {
    // https://github.com/biomejs/biome/issues/10386
    // Mixed item kinds (paragraph, thematic break, indent code, ATX header)
    // must not leak MdNewline separators as direct children of MdBulletList.
    let input = "* item with __bold__ and *italic*\n\
                 \n\
                 * item with `code` gg\n\
                 \n\
                 * item with `code`\n\
                 \n\
                 * - - -\n\
                 \n\
                 * - - -\n\
                 \n\
                 *     gg\n\
                 \n\
                 * # Header\n\
                 \n\
                 * **bold**\n";

    let parsed = parse_markdown(input);
    assert!(
        !parsed.has_errors(),
        "expected clean parse, got: {:?}",
        parsed.diagnostics()
    );

    let syntax = parsed.syntax();
    let stray: Vec<_> = syntax
        .descendants()
        .filter(|n| n.kind() == MarkdownSyntaxKind::MD_BULLET_LIST)
        .flat_map(|list| list.children())
        .filter(|child| child.kind() == MarkdownSyntaxKind::MD_NEWLINE)
        .map(|n| n.text_trimmed_range())
        .collect();
    assert!(
        stray.is_empty(),
        "blank-line separators must live inside an MdBullet item, not as direct \
         MdBulletList children. Stray MdNewline ranges: {stray:?}\n\n{syntax:#?}"
    );
}

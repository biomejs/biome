use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::{
    MarkdownSyntaxKind, MdContinuationIndent, MdListMarkerPrefix, MdOrderedListItem,
};
use biome_rowan::{AstNode, AstNodeList, Direction, TextRange};

fn indent_len(indent: impl AstNodeList) -> usize {
    // Indent runs are folded into single nodes, so measure text length
    // instead of counting nodes.
    indent
        .iter()
        .map(|node| usize::from(node.syntax().text_trimmed_range().len()))
        .sum()
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

/// Returns the text ranges of any `MdNewline` that appears as a direct child of
/// an `MdBulletList`. The grammar (`MdBulletList = MdBullet*`) forbids these:
/// separator blank lines must live inside the preceding item's block. We walk
/// the raw syntax tree (not the typed `MdBulletList` iterator, which silently
/// filters to `MdBullet`) so an invalid child cannot hide.
fn stray_bullet_list_newlines(input: &str) -> Vec<TextRange> {
    parse_markdown(input)
        .syntax()
        .descendants()
        .filter(|n| n.kind() == MarkdownSyntaxKind::MD_BULLET_LIST)
        .flat_map(|list| list.children())
        .filter(|child| child.kind() == MarkdownSyntaxKind::MD_NEWLINE)
        .map(|n| n.text_trimmed_range())
        .collect()
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

    let stray = stray_bullet_list_newlines(input);
    assert!(
        stray.is_empty(),
        "blank-line separators must live inside an MdBullet item, not as direct \
         MdBulletList children. Stray MdNewline ranges: {stray:?}\n\n{:#?}",
        parsed.syntax()
    );
}

#[test]
fn blockquoted_bullet_list_blank_separators_do_not_appear_as_siblings() {
    // The quote-prefixed blank-line path consumes blanks one line at a time
    // (each line keeps its own `>` prefix), so guard it explicitly: even there,
    // no MdNewline may end up as a direct MdBulletList child.
    let input = "> * first\n\
                 >\n\
                 > * second\n\
                 >\n\
                 >\n\
                 > * third\n";

    let stray = stray_bullet_list_newlines(input);
    assert!(
        stray.is_empty(),
        "blockquoted list separators leaked as MdBulletList children: {stray:?}\n\n{:#?}",
        parse_markdown(input).syntax()
    );
}

/// Counts the direct `MdBullet` children of the first `MdBulletList`.
fn bullet_item_count(input: &str) -> usize {
    bullet_item_counts(input).first().copied().unwrap_or(0)
}

/// Counts the direct `MdBullet` children of every `MdBulletList`, in
/// document order — one entry per list, so nesting is observable.
fn bullet_item_counts(input: &str) -> Vec<usize> {
    parse_markdown(input)
        .syntax()
        .descendants()
        .filter(|n| n.kind() == MarkdownSyntaxKind::MD_BULLET_LIST)
        .map(|list| {
            list.children()
                .filter(|c| c.kind() == MarkdownSyntaxKind::MD_BULLET)
                .count()
        })
        .collect()
}

/// Returns true if the typed AST debug-print contains a missing required slot.
fn has_missing_required(input: &str) -> bool {
    let parsed = parse_markdown(input);
    format!("{:#?}", parsed.tree()).contains("missing (required)")
}

/// Text of every `MD_CODE_LITERAL` token in the tree. Document-level fenced
/// code blocks store their content in exactly one such token; quote- and
/// list-nested ones must produce none.
fn code_literal_texts(input: &str) -> Vec<String> {
    parse_markdown(input)
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token())
        .filter(|token| token.kind() == MarkdownSyntaxKind::MD_CODE_LITERAL)
        .map(|token| token.text().to_string())
        .collect()
}

#[test]
fn document_fence_content_is_one_verbatim_literal() {
    // CRLF must survive verbatim inside the literal. This can't live in the
    // fixture suite: checkouts may normalize line endings.
    let input = "```\r\nfoo\r\n```\r\n";
    assert_eq!(code_literal_texts(input), ["\r\nfoo\r\n"]);
    assert!(
        !has_missing_required(input),
        "document-level fence left a missing required slot\n\n{:#?}",
        parse_markdown(input).tree()
    );
}

#[test]
fn unterminated_document_fence_literal_runs_to_eof() {
    let input = "```\nfoo";
    assert_eq!(code_literal_texts(input), ["\nfoo"]);
    assert!(
        !has_missing_required(input),
        "unterminated fence left a missing required slot\n\n{:#?}",
        parse_markdown(input).tree()
    );
}

#[test]
fn nested_fence_content_keeps_per_line_tokens() {
    // Container prefixes (`>`/list indent) interleave with fence content, so
    // quote- and list-nested fences keep the per-line representation.
    for input in ["> ```\n> a\n> ```\n", "- ```\n  a\n  ```\n"] {
        assert!(
            code_literal_texts(input).is_empty(),
            "nested fence content was folded into a code literal for {input:?}\n\n{:#?}",
            parse_markdown(input).tree()
        );
    }
}

#[test]
fn unterminated_fence_has_no_missing_required_slot() {
    // CommonMark §4.5: a fenced code block need not be closed; if the end of
    // the container is reached, the block ends there. The absent closing fence
    // must not leave a "missing (required)" `r_fence` slot in the CST.
    for input in ["```\n", "~~~\n", "```\naaa\n"] {
        assert!(
            !has_missing_required(input),
            "unterminated fence left a missing required slot for {input:?}\n\n{:#?}",
            parse_markdown(input).tree()
        );
    }
}

#[test]
fn fence_as_bullet_item_does_not_swallow_siblings() {
    // https://github.com/biomejs/biome/issues/ (fence-as-bullet differential)
    // `- ```` opens a fenced code block whose content indent is the item's
    // marker width (2). The next line `- x` is at column 0, below that indent,
    // so it cannot continue the item: item 1's fence ends unterminated and
    // `- x` starts a sibling. CommonMark yields three items, not one swallowed
    // block. The unterminated fences must also leave no missing required slot.
    let input = "- ```\n- x\n- ```\n";

    assert_eq!(
        bullet_item_count(input),
        3,
        "fence in a bullet item swallowed its siblings:\n{:#?}",
        parse_markdown(input).tree()
    );
    assert!(
        !has_missing_required(input),
        "fence-as-bullet left a missing required slot:\n{:#?}",
        parse_markdown(input).tree()
    );
}

#[test]
fn nested_fence_as_bullet_item_does_not_swallow_siblings() {
    // Same defect one level deep: the inner list's items must stay separate.
    // CommonMark yields an outer list of one item (`a`) whose inner list has
    // three items (empty fence, `x`, empty fence). Asserting the per-list
    // counts — not just the absence of a missing slot — guards the swallow.
    let input = "- a\n  - ```\n  - x\n  - ```\n";

    assert_eq!(
        bullet_item_counts(input),
        [1, 3],
        "nested fence in a bullet item swallowed its siblings:\n{:#?}",
        parse_markdown(input).tree()
    );
    assert!(
        !has_missing_required(input),
        "nested fence-as-bullet left a missing required slot:\n{:#?}",
        parse_markdown(input).tree()
    );
}

#[test]
fn blockquoted_fence_as_bullet_item_does_not_swallow_siblings() {
    // The fence-as-bullet break must fire before the line's blockquote prefix
    // is consumed into the code block; otherwise `> - x` has its `>` absorbed
    // and `- x` is mis-parsed as a paragraph inside item 1. CommonMark yields a
    // blockquote whose list has three items (empty fence, `x`, empty fence).
    let input = "> - ```\n> - x\n> - ```\n";

    assert_eq!(
        bullet_item_counts(input),
        [3],
        "blockquoted fence in a bullet item swallowed its siblings:\n{:#?}",
        parse_markdown(input).tree()
    );
    assert!(
        !has_missing_required(input),
        "blockquoted fence-as-bullet left a missing required slot:\n{:#?}",
        parse_markdown(input).tree()
    );
}

#[test]
fn no_fixture_has_bullet_list_newline_siblings() {
    // Corpus-level invariant: parse every checked-in Markdown fixture and assert
    // none of them produce an MdNewline as a direct MdBulletList child. This
    // guards the grammar `MdBulletList = MdBullet*` against regressions across
    // the whole spec/regression suite, not just the synthetic cases above.
    let suite = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/md_test_suite");
    let mut offenders: Vec<String> = Vec::new();

    let mut stack = vec![std::path::PathBuf::from(suite)];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("readable fixture dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let input = std::fs::read_to_string(&path).expect("readable fixture");
            let stray = stray_bullet_list_newlines(&input);
            if !stray.is_empty() {
                offenders.push(format!("{}: {stray:?}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "fixtures with MdNewline as a direct MdBulletList child:\n{}",
        offenders.join("\n")
    );
}

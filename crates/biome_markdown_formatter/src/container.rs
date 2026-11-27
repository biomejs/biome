use biome_markdown_syntax::{
    MarkdownSyntaxNode, MdBlockList, MdBullet, MdHardLine, MdIndentToken, MdQuote, MdQuotePrefix,
    list_ext::AnyListItem,
};
use biome_rowan::{AstNode, AstNodeList, Direction, WalkEvent};

/// Returns whether the nearest list item or block quote around `node` is a
/// list item that a block quote encloses.
pub(crate) fn is_in_quoted_list_item(node: &MarkdownSyntaxNode) -> bool {
    let mut ancestors = node.ancestors();
    ancestors
        .find(|ancestor| MdBullet::can_cast(ancestor.kind()) || MdQuote::can_cast(ancestor.kind()))
        .is_some_and(|container| MdBullet::can_cast(container.kind()))
        && ancestors.any(|ancestor| MdQuote::can_cast(ancestor.kind()))
}

/// Returns whether `node` isn't inside a block quote or list item.
pub(crate) fn is_outermost_container(node: &MarkdownSyntaxNode) -> bool {
    !node
        .ancestors()
        .skip(1)
        .any(|ancestor| MdBullet::can_cast(ancestor.kind()) || MdQuote::can_cast(ancestor.kind()))
}

/// Returns whether the outermost quote or list `node` must keep its source
/// columns: a quoted list has a lazy line after a hard break, or an indented
/// partial quote prefix follows a container holding a quoted list.
///
/// A hard break prevents joining its lazy continuation to the preceding line.
/// The parser represents some indented partial prefixes as separate blocks,
/// so removing their indentation can change paragraph and inline boundaries
/// on the next formatting pass. Both forms keep the container as written.
pub(crate) fn has_quote_continuation_to_preserve(node: &MarkdownSyntaxNode) -> bool {
    // An indented quote can start after an item it cannot continue. Keep
    // both containers: changing the list marker's width could otherwise
    // move the quote prefix into the item.
    let boundary_quote =
        MdQuote::cast(node.clone()).or_else(|| node.next_sibling().and_then(MdQuote::cast));
    if let Some(quote) = boundary_quote
        && quote
            .prefix()
            .is_ok_and(|prefix| !prefix.pre_marker_indent().is_empty())
        && quote
            .syntax()
            .prev_sibling()
            .filter(|previous| AnyListItem::can_cast(previous.kind()))
            .and_then(|previous| previous.last_token())
            .and_then(|token| token.parent())
            .is_some_and(|tail| is_in_quoted_list_item(&tail))
    {
        return true;
    }

    let kinds = MdQuote::KIND_SET
        .union(MdBullet::KIND_SET)
        .union(MdHardLine::KIND_SET)
        .union(MdQuotePrefix::KIND_SET);
    let mut quote_depth = 0usize;
    let mut last_quoted_list = None;
    let mut last_eligible_prefix_range = None;
    let mut prefix_count = 0;
    let mut prefix_has_indent = false;

    for event in node.preorder_of_kinds(kinds) {
        match event {
            WalkEvent::Enter(node) if MdQuote::can_cast(node.kind()) => quote_depth += 1,
            WalkEvent::Leave(node) if MdQuote::can_cast(node.kind()) => quote_depth -= 1,
            WalkEvent::Enter(node) if MdBullet::can_cast(node.kind()) && quote_depth > 0 => {
                last_quoted_list = Some((node.text_range_with_trivia(), quote_depth));
            }
            WalkEvent::Enter(hard_line) if MdHardLine::can_cast(hard_line.kind()) => {
                if hard_line
                    .siblings(Direction::Next)
                    .skip(1)
                    .find(|next| !MdIndentToken::can_cast(next.kind()))
                    .is_some_and(|next| !MdQuotePrefix::can_cast(next.kind()))
                    && is_in_quoted_list_item(&hard_line)
                {
                    return true;
                }
            }
            WalkEvent::Enter(prefix) if MdQuotePrefix::can_cast(prefix.kind()) => {
                let Some(prefix) = MdQuotePrefix::cast(prefix) else {
                    continue;
                };
                if !prefix
                    .syntax()
                    .parent()
                    .is_some_and(|parent| MdBlockList::can_cast(parent.kind()))
                {
                    continue;
                }
                // Range containment identifies the preceding quoted list
                // without rescanning its container. Eligibility continues
                // through adjacent quote markers on the same line.
                let follows_quoted_list = prefix.syntax().prev_sibling().is_some_and(|previous| {
                    let range = previous.text_range_with_trivia();
                    if last_eligible_prefix_range == Some(range) {
                        prefix_count += 1;
                        prefix_has_indent |= !prefix.pre_marker_indent().is_empty();
                        return true;
                    }
                    prefix_count = 1;
                    prefix_has_indent = !prefix.pre_marker_indent().is_empty();
                    (MdQuote::can_cast(previous.kind()) || AnyListItem::can_cast(previous.kind()))
                        && last_quoted_list.is_some_and(|(list, _)| range.contains_range(list))
                });
                last_eligible_prefix_range =
                    follows_quoted_list.then_some(prefix.syntax().text_range_with_trivia());
                let ends_prefix_run = prefix
                    .syntax()
                    .next_sibling()
                    .is_none_or(|next| !MdQuotePrefix::can_cast(next.kind()));
                // A complete prefix run inside a root list carries that
                // list's continuation indentation and keeps normal blank-line
                // cleanup. A root quote has no enclosing list to keep its
                // boundary separate from following indented code.
                let fully_prefixed_root_list = AnyListItem::can_cast(node.kind())
                    && last_quoted_list.is_some_and(|(_, depth)| prefix_count >= depth);
                if follows_quoted_list
                    && prefix_has_indent
                    && ends_prefix_run
                    && !fully_prefixed_root_list
                {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

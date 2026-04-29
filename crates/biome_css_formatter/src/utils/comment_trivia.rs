use crate::prelude::*;
use biome_css_syntax::{CssLanguage, CssSyntaxNode};
use biome_formatter::comments::{CommentKind, DecoratedComment};

/// Returns `true` when the last leading block comment stays with this node.
pub(crate) fn has_same_group_leading_block_comment(node: &CssSyntaxNode, f: &CssFormatter) -> bool {
    f.comments()
        .leading_comments(node)
        .last()
        .is_some_and(|comment| {
            matches!(
                comment.kind(),
                CommentKind::Block | CommentKind::InlineBlock
            ) && comment.lines_after() <= 1
        })
}

/// Returns `true` for comments that stay on the node's closing line.
///
/// Example: `(a, b) /* end */`.
pub(crate) fn has_inline_trailing_comment(node: &CssSyntaxNode) -> bool {
    node.last_token().is_some_and(|token| {
        let trailing = token.trailing_trivia();

        !trailing.pieces().any(|piece| piece.is_newline())
            && trailing.pieces().any(|piece| piece.is_comments())
    })
}

/// Returns true when `comment` is in the node's trailing trivia.
pub(crate) fn is_trailing_comment_on_node(
    node: &CssSyntaxNode,
    comment: &DecoratedComment<CssLanguage>,
) -> bool {
    let comment_range = comment.piece().text_range();

    node.last_token().is_some_and(|token| {
        token
            .trailing_trivia()
            .pieces()
            .any(|piece| piece.is_comments() && piece.text_range() == comment_range)
    })
}

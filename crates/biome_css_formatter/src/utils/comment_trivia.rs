use crate::prelude::*;
use biome_css_syntax::CssSyntaxNode;
use biome_formatter::comments::CommentKind;

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

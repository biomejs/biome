use crate::comments::CssCommentStyle;
use crate::prelude::*;
use biome_css_syntax::{CssLanguage, CssSyntaxNode, CssSyntaxToken};
use biome_formatter::comments::{CommentKind, CommentStyle, DecoratedComment, SourceComment};
use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{SyntaxResult, SyntaxTriviaPiece, SyntaxTriviaPieceComments};

/// Returns `true` for CSS `/* ... */` comments.
pub(crate) fn is_block_style_comment(piece: &SyntaxTriviaPiece<CssLanguage>) -> bool {
    piece
        .as_comments()
        .is_some_and(|comment| CssCommentStyle::get_comment_kind(&comment).is_inline())
}

/// Returns `true` for `// c` trivia.
///
/// ```scss
/// color: red; // c
/// ```
pub(crate) fn has_line_comment(trivia: SyntaxTrivia<CssLanguage>) -> bool {
    trivia
        .pieces()
        .filter_map(|piece| piece.as_comments())
        .any(|comment| CssCommentStyle::get_comment_kind(&comment).is_line())
}

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

        !trailing.pieces().any(|piece| piece.is_newline()) && token.has_trailing_comments()
    })
}

/// Returns `true` when `comment` is in the node's trailing trivia.
pub(crate) fn is_trailing_comment_on_node(
    node: &CssSyntaxNode,
    comment: &DecoratedComment<CssLanguage>,
) -> bool {
    let comment_piece = comment.piece().as_piece();

    node.last_token().is_some_and(|token| {
        token == comment_piece.token()
            && token
                .trailing_trivia()
                .text_range()
                .contains_range(comment_piece.text_range())
    })
}

/// Returns `true` when `comment` is in a node's leading trivia.
///
/// ```scss
/// // c
/// $color: red;
/// ```
pub(crate) fn is_leading_comment_on_node(
    node: &CssSyntaxNode,
    comment: &SyntaxTriviaPieceComments<CssLanguage>,
) -> bool {
    let comment_piece = comment.as_piece();

    node.first_token().is_some_and(|token| {
        token == comment_piece.token()
            && token
                .leading_trivia()
                .text_range()
                .contains_range(comment_piece.text_range())
    })
}

/// Returns `true` for the gap before `:` in `a { color/* c */ : red; }`.
pub(crate) fn has_source_gap_before_token(
    comments: &[SourceComment<CssLanguage>],
    token: &SyntaxResult<CssSyntaxToken>,
) -> bool {
    let (Some(last_comment), Ok(token)) = (comments.last(), token.as_ref()) else {
        return false;
    };

    last_comment.piece().text_range().end() < token.text_trimmed_range().start()
}

/// Returns `true` for a pre-token block-comment gap.
///
/// Ignores SCSS line comments before `;` for idempotency:
///
/// ```text
/// $x: red !default // c
/// ;
/// ```
pub(crate) fn has_block_comment_gap_before_token(token: &CssSyntaxToken) -> bool {
    let token_start = token.text_trimmed_range().start();

    if let Some(comment) = token
        .leading_trivia()
        .pieces()
        .filter(is_block_style_comment)
        .last()
    {
        return comment.text_range().end() < token_start;
    }

    token.prev_token().is_some_and(|previous_token| {
        previous_token
            .trailing_trivia()
            .pieces()
            .filter(is_block_style_comment)
            .last()
            .is_some_and(|comment| comment.text_range().end() < token_start)
    })
}

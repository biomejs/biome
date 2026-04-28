//! Comment placement rules for SCSS map closing comments.
//!
//! These rules attach comments before formatting so map renderers only need to
//! print dangling or trailing comments.

use crate::utils::scss_context::is_in_scss_include_arguments;
use crate::utils::scss_expression::include_keyword_argument_before_argument_list;
use biome_css_syntax::{
    CssLanguage, CssSyntaxKind, CssSyntaxNode, ScssMapExpression, ScssMapExpressionPair,
};
use biome_formatter::comments::{CommentPlacement, DecoratedComment};
use biome_rowan::AstNode;

/// Places comments that follow a map separator in an include argument.
///
/// Example: `@include mix((a: b, /* end */))`.
pub(crate) fn place_map_trailing_separator_comment(
    map_expression: &ScssMapExpression,
    preceding_pair: &ScssMapExpressionPair,
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    classify_map_trailing_separator_comment(map_expression, preceding_pair, &comment)
        .into_comment_placement(comment)
}

/// Semantic attachment chosen by the map comment classifier.
enum IncludeCommentAttachment {
    Leading(CssSyntaxNode),
    Trailing(CssSyntaxNode),
    Dangling(CssSyntaxNode),
    Default,
}

impl IncludeCommentAttachment {
    fn into_comment_placement(
        self,
        comment: DecoratedComment<CssLanguage>,
    ) -> CommentPlacement<CssLanguage> {
        match self {
            Self::Leading(node) => CommentPlacement::leading(node, comment),
            Self::Trailing(node) => CommentPlacement::trailing(node, comment),
            Self::Dangling(node) => CommentPlacement::dangling(node, comment),
            Self::Default => CommentPlacement::Default(comment),
        }
    }
}

/// Classifies comments after map separators before attaching them.
fn classify_map_trailing_separator_comment(
    map_expression: &ScssMapExpression,
    preceding_pair: &ScssMapExpressionPair,
    comment: &DecoratedComment<CssLanguage>,
) -> IncludeCommentAttachment {
    let is_include_argument = is_in_scss_include_arguments(map_expression.syntax());

    if is_include_argument
        && include_keyword_argument_before_argument_list(map_expression.syntax()).is_some()
        && comment.kind().is_line()
        && comment.text_position().is_end_of_line()
    {
        return IncludeCommentAttachment::Trailing(preceding_pair.syntax().clone());
    }

    if is_trailing_comment_on_node(preceding_pair.syntax(), comment) {
        return IncludeCommentAttachment::Default;
    }

    if is_include_argument {
        return attachment_before_next_or_closing(comment, map_expression.syntax().clone());
    }

    if !comment.kind().is_inline() || comment.text_position().is_own_line() {
        return IncludeCommentAttachment::Default;
    }

    attachment_before_closing_or_default(comment, map_expression.syntax().clone())
}

/// Places comments before the next item, or before a closing `)`.
fn attachment_before_next_or_closing(
    comment: &DecoratedComment<CssLanguage>,
    closing_owner: CssSyntaxNode,
) -> IncludeCommentAttachment {
    if let Some(following_node) = comment.following_node() {
        return IncludeCommentAttachment::Leading(following_node.clone());
    }

    attachment_before_closing_or_default(comment, closing_owner)
}

/// Places comments only when they belong before a closing `)`.
fn attachment_before_closing_or_default(
    comment: &DecoratedComment<CssLanguage>,
    closing_owner: CssSyntaxNode,
) -> IncludeCommentAttachment {
    if follows_closing_paren(comment) {
        IncludeCommentAttachment::Dangling(closing_owner)
    } else {
        IncludeCommentAttachment::Default
    }
}

fn follows_closing_paren(comment: &DecoratedComment<CssLanguage>) -> bool {
    comment.following_token().map(|token| token.kind()) == Some(CssSyntaxKind::R_PAREN)
}

fn is_trailing_comment_on_node(
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

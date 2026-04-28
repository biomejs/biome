//! Comment placement rules for SCSS `@include` arguments.
//!
//! Separator-path comments are attached before formatting so renderers only
//! need to print leading, trailing, or dangling comments.

use crate::utils::scss_context::is_in_scss_include_arguments;
use crate::utils::scss_expression::{
    include_keyword_argument_before_argument_list, is_self_breaking_value,
    scss_keyword_argument_from_syntax,
};
use biome_css_syntax::{
    CssLanguage, CssParameterList, CssSyntaxKind, CssSyntaxNode, ScssIncludeArgumentList,
    ScssKeywordArgument, ScssListExpression, ScssMapExpression, ScssMapExpressionPair,
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

/// Places comments on include-owned separator paths.
///
/// Example: `@include mix((a, b, /* inner */))`.
pub(crate) fn place_separated_list_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    classify_separated_list_comment(&comment).into_comment_placement(comment)
}

/// Semantic attachment chosen by the include comment classifier.
enum IncludeCommentAttachment {
    Leading(CssSyntaxNode),
    Trailing(CssSyntaxNode),
    Dangling(CssSyntaxNode),
    Default,
}

impl IncludeCommentAttachment {
    /// Converts the semantic attachment into formatter comment placement.
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

/// Classifies comments after include-owned list separators.
fn classify_separated_list_comment(
    comment: &DecoratedComment<CssLanguage>,
) -> IncludeCommentAttachment {
    let Some(preceding_node) = comment.preceding_node() else {
        return IncludeCommentAttachment::Default;
    };

    if !is_in_scss_include_arguments(preceding_node) {
        return IncludeCommentAttachment::Default;
    }

    if let Some(keyword_argument) = keyword_argument_value_comment_owner(comment) {
        return IncludeCommentAttachment::Dangling(keyword_argument.into_syntax());
    }

    let Some(closing_owner) = include_separator_comment_owner(preceding_node) else {
        return IncludeCommentAttachment::Default;
    };

    if comment.kind().is_line() {
        return classify_line_separator_comment(comment, preceding_node, closing_owner);
    }

    classify_block_separator_comment(comment, preceding_node, closing_owner)
}

/// Finds the keyword argument that owns a trailing self-breaking value comment.
///
/// Example: `@include mix($arg: (a, b) /* end */, $next: 1px)`.
fn keyword_argument_value_comment_owner(
    comment: &DecoratedComment<CssLanguage>,
) -> Option<ScssKeywordArgument> {
    if !comment.kind().is_inline() {
        return None;
    }

    if comment.text_position().is_own_line()
        && comment.following_token().map(|token| token.kind()) != Some(CssSyntaxKind::COMMA)
    {
        return None;
    }

    let enclosing_node = comment.enclosing_node();
    if include_keyword_argument_before_argument_list(enclosing_node).is_some() {
        return None;
    }

    let preceding_node = comment.preceding_node()?;
    let keyword_argument = include_keyword_argument_before_argument_list(preceding_node)
        .or_else(|| scss_keyword_argument_from_syntax(preceding_node))?;
    let value = keyword_argument.value().ok()?;

    is_self_breaking_value(&value).then_some(keyword_argument)
}

/// Finds the innermost list-like node that can own a closing comment.
fn include_separator_comment_owner(node: &CssSyntaxNode) -> Option<CssSyntaxNode> {
    node.ancestors().find(|ancestor| {
        ScssListExpression::can_cast(ancestor.kind())
            || CssParameterList::can_cast(ancestor.kind())
            || ScssIncludeArgumentList::can_cast(ancestor.kind())
    })
}

/// Classifies line comments after include-owned separators.
fn classify_line_separator_comment(
    comment: &DecoratedComment<CssLanguage>,
    preceding_node: &CssSyntaxNode,
    closing_owner: CssSyntaxNode,
) -> IncludeCommentAttachment {
    if should_keep_line_comment_trailing(comment, preceding_node) {
        return IncludeCommentAttachment::Trailing(preceding_node.clone());
    }

    attachment_before_next_or_closing(comment, closing_owner)
}

/// Keeps Prettier-style line comments that were written before the separator.
///
/// Example: `@include mix(1px // note\n, 2px)`.
fn should_keep_line_comment_trailing(
    comment: &DecoratedComment<CssLanguage>,
    preceding_node: &CssSyntaxNode,
) -> bool {
    if follows_closing_paren(comment) && is_trailing_comment_on_node(preceding_node, comment) {
        return true;
    }

    if scss_keyword_argument_from_syntax(preceding_node).is_some() {
        return false;
    }

    matches!(
        comment.following_token().map(|token| token.kind()),
        Some(CssSyntaxKind::COMMA | CssSyntaxKind::CSS_NUMBER_LITERAL)
    )
}

/// Classifies block comments after include-owned separators.
fn classify_block_separator_comment(
    comment: &DecoratedComment<CssLanguage>,
    preceding_node: &CssSyntaxNode,
    closing_owner: CssSyntaxNode,
) -> IncludeCommentAttachment {
    if follows_closing_paren(comment) {
        return IncludeCommentAttachment::Dangling(closing_owner);
    }

    let belongs_before_following = comment.text_position().is_own_line()
        || !is_trailing_comment_on_node(preceding_node, comment);

    if let Some(following_node) = comment
        .following_node()
        .filter(|_| belongs_before_following)
    {
        return IncludeCommentAttachment::Leading(separator_comment_owner(following_node));
    }

    IncludeCommentAttachment::Default
}

/// Places comments before the next item, or before a closing `)`.
fn attachment_before_next_or_closing(
    comment: &DecoratedComment<CssLanguage>,
    closing_owner: CssSyntaxNode,
) -> IncludeCommentAttachment {
    if let Some(following_node) = comment.following_node() {
        return IncludeCommentAttachment::Leading(separator_comment_owner(following_node));
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

/// Prefers the SCSS item that owns formatting over its expression wrapper.
fn separator_comment_owner(node: &CssSyntaxNode) -> CssSyntaxNode {
    scss_keyword_argument_from_syntax(node).map_or_else(|| node.clone(), AstNode::into_syntax)
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

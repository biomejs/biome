//! Comment placement rules for SCSS separated forms.
//!
//! Separator comments are attached before formatting so renderers only need to
//! print leading, trailing, or dangling comments.

use crate::utils::comment_trivia::{is_block_style_comment, is_trailing_comment_on_node};
use crate::utils::scss_expression::is_self_breaking_value;
use biome_css_syntax::{
    CssLanguage, CssParameterList, CssSyntaxKind, CssSyntaxNode, ScssIncludeArgumentList,
    ScssKeywordArgument, ScssListExpression, ScssListExpressionElement, ScssMapExpression,
    ScssMapExpressionPair, is_in_scss_include_arguments, scss_include_keyword_argument_owner,
    scss_keyword_argument_from_syntax,
};
use biome_formatter::comments::{CommentPlacement, DecoratedComment};
use biome_rowan::AstNode;

/// Places comments that follow a map separator.
///
/// Example: `(a: b, /* end */)`.
pub(crate) fn place_map_trailing_separator_comment(
    map_expression: &ScssMapExpression,
    preceding_pair: &ScssMapExpressionPair,
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    place_map_separator_comment(map_expression, preceding_pair, comment)
}

/// Places comments that follow a list separator.
///
/// Example: `$list: a, // next\nb;`.
pub(crate) fn place_list_trailing_separator_comment(
    list_expression: &ScssListExpression,
    preceding_element: &ScssListExpressionElement,
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    place_list_separator_comment(list_expression, preceding_element, comment)
}

/// Places comments on include-owned separator paths.
///
/// Example: `@include mix((a, b, /* inner */))`.
pub(crate) fn place_separated_list_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    place_include_separator_comment(comment)
}

/// Places comments after map separators.
fn place_map_separator_comment(
    map_expression: &ScssMapExpression,
    preceding_pair: &ScssMapExpressionPair,
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let is_include_argument = is_in_scss_include_arguments(map_expression.syntax());
    let is_separator_comment = is_trailing_separator_comment(preceding_pair.syntax(), &comment);

    if is_include_argument
        && scss_include_keyword_argument_owner(map_expression.syntax()).is_some()
        && comment.kind().is_line()
        && comment.text_position().is_end_of_line()
    {
        return CommentPlacement::trailing(preceding_pair.syntax().clone(), comment);
    }

    if !is_separator_comment && is_trailing_comment_on_node(preceding_pair.syntax(), &comment) {
        return CommentPlacement::Default(comment);
    }

    if is_include_argument {
        return attachment_before_next_or_closing(comment, map_expression.syntax().clone());
    }

    let closing_owner = map_expression.pairs().into_syntax();

    if !comment.kind().is_line()
        && is_block_group_before_map_closing(preceding_pair.syntax(), &comment)
    {
        return CommentPlacement::dangling(closing_owner, comment);
    }

    if is_separator_comment && comment.following_node().is_none() {
        return if comment.kind().is_line() {
            CommentPlacement::Default(comment)
        } else {
            attachment_before_closing_or_default(comment, closing_owner)
        };
    }

    if comment.kind().is_line() && comment.text_position().is_end_of_line() && is_separator_comment
    {
        return attachment_before_next_or_closing(comment, map_expression.syntax().clone());
    }

    if !comment.kind().is_inline() || comment.text_position().is_own_line() {
        return CommentPlacement::Default(comment);
    }

    attachment_before_next_or_closing(comment, closing_owner)
}

/// Places comments after list separators.
fn place_list_separator_comment(
    list_expression: &ScssListExpression,
    preceding_element: &ScssListExpressionElement,
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    if is_in_scss_include_arguments(list_expression.syntax()) {
        return CommentPlacement::Default(comment);
    }

    let is_separator_comment = is_trailing_separator_comment(preceding_element.syntax(), &comment);

    if comment.kind().is_line()
        && comment.text_position().is_end_of_line()
        && is_separator_comment
        && comment.following_node().is_some()
    {
        attachment_before_next_or_closing(comment, list_expression.syntax().clone())
    } else {
        CommentPlacement::Default(comment)
    }
}

/// Places comments after include-owned list separators.
fn place_include_separator_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    let Some(preceding_node) = comment.preceding_node().cloned() else {
        return CommentPlacement::Default(comment);
    };

    if !is_in_scss_include_arguments(&preceding_node) {
        return CommentPlacement::Default(comment);
    }

    if let Some(keyword_argument) = keyword_argument_value_comment_owner(&comment) {
        return CommentPlacement::dangling(keyword_argument.into_syntax(), comment);
    }

    let Some(closing_owner) = include_separator_comment_owner(&preceding_node) else {
        return CommentPlacement::Default(comment);
    };

    if comment.kind().is_line() {
        return place_line_separator_comment(comment, &preceding_node, closing_owner);
    }

    place_block_separator_comment(comment, &preceding_node, closing_owner)
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
    if scss_include_keyword_argument_owner(enclosing_node).is_some() {
        return None;
    }

    let preceding_node = comment.preceding_node()?;
    let keyword_argument = scss_include_keyword_argument_owner(preceding_node)
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

/// Places line comments after include-owned separators.
fn place_line_separator_comment(
    comment: DecoratedComment<CssLanguage>,
    preceding_node: &CssSyntaxNode,
    closing_owner: CssSyntaxNode,
) -> CommentPlacement<CssLanguage> {
    if should_keep_line_comment_trailing(&comment, preceding_node) {
        return CommentPlacement::trailing(preceding_node.clone(), comment);
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

/// Places block comments after include-owned separators.
fn place_block_separator_comment(
    comment: DecoratedComment<CssLanguage>,
    preceding_node: &CssSyntaxNode,
    closing_owner: CssSyntaxNode,
) -> CommentPlacement<CssLanguage> {
    if follows_closing_paren(&comment) {
        return CommentPlacement::dangling(closing_owner, comment);
    }

    let belongs_before_following = comment.text_position().is_own_line()
        || !is_trailing_comment_on_node(preceding_node, &comment);

    if let Some(following_node) = comment
        .following_node()
        .filter(|_| belongs_before_following)
    {
        let owner = separator_comment_owner(following_node);
        return CommentPlacement::leading(owner, comment);
    }

    CommentPlacement::Default(comment)
}

/// Places comments before the next item, or before a closing `)`.
fn attachment_before_next_or_closing(
    comment: DecoratedComment<CssLanguage>,
    closing_owner: CssSyntaxNode,
) -> CommentPlacement<CssLanguage> {
    if let Some(following_node) = comment.following_node() {
        let owner = separator_comment_owner(following_node);
        return CommentPlacement::leading(owner, comment);
    }

    attachment_before_closing_or_default(comment, closing_owner)
}

/// Places comments only when they belong before a closing `)`.
fn attachment_before_closing_or_default(
    comment: DecoratedComment<CssLanguage>,
    closing_owner: CssSyntaxNode,
) -> CommentPlacement<CssLanguage> {
    if follows_closing_paren(&comment) {
        CommentPlacement::dangling(closing_owner, comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

/// Prefers the SCSS item that owns formatting over its expression wrapper.
fn separator_comment_owner(node: &CssSyntaxNode) -> CssSyntaxNode {
    scss_keyword_argument_from_syntax(node).map_or_else(|| node.clone(), AstNode::into_syntax)
}

fn follows_closing_paren(comment: &DecoratedComment<CssLanguage>) -> bool {
    comment
        .following_token()
        .is_some_and(|token| token.kind() == CssSyntaxKind::R_PAREN)
}

/// Returns `true` for a block comment group between a final map comma and `)`.
///
/// Example: `(a: b, /* c1 */ /* c2 */)`.
fn is_block_group_before_map_closing(
    node: &CssSyntaxNode,
    comment: &DecoratedComment<CssLanguage>,
) -> bool {
    let Some(closing_paren) = comment
        .following_token()
        .filter(|token| token.kind() == CssSyntaxKind::R_PAREN)
    else {
        return false;
    };

    let Some(comma) = node
        .last_token()
        .and_then(|token| token.next_token())
        .filter(|token| token.kind() == CssSyntaxKind::COMMA)
    else {
        return false;
    };

    if comma.text_trimmed_range().end() > comment.piece().text_range().start() {
        return false;
    }

    comma
        .trailing_trivia()
        .pieces()
        .chain(closing_paren.leading_trivia().pieces())
        .filter(|piece| {
            is_block_style_comment(piece)
                && piece.text_range().start() >= comma.text_trimmed_range().end()
                && piece.text_range().end() <= closing_paren.text_trimmed_range().start()
        })
        .take(2)
        .count()
        > 1
}

/// Returns `true` when the comment follows the separated-list comma.
fn is_trailing_separator_comment(
    node: &CssSyntaxNode,
    comment: &DecoratedComment<CssLanguage>,
) -> bool {
    let comment_piece = comment.piece().as_piece();

    node.last_token()
        .and_then(|token| token.next_token())
        .is_some_and(|token| {
            token.kind() == CssSyntaxKind::COMMA
                && token == comment_piece.token()
                && token
                    .trailing_trivia()
                    .text_range()
                    .contains_range(comment_piece.text_range())
        })
}

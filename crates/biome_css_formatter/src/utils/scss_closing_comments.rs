use crate::prelude::*;
use biome_css_syntax::{CssSyntaxNode, ScssMapExpression, is_in_scss_include_arguments};
use biome_formatter::write;
use biome_rowan::AstNode;

/// Spacing policy for comments before an include-owned closing `)`.
///
/// Example: `@include mix((a, b) /* end */)`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ClosingCommentSpacing {
    /// Space before block comments, line break before line comments.
    Adaptive,
    /// Let the group choose a space or line break before the comment.
    SoftLineBreak,
}

/// Returns `true` when an include argument node owns comments before `)`.
pub(crate) fn owns_include_closing_comments(node: &CssSyntaxNode, f: &CssFormatter) -> bool {
    is_in_scss_include_arguments(node) && f.comments().has_dangling_comments(node)
}

/// Returns `true` when this map layout prints comments before `)`.
pub(crate) fn owns_map_closing_comments(node: &ScssMapExpression, f: &CssFormatter) -> bool {
    if is_in_scss_include_arguments(node.syntax()) {
        node.pairs().len() > 0 && owns_include_closing_comments(node.syntax(), f)
    } else {
        has_inline_map_closing_comments(node, f)
    }
}

/// Writes include-owned comments before the closing `)`.
pub(crate) fn write_include_closing_comments(
    node: &CssSyntaxNode,
    spacing: ClosingCommentSpacing,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    if !owns_include_closing_comments(node, f) {
        return Ok(());
    }

    write_closing_comments(node, spacing, f)
}

/// Writes comments that stay on the include path before the closing `)`.
fn write_closing_comments(
    node: &CssSyntaxNode,
    spacing: ClosingCommentSpacing,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let has_line_closing_comment = has_line_closing_comment(node, f);

    write!(
        f,
        [
            format_space_before_closing_comment(spacing, has_line_closing_comment),
            format_dangling_comments(node),
            has_line_closing_comment.then_some(hard_line_break())
        ]
    )
}

fn format_space_before_closing_comment(
    spacing: ClosingCommentSpacing,
    has_line_closing_comment: bool,
) -> impl Format<CssFormatContext> {
    format_with(move |f| match spacing {
        ClosingCommentSpacing::Adaptive if !has_line_closing_comment => write!(f, [space()]),
        ClosingCommentSpacing::Adaptive | ClosingCommentSpacing::SoftLineBreak => {
            write!(f, [soft_line_break_or_space()])
        }
    })
}

fn has_line_closing_comment(node: &CssSyntaxNode, f: &CssFormatter) -> bool {
    f.comments()
        .dangling_comments(node)
        .iter()
        .any(|comment| comment.kind().is_line())
}

/// Returns `true` for inline comments after the last map pair.
fn has_inline_map_closing_comments(node: &ScssMapExpression, f: &CssFormatter) -> bool {
    node.pairs().len() > 0
        && !f.comments().dangling_comments(node.syntax()).is_empty()
        && f.comments()
            .dangling_comments(node.syntax())
            .iter()
            .all(|comment| comment.kind().is_inline() && comment.lines_before() == 0)
}

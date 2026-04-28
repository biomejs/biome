use crate::prelude::*;
use crate::utils::scss_context::is_in_scss_include_arguments;
use biome_css_syntax::{CssSyntaxNode, ScssMapExpression};
use biome_formatter::write;
use biome_rowan::AstNode;

/// Spacing policy for comments before an include-owned closing `)`.
///
/// Example: `@include mix((a, b) /* end */)`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ScssIncludeClosingCommentSpacing {
    /// Uses a plain space unless a line comment forces a line break.
    AdaptiveSpace,
    /// Always uses `soft_line_break_or_space()` before the closing comment.
    SoftLineBreakOrSpace,
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
    spacing: ScssIncludeClosingCommentSpacing,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    if !owns_include_closing_comments(node, f) {
        return Ok(());
    }

    write_closing_comments(node, spacing, f)
}

/// Formats include-owned closing comments for list helpers.
pub(crate) fn format_include_closing_comments(
    node: &CssSyntaxNode,
    spacing: ScssIncludeClosingCommentSpacing,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| write_include_closing_comments(node, spacing, f))
}

/// Writes comments that stay on the include path before the closing `)`.
fn write_closing_comments(
    node: &CssSyntaxNode,
    spacing: ScssIncludeClosingCommentSpacing,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let has_line_closing_comment = f
        .comments()
        .dangling_comments(node)
        .iter()
        .any(|comment| comment.kind().is_line());

    write!(
        f,
        [
            format_with(|f| match spacing {
                ScssIncludeClosingCommentSpacing::AdaptiveSpace => {
                    if has_line_closing_comment {
                        write!(f, [soft_line_break_or_space()])
                    } else {
                        write!(f, [space()])
                    }
                }
                ScssIncludeClosingCommentSpacing::SoftLineBreakOrSpace => {
                    write!(f, [soft_line_break_or_space()])
                }
            }),
            format_dangling_comments(node),
            has_line_closing_comment.then_some(hard_line_break())
        ]
    )
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

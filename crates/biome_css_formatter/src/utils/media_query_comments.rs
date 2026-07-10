use crate::prelude::*;
use crate::utils::comment_trivia::has_line_comment;
use biome_css_syntax::{AnyCssMediaQuery, CssMediaQueryList, CssSyntaxKind, CssSyntaxNode};
use biome_formatter::write;
use biome_rowan::AstSeparatedList;

/// Fills media queries, breaking before comment-started rows.
///
/// ```scss
/// @import "a.css" print,
/// // c
/// screen;
/// ```
pub(crate) fn fill_media_queries(
    node: &CssMediaQueryList,
    mut fill_entry: impl FnMut(&dyn Format<CssFormatContext>, &dyn Format<CssFormatContext>),
) {
    let mut prev_comment_row = false;

    for (element, formatted) in node.elements().zip(node.format_separated(",")) {
        let query = element.node().ok();
        let is_comment_row = has_leading_line_comment(query);
        let break_before = prev_comment_row || is_comment_row;
        let separator = format_once(move |f| {
            if break_before {
                write!(f, [hard_line_break()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        fill_entry(&separator, &formatted);

        prev_comment_row = is_comment_row;
    }
}

/// Prints a media query after separator-owned leading comments.
///
/// ```scss
/// @media print,
/// // c
/// screen {}
/// ```
pub(crate) fn fmt_media_query_node(
    node: &CssSyntaxNode,
    f: &mut CssFormatter,
    fmt_node: impl Fn(&mut CssFormatter) -> FormatResult<()>,
) -> FormatResult<()> {
    if has_comma_comment(node, f) {
        format_leading_comments(node)
            .with_following_content(fmt_node)
            .fmt(f)
    } else {
        fmt_node(f)
    }
}

/// Skips normal leading comments when the separator owns them.
///
/// ```scss
/// @media print,
/// // c
/// screen {}
/// ```
pub(crate) fn fmt_media_query_leading(
    node: &CssSyntaxNode,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    if has_comma_comment(node, f) {
        Ok(())
    } else {
        format_leading_comments(node).fmt(f)
    }
}

/// Detects comments attached to the comma before a media query.
///
/// ```scss
/// @media print, // c
/// screen {}
/// ```
fn has_comma_comment(node: &CssSyntaxNode, f: &CssFormatter) -> bool {
    f.comments().leading_comments(node).iter().any(|comment| {
        let comment_piece = comment.piece().as_piece();
        let token = comment_piece.token();

        token.kind() == CssSyntaxKind::COMMA
            || token
                .prev_token()
                .is_some_and(|token| token.kind() == CssSyntaxKind::COMMA)
    })
}

/// Detects media query rows that start with a line comment.
///
/// ```scss
/// @media print,
/// // c
/// screen {}
/// ```
fn has_leading_line_comment(query: Option<&AnyCssMediaQuery>) -> bool {
    query.is_some_and(|query| {
        query
            .syntax()
            .first_token()
            .is_some_and(|token| has_line_comment(token.leading_trivia()))
    })
}

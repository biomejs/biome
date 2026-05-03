use crate::prelude::*;
use biome_css_syntax::{CssSyntaxNode, CssSyntaxToken};
use biome_formatter::write;

/// Formats the gap after `in` in source-tight cases like `@each $x in$list`.
pub(crate) fn format_after_each_in<'a>(
    in_token: &'a CssSyntaxToken,
    iterable: &'a CssSyntaxNode,
) -> impl Format<CssFormatContext> + 'a {
    format_with(move |f| {
        if is_source_tight_after_each_in(in_token, iterable) {
            Ok(())
        } else {
            write!(f, [space()])
        }
    })
}

/// Returns `true` for source-tight `in` operands.
///
/// Prettier preserves `@each $x in$list` and `@each $x in(a: b)`, but prints
/// a space for `@each $x in $list` and `@each $x in (a: b)`.
fn is_source_tight_after_each_in(in_token: &CssSyntaxToken, iterable: &CssSyntaxNode) -> bool {
    if in_token.has_trailing_whitespace() || in_token.has_trailing_comments() {
        return false;
    }

    iterable.first_token().is_some_and(|token| {
        !token.has_leading_whitespace_or_newline() && !token.has_leading_comments()
    })
}

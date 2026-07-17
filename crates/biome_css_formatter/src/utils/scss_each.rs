use crate::prelude::*;
use biome_css_syntax::{CssSyntaxNode, CssSyntaxToken};
use biome_formatter::write;

/// Formats the gap after `in` in `@each` headers.
///
/// Prettier preserves source-tight value starts:
///
/// ```scss
/// @each $x in$list {}
/// @each $x in(a: b) {}
/// ```
///
/// If the source already has a gap, Biome prints one space:
///
/// ```scss
/// @each $x in $list {}
/// @each $x in (a: b) {}
/// ```
pub(crate) struct FormatGapAfterEachIn<'a> {
    in_token: &'a CssSyntaxToken,
    value: &'a CssSyntaxNode,
}

impl<'a> FormatGapAfterEachIn<'a> {
    pub(crate) fn new(in_token: &'a CssSyntaxToken, value: &'a CssSyntaxNode) -> Self {
        Self { in_token, value }
    }
}

impl Format<CssFormatContext> for FormatGapAfterEachIn<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let in_token = self.in_token;
        let value = self.value;

        if is_source_tight_after_each_in(in_token, value) {
            Ok(())
        } else {
            write!(f, [space()])
        }
    }
}

/// Returns `true` for source-tight `in` values.
///
/// Prettier preserves `@each $x in$list` and `@each $x in(a: b)`, but prints
/// a space for `@each $x in $list` and `@each $x in (a: b)`.
fn is_source_tight_after_each_in(in_token: &CssSyntaxToken, value: &CssSyntaxNode) -> bool {
    if in_token.has_trailing_whitespace() || in_token.has_trailing_comments() {
        return false;
    }

    value.first_token().is_some_and(|token| {
        !token.has_leading_whitespace_or_newline() && !token.has_leading_comments()
    })
}

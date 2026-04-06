use crate::parser::CssParser;
use crate::syntax::scss::is_nth_at_scss_interpolation;
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier};
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATED_IDENTIFIER_HYPHEN;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature};

#[inline]
pub(crate) fn is_at_scss_interpolated_identifier(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_nth_at_identifier(p, n) || is_nth_at_scss_interpolation(p, n))
}

/// Returns `true` when the current token continues an interpolated identifier
/// with no separating trivia.
///
/// Examples:
/// - `foo#{$bar}`
/// - `#{$a}-#{$b}`
#[inline]
pub(super) fn is_at_identifier_continuation(p: &mut CssParser) -> bool {
    is_at_adjacent_identifier(p) || is_at_identifier_hyphen(p)
}

#[inline]
fn is_at_adjacent_identifier(p: &mut CssParser) -> bool {
    !p.has_preceding_whitespace() && is_at_scss_interpolated_identifier(p)
}

/// Returns `true` when `-` belongs to the current interpolated identifier,
/// such as the hyphen in `#{$a}-#{$b}`.
#[inline]
pub(super) fn is_at_identifier_hyphen(p: &mut CssParser) -> bool {
    p.at(T![-])
        && !p.has_preceding_whitespace()
        && is_nth_at_scss_interpolated_identifier(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

#[inline]
pub(super) fn parse_identifier_hyphen(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier_hyphen(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![-]);
    Present(m.complete(p, SCSS_INTERPOLATED_IDENTIFIER_HYPHEN))
}

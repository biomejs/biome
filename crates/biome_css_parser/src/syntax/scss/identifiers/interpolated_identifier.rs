use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::is_nth_at_identifier;
use crate::syntax::scss::is_nth_at_scss_interpolation;
use biome_css_syntax::CssSyntaxKind::{
    EOF, SCSS_INTERPOLATED_IDENTIFIER_HYPHEN, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress};

#[inline]
pub(crate) fn is_at_scss_interpolated_identifier(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n) || is_nth_at_scss_interpolation(p, n)
}

/// Parses source-tight identifier parts after `first_part` until whitespace or
/// unsupported syntax ends the identifier.
///
/// Both interpolations and the intervening hyphens belong to one selector
/// identifier:
/// ```scss
/// .#{$block}--#{$modifier} {}
/// ```
pub(super) fn parse_scss_interpolated_identifier_parts(
    p: &mut CssParser,
    first_part: CompletedMarker,
    mut parse_part: impl FnMut(&mut CssParser) -> ParsedSyntax,
) -> CompletedMarker {
    let list = first_part.precede(p);
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && !p.has_preceding_whitespace() {
        progress.assert_progressing(p);

        if parse_part(p).is_absent() {
            break;
        }
    }

    list.complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST)
}

/// Returns `true` when `-` belongs to the current interpolated identifier.
///
/// Example:
/// ```scss
/// .#{$block}-#{$element} {}
/// ```
#[inline]
pub(super) fn is_at_identifier_hyphen(p: &mut CssParser) -> bool {
    is_at_identifier_hyphen_part(p)
        && !p.has_preceding_whitespace()
        && is_nth_at_scss_interpolated_identifier(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

/// Parses a source-tight hyphen that continues an interpolated identifier.
///
/// Example:
/// ```scss
/// .#{$block}-#{$element} {}
/// ```
#[inline]
pub(super) fn parse_identifier_hyphen(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier_hyphen(p) {
        return Absent;
    }

    parse_identifier_hyphen_part(p, CssLexContext::Regular)
}

/// Returns whether the token at `n` is a raw interpolated-identifier hyphen.
///
/// Callers still own the context checks used by selectors and declarations:
/// ```scss
/// .-#{$name} {}
///
/// :root {
///   --#{$prop}: 10px;
/// }
/// ```
#[inline]
pub(super) fn is_nth_at_identifier_hyphen_part(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![-])
}

#[inline]
fn is_at_identifier_hyphen_part(p: &mut CssParser) -> bool {
    is_nth_at_identifier_hyphen_part(p, 0)
}

/// Parses a raw `-` as one interpolated-identifier hyphen part.
///
/// Call only after a context-specific guard accepts the hyphen:
/// ```scss
/// .-#{$name} {}
/// ```
#[inline]
pub(super) fn parse_identifier_hyphen_part(
    p: &mut CssParser,
    context: CssLexContext,
) -> ParsedSyntax {
    if !is_at_identifier_hyphen_part(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![-], context);
    Present(m.complete(p, SCSS_INTERPOLATED_IDENTIFIER_HYPHEN))
}

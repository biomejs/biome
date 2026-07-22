use crate::parser::CssParser;
use crate::syntax::is_nth_at_identifier;
use crate::syntax::scss::expression::parse_scss_selector_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_at_identifier_hyphen, is_at_scss_interpolated_identifier, parse_identifier_hyphen,
    parse_scss_interpolated_identifier_parts,
};
use crate::syntax::scss::{is_at_scss_interpolation, is_nth_at_scss_interpolation};
use crate::syntax::selector::{
    parse_selector_custom_identifier_fragment, parse_selector_identifier_fragment,
};
use biome_css_syntax::CssSyntaxKind::{SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATION};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Returns whether the current token can start an SCSS-interpolated selector
/// identifier.
///
/// Examples: `#{$tag}` and `button-#{$variant}`.
#[inline]
pub(crate) fn is_at_scss_interpolated_selector_identifier(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_selector_identifier(p, 0)
}

/// Returns whether the token at `n` can start an SCSS-interpolated selector
/// identifier.
///
/// This is intentionally selector-specific: selector names may be built from an
/// interpolation alone (`#{$tag}`) or from a plain identifier followed by an
/// adjacent interpolation suffix (`button#{$state}` or `button-#{$state}`).
#[inline]
pub(crate) fn is_nth_at_scss_interpolated_selector_identifier(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_scss_interpolation(p, n)
        || is_nth_at_identifier(p, n) && is_nth_at_scss_selector_identifier_suffix(p, n + 1)
}

/// Returns whether the token at `n` continues a selector identifier with SCSS
/// interpolation and no separating whitespace.
///
/// The suffix can be a direct interpolation (`#{$state}`) or a hyphen followed
/// by interpolation (`-#{$state}`), matching selector names such as
/// `button#{$state}` and `button-#{$state}`.
#[inline]
fn is_nth_at_scss_selector_identifier_suffix(p: &mut CssParser, n: usize) -> bool {
    !p.has_nth_preceding_whitespace(n)
        && (is_nth_at_scss_interpolation(p, n)
            || p.nth_at(n, T![-])
                && !p.has_nth_preceding_whitespace(n + 1)
                && is_nth_at_scss_interpolation(p, n + 1))
}

#[inline]
fn is_at_selector_identifier_part(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_identifier(p) || is_at_identifier_hyphen(p)
}

/// Parses SCSS-interpolated selector name slots.
///
/// This selector-specific variant keeps the closing `}` of an interpolation in
/// selector lexing mode so following whitespace is tokenized as a selector
/// combinator (`CSS_SPACE_LITERAL`) instead of being skipped as trivia.
///
/// Examples:
/// ```scss
/// .icon-#{$name} {}
/// button-#{$variant} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_selector_identifier_with_fragment(p, parse_selector_identifier_fragment)
}

/// Parses SCSS-interpolated selector custom identifiers.
///
/// Example:
/// ```scss
/// .button-#{$variant} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_custom_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_selector_identifier_with_fragment(p, parse_selector_custom_identifier_fragment)
}

/// Parses selector identifier grammar with caller-owned non-interpolation parts.
///
/// Examples: `button-#{$variant}` and `.button-#{$variant}`.
fn parse_scss_selector_identifier_with_fragment(
    p: &mut CssParser,
    parse_selector_fragment: fn(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_identifier(p) {
        return Absent;
    }

    let Present(first_fragment) = parse_selector_identifier_part(p, parse_selector_fragment) else {
        return Absent;
    };

    // A plain selector identifier only becomes an interpolated identifier when
    // another selector identifier fragment follows with no separating trivia.
    if first_fragment.kind(p) != SCSS_INTERPOLATION
        && (p.has_preceding_whitespace() || !is_at_selector_identifier_part(p))
    {
        return Present(first_fragment);
    }

    let parts = parse_scss_interpolated_identifier_parts(p, first_fragment, |p| {
        parse_selector_identifier_part(p, parse_selector_fragment)
    });

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

#[inline]
fn parse_selector_identifier_part(
    p: &mut CssParser,
    parse_selector_fragment: fn(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_selector_interpolation(p)
    } else if is_at_identifier_hyphen(p) {
        parse_identifier_hyphen(p)
    } else {
        parse_selector_fragment(p)
    }
}

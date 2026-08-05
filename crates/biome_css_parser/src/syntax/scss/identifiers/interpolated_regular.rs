use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::scss::expression::parse_scss_regular_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_at_identifier_hyphen, is_at_scss_interpolated_identifier, is_nth_at_identifier_hyphen_part,
    is_nth_source_tight, parse_identifier_hyphen, parse_identifier_hyphen_part,
    parse_scss_interpolated_identifier_parts,
};
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolated_identifier, is_nth_at_scss_interpolation,
};
use crate::syntax::{is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATION};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Parses SCSS-interpolated name slots.
///
/// A bare interpolation in a name slot becomes an interpolated identifier
/// because callers expect a name node.
///
/// Examples:
/// ```scss
/// @media (#{$feature}: block) {}
/// [data-#{$name}] {}
/// .a { value: foo#{1 + 1}(arg); }
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolated_identifier(p) {
        parse_scss_interpolated_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
}

/// Parses identifier-shaped SCSS syntax that may contain interpolation parts.
///
/// This is different from [`parse_scss_regular_interpolation`], which parses
/// exactly one standalone interpolation value.
/// This helper parses identifier grammar, so it can consume adjacent
/// identifier and interpolation fragments with no intervening whitespace or
/// line break and combine them into one identifier-shaped node.
///
/// Examples:
/// ```scss
/// .box {
///   margin-#{$side}: 1rem;
///   #{$name}: value;
///   animation-name: size-#{$axis}-min;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_scss_interpolated_identifier(p, 0) {
        return Absent;
    }

    let Present(first_fragment) = parse_regular_identifier_part(p) else {
        return Absent;
    };

    // A plain identifier only becomes an interpolated identifier when a
    // source-tight fragment leading to interpolation follows.
    if first_fragment.kind(p) != SCSS_INTERPOLATION
        && (!is_nth_source_tight(p, 0) || !is_at_regular_interpolated_identifier_suffix(p))
    {
        return Present(first_fragment);
    }

    let parts =
        parse_scss_interpolated_identifier_parts(p, first_fragment, parse_regular_identifier_part);

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

/// Returns whether source-tight parts after the first parsed part lead to interpolation.
///
/// ```scss
/// .button-#{$state} {}
/// ```
#[inline]
fn is_at_regular_interpolated_identifier_suffix(p: &mut CssParser) -> bool {
    if is_at_scss_interpolation(p) {
        // `.button-#{$state} {}` continues directly with interpolation.
        return true;
    }

    if is_at_identifier_hyphen(p) {
        // `.#{$block}-#{$element} {}` continues through a raw hyphen.
        return is_nth_at_scss_interpolation(p, 1)
            || is_nth_at_identifier_before_interpolation(p, 1);
    }

    // `.#{$block}item#{$modifier} {}` has a plain fragment before interpolation.
    is_nth_at_identifier_before_interpolation(p, 0)
}

/// Returns whether the token at `n` is a plain identifier followed directly by
/// interpolation.
///
/// ```scss
/// .#{$block}item#{$modifier} {}
/// ```
#[inline]
fn is_nth_at_identifier_before_interpolation(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
        && is_nth_source_tight(p, n + 1)
        && is_nth_at_scss_interpolation(p, n + 1)
}

/// Returns whether an interpolated identifier starts with a single hyphen.
///
/// Example:
/// ```scss
/// .box {
///   -#{$prefix}-radius: 4px;
/// }
/// ```
#[inline]
pub(crate) fn is_nth_at_scss_hyphen_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier_hyphen_part(p, n)
        && is_nth_source_tight(p, n + 1)
        && is_nth_at_scss_interpolation(p, n + 1)
}

/// Parses an interpolated identifier that starts with a single hyphen.
///
/// Example:
/// ```scss
/// .box {
///   -#{$prefix}-radius: 4px;
/// }
/// ```
#[inline]
pub(crate) fn parse_scss_hyphen_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_scss_hyphen_interpolated_identifier(p, 0) {
        return Absent;
    }

    let Present(first_fragment) = parse_identifier_hyphen_part(p, CssLexContext::Regular) else {
        return Absent;
    };

    let parts =
        parse_scss_interpolated_identifier_parts(p, first_fragment, parse_regular_identifier_part);

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

/// Parses SCSS interpolation or adjacent identifier fragments in value slots.
///
/// Standalone interpolation remains a `ScssInterpolation`, while an adjacent
/// identifier suffix produces a `ScssInterpolatedIdentifier`.
///
/// Examples:
/// ```scss
/// .box {
///   animation-name: #{$name};
///   transition-property: #{$name}-suffix;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolation_or_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_scss_interpolated_identifier(p, 0) {
        return Absent;
    }

    if is_at_scss_interpolation(p) {
        let Present(interpolation) = parse_scss_regular_interpolation(p) else {
            return Absent;
        };

        if is_nth_source_tight(p, 0) && is_at_regular_identifier_part(p) {
            let parts = parse_scss_interpolated_identifier_parts(
                p,
                interpolation,
                parse_regular_identifier_part,
            );

            return Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER));
        }

        return Present(interpolation);
    }

    parse_scss_interpolated_identifier(p)
}

/// Returns whether the current token can be parsed as an interpolation,
/// source-tight hyphen, or plain identifier part in regular lexing context.
///
/// Example:
/// ```scss
/// .box {
///   animation-name: prefix-#{$name}-suffix;
/// }
/// ```
#[inline]
fn is_at_regular_identifier_part(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_identifier(p) || is_at_identifier_hyphen(p)
}

/// Parses one interpolation, source-tight hyphen, or plain identifier part in
/// regular lexing context.
///
/// Example:
/// ```scss
/// .box {
///   animation-name: prefix-#{$name}-suffix;
/// }
/// ```
#[inline]
fn parse_regular_identifier_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else if is_at_identifier_hyphen(p) {
        parse_identifier_hyphen(p)
    } else {
        parse_regular_identifier(p)
    }
}

use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::expression::parse_scss_regular_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_at_identifier_continuation, is_at_scss_interpolated_identifier,
    is_nth_at_identifier_hyphen_part, parse_identifier_hyphen_part,
    parse_scss_interpolated_identifier_parts,
};
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolated_identifier, is_nth_at_scss_interpolation,
};
use biome_css_syntax::CssSyntaxKind::{SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATION};
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Parses SCSS-interpolated name slots.
///
/// Bare interpolation like `#{$feature}` becomes an interpolated identifier
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
/// This is different from [`parse_scss_regular_interpolation`],
/// which parses exactly one standalone interpolation value such as `#{$name}`.
/// This helper parses identifier grammar, so it can consume adjacent
/// identifier and interpolation fragments with no intervening trivia and
/// combine them into one identifier-shaped node.
///
/// Examples:
/// ```scss
/// margin-#{$side}
/// #{$name}
/// size-#{$axis}-min
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

    // A plain identifier only becomes an interpolated identifier when another
    // identifier fragment follows with no separating trivia.
    if first_fragment.kind(p) != SCSS_INTERPOLATION && !is_at_identifier_continuation(p) {
        return Present(first_fragment);
    }

    let parts =
        parse_scss_interpolated_identifier_parts(p, first_fragment, parse_regular_identifier_part);

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

/// Returns whether an interpolated identifier starts with a single hyphen.
///
/// Example: `-#{$prefix}-radius` in `-#{$prefix}-radius: 4px;`.
#[inline]
pub(crate) fn is_nth_at_scss_hyphen_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier_hyphen_part(p, n)
        && !p.has_nth_preceding_whitespace(n + 1)
        && is_nth_at_scss_interpolation(p, n + 1)
}

/// Parses an interpolated identifier that starts with a single hyphen.
///
/// Example: `-#{$prefix}-radius` in `-#{$prefix}-radius: 4px;`.
#[inline]
pub(crate) fn parse_scss_hyphen_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_scss_hyphen_interpolated_identifier(p, 0) {
        return Absent;
    }

    let Present(first_fragment) = parse_identifier_hyphen_part(p) else {
        return Absent;
    };

    let parts =
        parse_scss_interpolated_identifier_parts(p, first_fragment, parse_regular_identifier_part);

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

/// Parses SCSS interpolation or adjacent identifier fragments in value slots.
///
/// Standalone interpolation like `#{$name}` remains a `ScssInterpolation`,
/// while adjacent forms such as `#{$name}-suffix` become a
/// `ScssInterpolatedIdentifier`.
///
/// Examples:
/// ```scss
/// #{$name}
/// #{$name}-suffix
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

        if is_at_identifier_continuation(p) {
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

#[inline]
fn parse_regular_identifier_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else {
        parse_regular_identifier(p)
    }
}

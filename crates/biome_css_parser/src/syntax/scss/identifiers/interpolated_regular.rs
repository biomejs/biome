use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::expression::parse_scss_regular_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    complete_scss_interpolated_identifier, is_at_identifier_continuation,
    is_at_scss_interpolated_identifier,
};
use crate::syntax::scss::is_at_scss_interpolation;
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATION;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

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
    if !is_at_scss_interpolated_identifier(p) {
        return Absent;
    }

    let Present(first_fragment) = parse_regular_part(p) else {
        return Absent;
    };

    // A plain identifier only becomes an interpolated identifier when another
    // identifier fragment follows with no separating trivia.
    if first_fragment.kind(p) != SCSS_INTERPOLATION && !is_at_identifier_continuation(p) {
        return Present(first_fragment);
    }

    Present(complete_scss_interpolated_identifier(
        p,
        first_fragment,
        parse_regular_part,
    ))
}

/// Parses an interpolation-led SCSS value as an interpolated identifier only
/// when adjacent identifier fragments follow immediately.
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
pub(crate) fn parse_scss_identifier_or_interpolation(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_identifier(p) {
        return Absent;
    }

    if is_at_scss_interpolation(p) {
        let Present(interpolation) = parse_scss_regular_interpolation(p) else {
            return Absent;
        };

        if is_at_identifier_continuation(p) {
            return Present(complete_scss_interpolated_identifier(
                p,
                interpolation,
                parse_regular_part,
            ));
        }

        return Present(interpolation);
    }

    parse_scss_interpolated_identifier(p)
}

#[inline]
fn parse_regular_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else {
        parse_regular_identifier(p)
    }
}

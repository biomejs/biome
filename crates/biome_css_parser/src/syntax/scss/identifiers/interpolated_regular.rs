use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::expression::parse_scss_regular_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_at_identifier_continuation, is_at_identifier_hyphen, is_at_scss_interpolated_identifier,
    parse_identifier_hyphen,
};
use crate::syntax::scss::is_at_scss_interpolation;
use biome_css_syntax::CssSyntaxKind::{
    EOF, SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST, SCSS_INTERPOLATION,
};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress};

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

    if first_fragment.kind(p) != SCSS_INTERPOLATION && !is_at_identifier_continuation(p) {
        return Present(first_fragment);
    }

    Present(finish_regular_identifier(p, first_fragment))
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
        let Some(interpolation) = parse_scss_regular_interpolation(p).ok() else {
            return Absent;
        };

        if is_at_identifier_continuation(p) {
            return Present(finish_regular_identifier(p, interpolation));
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

#[inline]
fn finish_regular_identifier(
    p: &mut CssParser,
    first_fragment: CompletedMarker,
) -> CompletedMarker {
    let list = first_fragment.precede(p);
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && is_at_identifier_continuation(p) {
        progress.assert_progressing(p);

        if is_at_identifier_hyphen(p) {
            // Safe: guarded by `is_at_identifier_hyphen`.
            parse_identifier_hyphen(p).ok();
        } else if parse_regular_part(p).is_absent() {
            break;
        }
    }

    let list = list.complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
    list.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER)
}

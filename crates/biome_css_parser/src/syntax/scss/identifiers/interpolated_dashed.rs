use crate::parser::CssParser;
use crate::syntax::scss::expression::parse_scss_regular_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_nth_at_identifier_hyphen_part, parse_identifier_hyphen_part,
    parse_scss_interpolated_identifier_parts,
};
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolated_identifier, is_nth_at_scss_interpolation,
};
use crate::syntax::{CssSyntaxFeatures, is_at_dashed_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATED_DASHED_IDENTIFIER;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::SyntaxFeature;

#[inline]
pub(crate) fn is_at_scss_interpolated_dashed_identifier(p: &mut CssParser) -> bool {
    is_at_dashed_identifier_with_interpolation_suffix(p)
        || is_nth_at_scss_interpolated_dashed_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_dashed_identifier(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && is_nth_at_identifier_hyphen_part(p, n)
        && is_nth_at_identifier_hyphen_part(p, n + 1)
        && !p.has_nth_preceding_whitespace(n + 1)
        && is_nth_at_scss_interpolated_identifier(p, n + 2)
        && !p.has_nth_preceding_whitespace(n + 2)
}

/// Parses an interpolated custom-property name.
///
/// Examples:
/// ```scss
/// --#{$prop}: 10px;
/// --theme-#{$slot}: red;
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_interpolated_dashed_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_dashed_identifier(p) {
        return Absent;
    }

    // `--theme-#{$slot}` starts as one dashed identifier token, while
    // `--#{$prop}` starts as two `-` tokens followed by interpolation.
    let first_part = if is_at_dashed_identifier(p) {
        parse_regular_identifier(p)
    } else {
        parse_identifier_hyphen_part(p)
    };

    let Present(first_part) = first_part else {
        return Absent;
    };

    let parts =
        parse_scss_interpolated_identifier_parts(p, first_part, parse_dashed_identifier_part);

    Present(
        parts
            .precede(p)
            .complete(p, SCSS_INTERPOLATED_DASHED_IDENTIFIER),
    )
}

#[inline]
fn is_at_dashed_identifier_with_interpolation_suffix(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && is_at_dashed_identifier(p)
        && is_nth_at_scss_interpolation(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

#[inline]
fn parse_dashed_identifier_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else {
        parse_regular_identifier(p)
    }
}

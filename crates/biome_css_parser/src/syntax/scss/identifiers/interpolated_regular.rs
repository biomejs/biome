use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::expression::{ScssInterpolationMode, parse_scss_interpolation_with_mode};
use crate::syntax::scss::identifiers::interpolated_identifier::parse_scss_interpolated_identifier_with;
use crate::syntax::scss::is_at_scss_interpolation;
use biome_parser::prelude::ParsedSyntax;

/// Parses an identifier that may be formed by adjacent identifier and
/// interpolation fragments with no intervening trivia.
///
/// Examples:
/// ```scss
/// margin-#{$side}
/// #{$name}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_interpolated_identifier_with(p, parse_regular_interpolated_identifier_any_fragment)
}

#[inline]
fn parse_regular_interpolated_identifier_any_fragment(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else {
        parse_regular_identifier(p)
    }
}

/// Parses a standalone SCSS interpolation expression such as `#{$value}`.
///
/// Example:
/// ```scss
/// $value: #{$name};
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_regular_interpolation(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_interpolation_with_mode(p, ScssInterpolationMode::Regular)
}

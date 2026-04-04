use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::expression::parse_scss_regular_interpolation as parse_regular_interpolation_expression;
use crate::syntax::scss::identifiers::interpolated_identifier::parse_scss_interpolated_identifier_with;
use crate::syntax::scss::is_at_scss_interpolation;
use biome_parser::prelude::ParsedSyntax;

/// Parses identifier-shaped SCSS syntax that may contain interpolation parts.
///
/// This is different from [`parse_scss_regular_interpolation`], which parses
/// exactly one standalone interpolation value such as `#{$name}`. This helper
/// parses identifier grammar, so it can consume adjacent identifier and
/// interpolation fragments with no intervening trivia and combine them into one
/// identifier-shaped node.
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

/// Parses one standalone SCSS interpolation value such as `#{$value}`.
///
/// This is different from [`parse_scss_interpolated_identifier`], which parses
/// identifier grammar that may include interpolation parts. This helper does
/// not try to merge adjacent identifier fragments; it only parses the single
/// interpolation expression at the current position.
///
/// Example:
/// ```scss
/// $value: #{$name};
/// width: #{$value};
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_regular_interpolation(p: &mut CssParser) -> ParsedSyntax {
    parse_regular_interpolation_expression(p)
}

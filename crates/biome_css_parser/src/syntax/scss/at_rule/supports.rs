use super::super::{is_at_scss_interpolation, parse_scss_regular_interpolation};
use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::SCSS_SUPPORTS_INTERPOLATED_CONDITION;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Returns true when Sass interpolation can stand in for a supports condition operand.
#[inline]
pub(crate) fn is_at_scss_supports_interpolated_condition(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
}

/// Parses Sass interpolation standing in for a complete `@supports` condition operand.
///
/// Examples:
/// ```scss
/// @supports #{"(display: grid)"} {}
/// @supports #{$condition} and (gap: 1rem) {}
/// @supports not #{$condition} {}
/// ```
#[inline]
pub(crate) fn parse_scss_supports_interpolated_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_supports_interpolated_condition(p) {
        return Absent;
    }

    let m = p.start();
    parse_scss_regular_interpolation(p).ok();

    Present(m.complete(p, SCSS_SUPPORTS_INTERPOLATED_CONDITION))
}

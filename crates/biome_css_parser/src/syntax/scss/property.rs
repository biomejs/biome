use crate::parser::CssParser;
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier};
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

use super::{is_nth_at_scss_interpolation, parse_scss_interpolated_identifier};

/// Detects an interpolation-bearing SCSS property name.
///
/// Unlike [`super::is_at_scss_nesting_declaration`], this also covers
/// interpolation-bearing custom property names such as `--theme-#{$slot}:`
/// that cannot start SCSS nested-property syntax.
///
/// More generally, this covers names that either start with interpolation or
/// continue into interpolation:
///
/// ```scss
/// #{$name}: 1px;
/// margin-#{$side}: 1px;
/// ```
#[inline]
pub(crate) fn is_at_scss_interpolated_property(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_property(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_property(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        //`#{$name}: 1px;`
        && (is_nth_at_scss_interpolation(p, n)
            //`margin-#{$side}: 1px;`
            || (is_nth_at_identifier(p, n) && is_nth_at_scss_interpolation(p, n + 1)))
}

#[inline]
pub(crate) fn parse_scss_interpolated_property_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_property(p) {
        return Absent;
    }

    parse_scss_interpolated_identifier(p)
}

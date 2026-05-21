use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::is_at_identifier;
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolation, parse_scss_interpolated_identifier,
};
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::SyntaxFeature;

#[inline]
pub(crate) fn is_at_scss_interpolated_attribute_identifier(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_at_scss_interpolation(p)
            || is_at_identifier(p)
                && is_nth_at_scss_interpolation(p, 1)
                && !p.has_nth_preceding_whitespace(1))
}

/// Parses an interpolated attribute selector modifier.
///
/// Example:
/// ```scss
/// [title="x" #{$mod}]
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_interpolated_attribute_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_attribute_identifier(p) {
        return Absent;
    }

    parse_scss_interpolated_identifier(p)
}

use crate::parser::CssParser;
use crate::syntax::parse_error::scss_only_syntax_error;
use crate::syntax::scss::{
    is_at_scss_interpolated_dashed_identifier, parse_scss_interpolated_dashed_identifier,
};
use crate::syntax::{CssSyntaxFeatures, parse_dashed_identifier};
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax;

/// Requires a literal `--` prefix, allowing interpolation in SCSS names.
///
/// ```scss
/// @property --#{$name} { syntax: "*"; inherits: false; }
/// @property --theme-#{$name} { syntax: "*"; inherits: false; }
/// ```
#[inline]
pub(super) fn parse_dashed_at_rule_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolated_dashed_identifier(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_interpolated_dashed_identifier,
            |p, marker| {
                scss_only_syntax_error(p, "SCSS interpolated dashed identifiers", marker.range(p))
            },
        )
    } else {
        parse_dashed_identifier(p)
    }
}

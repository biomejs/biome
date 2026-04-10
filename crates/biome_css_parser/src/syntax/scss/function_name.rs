use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use biome_css_syntax::T;
use biome_parser::CompletedMarker;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

use super::{
    is_at_scss_interpolated_identifier, is_at_scss_qualified_name,
    parse_scss_interpolated_identifier, parse_scss_qualified_name,
};

#[inline]
pub(crate) fn is_at_scss_function_name(p: &mut CssParser) -> bool {
    is_at_scss_qualified_name(p) || is_at_scss_interpolated_identifier(p)
}

/// Parses an SCSS function name, including module-qualified names and
/// interpolation-shaped plain CSS function names.
///
/// Examples:
/// ```scss
/// color.adjust($c, $lightness: 10%);
/// foo#{1 + 1}(arg);
/// ```
///
/// Docs:
/// - https://sass-lang.com/documentation/modules
/// - https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_function_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_function_name(p) {
        return Absent;
    }

    if is_at_scss_qualified_name(p) {
        let has_dollar_member = p.nth_at(2, T![$]);
        parse_scss_qualified_name(p).map(|marker| {
            add_scss_variable_member_function_name_diagnostic(p, has_dollar_member, marker)
        })
    } else if is_at_scss_interpolated_identifier(p) {
        parse_scss_interpolated_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
}

#[inline]
pub(crate) fn add_scss_variable_member_function_name_diagnostic(
    p: &mut CssParser,
    has_dollar_member: bool,
    marker: CompletedMarker,
) -> CompletedMarker {
    if has_dollar_member && p.at(T!['(']) {
        p.error(
            p.err_builder(
                "Expected an identifier but instead found a variable member.",
                p.cur_range(),
            )
            .with_hint("Function names after `module.` cannot start with `$`."),
        );
    }

    marker
}

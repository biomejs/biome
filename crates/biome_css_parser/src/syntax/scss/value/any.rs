use crate::parser::CssParser;
use crate::syntax::scss::{
    add_scss_variable_member_function_name_diagnostic, is_at_scss_function, is_at_scss_identifier,
    is_at_scss_interpolated_function_or_value, is_at_scss_interpolated_string,
    is_at_scss_parent_selector_value, is_at_scss_qualified_name, parse_scss_function,
    parse_scss_identifier, parse_scss_interpolated_function_or_value,
    parse_scss_interpolated_string, parse_scss_parent_selector_value, parse_scss_qualified_name,
};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

#[inline]
pub(crate) fn is_at_any_scss_value(p: &mut CssParser) -> bool {
    is_at_scss_function(p)
        || is_at_scss_identifier(p)
        || is_at_scss_qualified_name(p)
        || is_at_scss_interpolated_function_or_value(p)
        || is_at_scss_parent_selector_value(p)
        || is_at_scss_interpolated_string(p)
}

/// Parses one SCSS-only value form.
///
/// This covers the SCSS-specific value families that do not belong to the
/// shared CSS value parser, including variables, qualified names, parent
/// selectors, interpolated strings, and interpolation-led function-or-value
/// forms.
///
/// Examples:
/// ```scss
/// $value
/// module.$value
/// &-suffix
/// "#{$name}"
/// foo#{1 + 1}(arg)
/// ```
///
/// Docs:
/// - https://sass-lang.com/documentation/syntax/structure
/// - https://sass-lang.com/documentation/modules
/// - https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_any_scss_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_scss_value(p) {
        return Absent;
    }

    if is_at_scss_function(p) {
        parse_scss_function(p)
    } else if is_at_scss_identifier(p) {
        parse_scss_identifier(p)
    } else if is_at_scss_qualified_name(p) {
        let has_dollar_member = p.nth_at(2, T![$]);
        parse_scss_qualified_name(p).map(|marker| {
            add_scss_variable_member_function_name_diagnostic(p, has_dollar_member, marker)
        })
    } else if is_at_scss_interpolated_function_or_value(p) {
        parse_scss_interpolated_function_or_value(p)
    } else if is_at_scss_parent_selector_value(p) {
        parse_scss_parent_selector_value(p)
    } else {
        parse_scss_interpolated_string(p)
    }
}

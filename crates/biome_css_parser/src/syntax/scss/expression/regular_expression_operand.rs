use super::map::parse_scss_parenthesized_or_map_expression;
use crate::parser::CssParser;
use crate::syntax::declaration::{is_at_declaration_important, parse_declaration_important};
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::scss::{is_at_any_scss_value, parse_any_scss_value};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;

/// Parses regular SCSS expression operands such as `($value)`, `$value`,
/// `!important`, or `foo($arg)`.
///
/// Examples:
/// ```scss
/// ($value)
/// $value
/// !important
/// foo($arg)
/// ```
///
/// Docs: https://sass-lang.com/documentation/syntax/structure
#[inline]
pub(super) fn parse_scss_regular_expression_operand(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        parse_scss_parenthesized_or_map_expression(p)
    } else if is_at_declaration_important(p) {
        parse_declaration_important(p)
    } else if is_at_any_scss_value(p) {
        parse_any_scss_value(p)
    } else {
        parse_generic_component_value(p)
    }
}

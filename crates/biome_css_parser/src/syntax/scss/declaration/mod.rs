mod nesting;
mod variable;
mod variable_modifier;

use crate::parser::CssParser;
use crate::syntax::declaration::parse_declaration_with_semicolon;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;

pub(crate) use nesting::{
    is_at_scss_nesting_declaration, parse_scss_nesting_declaration,
    try_parse_scss_nesting_declaration,
};
pub(crate) use variable::{is_at_scss_declaration, parse_scss_declaration};
pub(crate) use variable_modifier::is_at_scss_variable_modifier;

#[inline]
pub(crate) fn parse_scss_interpolated_property_declaration(p: &mut CssParser) -> ParsedSyntax {
    try_parse_scss_nesting_declaration(p, T!['}']).unwrap_or_else(|_| {
        // Declaration-only contexts have no selector fallback, so once the
        // nesting-specific probe fails we can commit to a regular declaration.
        parse_declaration_with_semicolon(p)
    })
}

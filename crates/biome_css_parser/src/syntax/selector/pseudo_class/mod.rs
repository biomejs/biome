mod function_compound_selector;
mod function_compound_selector_list;
mod function_custom_identifier_list;
mod function_identifier;
mod function_nth;
mod function_relative_selector_list;
mod function_selector;
mod function_selector_list;
mod function_value_list;
mod identifier;

use self::function_compound_selector_list::{
    is_at_pseudo_class_function_compound_selector_list,
    parse_pseudo_class_function_compound_selector_list,
};
use self::function_identifier::{
    is_at_pseudo_class_function_identifier, parse_pseudo_class_function_identifier,
};
use self::function_nth::{is_at_pseudo_class_function_nth, parse_pseudo_class_function_nth};
use self::function_relative_selector_list::{
    is_at_pseudo_class_function_relative_selector_list,
    parse_pseudo_class_function_relative_selector_list,
};
use self::function_selector::{
    is_at_pseudo_class_function_selector, parse_pseudo_class_function_selector,
};
use self::function_selector_list::{
    is_at_pseudo_class_function_selector_list, parse_pseudo_class_function_selector_list,
};
use self::function_value_list::{
    is_at_pseudo_class_function_value_list, parse_pseudo_class_function_value_list,
};
use self::identifier::parse_pseudo_class_identifier;
use crate::parser::CssParser;
use crate::syntax::is_at_identifier;
use crate::syntax::parse_error::expected_any_pseudo_class;
use crate::syntax::selector::pseudo_class::function_custom_identifier_list::{
    is_at_pseudo_class_function_custom_identifier_list,
    parse_pseudo_class_function_custom_identifier_list,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use function_compound_selector::{
    is_at_pseudo_class_function_compound_selector, parse_pseudo_class_function_compound_selector,
};

#[inline]
pub(crate) fn parse_pseudo_class_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);

    // Show the error under the token next to the ':'
    let range = p.cur_range();

    let kind = match parse_pseudo_class(p) {
        Present(_) => CSS_PSEUDO_CLASS_SELECTOR,
        Absent => {
            p.error(expected_any_pseudo_class(p, range));
            CSS_BOGUS_SUB_SELECTOR
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
fn parse_pseudo_class(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    if is_at_pseudo_class_function_identifier(p) {
        parse_pseudo_class_function_identifier(p)
    } else if is_at_pseudo_class_function_selector(p) {
        parse_pseudo_class_function_selector(p)
    } else if is_at_pseudo_class_function_selector_list(p) {
        parse_pseudo_class_function_selector_list(p)
    } else if is_at_pseudo_class_function_compound_selector(p) {
        parse_pseudo_class_function_compound_selector(p)
    } else if is_at_pseudo_class_function_compound_selector_list(p) {
        parse_pseudo_class_function_compound_selector_list(p)
    } else if is_at_pseudo_class_function_relative_selector_list(p) {
        parse_pseudo_class_function_relative_selector_list(p)
    } else if is_at_pseudo_class_function_value_list(p) {
        parse_pseudo_class_function_value_list(p)
    } else if is_at_pseudo_class_function_nth(p) {
        parse_pseudo_class_function_nth(p)
    } else if is_at_pseudo_class_function_custom_identifier_list(p) {
        parse_pseudo_class_function_custom_identifier_list(p)
    } else {
        parse_pseudo_class_identifier(p)
    }
}

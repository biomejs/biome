mod declaration;
mod expression;

use crate::parser::CssParser;
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{
    SCSS_IDENTIFIER, SCSS_PARENT_SELECTOR_VALUE, SCSS_QUALIFIED_NAME,
};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

pub(crate) use declaration::{
    is_at_scss_declaration, is_at_scss_nesting_declaration, parse_scss_declaration,
    parse_scss_nesting_declaration,
};
pub(crate) use expression::parse_scss_expression;

#[inline]
pub(crate) fn is_at_scss_identifier(p: &mut CssParser) -> bool {
    p.at(T![$]) && is_nth_at_identifier(p, 1)
}

#[inline]
pub(crate) fn parse_scss_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![$]);
    parse_regular_identifier(p).ok();
    Present(m.complete(p, SCSS_IDENTIFIER))
}

#[inline]
pub(crate) fn is_at_scss_qualified_name(p: &mut CssParser) -> bool {
    is_nth_at_scss_qualified_name(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_qualified_name(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
        && p.nth_at(n + 1, T![.])
        && ((p.nth_at(n + 2, T![$]) && is_nth_at_identifier(p, n + 3))
            || is_nth_at_identifier(p, n + 2))
}

#[inline]
pub(crate) fn parse_scss_qualified_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_qualified_name(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![.]);

    if is_at_scss_identifier(p) {
        parse_scss_identifier(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    Present(m.complete(p, SCSS_QUALIFIED_NAME))
}

#[inline]
pub(crate) fn parse_scss_function_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_qualified_name(p) {
        parse_scss_qualified_name(p)
    } else {
        parse_regular_identifier(p)
    }
}

#[inline]
pub(crate) fn is_at_scss_parent_selector_value(p: &mut CssParser) -> bool {
    // `&` is a generic token in CSS parsing/recovery. Keep the SCSS gate here so
    // plain CSS doesn't accidentally route through SCSS-only diagnostics.
    CssSyntaxFeatures::Scss.is_supported(p) && p.at(T![&])
}

#[inline]
pub(crate) fn parse_scss_parent_selector_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector_value(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![&]);
    Present(m.complete(p, SCSS_PARENT_SELECTOR_VALUE))
}

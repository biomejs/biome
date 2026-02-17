mod declaration;

use crate::parser::CssParser;
use crate::syntax::{is_at_identifier, is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{SCSS_IDENTIFIER, SCSS_QUALIFIED_NAME};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

pub(crate) use declaration::{is_at_scss_declaration, parse_scss_declaration};

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
    is_at_identifier(p)
        && p.nth_at(1, T![.])
        && ((p.nth_at(2, T![$]) && is_nth_at_identifier(p, 3)) || is_nth_at_identifier(p, 2))
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
pub(crate) fn is_at_scss_qualified_name_function(p: &mut CssParser) -> bool {
    is_at_scss_qualified_name(p) && p.nth_at(3, T!['('])
}

#[inline]
pub(crate) fn parse_scss_function_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_qualified_name(p) {
        parse_scss_qualified_name(p)
    } else {
        parse_regular_identifier(p)
    }
}

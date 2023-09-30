use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{is_at_identifier, parse_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

#[inline]
pub(crate) fn parse_class_selector_pattern(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![.]);

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_CLASS_SELECTOR_PATTERN))
}

#[inline]
pub(crate) fn parse_id_selector_pattern(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![#]);

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_ID_SELECTOR_PATTERN))
}

#[inline]
pub(crate) fn parse_universal_selector_pattern(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![*]);

    Present(m.complete(p, CSS_UNIVERSAL_SELECTOR_PATTERN))
}

#[inline]
pub(crate) fn parse_type_selector_pattern(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }
    let m = p.start();

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_TYPE_SELECTOR_PATTERN))
}

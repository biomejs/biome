use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, prelude::ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::value::parse_error::expected_expression;

#[inline]
pub(crate) fn is_at_attr_function(p: &mut CssParser) -> bool {
    p.at(T![attr]) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_attr_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_function(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![attr]);
    p.bump(T!['(']);

    AttrNameList.parse_list(p);
    parse_attr_type(p).ok();

    p.expect(T![')']);

    Present(m.complete(p, CSS_ATTR_FUNCTION))
}

#[inline]
fn is_at_attr_type(p: &mut CssParser) -> bool {
    p.cur_text() == "raw-string" || p.cur_text() == "number"
}

#[inline]
fn parse_attr_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_type(p) {
        return Absent;
    }

    let m = p.start();

    if p.cur_text() == "raw-string" {
        p.bump(T![raw_string]);
    } else if p.cur_text() == "number" {
        p.bump(T![number]);
    }

    Present(m.complete(p, ANY_CSS_ATTR_TYPE))
}

struct AttrNameListParseRecovery;

impl ParseRecovery for AttrNameListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    // TODO: custom bogus?
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // TODO:
        p.at(T![')']) || p.at(T![|])
    }
}

struct AttrNameList;

impl ParseSeparatedList for AttrNameList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_ATTR_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_regular_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_attr_type(p) || p.at(T![,]) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        // TODO: right expected fn
        parsed_element.or_recover(p, &AttrNameListParseRecovery, expected_expression)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

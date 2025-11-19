use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::TokenSet;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::token_set;
use biome_parser::{Parser, prelude::ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::is_at_string;
use crate::syntax::parse_string;
use crate::syntax::value::parse_error::expected_expression;

const SYNTAX_TYPE_NAME_SET: TokenSet<CssSyntaxKind> = token_set![
    T![angle],
    T![color],
    T![custom_ident],
    T![image],
    T![integer],
    T![length],
    T![length_percentage],
    T![percentage],
    T![resolution],
    T![string],
    T![number],
    T![time],
    T![url],
    T![transform_function],
];

#[inline]
pub(crate) fn is_at_type_function(p: &mut CssParser) -> bool {
    p.at(T![type]) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_type_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_type_function(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![type]);
    p.bump(T!['(']);
    parse_any_syntax(p).ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_TYPE_FUNCTION))
}

#[inline]
fn parse_any_syntax(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![*]) {
        let m = p.start();
        p.bump(T![*]);
        return Present(m.complete(p, CSS_WILDCARD));
    }

    if is_at_syntax_type(p) {
        // let m = p.start();
        let list = SyntaxComponentList.parse_list(p);
        dbg!(list);
        // return Present(m.complete(p, CSS_SYNTAX_COMPONENT_LIST));
    }

    if is_at_string(p) {
        return parse_string(p);
    }

    Absent
}

#[inline]
fn parse_syntax_component(p: &mut CssParser) -> ParsedSyntax {
    // todo: guard
    let m = p.start();

    parse_any_syntax_single_component(p).ok();
    // parse_syntax_multiplier(p).ok();

    Present(m.complete(p, CSS_SYNTAX_COMPONENT))
}

#[inline]
fn parse_any_syntax_single_component(p: &mut CssParser) -> ParsedSyntax {
    if is_at_syntax_type(p) {
        return parse_syntax_type(p);
    }

    // todo: ident

    Absent
}

#[inline]
fn parse_syntax_multiplier(p: &mut CssParser) -> ParsedSyntax {
    // todo: guard
    Absent
}

#[inline]
fn is_at_syntax_type(p: &mut CssParser) -> bool {
    p.at(T![<]) && p.nth_at_ts(1, SYNTAX_TYPE_NAME_SET)
}

#[inline]
fn parse_syntax_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_syntax_type(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![<]);
    p.bump_ts(SYNTAX_TYPE_NAME_SET);
    p.expect(T![>]);

    Present(m.complete(p, CSS_SYNTAX_TYPE))
}

struct SyntaxTypeListParseRecovery;

impl ParseRecovery for SyntaxTypeListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SYNTAX_SINGLE_COMPONENT;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at(T![;]) || p.has_preceding_line_break()
    }
}

struct SyntaxComponentList;

impl ParseSeparatedList for SyntaxComponentList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SYNTAX_COMPONENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_syntax_component(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        // TODO: right expected fn
        parsed_element.or_recover(p, &SyntaxTypeListParseRecovery, expected_expression)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

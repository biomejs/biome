use biome_css_syntax::CssSyntaxKind::{
    CSS_BOOLEAN_MEDIA_QUERY, CSS_CUSTOM_MEDIA_AT_RULE, CSS_CUSTOM_MEDIA_AT_RULE_DECLARATOR,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

use crate::parser::CssParser;
use crate::syntax::at_rule::media::MediaQueryList;
use crate::syntax::parse_dashed_identifier;
use crate::syntax::parse_error::expected_dashed_identifier;

#[inline]
fn is_at_custom_media_at_rule(p: &mut CssParser) -> bool {
    p.at(T![custom_media])
}

#[inline]
pub(crate) fn parse_custom_media_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_custom_media_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    parse_custom_media_at_rule_declarator(p, T![;]).ok();
    p.eat(T![;]);

    Present(m.complete(p, CSS_CUSTOM_MEDIA_AT_RULE))
}

#[inline]
fn parse_custom_media_at_rule_declarator(
    p: &mut CssParser,
    end_kind: CssSyntaxKind,
) -> ParsedSyntax {
    if !is_at_custom_media_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![custom_media]);

    parse_dashed_identifier(p).or_add_diagnostic(p, expected_dashed_identifier);

    if is_at_boolean_media_query(p) {
        parse_boolean_media_query(p).ok();
    } else {
        MediaQueryList::new(end_kind).parse_list(p);
    }

    Present(m.complete(p, CSS_CUSTOM_MEDIA_AT_RULE_DECLARATOR))
}

const BOOLEAN_MEDIA_QUERY_SET: TokenSet<CssSyntaxKind> = token_set![T![true], T![false]];

#[inline]
fn is_at_boolean_media_query(p: &mut CssParser) -> bool {
    p.at(T![true]) || p.at(T![false])
}

#[inline]
fn parse_boolean_media_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_boolean_media_query(p) {
        return Absent;
    }

    let m = p.start();

    p.eat_ts(BOOLEAN_MEDIA_QUERY_SET);

    Present(m.complete(p, CSS_BOOLEAN_MEDIA_QUERY))
}

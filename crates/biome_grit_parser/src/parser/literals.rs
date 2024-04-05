use super::parse_error::{expected_list_pattern, expected_map_element, expected_pattern};
use super::parse_name;
use super::patterns::{parse_maybe_curly_pattern, parse_pattern};
use super::GritParser;
use crate::constants::*;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::{ParsedSyntax::*, *};

pub(crate) fn parse_literal(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        TRUE_KW | FALSE_KW => parse_boolean_literal(p),
        GRIT_DOUBLE => parse_double_literal(p),
        GRIT_INT => parse_int_literal(p),
        GRIT_NEGATIVE_INT => parse_negative_int_literal(p),
        GRIT_STRING => parse_string_literal(p),
        UNDEFINED_KW => parse_undefined_literal(p),
        T!['{'] => parse_map(p),
        GRIT_NAME if p.lookahead() == T!['['] => parse_list(p),
        T!['['] => parse_list(p),
        kind if CODE_SNIPPET_SET.contains(kind) => parse_code_snippet(p),
        _ => Absent,
    }
}

#[inline]
fn parse_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
pub(crate) fn parse_boolean_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(BOOLEAN_VALUE_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(BOOLEAN_VALUE_SET);
    Present(m.complete(p, GRIT_BOOLEAN_LITERAL))
}

#[inline]
fn parse_code_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(CODE_SNIPPET_SET) {
        return Absent;
    }

    let m = p.start();

    let syntax = match p.cur() {
        GRIT_BACKTICK_SNIPPET => parse_backtick_snippet_literal(p),
        GRIT_RAW_BACKTICK_SNIPPET => parse_raw_backtick_snippet_literal(p),
        _ => parse_language_specific_snippet(p),
    };

    match syntax {
        Present(_) => Present(m.complete(p, GRIT_CODE_SNIPPET)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_dotdotdot(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(DOT3) {
        return Absent;
    }

    let m = p.start();
    p.bump(DOT3);
    parse_maybe_curly_pattern(p).ok();
    Present(m.complete(p, GRIT_DOTDOTDOT))
}

#[inline]
pub(crate) fn parse_double_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_DOUBLE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_DOUBLE);
    Present(m.complete(p, GRIT_DOUBLE_LITERAL))
}

#[inline]
pub(crate) fn parse_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_INT);
    Present(m.complete(p, GRIT_INT_LITERAL))
}

#[inline]
fn parse_language_specific_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_SET) {
        return Absent;
    }

    let m = p.start();

    {
        let m = p.start();
        p.bump_ts(SUPPORTED_LANGUAGE_SET);
        m.complete(p, GRIT_LANGUAGE_NAME);
    }

    p.expect(GRIT_STRING);

    Present(m.complete(p, GRIT_LANGUAGE_SPECIFIC_SNIPPET))
}

#[inline]
pub(crate) fn parse_list(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['[']) && !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).ok();

    p.expect(T!['[']);

    ListPatternList.parse_list(p);

    p.expect(T![']']);
    Present(m.complete(p, GRIT_LIST))
}

struct ListPatternList;

impl ParseSeparatedList for ListPatternList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_LIST_PATTERN_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_list_pattern(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![']'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_LIST_RECOVERY_SET),
            expected_list_pattern,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn parse_list_pattern(p: &mut GritParser) -> ParsedSyntax {
    if p.at(DOT3) {
        parse_dotdotdot(p)
    } else {
        parse_pattern(p)
    }
}

#[inline]
pub(crate) fn parse_map(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    MapElementList.parse_list(p);

    p.eat(T!['}']);
    Present(m.complete(p, GRIT_MAP))
}

struct MapElementList;

impl ParseSeparatedList for MapElementList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_MAP_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_map_element(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_MAP_ELEMENT, ELEMENT_LIST_RECOVERY_SET),
            expected_map_element,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn parse_map_element(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).ok();
    p.expect(T![:]);
    parse_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, ELEMENT_LIST_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_MAP_ELEMENT))
}

#[inline]
pub(crate) fn parse_negative_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NEGATIVE_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_NEGATIVE_INT);
    Present(m.complete(p, GRIT_NEGATIVE_INT_LITERAL))
}

#[inline]
fn parse_raw_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_RAW_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_RAW_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_RAW_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
fn parse_string_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_STRING) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_STRING);
    Present(m.complete(p, GRIT_STRING_LITERAL))
}

#[inline]
fn parse_undefined_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(UNDEFINED_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(UNDEFINED_KW);
    Present(m.complete(p, GRIT_UNDEFINED_LITERAL))
}

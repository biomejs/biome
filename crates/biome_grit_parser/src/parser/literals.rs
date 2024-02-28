use super::constants::*;
use super::parse_error::expected_pattern;
use super::parse_name;
use super::patterns::{parse_maybe_curly_pattern, parse_pattern};
use super::GritParser;
use biome_grit_syntax::GritSyntaxKind::*;
use biome_grit_syntax::T;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
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
        GRIT_NAME | T!['['] => parse_list(p),
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
    if !p.at(DOLLAR_DOT3) {
        return Absent;
    }

    let m = p.start();
    p.bump(DOLLAR_DOT3);
    let _ = parse_maybe_curly_pattern(p);
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

    p.eat(GRIT_STRING);

    Present(m.complete(p, GRIT_LANGUAGE_SPECIFIC_SNIPPET))
}

#[inline]
pub(crate) fn parse_list(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['[']) && !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_name(p);

    if !p.eat(T!['[']) {
        m.abandon(p);
        return Absent;
    }

    let _ = parse_list_pattern_list(p);

    p.eat(T![']']);
    Present(m.complete(p, GRIT_LIST))
}

#[inline]
fn parse_list_pattern_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_list_pattern(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    while p.eat(T![,]) {
        if parse_list_pattern(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_LIST_PATTERN_LIST))
}

#[inline]
fn parse_list_pattern(p: &mut GritParser) -> ParsedSyntax {
    if p.at(DOLLAR_DOT3) {
        parse_dotdotdot(p)
    } else {
        parse_literal(p)
    }
}

#[inline]
pub(crate) fn parse_map(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    let _ = parse_map_element_list(p);

    p.eat(T!['}']);
    Present(m.complete(p, GRIT_MAP))
}

#[inline]
fn parse_map_element_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_map_element(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    while p.eat(T![,]) {
        if parse_map_element(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_MAP_ELEMENT_LIST))
}

#[inline]
fn parse_map_element(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_name(p);
    p.eat(T![:]);

    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, ELEMENT_LIST_RECOVERY_SET),
        expected_pattern,
    );

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

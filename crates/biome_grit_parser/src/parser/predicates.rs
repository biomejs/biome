use super::literals::{parse_boolean_literal, parse_literal};
use super::patterns::parse_pattern;
use super::GritParser;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::prelude::{ParsedSyntax::*, *};

const NOT_SET: TokenSet<GritSyntaxKind> = token_set![NOT_KW, T![!]];

pub(crate) fn parse_predicate(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        NOT_KW | T![!] => parse_predicate_not(p),
        MAYBE_KW => parse_predicate_maybe(p),
        AND_KW | T!['{'] => parse_predicate_and(p),
        OR_KW => parse_predicate_or(p),
        ANY_KW => parse_predicate_any(p),
        IF_KW => parse_predicate_if_else(p),
        // TODO: GritPredicateAssignment => {}
        // TODO: GritPredicateAccumulate => {}
        // TODO: GritPredicateRewrite => {}
        // TODO: GritPredicateGreater => {}
        // TODO: GritPredicateLess => {}
        // TODO: GritPredicateGreaterEqual => {}
        // TODO: GritPredicateLessEqual => {}
        // TODO: GritPredicateNotEqual => {}
        // TODO: GritPredicateEqual => {}
        // TODO: GritPredicateCall => {}
        T!['('] => parse_bracketed_predicate(p),
        T![true] | T![false] => parse_boolean_literal(p),
        // TODO: GritPredicateReturn => {}
        _ => parse_predicate_match(p),
    }
}

#[inline]
fn parse_bracketed_predicate(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let result = parse_predicate(p);

    p.eat(T![')']);

    match result {
        Present(_) => Present(m.complete(p, BRACKETED_GRIT_PREDICATE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_and(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(AND_KW) && !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.eat(AND_KW);
    p.eat(T!['{']);

    if p.eat(T!['}']) {
        return Present(m.complete(p, GRIT_PREDICATE_AND));
    }

    let result = parse_predicate_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_AND)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_any(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ANY_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ANY_KW);
    p.eat(T!['{']);

    if p.eat(T!['}']) {
        return Present(m.complete(p, GRIT_PREDICATE_ANY));
    }

    let result = parse_predicate_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_ANY)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_if_else(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(IF_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(IF_KW);
    p.eat(T!['(']);

    let _ = parse_predicate(p);

    p.eat(T![')']);

    let _ = parse_predicate(p);

    let _ = parse_predicate_else_clause(p);

    Present(m.complete(p, GRIT_PREDICATE_IF_ELSE))
}

#[inline]
fn parse_predicate_else_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ELSE_KW);

    match parse_predicate(p) {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_ELSE_CLAUSE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_predicate(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    while p.eat(T![,]) {
        if parse_predicate(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_PREDICATE_LIST))
}

#[inline]
fn parse_predicate_match(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_predicate_match_subject(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    if !p.eat(T![<:]) {
        m.abandon(p);
        return Absent;
    }

    let _ = parse_pattern(p);

    Present(m.complete(p, GRIT_PREDICATE_MATCH))
}

#[inline]
fn parse_predicate_match_subject(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        GRIT_VARIABLE => {
            let m = p.start();
            p.bump(GRIT_VARIABLE);
            Present(m.complete(p, GRIT_VARIABLE))
        }
        // TODO: GritMapAccessor => {}
        // TODO: GritListAccessor => {}
        _ => parse_literal(p),
    }
}

#[inline]
fn parse_predicate_maybe(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MAYBE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MAYBE_KW);
    match parse_predicate(p) {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_MAYBE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_not(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(NOT_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(NOT_SET);
    match parse_predicate(p) {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_NOT)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_predicate_or(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(OR_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(OR_KW);
    p.eat(T!['{']);

    let result = parse_predicate_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PREDICATE_OR)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

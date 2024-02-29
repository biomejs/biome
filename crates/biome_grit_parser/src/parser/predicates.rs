use super::literals::{parse_boolean_literal, parse_literal};
use super::parse_error::expected_pattern;
use super::patterns::{parse_container, parse_pattern};
use super::{constants::*, parse_name, parse_named_arg_list};
use super::{parse_not, GritParser};
use biome_grit_syntax::GritSyntaxKind::*;
use biome_grit_syntax::T;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::{ParsedSyntax::*, *};

pub(crate) fn parse_predicate(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        NOT_KW | T![!] => parse_predicate_not(p),
        MAYBE_KW => parse_predicate_maybe(p),
        AND_KW | T!['{'] => parse_predicate_and(p),
        OR_KW => parse_predicate_or(p),
        ANY_KW => parse_predicate_any(p),
        IF_KW => parse_predicate_if_else(p),
        GRIT_NAME if p.lookahead() == T!['('] => parse_predicate_call(p),
        T!['('] => parse_bracketed_predicate(p),
        T![true] | T![false] => parse_boolean_literal(p),
        RETURN_KW => parse_predicate_return(p),
        _ => parse_infix_predicate(p),
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

enum InfixPredicateKind {
    Accumulate,
    Assignment,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Match,
    NotEqual,
    Rewrite,
}

/// Parses any of the infix predicates without need for backtracking.
#[inline]
fn parse_infix_predicate(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    let Present(mut subject) = parse_container(p).or_else(|| parse_literal(p)) else {
        m.abandon(p);
        return Absent;
    };

    use InfixPredicateKind::*;
    let kind = match p.cur() {
        T![+=] => Accumulate,
        T![=] => Assignment,
        T![==] => Equal,
        T![>] => Greater,
        T![>=] => GreaterEqual,
        T![<] => Less,
        T![<=] => LessEqual,
        T![<:] => Match,
        T![!=] => NotEqual,
        T![=>] => Rewrite,
        _ => {
            m.abandon(p);
            return Absent;
        }
    };

    // Verify the subjects are allowed for the matched predicate kind:
    match kind {
        Accumulate | Equal | Greater | GreaterEqual | Less | LessEqual | NotEqual | Rewrite
            if subject.kind(p) != GRIT_VARIABLE =>
        {
            subject.change_to_bogus(p);
        }
        Assignment if !CONTAINER_SET.contains(subject.kind(p)) => {
            subject.change_to_bogus(p);
        }
        _ => {}
    }

    p.bump_any();

    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, PREDICATE_RECOVERY_SET),
        expected_pattern,
    );

    let kind = match kind {
        Accumulate => GRIT_PREDICATE_ACCUMULATE,
        Assignment => GRIT_PREDICATE_ASSIGNMENT,
        Equal => GRIT_PREDICATE_EQUAL,
        Greater => GRIT_PREDICATE_GREATER,
        GreaterEqual => GRIT_PREDICATE_GREATER_EQUAL,
        Less => GRIT_PREDICATE_LESS,
        LessEqual => GRIT_PREDICATE_LESS_EQUAL,
        Match => GRIT_PREDICATE_MATCH,
        NotEqual => GRIT_PREDICATE_NOT_EQUAL,
        Rewrite => GRIT_PREDICATE_REWRITE,
    };
    Present(m.complete(p, kind))
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

    let _ = parse_predicate_list(p);

    p.eat(T!['}']);

    Present(m.complete(p, GRIT_PREDICATE_AND))
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
fn parse_predicate_call(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();

    let _ = parse_name(p);
    p.eat(T!['(']);
    let _ = parse_named_arg_list(p);
    p.eat(T![')']);

    Present(m.complete(p, GRIT_PREDICATE_CALL))
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
pub(crate) fn parse_predicate_list(p: &mut GritParser) -> ParsedSyntax {
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
    let m = match parse_not(p) {
        Present(syntax) => syntax.precede(p),
        Absent => return Absent,
    };

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

#[inline]
fn parse_predicate_return(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(RETURN_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(RETURN_KW);

    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, PREDICATE_RECOVERY_SET),
        expected_pattern,
    );

    Present(m.complete(p, GRIT_PREDICATE_RETURN))
}

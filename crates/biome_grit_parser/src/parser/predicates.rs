use super::literals::{parse_boolean_literal, parse_literal};
use super::parse_name;
use super::patterns::{parse_container, parse_pattern};
use super::{parse_error::*, parse_maybe_named_arg};
use super::{parse_not, GritParser};
use crate::constants::*;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::expected_node;
use biome_parser::parse_lists::ParseSeparatedList;
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

    parse_expected_predicate(p);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_BRACKETED_PREDICATE))
}

#[inline]
pub(crate) fn parse_expected_predicate(p: &mut GritParser) {
    parse_predicate(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PREDICATE, PREDICATE_RECOVERY_SET),
            expected_predicate,
        )
        .ok();
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
    Bogus,
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
        T![,] | T!['}'] => {
            p.error(expected_node("operator", p.cur_range(), p));
            return Present(m.complete(p, GRIT_BOGUS_PREDICATE));
        }
        _ => {
            p.error(expected_predicate_infix_operator(p, p.cur_range()));
            Bogus
        }
    };

    // Verify the subjects are allowed for the matched predicate kind:
    match kind {
        Accumulate | Equal | Greater | GreaterEqual | Less | LessEqual | NotEqual | Rewrite
            if subject.kind(p) != GRIT_VARIABLE =>
        {
            p.error(
                ParseDiagnostic::new("Expected a variable.", subject.range(p))
                    .with_detail(p.cur_range(), "This operator only works on variables."),
            );
            subject.change_to_bogus(p);
        }
        Assignment if !CONTAINER_SET.contains(subject.kind(p)) => {
            p.error(
                ParseDiagnostic::new("Expected a variable or container.", subject.range(p))
                    .with_detail(
                        p.cur_range(),
                        "Assignment only works on variables and containers.",
                    ),
            );
            subject.change_to_bogus(p);
        }
        _ => {}
    }

    p.bump_any();

    parse_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PREDICATE_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

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
        Bogus => GRIT_BOGUS_PREDICATE,
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
    p.expect(T!['{']);

    PredicateList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PREDICATE_AND))
}

#[inline]
fn parse_predicate_any(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ANY_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ANY_KW);
    p.expect(T!['{']);

    PredicateList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PREDICATE_ANY))
}

pub(crate) struct PredicateCallArgList;

impl ParseSeparatedList for PredicateCallArgList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_NAMED_ARG_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_maybe_named_arg(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_NAMED_ARG, ARG_LIST_RECOVERY_SET),
            expected_predicate_call_arg,
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
fn parse_predicate_call(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();

    parse_name(p).ok();
    p.expect(T!['(']);

    PredicateCallArgList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_PREDICATE_CALL))
}

#[inline]
fn parse_predicate_if_else(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(IF_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(IF_KW);
    p.expect(T!['(']);

    parse_expected_predicate(p);

    p.expect(T![')']);

    parse_expected_predicate(p);

    parse_predicate_else_clause(p).ok();

    Present(m.complete(p, GRIT_PREDICATE_IF_ELSE))
}

#[inline]
fn parse_predicate_else_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ELSE_KW);

    parse_expected_predicate(p);

    Present(m.complete(p, GRIT_PREDICATE_ELSE_CLAUSE))
}

pub(crate) struct PredicateList;

impl ParseSeparatedList for PredicateList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_PREDICATE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_predicate(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set!(T![')'], T!['}']))
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        let current_token = p.cur();
        let current_range = p.cur_range();
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PREDICATE, PREDICATE_RECOVERY_SET),
            |p, range| {
                if current_token == GRIT_NAME {
                    ParseDiagnostic::new("Expected a predicate here.", range)
                        .with_detail(current_range, "Should this be a variable?")
                } else {
                    expected_node("predicate", range, p)
                }
            },
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
fn parse_predicate_maybe(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MAYBE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MAYBE_KW);

    parse_expected_predicate(p);

    Present(m.complete(p, GRIT_PREDICATE_MAYBE))
}

#[inline]
fn parse_predicate_not(p: &mut GritParser) -> ParsedSyntax {
    let m = match parse_not(p) {
        Present(syntax) => syntax.precede(p),
        Absent => return Absent,
    };

    parse_expected_predicate(p);

    Present(m.complete(p, GRIT_PREDICATE_NOT))
}

#[inline]
fn parse_predicate_or(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(OR_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(OR_KW);
    p.expect(T!['{']);

    PredicateList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PREDICATE_OR))
}

#[inline]
fn parse_predicate_return(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(RETURN_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(RETURN_KW);

    parse_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PREDICATE_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_PREDICATE_RETURN))
}

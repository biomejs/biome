use super::constants::*;
use super::literals::*;
use super::parse_error::{expected_list_index, expected_map_key, expected_pattern};
use super::predicates::parse_predicate;
use super::{parse_name, parse_variable, parse_variable_list, GritParser};
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet};
use biome_parser::prelude::{ParsedSyntax::*, *};

pub(crate) fn parse_pattern(p: &mut GritParser) -> ParsedSyntax {
    let left = match p.cur() {
        NOT_KW | T![!] => parse_pattern_not(p),
        OR_KW => parse_pattern_or(p),
        ORELSE_KW => parse_pattern_orelse(p),
        ANY_KW => parse_pattern_any(p),
        AND_KW => parse_pattern_and(p),
        MAYBE_KW => parse_pattern_maybe(p),
        IF_KW => parse_pattern_if_else(p),
        CONTAINS_KW => parse_pattern_contains(p),
        INCLUDES_KW => parse_pattern_includes(p),
        AFTER_KW => parse_pattern_after(p),
        BEFORE_KW => parse_pattern_before(p),
        WITHIN_KW => parse_pattern_within(p),
        BUBBLE_KW => parse_bubble(p),
        GRIT_NAME => parse_node_like(p),
        T![.] => parse_dot(p),
        SOME_KW => parse_some(p),
        EVERY_KW => parse_every(p),
        GRIT_UNDERSCORE => parse_grit_undescore(p),
        GRIT_VARIABLE => parse_variable(p),
        GRIT_REGEX | GRIT_SNIPPET_REGEX => parse_regex_pattern(p),
        LIKE_KW => parse_like(p),
        // TODO: GritMulOperation => {}
        // TODO: GritDivOperation => {}
        // TODO: GritModOperation => {}
        // TODO: GritAddOperation => {}
        // TODO: GritSubOperation => {}
        SEQUENTIAL_KW => parse_sequential(p),
        MULTIFILE_KW => parse_files(p),
        T!['('] => parse_bracketed_pattern(p),
        _ => parse_literal(p),
    };

    let Present(left) = left else {
        return Absent;
    };

    let left = match p.cur() {
        AS_KW => parse_pattern_as(p, left),
        _ => Present(left),
    };

    let Present(left) = left else {
        return Absent;
    };

    let left = match p.cur() {
        T![=>] => parse_rewrite(p, left),
        T![.] => parse_map_accessor(p, left),
        T!['['] => parse_list_accessor(p, left),
        LIMIT_KW => parse_pattern_limit(p, left),
        WHERE_KW => parse_pattern_where(p, left),
        _ => Present(left),
    };

    let Present(left) = left else {
        return Absent;
    };

    match p.cur() {
        T![=] if CONTAINER_SET.contains(left.kind(p)) => parse_assignment_as_pattern(p, left),
        T![+=] => parse_pattern_accumulate(p, left),
        _ => Present(left),
    }
}

#[inline]
fn parse_assignment_as_pattern(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![=]);

    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
        expected_pattern,
    );

    Present(m.complete(p, GRIT_ASSIGNMENT_AS_PATTERN))
}

#[inline]
fn parse_bracketed_pattern(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let result = parse_pattern(p);

    p.eat(T![')']);

    match result {
        Present(_) => Present(m.complete(p, BRACKETED_GRIT_PATTERN)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_bubble(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(BUBBLE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(BUBBLE_KW);

    let _ = parse_bubble_scope(p);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_BUBBLE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_bubble_scope(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    if p.eat(T![')']) {
        return Present(m.complete(p, GRIT_BUBBLE_SCOPE));
    }

    let _ = parse_variable_list(p);

    p.eat(T![')']);

    Present(m.complete(p, GRIT_BUBBLE))
}

#[inline]
pub(crate) fn parse_container(p: &mut GritParser) -> ParsedSyntax {
    let left = match p.cur() {
        GRIT_VARIABLE => parse_variable(p),
        T!['{'] => parse_map(p),
        GRIT_NAME | T!['['] => parse_list(p),
        _ => Absent,
    };

    let Present(left) = left else {
        return Absent;
    };

    match p.cur() {
        T![.] => parse_map_accessor(p, left),
        T!['['] => parse_list_accessor(p, left),
        _ if left.kind(p) == GRIT_VARIABLE => Present(left),
        _ => Absent,
    }
}

#[inline]
fn parse_curly_pattern(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    let result = parse_pattern(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, CURLY_GRIT_PATTERN)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_dot(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![.]);
    Present(m.complete(p, GRIT_DOT))
}

#[inline]
fn parse_every(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(EVERY_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(EVERY_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_EVERY)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_files(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MULTIFILE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MULTIFILE_KW);

    p.eat(T!['{']);

    let result = parse_pattern_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_FILES)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_grit_undescore(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_UNDERSCORE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_UNDERSCORE);
    Present(m.complete(p, GRIT_UNDERSCORE))
}

#[inline]
fn parse_like(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(LIKE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(LIKE_KW);

    let _ = parse_like_threshold(p);

    p.eat(T!['{']);

    let result = parse_pattern(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_LIKE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_like_threshold(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let result = parse_pattern(p);

    p.eat(T![')']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_LIKE_THRESHOLD)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
pub(crate) fn parse_list_accessor(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T!['[']);

    let _ = parse_list_index(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, ACCESSOR_RECOVERY_SET),
        expected_list_index,
    );

    p.eat(T![']']);

    // FIXME: We should check whether the accessor needs to be extended.

    Present(m.complete(p, GRIT_LIST_ACCESSOR))
}

#[inline]
fn parse_list_index(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    let _ = match p.cur() {
        GRIT_INT => parse_int_literal(p),
        GRIT_NEGATIVE_INT => parse_negative_int_literal(p),
        _ => match parse_container(p) {
            Present(syntax) => Present(syntax),
            Absent => {
                m.abandon(p);
                return Absent;
            }
        },
    };

    Present(m.complete(p, GRIT_LIST_INDEX))
}

#[inline]
pub(crate) fn parse_map_accessor(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![.]);

    let _ = parse_map_key(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, ACCESSOR_RECOVERY_SET),
        expected_map_key,
    );

    // FIXME: We should check whether the accessor needs to be extended.

    Present(m.complete(p, GRIT_MAP_ACCESSOR))
}

#[inline]
fn parse_map_key(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        GRIT_NAME => parse_name(p),
        GRIT_VARIABLE => parse_variable(p),
        _ => Absent,
    }
}

#[inline]
pub(crate) fn parse_maybe_curly_pattern(p: &mut GritParser) -> ParsedSyntax {
    if p.at(T!['{']) {
        parse_curly_pattern(p)
    } else {
        parse_pattern(p)
    }
}

struct ArgListParseRecovery;
impl ParseRecovery for ArgListParseRecovery {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRIT_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }
}

#[inline]
fn parse_maybe_named_arg(p: &mut GritParser) -> ParsedSyntax {
    if p.at(GRIT_NAME) {
        parse_named_arg(p)
    } else {
        parse_pattern(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(GRIT_BOGUS, ARG_LIST_RECOVERY_SET),
                expected_pattern,
            )
            .ok()
            .into()
    }
}

#[inline]
fn parse_named_arg(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_name(p);
    p.eat(T![=]);
    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, ARG_LIST_RECOVERY_SET),
        expected_pattern,
    );
    Present(m.complete(p, GRIT_NAMED_ARG))
}

#[inline]
fn parse_named_arg_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_maybe_named_arg(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    while p.eat(T![,]) {
        if p.at(T![')']) || parse_maybe_named_arg(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_NAMED_ARG_LIST))
}

#[inline]
fn parse_node_like(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_name(p);
    p.eat(T!['(']);

    if parse_named_arg_list(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    p.eat(T![')']);

    Present(m.complete(p, GRIT_NODE_LIKE))
}

#[inline]
fn parse_pattern_accumulate(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![+=]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![+=]);

    let _ = parse_pattern(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
        expected_pattern,
    );

    Present(m.complete(p, GRIT_PATTERN_ACCUMULATE))
}

#[inline]
fn parse_pattern_after(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(AFTER_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(AFTER_KW);
    match parse_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_AFTER)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_and(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(AND_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(AND_KW);
    p.eat(T!['{']);

    if p.eat(T!['}']) {
        return Present(m.complete(p, GRIT_PATTERN_AND));
    }

    let result = parse_pattern_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_AND)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_any(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ANY_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ANY_KW);
    p.eat(T!['{']);

    if p.eat(T!['}']) {
        return Present(m.complete(p, GRIT_PATTERN_ANY));
    }

    let result = parse_pattern_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_ANY)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_arg_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();
    match parse_variable_list(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_ARG_LIST)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_as(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(AS_KW) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(AS_KW);

    match parse_variable(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_AS)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_before(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(BEFORE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(BEFORE_KW);
    match parse_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_BEFORE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_contains(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(CONTAINS_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(CONTAINS_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => {
            let _ = parse_pattern_contains_until_clause(p);

            Present(m.complete(p, GRIT_PATTERN_CONTAINS))
        }
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_contains_until_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(UNTIL_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(UNTIL_KW);

    match parse_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_if_else(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(IF_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(IF_KW);
    p.eat(T!['(']);

    let _ = parse_predicate(p);

    p.eat(T![')']);

    let _ = parse_maybe_curly_pattern(p);

    let _ = parse_pattern_else_clause(p);

    Present(m.complete(p, GRIT_PATTERN_IF_ELSE))
}

#[inline]
fn parse_pattern_else_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ELSE_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_ELSE_CLAUSE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_includes(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(INCLUDES_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(INCLUDES_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_INCLUDES)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_limit(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(LIMIT_KW) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(LIMIT_KW);

    match parse_int_literal(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_LIMIT)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_pattern(p) == Absent {
        m.abandon(p);
        return Absent;
    }

    while p.eat(T![,]) {
        if parse_pattern(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_PATTERN_LIST))
}

#[inline]
fn parse_pattern_maybe(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MAYBE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MAYBE_KW);
    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_MAYBE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_not(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(NOT_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(NOT_SET);
    match parse_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_NOT)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_or(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(OR_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(OR_KW);
    p.eat(T!['{']);

    let result = parse_pattern_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_OR)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_orelse(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ORELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ORELSE_KW);
    p.eat(T!['{']);

    if p.eat(T!['}']) {
        return Present(m.complete(p, GRIT_PATTERN_OR_ELSE));
    }

    let _ = parse_pattern_list(p);

    p.eat(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_OR_ELSE))
}

#[inline]
fn parse_pattern_where(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(WHERE_KW) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(WHERE_KW);

    match parse_predicate(p) {
        Present(_) => Present(m.complete(p, GRIT_PATTERN_WHERE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_pattern_within(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(WITHIN_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(WITHIN_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_WITHIN)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_regex_pattern(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(REGEX_SET) {
        return Absent;
    }

    let m = p.start();
    if p.cur() == GRIT_REGEX {
        let m = p.start();
        p.bump(GRIT_REGEX);
        m.complete(p, GRIT_REGEX_LITERAL);
    } else {
        let m = p.start();
        p.bump(GRIT_SNIPPET_REGEX);
        m.complete(p, GRIT_SNIPPET_REGEX_LITERAL);
    }

    let _ = parse_regex_pattern_variables(p);

    Present(m.complete(p, GRIT_REGEX_PATTERN))
}

#[inline]
fn parse_regex_pattern_variables(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let _ = parse_pattern_arg_list(p);

    p.eat(T![')']);

    Present(m.complete(p, GRIT_REGEX_PATTERN_VARIABLES))
}

#[inline]
fn parse_rewrite(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![=>]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![=>]);

    match parse_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_REWRITE)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_sequential(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(SEQUENTIAL_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(SEQUENTIAL_KW);

    p.eat(T!['{']);

    let result = parse_pattern_list(p);

    p.eat(T!['}']);

    match result {
        Present(_) => Present(m.complete(p, GRIT_SEQUENTIAL)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_some(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(SOME_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(SOME_KW);

    match parse_maybe_curly_pattern(p) {
        Present(_) => Present(m.complete(p, GRIT_SOME)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

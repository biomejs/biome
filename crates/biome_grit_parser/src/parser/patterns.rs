use super::literals::*;
use super::parse_error::*;
use super::predicates::parse_expected_predicate;
use super::VariableList;
use super::{parse_maybe_named_arg, parse_variable_list};
use super::{parse_name, parse_not, parse_variable, GritParser};
use crate::constants::*;
use biome_grit_syntax::GritSyntaxKind;
use biome_grit_syntax::GritSyntaxKind::*;
use biome_grit_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::{ParsedSyntax::*, *};

pub(crate) fn parse_pattern(p: &mut GritParser) -> ParsedSyntax {
    parse_pattern_with_precedence(p, PRECEDENCE_PATTERN)
}

#[inline]
fn parse_pattern_with_precedence(p: &mut GritParser, min_precedence: isize) -> ParsedSyntax {
    let Present(mut left) = parse_pattern_non_greedy(p) else {
        return Absent;
    };

    loop {
        macro_rules! parse_and_continue {
            ($parse:expr) => {
                match $parse {
                    Present(syntax) => {
                        left = syntax;
                        continue;
                    }
                    // This should never happen, but if it does we cannot uphold
                    // the parser's invariants while also satisfying the borrow
                    // checker. Safest to just panic here.
                    Absent => panic!("failed to parse subpattern"),
                }
            };
        }

        if PRECEDENCE_PATTERN_AS > min_precedence && p.cur() == AS_KW {
            parse_and_continue!(parse_pattern_as(p, left));
        }

        if let Some(math_operator) = MathOperator::from_token(p.cur()) {
            if math_operator.precedence() > min_precedence {
                parse_and_continue!(parse_math_pattern(p, left, math_operator));
            }
        }

        if PRECEDENCE_REWRITE > min_precedence && p.cur() == T![=>] {
            parse_and_continue!(parse_rewrite(p, left));
        }

        if PRECEDENCE_ACCUMULATE > min_precedence && p.cur() == T![+=] {
            parse_and_continue!(parse_pattern_accumulate(p, left));
        }

        if PRECEDENCE_PATTERN_LIMIT > min_precedence && p.cur() == LIMIT_KW {
            parse_and_continue!(parse_pattern_limit(p, left));
        }

        if PRECEDENCE_PATTERN_WHERE > min_precedence && p.cur() == WHERE_KW {
            parse_and_continue!(parse_pattern_where(p, left));
        }

        if PRECEDENCE_NOT > min_precedence
            && p.cur() == T![=]
            && CONTAINER_SET.contains(left.kind(p))
        {
            parse_and_continue!(parse_assignment_as_pattern(p, left));
        }

        break;
    }

    match p.cur() {
        T![.] => parse_map_accessor(p, left),
        T!['['] => parse_list_accessor(p, left),
        _ => Present(left),
    }
}

#[inline]
fn parse_pattern_non_greedy(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
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
        GRIT_NAME if p.lookahead() == T!['('] => parse_node_like(p),
        T![.] => parse_dot(p),
        SOME_KW => parse_some(p),
        EVERY_KW => parse_every(p),
        DOLLAR_UNDERSCORE => parse_grit_underscore(p),
        GRIT_UNDERSCORE => parse_grit_underscore(p),
        GRIT_VARIABLE => parse_variable(p),
        GRIT_REGEX | GRIT_SNIPPET_REGEX => parse_regex_pattern(p),
        LIKE_KW => parse_like(p),
        SEQUENTIAL_KW => parse_sequential(p),
        MULTIFILE_KW => parse_files(p),
        T!['('] => parse_bracketed_pattern(p),
        _ => parse_literal(p),
    }
}

#[inline]
fn parse_assignment_as_pattern(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![=]);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN_AS);

    Present(m.complete(p, GRIT_ASSIGNMENT_AS_PATTERN))
}

#[inline]
fn parse_bracketed_pattern(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_BRACKETED_PATTERN))
}

#[inline]
fn parse_bubble(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(BUBBLE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(BUBBLE_KW);

    parse_bubble_scope(p).ok();
    parse_maybe_curly_pattern(p).ok();

    Present(m.complete(p, GRIT_BUBBLE))
}

#[inline]
fn parse_bubble_scope(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    VariableList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_BUBBLE_SCOPE))
}

#[inline]
pub(crate) fn parse_container(p: &mut GritParser) -> ParsedSyntax {
    let left = match p.cur() {
        GRIT_VARIABLE => parse_variable(p),
        T!['{'] => parse_map(p),
        GRIT_NAME if p.lookahead() == T!['['] => parse_list(p),
        T!['['] => parse_list(p),
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

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_CURLY_PATTERN))
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

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_EVERY))
}

#[inline]
fn parse_expected_pattern_with_precedence(p: &mut GritParser, min_precedence: isize) {
    parse_pattern_with_precedence(p, min_precedence)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();
}

#[inline]
fn parse_files(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MULTIFILE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MULTIFILE_KW);

    p.expect(T!['{']);

    PatternList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_FILES))
}

#[inline]
fn parse_grit_underscore(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_UNDERSCORE) && !p.at(DOLLAR_UNDERSCORE) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, GRIT_UNDERSCORE))
}

#[inline]
fn parse_like(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(LIKE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(LIKE_KW);

    parse_like_threshold(p).ok();

    p.expect(T!['{']);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_LIKE))
}

#[inline]
fn parse_like_threshold(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_LIKE_THRESHOLD))
}

#[inline]
pub(crate) fn parse_list_accessor(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T!['[']);

    parse_list_index(p).ok();

    p.expect(T![']']);

    let left = m.complete(p, GRIT_LIST_ACCESSOR);

    match p.cur() {
        T!['['] => parse_list_accessor(p, left),
        T![.] => parse_map_accessor(p, left),
        _ => Present(left),
    }
}

#[inline]
fn parse_list_index(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        GRIT_INT => parse_int_literal(p),
        GRIT_NEGATIVE_INT => parse_negative_int_literal(p),
        _ => parse_container(p),
    }
}

#[inline]
pub(crate) fn parse_map_accessor(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![.]);

    parse_map_key(p).ok();

    let left = m.complete(p, GRIT_MAP_ACCESSOR);

    match p.cur() {
        T!['['] => parse_list_accessor(p, left),
        T![.] => parse_map_accessor(p, left),
        _ => Present(left),
    }
}

#[inline]
fn parse_map_key(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        GRIT_NAME => parse_name(p),
        GRIT_VARIABLE => parse_variable(p),
        _ => Absent,
    }
}

#[derive(Clone, Copy)]
enum MathOperator {
    Mul,
    Div,
    Mod,
    Add,
    Sub,
}

impl MathOperator {
    #[inline]
    fn from_token(token_kind: GritSyntaxKind) -> Option<Self> {
        use MathOperator::*;
        match token_kind {
            T![*] => Some(Mul),
            T![/] => Some(Div),
            T![%] => Some(Mod),
            T![+] => Some(Add),
            T![-] => Some(Sub),
            _ => None,
        }
    }

    #[inline]
    fn operation_kind(self) -> GritSyntaxKind {
        use MathOperator::*;
        match self {
            Mul => GRIT_MUL_OPERATION,
            Div => GRIT_DIV_OPERATION,
            Mod => GRIT_MOD_OPERATION,
            Add => GRIT_ADD_OPERATION,
            Sub => GRIT_SUB_OPERATION,
        }
    }

    #[inline]
    fn precedence(self) -> isize {
        use MathOperator::*;
        match self {
            Mul => PRECEDENCE_MUL,
            Div => PRECEDENCE_DIV,
            Mod => PRECEDENCE_MOD,
            Add => PRECEDENCE_ADD,
            Sub => PRECEDENCE_SUB,
        }
    }

    #[inline]
    fn token_kind(self) -> GritSyntaxKind {
        use MathOperator::*;
        match self {
            Mul => T![*],
            Div => T![/],
            Mod => T![%],
            Add => T![+],
            Sub => T![-],
        }
    }
}

trait OptionalMathOperator {
    fn with_min_precedence(self, min_precedence: isize) -> Self;
}

impl OptionalMathOperator for Option<MathOperator> {
    #[inline]
    fn with_min_precedence(self, min_precedence: isize) -> Self {
        self.filter(|operator| operator.precedence() >= min_precedence)
    }
}

// This is the only place we need an operator-precedence parser for GritQL,
// since math operators are the only ones in the grammar that have
// left-associativity.
//
// See also: https://en.wikipedia.org/wiki/Operator-precedence_parser
fn parse_math_pattern(
    p: &mut GritParser,
    mut left: CompletedMarker,
    operator: MathOperator,
) -> ParsedSyntax {
    if !p.at(operator.token_kind()) {
        return Absent;
    }

    let min_precedence = operator.precedence();

    let mut lookahead = MathOperator::from_token(p.cur());
    while let Some(operator) = lookahead.with_min_precedence(min_precedence) {
        let m = left.precede(p);

        p.bump(operator.token_kind());

        if let Present(mut right) = parse_pattern_non_greedy(p) {
            lookahead = MathOperator::from_token(p.cur());
            while let Some(tighter_operator) =
                lookahead.with_min_precedence(operator.precedence() + 1)
            {
                right = match parse_math_pattern(p, right, tighter_operator) {
                    Present(right) => right,
                    Absent => break,
                };

                lookahead = MathOperator::from_token(p.cur());
            }
        }

        left = m.complete(p, operator.operation_kind())
    }

    Present(left)
}

#[inline]
pub(crate) fn parse_maybe_curly_pattern(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        T!['{'] => parse_curly_pattern(p),
        _ => parse_pattern(p),
    }
}

pub(crate) struct NodeArgList;

impl ParseSeparatedList for NodeArgList {
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
            expected_node_arg,
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
fn parse_node_like(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).ok();
    p.expect(T!['(']);

    NodeArgList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, GRIT_NODE_LIKE))
}

#[inline]
fn parse_pattern_accumulate(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(T![+=]) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(T![+=]);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_ACCUMULATE);

    Present(m.complete(p, GRIT_PATTERN_ACCUMULATE))
}

#[inline]
fn parse_pattern_after(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(AFTER_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(AFTER_KW);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    Present(m.complete(p, GRIT_PATTERN_AFTER))
}

#[inline]
fn parse_pattern_and(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(AND_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(AND_KW);
    p.expect(T!['{']);

    PatternList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_AND))
}

#[inline]
fn parse_pattern_any(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ANY_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ANY_KW);
    p.expect(T!['{']);

    PatternList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_ANY))
}

#[inline]
fn parse_pattern_as(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(AS_KW) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(AS_KW);

    if parse_variable(p) == Absent {
        p.err_and_bump(expected_variable(p, p.cur_range()), GRIT_BOGUS_PATTERN);
    }

    Present(m.complete(p, GRIT_PATTERN_AS))
}

#[inline]
fn parse_pattern_before(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(BEFORE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(BEFORE_KW);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    Present(m.complete(p, GRIT_PATTERN_BEFORE))
}

#[inline]
fn parse_pattern_contains(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(CONTAINS_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(CONTAINS_KW);

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_UNTIL_RECOVERY_SET),
            expected_pattern,
        )
        .ok();
    parse_pattern_until_clause(p).ok();

    Present(m.complete(p, GRIT_PATTERN_CONTAINS))
}

#[inline]
fn parse_pattern_until_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(UNTIL_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(UNTIL_KW);

    parse_expected_pattern_with_precedence(p, PRECEDENCE_PATTERN);

    Present(m.complete(p, GRIT_PATTERN_UNTIL_CLAUSE))
}

#[inline]
fn parse_pattern_if_else(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(IF_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(IF_KW);
    p.expect(T!['(']);

    parse_expected_predicate(p);

    p.expect(T![')']);

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_ELSE_RECOVERY_SET),
            expected_pattern,
        )
        .ok();
    parse_pattern_else_clause(p).ok();

    Present(m.complete(p, GRIT_PATTERN_IF_ELSE))
}

#[inline]
fn parse_pattern_else_clause(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ELSE_KW);

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_PATTERN_ELSE_CLAUSE))
}

#[inline]
fn parse_pattern_includes(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(INCLUDES_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(INCLUDES_KW);

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_PATTERN_INCLUDES))
}

#[inline]
fn parse_pattern_limit(p: &mut GritParser, left: CompletedMarker) -> ParsedSyntax {
    if !p.at(LIMIT_KW) {
        return Absent;
    }

    let m = left.precede(p);
    p.bump(LIMIT_KW);

    if parse_int_literal(p) == Absent {
        p.err_and_bump(expected_int_literal(p, p.cur_range()), GRIT_BOGUS_LITERAL);
    }

    Present(m.complete(p, GRIT_PATTERN_LIMIT))
}

pub(crate) struct PatternList;

impl ParseSeparatedList for PatternList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_PATTERN_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_pattern(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set!(T![')'], T!['}']))
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
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
fn parse_pattern_maybe(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(MAYBE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(MAYBE_KW);
    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_PATTERN_MAYBE))
}

#[inline]
fn parse_pattern_not(p: &mut GritParser) -> ParsedSyntax {
    let m = match parse_not(p) {
        Present(syntax) => syntax.precede(p),
        Absent => return Absent,
    };

    parse_pattern_with_precedence(p, PRECEDENCE_NOT)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_PATTERN_NOT))
}

#[inline]
fn parse_pattern_or(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(OR_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(OR_KW);
    p.eat(T!['{']);

    PatternList.parse_list(p);

    p.eat(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_OR))
}

#[inline]
fn parse_pattern_orelse(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(ORELSE_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(ORELSE_KW);
    p.eat(T!['{']);

    PatternList.parse_list(p);

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

    parse_expected_predicate(p);

    Present(m.complete(p, GRIT_PATTERN_WHERE))
}

#[inline]
fn parse_pattern_within(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(WITHIN_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(WITHIN_KW);

    parse_maybe_curly_pattern(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();
    parse_pattern_until_clause(p).ok();

    Present(m.complete(p, GRIT_WITHIN))
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
        p.expect(GRIT_SNIPPET_REGEX);
        m.complete(p, GRIT_SNIPPET_REGEX_LITERAL);
    }

    parse_regex_pattern_variables(p).ok();

    Present(m.complete(p, GRIT_REGEX_PATTERN))
}

#[inline]
fn parse_regex_pattern_variables(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    parse_variable_list(p);

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

    parse_pattern_with_precedence(p, PRECEDENCE_REWRITE)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_PATTERN, PATTERN_RECOVERY_SET),
            expected_pattern,
        )
        .ok();

    Present(m.complete(p, GRIT_REWRITE))
}

#[inline]
fn parse_sequential(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(SEQUENTIAL_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(SEQUENTIAL_KW);

    p.eat(T!['{']);

    PatternList.parse_list(p);

    p.eat(T!['}']);

    Present(m.complete(p, GRIT_SEQUENTIAL))
}

#[inline]
fn parse_some(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(SOME_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(SOME_KW);

    parse_maybe_curly_pattern(p).ok();

    Present(m.complete(p, GRIT_SOME))
}

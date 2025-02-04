use super::parse_error::{expected_definition, too_many_patterns};
use super::patterns::{parse_pattern, PatternList};
use super::predicates::PredicateList;
use super::{
    parse_language_declaration, parse_name, parse_variable_list, GritParser, VariableList,
};
use crate::constants::*;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax::*;
use biome_parser::{parsed_syntax::ParsedSyntax, Parser};

pub(crate) struct DefinitionList {
    has_pattern: bool,
}

impl DefinitionList {
    pub(crate) fn new() -> Self {
        Self { has_pattern: false }
    }
}

impl ParseNodeList for DefinitionList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let mut syntax = parse_definition(p);

        if syntax == Absent {
            if let Present(pattern) = parse_pattern(p) {
                if self.has_pattern {
                    p.error(too_many_patterns(p, pattern.range(p)));
                }

                syntax = Present(pattern);
                self.has_pattern = true
            }
        }

        syntax
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_DEFINITION, DEFINITION_LIST_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_definition,
        )
    }
}

#[inline]
fn parse_definition(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        FUNCTION_KW => parse_function_definition(p),
        PATTERN_KW | PRIVATE_KW => parse_pattern_definition(p),
        PREDICATE_KW => parse_predicate_definition(p),
        _ => Absent,
    }
}

#[inline]
fn parse_javascript_body_wrapper(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_JAVASCRIPT_BODY) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_JAVASCRIPT_BODY);
    Present(m.complete(p, GRIT_JAVASCRIPT_BODY_WRAPPER))
}

#[inline]
fn parse_function_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(FUNCTION_KW) {
        return Absent;
    }

    let m = p.start();

    p.bump(FUNCTION_KW);
    parse_name(p).ok();
    p.expect(T!['(']);
    VariableList.parse_list(p);
    p.expect(T![')']);

    if p.at(JS_KW) {
        p.bump(JS_KW);
        parse_javascript_body_wrapper(p).ok();
        Present(m.complete(p, GRIT_JAVASCRIPT_FUNCTION_DEFINITION))
    } else {
        parse_curly_predicate_list(p).ok();
        Present(m.complete(p, GRIT_FUNCTION_DEFINITION))
    }
}

#[inline]
fn parse_curly_predicate_list(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['{']);
    PredicateList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PREDICATE_CURLY))
}

#[inline]
fn parse_pattern_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(PATTERN_KW) && (!p.at(PRIVATE_KW) || p.lookahead() != PATTERN_KW) {
        return Absent;
    }

    let m = p.start();

    p.eat(PRIVATE_KW);
    p.bump(PATTERN_KW);
    parse_name(p).ok();
    p.expect(T!['(']);
    parse_variable_list(p);
    p.expect(T![')']);
    parse_language_declaration(p).ok();
    parse_pattern_definition_body(p).ok();

    Present(m.complete(p, GRIT_PATTERN_DEFINITION))
}

#[inline]
fn parse_pattern_definition_body(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['{']);
    PatternList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_DEFINITION_BODY))
}

#[inline]
fn parse_predicate_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(PREDICATE_KW) {
        return Absent;
    }

    let m = p.start();

    p.bump(PREDICATE_KW);
    parse_name(p).ok();
    p.expect(T!['(']);
    parse_variable_list(p);
    p.expect(T![')']);
    parse_curly_predicate_list(p).ok();

    Present(m.complete(p, GRIT_PREDICATE_DEFINITION))
}

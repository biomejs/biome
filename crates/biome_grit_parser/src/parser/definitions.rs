use super::patterns::parse_pattern_list;
use super::predicates::parse_predicate_list;
use super::{parse_language_declaration, parse_name, parse_variable_list, GritParser};
use biome_grit_syntax::{GritSyntaxKind::*, T};
use biome_parser::prelude::ParsedSyntax::*;
use biome_parser::{parsed_syntax::ParsedSyntax, Parser};

pub(crate) fn parse_definition_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if parse_definition(p) != Absent {
        loop {
            if !p.has_preceding_line_break() || parse_definition(p) == Absent {
                break;
            }
        }
    }

    Present(m.complete(p, GRIT_DEFINITION_LIST))
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
fn parse_function_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(FUNCTION_KW) {
        return Absent;
    }

    let m = p.start();

    p.bump(FUNCTION_KW);
    let _ = parse_name(p);
    p.eat(T!['(']);
    let _ = parse_variable_list(p);
    p.eat(T![')']);
    let _ = parse_curly_predicate_list(p);

    Present(m.complete(p, GRIT_FUNCTION_DEFINITION))
}

#[inline]
fn parse_curly_predicate_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    p.eat(T!['{']);
    let _ = parse_predicate_list(p);
    p.eat(T!['}']);

    Present(m.complete(p, GRIT_CURLY_PREDICATE_LIST))
}

#[inline]
fn parse_pattern_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(PATTERN_KW) && (!p.at(PRIVATE_KW) || p.lookahead() != PATTERN_KW) {
        return Absent;
    }

    let m = p.start();

    p.eat(PRIVATE_KW);
    p.bump(PATTERN_KW);
    let _ = parse_name(p);
    p.eat(T!['(']);
    let _ = parse_pattern_arg_list(p);
    p.eat(T![')']);
    let _ = parse_language_declaration(p);
    let _ = parse_pattern_definition_body(p);

    Present(m.complete(p, GRIT_PATTERN_DEFINITION))
}

#[inline]
fn parse_pattern_arg_list(p: &mut GritParser) -> ParsedSyntax {
    parse_variable_list(p).map(|syntax| syntax.precede(p).complete(p, GRIT_PATTERN_ARG_LIST))
}

#[inline]
fn parse_pattern_definition_body(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['{']);
    let _ = parse_pattern_list(p);
    p.eat(T!['}']);

    Present(m.complete(p, GRIT_PATTERN_DEFINITION_BODY))
}

#[inline]
fn parse_predicate_definition(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(PREDICATE_KW) {
        return Absent;
    }

    let m = p.start();

    p.bump(PREDICATE_KW);
    let _ = parse_name(p);
    p.eat(T!['(']);
    let _ = parse_pattern_arg_list(p);
    p.eat(T![')']);
    let _ = parse_curly_predicate_list(p);

    Present(m.complete(p, GRIT_PREDICATE_DEFINITION))
}

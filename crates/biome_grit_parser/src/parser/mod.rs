mod constants;
mod definitions;
mod literals;
mod parse_error;
mod patterns;
mod predicates;

use crate::token_source::GritTokenSource;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_rowan::TextRange;
use constants::*;
use definitions::parse_definition_list;
use literals::parse_double_literal;
use parse_error::{expected_language_flavor, expected_language_name, expected_pattern};
use patterns::parse_pattern;

pub(crate) struct GritParser<'source> {
    context: ParserContext<GritSyntaxKind>,
    source: GritTokenSource<'source>,
}

impl<'source> GritParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: GritTokenSource::from_str(source),
        }
    }

    pub fn lookahead(&mut self) -> GritSyntaxKind {
        self.source.lookahead()
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<GritSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for GritParser<'source> {
    type Kind = GritSyntaxKind;
    type Source = GritTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

pub(crate) fn parse_root(p: &mut GritParser) -> CompletedMarker {
    let m = p.start();

    p.eat(UNICODE_BOM);

    let _ = parse_version(p);
    let _ = parse_language_declaration(p);
    let _ = parse_definition_list(p);
    let _ = parse_pattern(p);
    let _ = parse_definition_list(p);

    p.eat(EOF);

    m.complete(p, GRIT_ROOT)
}

fn parse_version(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T![engine]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![engine]);

    let engine_range = p.cur_range();
    if p.eat(T![biome]) {
        if p.eat(T!['(']) {
            match parse_double_literal(p) {
                Present(_) => {
                    p.eat(T![')']);
                }
                Absent => p.error(p.err_builder("Expected version as a double", p.cur_range())),
            }
        } else {
            let engine_end = engine_range.end();
            p.error(p.err_builder(
                "Expected an engine version",
                TextRange::new(engine_end, engine_end),
            ))
        }
    } else {
        p.error(p.err_builder("Engine must be `biome`", engine_range));
    }

    Present(m.complete(p, GRIT_VERSION))
}

fn parse_language_declaration(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T![language]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![language]);

    let _ = parse_language_name(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, LANGUAGE_NAME_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_language_name,
    );

    let _ = parse_language_flavor(p);

    p.eat(T![;]);

    Present(m.complete(p, GRIT_LANGUAGE_DECLARATION))
}

fn parse_language_name(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(SUPPORTED_LANGUAGE_SET);
    Present(m.complete(p, GRIT_LANGUAGE_NAME))
}

fn parse_language_flavor(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    if parse_language_flavor_list(p) == Absent {
        p.error(p.err_builder("Expected a language flavor", p.cur_range()));
        m.abandon(p);
        return Absent;
    }

    p.eat(T![')']);

    Present(m.complete(p, GRIT_LANGUAGE_FLAVOR))
}

fn parse_language_flavor_list(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    let _ = parse_language_flavor_kind(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(GRIT_BOGUS, LANGUAGE_FLAVOR_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_language_flavor,
    );

    while p.eat(T![,]) {
        let _ = parse_language_flavor_kind(p);
    }

    Present(m.complete(p, GRIT_LANGUAGE_FLAVOR_LIST))
}

fn parse_language_flavor_kind(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_FLAVOR_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(SUPPORTED_LANGUAGE_FLAVOR_SET);
    Present(m.complete(p, GRIT_LANGUAGE_FLAVOR_KIND))
}

#[inline]
fn parse_maybe_named_arg(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        T![')'] => Absent,
        GRIT_NAME => parse_named_arg(p),
        _ => parse_pattern(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(GRIT_BOGUS, ARG_LIST_RECOVERY_SET),
                expected_pattern,
            )
            .ok()
            .into(),
    }
}

#[inline]
fn parse_name(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NAME) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_NAME);
    Present(m.complete(p, GRIT_NAME))
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

    let _ = parse_maybe_named_arg(p);

    while p.eat(T![,]) {
        if parse_maybe_named_arg(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_NAMED_ARG_LIST))
}

#[inline]
fn parse_not(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(NOT_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(NOT_SET);
    Present(m.complete(p, GRIT_NOT))
}

#[inline]
fn parse_variable(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_VARIABLE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_VARIABLE);
    Present(m.complete(p, GRIT_VARIABLE))
}

#[inline]
fn parse_variable_list(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_VARIABLE) {
        return Absent;
    }

    let m = p.start();
    let _ = parse_variable(p);

    while p.eat(T![,]) {
        if parse_variable(p) == Absent {
            break;
        }
    }

    Present(m.complete(p, GRIT_VARIABLE_LIST))
}

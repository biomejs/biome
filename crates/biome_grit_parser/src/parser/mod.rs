mod definitions;
mod literals;
mod parse_error;
mod patterns;
mod predicates;
mod tests;

use crate::constants::*;
use crate::token_source::GritTokenSource;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_rowan::TextRange;
use definitions::DefinitionList;
use literals::parse_double_literal;
use patterns::parse_pattern;

use self::parse_error::{
    expected_engine_version, expected_language_flavor, expected_language_name, expected_variable,
};

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

pub(crate) fn parse_root(p: &mut GritParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);

    parse_version(p).ok();
    parse_language_declaration(p).ok();

    DefinitionList::new().parse_list(p);

    p.expect(EOF);

    m.complete(p, GRIT_ROOT);
}

fn parse_version(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T![engine]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![engine]);

    let mut is_supported = true;

    let engine_range = p.cur_range();

    if parse_engine_name(p) != Absent {
        if p.eat(T!['(']) {
            match parse_double_literal(p) {
                Present(_) => {
                    p.expect(T![')']);
                }
                Absent => {
                    if p.at(T![')']) {
                        p.error(expected_engine_version(p, p.cur_range()));
                    } else {
                        p.error(p.err_builder("Expected version to be a double", p.cur_range()));
                        p.bump_any();
                    }
                    p.bump(T![')']);
                    is_supported = false;
                }
            }
        } else {
            let engine_end = engine_range.end();
            p.error(expected_engine_version(
                p,
                TextRange::new(engine_end, engine_end),
            ));
            is_supported = false;
        }
    } else {
        p.error(p.err_builder("Engine must be `biome`", engine_range));
        is_supported = false;
    }

    Present(m.complete(
        p,
        if is_supported {
            GRIT_VERSION
        } else {
            GRIT_BOGUS_VERSION
        },
    ))
}

fn parse_engine_name(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_ENGINE_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(SUPPORTED_ENGINE_SET);
    Present(m.complete(p, GRIT_ENGINE_NAME))
}

fn parse_language_declaration(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(T![language]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![language]);

    let mut is_supported = true;

    if parse_language_name(p) == Absent {
        p.error(expected_language_name(p, p.cur_range()));
        p.eat(GRIT_NAME);
        is_supported = false;
    }

    if p.at(T!['(']) {
        let m = p.start();
        p.bump(T!['(']);

        LanguageFlavorList.parse_list(p);

        p.expect(T![')']);

        m.complete(p, GRIT_LANGUAGE_FLAVOR);
    }

    p.eat(T![;]);

    Present(m.complete(
        p,
        if is_supported {
            GRIT_LANGUAGE_DECLARATION
        } else {
            GRIT_BOGUS_LANGUAGE_DECLARATION
        },
    ))
}

pub(crate) struct LanguageFlavorList;

impl ParseSeparatedList for LanguageFlavorList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_LANGUAGE_FLAVOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_language_flavor_kind(p)
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
            &ParseRecoveryTokenSet::new(GRIT_BOGUS_LANGUAGE_FLAVOR_KIND, ARG_LIST_RECOVERY_SET),
            expected_language_flavor,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_language_name(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_SET) {
        let m = p.start();
        p.error(expected_language_name(p, p.cur_range()));
        p.bump_any();
        return Present(m.complete(p, GRIT_BOGUS_LANGUAGE_NAME));
    }

    let m = p.start();
    p.bump_ts(SUPPORTED_LANGUAGE_SET);
    Present(m.complete(p, GRIT_LANGUAGE_NAME))
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
        _ => parse_pattern(p),
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
    parse_name(p).ok();
    p.eat(T![=]);
    parse_pattern(p).ok();
    Present(m.complete(p, GRIT_NAMED_ARG))
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
fn parse_variable_list(p: &mut GritParser) {
    VariableList.parse_list(p);
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

pub(crate) struct VariableList;

impl ParseSeparatedList for VariableList {
    type Kind = GritSyntaxKind;
    type Parser<'source> = GritParser<'source>;

    const LIST_KIND: Self::Kind = GRIT_VARIABLE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_variable(p)
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
            &ParseRecoveryTokenSet::new(GRIT_BOGUS, ARG_LIST_RECOVERY_SET),
            expected_variable,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

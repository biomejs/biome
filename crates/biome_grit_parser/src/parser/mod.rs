mod literals;
mod patterns;
mod predicates;

use crate::token_source::GritTokenSource;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_rowan::TextRange;
use literals::parse_double_literal;
use patterns::parse_pattern;

const SUPPORTED_LANGUAGE_SET: TokenSet<GritSyntaxKind> =
    token_set![T![js], T![json], T![css], T![grit], T![html]];

const SUPPORTED_LANGUAGE_FLAVOR_SET: TokenSet<GritSyntaxKind> = token_set![T![typescript], T![jsx]];

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

    parse_version(p);
    parse_language(p);
    parse_definition_list(p);
    let _ = parse_pattern(p);
    parse_definition_list(p);

    p.eat(EOF);

    m.complete(p, GRIT_ROOT)
}

fn parse_version(p: &mut GritParser) -> Option<CompletedMarker> {
    if !p.at(T![engine]) {
        return None;
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

    let result = m.complete(p, GRIT_VERSION);

    Some(result)
}

fn parse_language(p: &mut GritParser) -> Option<CompletedMarker> {
    if !p.at(T![language]) {
        return None;
    }

    let m = p.start();
    p.bump(T![language]);

    if p.at_ts(SUPPORTED_LANGUAGE_SET) {
        let m = p.start();
        p.bump_ts(SUPPORTED_LANGUAGE_SET);
        m.complete(p, GRIT_LANGUAGE_NAME);
    } else {
        p.error(p.err_builder(
            "Expected a supported language; must be one of `js`, `json`, `css`, `html`, or `grit`",
            p.cur_range(),
        ))
    }

    let _ = parse_language_flavor(p);

    p.eat(T![;]);

    let result = m.complete(p, GRIT_LANGUAGE_DECLARATION);

    Some(result)
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

    let _ = parse_language_flavor_kind(p);

    while p.eat(T![,]) {
        let _ = parse_language_flavor_kind(p);
    }

    Present(m.complete(p, GRIT_LANGUAGE_FLAVOR_LIST))
}

fn parse_language_flavor_kind(p: &mut GritParser) -> ParsedSyntax {
    let m = p.start();

    if p.at_ts(SUPPORTED_LANGUAGE_FLAVOR_SET) {
        p.bump_ts(SUPPORTED_LANGUAGE_FLAVOR_SET);
        Present(m.complete(p, GRIT_LANGUAGE_FLAVOR_KIND))
    } else {
        p.error(p.err_builder(
            "Expected a supported language flavor; must be one of `typescript` or `jsx`",
            p.cur_range(),
        ));
        m.abandon(p);
        Absent
    }
}

fn parse_definition_list(p: &mut GritParser) -> CompletedMarker {
    let m = p.start();

    match p.cur() {
        GRIT_BOGUS => {
            let m = p.start();
            p.bump(GRIT_BOGUS);
            m.complete(p, GRIT_BOGUS_DEFINITION);
        }
        _ => {}
    }

    m.complete(p, GRIT_DEFINITION_LIST)
}

#[inline]
fn parse_variable_list(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_VARIABLE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_VARIABLE);

    while p.eat(T![,]) {
        if !p.eat(GRIT_VARIABLE) {
            break;
        }
    }

    Present(m.complete(p, GRIT_VARIABLE_LIST))
}

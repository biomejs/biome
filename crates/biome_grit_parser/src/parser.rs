use crate::token_source::GritTokenSource;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_rowan::TextRange;

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
    //parse_pattern(p);
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
            match parse_double_value(p) {
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

fn parse_double_value(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_DOUBLE_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_DOUBLE_LITERAL);
    Present(m.complete(p, GRIT_DOUBLE_VALUE))
}

fn parse_language(p: &mut GritParser) -> Option<CompletedMarker> {
    if !p.at(T![language]) {
        return None;
    }

    let m = p.start();
    p.bump(T![language]);

    if !p.eat(GRIT_LANGUAGE_NAME) {
        p.error(p.err_builder("Expected a supported language", p.cur_range()))
    }

    parse_language_flavor(p);

    p.eat(T![;]);

    let result = m.complete(p, GRIT_LANGUAGE_DECLARATION);

    Some(result)
}

fn parse_language_flavor(p: &mut GritParser) -> Option<CompletedMarker> {
    if !p.at(T!['(']) {
        return None;
    }

    let m = p.start();
    p.eat(T!['(']);

    while p.eat(GRIT_LANGUAGE_FLAVOR_KIND) {
        if !p.eat(T![,]) {
            break;
        }
    }

    p.expect(T![')']);

    let result = m.complete(p, GRIT_LANGUAGE_FLAVOR);

    Some(result)
}

fn parse_definition_list(p: &mut GritParser) -> CompletedMarker {
    let m = p.start();

    // TODO

    m.complete(p, GRIT_DEFINITION_LIST)
}

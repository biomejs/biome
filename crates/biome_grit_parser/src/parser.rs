use crate::token_source::GritTokenSource;
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use biome_rowan::TextRange;

const BOOLEAN_VALUE_SET: TokenSet<GritSyntaxKind> = token_set![TRUE_KW, FALSE_KW];

const SUPPORTED_LANGUAGE_SET: TokenSet<GritSyntaxKind> =
    token_set![T![js], T![json], T![css], T![grit], T![html]];

const SUPPORTED_LANGUAGE_FLAVOR_SET: TokenSet<GritSyntaxKind> = token_set![T![typescript], T![jsx]];

const CODE_SNIPPET_SET: TokenSet<GritSyntaxKind> =
    SUPPORTED_LANGUAGE_SET.union(token_set![GRIT_BACKTICK_SNIPPET, GRIT_RAW_BACKTICK_SNIPPET]);

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
    p.bump(T!['(']);

    {
        let m = p.start();

        loop {
            if p.at_ts(SUPPORTED_LANGUAGE_FLAVOR_SET) {
                let m = p.start();
                p.bump_ts(SUPPORTED_LANGUAGE_FLAVOR_SET);
                m.complete(p, GRIT_LANGUAGE_FLAVOR_KIND);

                if !p.eat(T![,]) {
                    break;
                }
            } else if p.at(T![')']) {
                break;
            } else {
                p.error(p.err_builder(
                    "Expected a supported language flavor; must be one of `typescript` or `jsx`",
                    p.cur_range(),
                ));
                break;
            }
        }

        m.complete(p, GRIT_LANGUAGE_FLAVOR_LIST);
    }

    p.eat(T![')']);

    let result = m.complete(p, GRIT_LANGUAGE_FLAVOR);

    Some(result)
}

fn parse_definition_list(p: &mut GritParser) -> CompletedMarker {
    let m = p.start();

    // TODO

    m.complete(p, GRIT_DEFINITION_LIST)
}

fn parse_pattern(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        _ => parse_literal(p),
    }
}

fn parse_literal(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        TRUE_KW | FALSE_KW => parse_boolean_literal(p),
        GRIT_DOUBLE => parse_double_literal(p),
        GRIT_INT => parse_int_literal(p),
        GRIT_NEGATIVE_INT => parse_negative_int_literal(p),
        GRIT_STRING => parse_string_literal(p),
        UNDEFINED_KW => parse_undefined_literal(p),
        kind if CODE_SNIPPET_SET.contains(kind) => parse_code_snippet(p),
        // TODO: List
        // TODO: Map
        _ => Absent,
    }
}

#[inline]
fn parse_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
fn parse_boolean_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(BOOLEAN_VALUE_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(BOOLEAN_VALUE_SET);
    Present(m.complete(p, GRIT_BOOLEAN_LITERAL))
}

#[inline]
fn parse_code_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(CODE_SNIPPET_SET) {
        return Absent;
    }

    let m = p.start();

    let syntax = match p.cur() {
        GRIT_BACKTICK_SNIPPET => parse_backtick_snippet_literal(p),
        GRIT_RAW_BACKTICK_SNIPPET => parse_raw_backtick_snippet_literal(p),
        _ => parse_language_specific_snippet(p),
    };

    match syntax {
        Present(_) => Present(m.complete(p, GRIT_CODE_SNIPPET)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
fn parse_double_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_DOUBLE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_DOUBLE);
    Present(m.complete(p, GRIT_DOUBLE_LITERAL))
}

#[inline]
fn parse_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_INT);
    Present(m.complete(p, GRIT_INT_LITERAL))
}

#[inline]
fn parse_language_specific_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_SET) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(SUPPORTED_LANGUAGE_SET);
    p.eat(GRIT_STRING);

    Present(m.complete(p, GRIT_LANGUAGE_SPECIFIC_SNIPPET))
}

#[inline]
fn parse_negative_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NEGATIVE_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_NEGATIVE_INT);
    Present(m.complete(p, GRIT_NEGATIVE_INT_LITERAL))
}

#[inline]
fn parse_raw_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_RAW_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_RAW_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_RAW_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
fn parse_string_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_STRING) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_STRING);
    Present(m.complete(p, GRIT_STRING_LITERAL))
}

#[inline]
fn parse_undefined_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(UNDEFINED_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(UNDEFINED_KW);
    Present(m.complete(p, GRIT_UNDEFINED_LITERAL))
}

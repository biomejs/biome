use biome_parser::{
    CompletedMarker, Parser, ParserContext,
    diagnostic::merge_diagnostics,
    event::Event,
    parse_lists::ParseNodeList,
    prelude::{ParseDiagnostic, TokenSource, Trivia},
};
use biome_yaml_syntax::YamlSyntaxKind::{self, *};
use document::DocumentList;

use crate::token_source::YamlTokenSource;

mod block;
mod document;
mod flow;
mod parse_error;

pub(crate) struct YamlParser<'source> {
    context: ParserContext<YamlSyntaxKind>,
    source: YamlTokenSource<'source>,
}

impl<'source> YamlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: YamlTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<YamlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for YamlParser<'source> {
    type Kind = YamlSyntaxKind;

    type Source = YamlTokenSource<'source>;

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

pub(crate) fn parse_root(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();

    DocumentList.parse_list(p);
    p.expect(EOF);

    m.complete(p, YAML_ROOT)
}

use biome_parser::{
    CompletedMarker, Parser, ParserContext,
    diagnostic::merge_diagnostics,
    event::Event,
    parse_lists::ParseNodeList,
    prelude::{ParseDiagnostic, TokenSource, Trivia},
};
use biome_rowan::TextRange;
use biome_yaml_syntax::YamlSyntaxKind::{self, *};
use document::DocumentList;

use crate::token_source::YamlTokenSource;

mod block;
mod document;
mod flow;
mod parse_error;
mod property;

pub(crate) struct YamlParser<'source> {
    context: ParserContext<YamlSyntaxKind>,
    source: YamlTokenSource<'source>,
    tag_handles: Vec<TextRange>,
}

impl<'source> YamlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: YamlTokenSource::from_str(source),
            tag_handles: Vec::new(),
        }
    }

    pub(crate) fn clear_tag_handles(&mut self) {
        self.tag_handles.clear();
    }

    pub(crate) fn declare_tag_handle(&mut self, range: TextRange) {
        self.tag_handles.push(range);
    }

    pub(crate) fn is_tag_handle_declared(&self, handle: &str) -> bool {
        self.tag_handles.iter().any(|range| {
            let range: std::ops::Range<usize> = (*range).into();
            self.source.text().get(range) == Some(handle)
        })
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

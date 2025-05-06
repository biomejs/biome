use biome_parser::{
    CompletedMarker, Parser, ParserContext, ParserContextCheckpoint,
    diagnostic::merge_diagnostics,
    event::Event,
    parse_lists::ParseNodeList,
    prelude::{ParseDiagnostic, TokenSource, Trivia},
};
use biome_yaml_syntax::YamlSyntaxKind::{self, *};
use document::DocumentList;

use crate::{
    lexer::YamlLexContext,
    token_source::{YamlTokenSource, YamlTokenSourceCheckpoint},
};

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

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token
    pub fn re_lex(&mut self, context: YamlLexContext) -> YamlSyntaxKind {
        self.source_mut().re_lex(context)
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

    #[expect(dead_code)]
    pub fn checkpoint(&self) -> YamlParserCheckpoint {
        YamlParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            // `state` is not checkpointed because it (currently) only contains
            // scoped properties that aren't only dependent on checkpoints and
            // should be reset manually when the scope of their use is exited.
        }
    }

    #[expect(dead_code)]
    pub fn rewind(&mut self, checkpoint: YamlParserCheckpoint) {
        let YamlParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        // `state` is not checkpointed because it (currently) only contains
        // scoped properties that aren't only dependent on checkpoints and
        // should be reset manually when the scope of their use is exited.
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

pub struct YamlParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: YamlTokenSourceCheckpoint,
}

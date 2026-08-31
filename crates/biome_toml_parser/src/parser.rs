use crate::token_source::TomlTokenSource;
use biome_parser::{
    Parser, ParserContext,
    diagnostic::merge_diagnostics,
    event::Event,
    prelude::{ParseDiagnostic, TokenSource},
    token_source::Trivia,
};
use biome_toml_syntax::TomlSyntaxKind;

pub(crate) struct TomlParser<'source> {
    context: ParserContext<TomlSyntaxKind>,
    source: TomlTokenSource<'source>,
}

impl<'source> TomlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: TomlTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<TomlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parser_diagnostics) = self.context.finish();
        let diagnostics = merge_diagnostics(lexer_diagnostics, parser_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for TomlParser<'source> {
    type Kind = TomlSyntaxKind;
    type Source = TomlTokenSource<'source>;

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

use crate::lexer::TomlLexContext;
use crate::token_source::TomlTokenSource;
use biome_parser::ParserContext;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
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

    /// Lexes the current token again in `context`.
    pub fn re_lex(&mut self, context: TomlLexContext) -> TomlSyntaxKind {
        self.source.re_lex(context)
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<TomlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

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

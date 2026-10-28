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
    /// The opening brackets of the arrays and inline tables that enclose the
    /// current token, innermost last.
    open_brackets: Vec<TomlSyntaxKind>,
}

impl<'source> TomlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: TomlTokenSource::from_str(source),
            open_brackets: Vec::new(),
        }
    }

    pub fn nesting_depth(&self) -> usize {
        self.open_brackets.len()
    }

    /// Enters an array or inline table opened by `bracket`.
    pub fn enter_nesting(&mut self, bracket: TomlSyntaxKind) {
        self.open_brackets.push(bracket);
    }

    pub fn exit_nesting(&mut self) {
        self.open_brackets.pop();
    }

    /// Returns `true` if a value opened by `bracket` encloses the innermost
    /// array or inline table.
    pub fn is_enclosed_by(&self, bracket: TomlSyntaxKind) -> bool {
        self.open_brackets
            .split_last()
            .is_some_and(|(_, outer)| outer.contains(&bracket))
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

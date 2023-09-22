use crate::token_source::JsonTokenSource;
use biome_json_syntax::JsonSyntaxKind;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;

pub(crate) struct JsonParser<'source> {
    context: ParserContext<JsonSyntaxKind>,
    source: JsonTokenSource<'source>,
    options: JsonParserOptions,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JsonParserOptions {
    pub allow_comments: bool,
    pub allow_trailing_commas: bool,
}

impl JsonParserOptions {
    pub fn with_allow_comments(mut self) -> Self {
        self.allow_comments = true;
        self
    }

    pub fn with_allow_trailing_commas(mut self) -> Self {
        self.allow_trailing_commas = true;
        self
    }
}

impl<'source> JsonParser<'source> {
    pub fn new(source: &'source str, options: JsonParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: JsonTokenSource::from_str(source, options),
            options,
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<JsonSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }

    pub fn options(&self) -> &JsonParserOptions {
        &self.options
    }
}

impl<'source> Parser for JsonParser<'source> {
    type Kind = JsonSyntaxKind;
    type Source = JsonTokenSource<'source>;

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

use crate::token_source::JsonTokenSource;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use rome_json_syntax::JsonSyntaxKind;

pub(crate) struct JsonParser<'source> {
    context: ParserContext<JsonSyntaxKind>,
    source: JsonTokenSource<'source>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JsonParserOptions {
    pub allow_comments: bool,
}

impl JsonParserOptions {
    pub fn with_allow_comments(mut self) -> Self {
        self.allow_comments = true;
        self
    }
}

impl<'source> JsonParser<'source> {
    pub fn new(source: &'source str, config: JsonParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: JsonTokenSource::from_str(source, config),
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

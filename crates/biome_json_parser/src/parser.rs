use crate::token_source::JsonTokenSource;
use biome_json_syntax::{JsonFileSource, JsonSyntaxKind};
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
    #[must_use]
    pub fn with_allow_comments(mut self) -> Self {
        self.allow_comments = true;
        self
    }

    #[must_use]
    pub fn with_allow_trailing_commas(mut self) -> Self {
        self.allow_trailing_commas = true;
        self
    }
}

impl From<&JsonFileSource> for JsonParserOptions {
    fn from(file_source: &JsonFileSource) -> Self {
        let mut options = Self::default();
        if file_source.allow_comments() {
            options = options.with_allow_comments();
        }
        if file_source.allow_trailing_commas() {
            options = options.with_allow_trailing_commas();
        }
        options
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

#[cfg(test)]
mod tests {
    use crate::JsonParserOptions;
    use biome_json_syntax::JsonFileSource;

    #[test]
    fn parse_options_from_json_file_source_respects_allow_comments_and_allow_trailing_comma() {
        let p1 = JsonParserOptions::from(&JsonFileSource::json());
        assert!(!p1.allow_comments);
        assert!(!p1.allow_trailing_commas);

        let p2 = JsonParserOptions::from(&JsonFileSource::json_allow_comments(""));
        assert!(p2.allow_comments);
        assert!(!p2.allow_trailing_commas);

        let p3 =
            JsonParserOptions::from(&JsonFileSource::json_allow_comments_and_trailing_commas(""));
        assert!(p3.allow_comments);
        assert!(p3.allow_trailing_commas);
    }
}

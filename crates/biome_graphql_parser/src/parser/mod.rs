mod argument;
mod definitions;
mod directive;
mod parse_error;
mod r#type;
mod value;
mod variable;
use crate::token_source::GraphqlTokenSource;
use biome_graphql_syntax::GraphqlSyntaxKind::{self, *};
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use definitions::DefinitionList;

use self::value::{is_at_string, parse_string};

pub(crate) struct GraphqlParser<'source> {
    context: ParserContext<GraphqlSyntaxKind>,
    source: GraphqlTokenSource<'source>,
}

impl<'source> GraphqlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: GraphqlTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<GraphqlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for GraphqlParser<'source> {
    type Kind = GraphqlSyntaxKind;
    type Source = GraphqlTokenSource<'source>;

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

pub(crate) fn parse_root(p: &mut GraphqlParser) -> CompletedMarker {
    let m = p.start();

    p.eat(UNICODE_BOM);

    DefinitionList.parse_list(p);

    p.expect(EOF);

    m.complete(p, GRAPHQL_ROOT)
}

#[inline]
fn parse_name(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(GRAPHQL_NAME) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRAPHQL_NAME);
    Present(m.complete(p, GRAPHQL_NAME))
}

#[inline]
fn parse_description(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_string(p) {
        return Absent;
    }

    let m = p.start();
    // already checked for in `is_at_string`
    parse_string(p).ok();

    Present(m.complete(p, GRAPHQL_DESCRIPTION))
}

#[inline]
fn is_at_name(p: &GraphqlParser) -> bool {
    p.at(GRAPHQL_NAME)
}

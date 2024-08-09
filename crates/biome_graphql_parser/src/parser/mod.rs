mod argument;
mod definitions;
mod directive;
mod parse_error;
mod r#type;
mod value;
mod variable;
use crate::token_source::GraphqlTokenSource;
use biome_graphql_syntax::GraphqlSyntaxKind::{self, *};
use biome_graphql_syntax::T;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::prelude::{ParsedSyntax::*, *};
use biome_parser::token_source::Trivia;
use biome_parser::ParserContext;
use definitions::DefinitionList;

use self::value::{is_at_string, parse_string};

/// Graphql allow keywords to be used as names
const GRAPHQL_POTENTIAL_NAME_SET: TokenSet<GraphqlSyntaxKind> = token_set![
    T![ident],
    TRUE_KW,
    FALSE_KW,
    QUERY_KW,
    MUTATION_KW,
    SUBSCRIPTION_KW,
    FRAGMENT_KW,
    ON_KW,
    NULL_KW,
    SCHEMA_KW,
    EXTEND_KW,
    SCALAR_KW,
    TYPE_KW,
    IMPLEMENTS_KW,
    INTERFACE_KW,
    UNION_KW,
    ENUM_KW,
    INPUT_KW,
    DIRECTIVE_KW,
    REPEATABLE_KW,
    UPPER_QUERY_KW,
    UPPER_MUTATION_KW,
    UPPER_SUBSCRIPTION_KW,
    UPPER_FIELD_KW,
    FRAGMENT_DEFINITION_KW,
    FRAGMENT_SPREAD_KW,
    INLINE_FRAGMENT_KW,
    VARIABLE_DEFINITION_KW,
    UPPER_SCHEMA_KW,
    UPPER_SCALAR_KW,
    UPPER_OBJECT_KW,
    FIELD_DEFINITION_KW,
    ARGUMENT_DEFINITION_KW,
    UPPER_INTERFACE_KW,
    UPPER_UNION_KW,
    UPPER_ENUM_KW,
    ENUM_VALUE_KW,
    INPUT_OBJECT_KW,
    INPUT_FIELD_DEFINITION_KW,
];

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
fn parse_literal_name(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_nth_at_name(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![ident]);
    Present(m.complete(p, GRAPHQL_LITERAL_NAME))
}

#[inline]
fn parse_binding(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_nth_at_name(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![ident]);
    Present(m.complete(p, GRAPHQL_NAME_BINDING))
}

#[inline]
fn parse_reference(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_nth_at_name(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![ident]);
    Present(m.complete(p, GRAPHQL_NAME_REFERENCE))
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
fn is_nth_at_name(p: &mut GraphqlParser, n: usize) -> bool {
    p.nth_at_ts(n, GRAPHQL_POTENTIAL_NAME_SET)
}

#[inline]
fn is_nth_at_non_kw_name(p: &mut GraphqlParser, n: usize) -> bool {
    p.nth_at(n, T![ident])
}

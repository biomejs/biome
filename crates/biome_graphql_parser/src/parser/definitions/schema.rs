use crate::parser::{
    directive::DirectiveList,
    parse_description,
    parse_error::{
        expected_named_type, expected_operation_type, expected_root_operation_type_definition,
        expected_schema_extension,
    },
    r#type::parse_named_type,
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, token_source::TokenSource, CompletedMarker, Parser,
};

use super::{is_at_definition, operation::OPERATION_TYPE};

#[inline]
pub(crate) fn parse_schema_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    // description is optional
    parse_description(p).ok();

    p.bump(T![schema]);

    DirectiveList.parse_list(p);
    parse_root_operation_types(p);

    Present(m.complete(p, GRAPHQL_SCHEMA_DEFINITION))
}

/// Must only be called if the next 2 token is `extend` and `schema`, otherwise it will panic.
#[inline]
pub(crate) fn parse_schema_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.bump(T![schema]);

    let pos = p.source().position();
    DirectiveList.parse_list(p);
    let directive_empty = p.source().position() == pos;

    if is_at_root_operation_types(p) {
        parse_root_operation_types(p);
    } else if directive_empty {
        p.error(expected_schema_extension(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_SCHEMA_EXTENSION))
}

#[inline]
fn parse_root_operation_types(p: &mut GraphqlParser) -> CompletedMarker {
    let m = p.start();
    p.expect(T!['{']);
    RootOperationTypeDefinitionList.parse_list(p);
    p.expect(T!['}']);
    m.complete(p, GRAPHQL_ROOT_OPERATION_TYPES)
}

#[derive(Default)]
struct RootOperationTypeDefinitionList;

impl ParseNodeList for RootOperationTypeDefinitionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_root_operation_type_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_root_operation_type_definition_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &RootOperationTypeDefinitionListRecovery,
            expected_root_operation_type_definition,
        )
    }
}

struct RootOperationTypeDefinitionListRecovery;

impl ParseRecovery for RootOperationTypeDefinitionListRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_root_operation_type_definition(p) || is_at_root_operation_type_definition_end(p)
    }
}

#[inline]
fn parse_root_operation_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !(is_at_root_operation_type_definition(p)) {
        return Absent;
    }
    let m = p.start();
    if p.at_ts(OPERATION_TYPE) {
        let m = p.start();
        p.bump_ts(OPERATION_TYPE);
        m.complete(p, GRAPHQL_OPERATION_TYPE);
    }
    // missing operation type
    else if p.at(T![:]) {
        p.error(expected_operation_type(p, p.cur_range()));
    }
    // handle typo in operation type
    else {
        p.error(expected_operation_type(p, p.cur_range()));
        p.bump_any();
    }
    p.expect(T![:]);
    parse_named_type(p).or_add_diagnostic(p, expected_named_type);

    Present(m.complete(p, GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION))
}

#[inline]
fn is_at_root_operation_types(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T!['{'])
        // Must have a colon to be recognized as a root operation type definition
        // else it's likely a new operation definition
        || (p.at_ts(OPERATION_TYPE) && p.nth_at(1, T![:]))
}

#[inline]
fn is_at_root_operation_type_definition(p: &mut GraphqlParser<'_>) -> bool {
    p.at_ts(OPERATION_TYPE)
        // missing operation type
        || p.at(T![:])
        // there is likely a typo in the operation type
        || p.nth_at(1, T![:])
}

/// To prevent a missing closing brace from causing the parser to include the next definition
/// as part of the root operation type definition
/// ```graphql
/// schema {
///    query: Query
///    mutation: Mutation
///
/// schema {
///   query: Query
/// }
#[inline]
fn is_at_root_operation_type_definition_end(p: &mut GraphqlParser<'_>) -> bool {
    // stop at closing brace or at the start of a new definition
    p.at(T!['}'])
        || (!p.at_ts(OPERATION_TYPE) && is_at_definition(p))
        // start of a new operation definition
        || (p.at_ts(OPERATION_TYPE) && !p.nth_at(1, T![:]))
}

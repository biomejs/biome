mod operation;

use crate::parser::{parse_error::expected_any_definition, GraphqlParser};
use biome_graphql_syntax::GraphqlSyntaxKind::{self, *};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use self::operation::{is_at_operation, parse_operation_definition};

struct DefinitionListParseRecovery;

impl ParseRecovery for DefinitionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS_DEFINITION;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // TODO: recover at any definition
        is_at_operation(p)
    }
}

#[derive(Default)]
pub(crate) struct DefinitionList;

impl ParseNodeList for DefinitionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &DefinitionListParseRecovery, expected_any_definition)
    }
}

#[inline]
fn parse_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    match p.cur() {
        // TODO: parse any definition
        _ if is_at_operation(p) => parse_operation_definition(p),
        _ => Absent,
    }
}

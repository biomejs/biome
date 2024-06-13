mod directive;
mod r#enum;
mod field;
mod fragment;
mod input_object;
mod interface;
mod object;
mod operation;
mod scalar;
mod schema;
mod union;

use crate::parser::{parse_error::expected_any_definition, GraphqlParser};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use self::{
    directive::parse_directive_definition,
    fragment::parse_fragment_definition,
    input_object::{parse_input_object_type_definition, parse_input_object_type_extension},
    interface::{parse_interface_type_definition, parse_interface_type_extension},
    object::{parse_object_type_definition, parse_object_type_extension},
    operation::{parse_operation_definition, parse_selection_set},
    r#enum::{parse_enum_type_definition, parse_enum_type_extension},
    scalar::{parse_scalar_type_definition, parse_scalar_type_extension},
    schema::{parse_schema_definition, parse_schema_extension},
    union::{parse_union_type_definition, parse_union_type_extension},
};

use super::value::is_at_string;

struct DefinitionListParseRecovery;

impl ParseRecovery for DefinitionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS_DEFINITION;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_definition(p)
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
    let keyword = if is_at_string(p) { p.nth(1) } else { p.cur() };
    match keyword {
        T![query] | T![mutation] | T![subscription] => parse_operation_definition(p),
        T!['{'] => parse_selection_set(p),
        T![fragment] => parse_fragment_definition(p),
        T![schema] => parse_schema_definition(p),
        T![scalar] => parse_scalar_type_definition(p),
        T![type] => parse_object_type_definition(p),
        T![interface] => parse_interface_type_definition(p),
        T![union] => parse_union_type_definition(p),
        T![enum] => parse_enum_type_definition(p),
        T![input] => parse_input_object_type_definition(p),
        T![directive] => parse_directive_definition(p),
        T![extend] => parse_extension(p),
        _ => Absent,
    }
}

#[inline]
fn parse_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    match p.nth(1) {
        T![schema] => parse_schema_extension(p),
        T![scalar] => parse_scalar_type_extension(p),
        T![type] => parse_object_type_extension(p),
        T![interface] => parse_interface_type_extension(p),
        T![union] => parse_union_type_extension(p),
        T![enum] => parse_enum_type_extension(p),
        T![input] => parse_input_object_type_extension(p),
        _ => Absent,
    }
}

#[inline]
fn is_at_definition(p: &mut GraphqlParser<'_>) -> bool {
    let keyword = if is_at_string(p) { p.nth(1) } else { p.cur() };
    matches!(
        keyword,
        T![query]
            | T![mutation]
            | T![subscription]
            | T!['{']
            | T![fragment]
            | T![schema]
            | T![scalar]
            | T![type]
            | T![interface]
            | T![union]
            | T![enum]
            | T![input]
            | T![directive]
            | T![extend]
    )
}

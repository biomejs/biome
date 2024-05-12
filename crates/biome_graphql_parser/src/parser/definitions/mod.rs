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
use biome_graphql_syntax::GraphqlSyntaxKind::{self, *};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use self::{
    directive::{is_at_directive_definition, parse_directive_definition},
    fragment::{is_at_fragment_definition, parse_fragment_definition},
    input_object::{is_at_input_object_type_definition, parse_input_object_type_definition},
    interface::{is_at_interface_type_definition, parse_interface_type_definition},
    object::{is_at_object_type_definition, parse_object_type_definition},
    operation::{is_at_operation, parse_operation_definition},
    r#enum::{is_at_enum_type_definition, parse_enum_type_definition},
    scalar::{is_at_scalar_type_definition, parse_scalar_type_definition},
    schema::{is_at_schema_definition, parse_schema_definition},
    union::{is_at_union_type_definition, parse_union_type_definition},
};
pub(crate) use operation::is_at_selection_set_end;

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
    if is_at_operation(p) {
        parse_operation_definition(p)
    } else if is_at_fragment_definition(p) {
        parse_fragment_definition(p)
    } else if is_at_schema_definition(p) {
        parse_schema_definition(p)
    } else if is_at_scalar_type_definition(p) {
        parse_scalar_type_definition(p)
    } else if is_at_object_type_definition(p) {
        parse_object_type_definition(p)
    } else if is_at_interface_type_definition(p) {
        parse_interface_type_definition(p)
    } else if is_at_union_type_definition(p) {
        parse_union_type_definition(p)
    } else if is_at_enum_type_definition(p) {
        parse_enum_type_definition(p)
    } else if is_at_input_object_type_definition(p) {
        parse_input_object_type_definition(p)
    } else if is_at_directive_definition(p) {
        parse_directive_definition(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_definition(p: &mut GraphqlParser<'_>) -> bool {
    is_at_operation(p)
        || is_at_fragment_definition(p)
        || is_at_schema_definition(p)
        || is_at_scalar_type_definition(p)
        || is_at_object_type_definition(p)
        || is_at_interface_type_definition(p)
        || is_at_union_type_definition(p)
        || is_at_enum_type_definition(p)
        || is_at_input_object_type_definition(p)
        || is_at_directive_definition(p)
}

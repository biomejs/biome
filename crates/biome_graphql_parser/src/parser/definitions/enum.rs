use crate::parser::{
    directive::{is_at_directive, DirectiveList},
    is_at_name, parse_description,
    parse_error::expected_name,
    parse_name,
    value::{is_at_string, parse_enum_value},
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::is_at_definition;

#[inline]
pub(crate) fn parse_enum_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum_type_definition(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![enum]);

    parse_name(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    // enum values are optional
    parse_enum_values(p).ok();

    Present(m.complete(p, GRAPHQL_ENUM_TYPE_DEFINITION))
}

#[inline]
fn parse_enum_values(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum_values(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);

    EnumValueList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, GRAPHQL_ENUM_VALUES_DEFINITION))
}

#[derive(Default)]
struct EnumValueList;

impl ParseNodeList for EnumValueList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_ENUM_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_enum_value_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_enum_values_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &EnumValueListParseRecovery, expected_name)
    }
}

struct EnumValueListParseRecovery;

impl ParseRecovery for EnumValueListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // After a enum definition is a new type definition so it's safe to
        // assume any name we see before a new type definition is a enum
        // value
        is_at_name(p) || is_at_enum_values_end(p)
    }
}

#[inline]
pub(crate) fn parse_enum_value_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum_value(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    parse_enum_value(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_ENUM_VALUE_DEFINITION))
}

#[inline]
pub(crate) fn is_at_enum_type_definition(p: &mut GraphqlParser) -> bool {
    p.at(T![enum]) || (is_at_string(p) && p.nth_at(1, T![enum]))
}

#[inline]
fn is_at_enum_values(p: &mut GraphqlParser) -> bool {
    p.at(T!['{'])
    // After an enum definition is a new type definition
    // so it's safe to assume any name we see before a new type definition is
    // an enum value
    || is_at_name(p)
    || (is_at_string(p) && p.nth_at(1, GRAPHQL_NAME))
}

#[inline]
fn is_at_enum_value(p: &mut GraphqlParser) -> bool {
    is_at_name(p) || (is_at_string(p) && p.nth_at(1, GRAPHQL_NAME)) || is_at_directive(p)
}

#[inline]
fn is_at_enum_values_end(p: &mut GraphqlParser) -> bool {
    p.at(T!['}']) || is_at_definition(p)
}

use crate::parser::{
    directive::{is_at_directive, DirectiveList},
    is_nth_at_name, parse_description,
    parse_error::{expected_name, expected_named_type},
    parse_name,
    r#type::parse_named_type,
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::{
    parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, Parser,
};

use super::{
    field::{is_at_fields, is_at_fields_end, parse_fields_definition},
    is_at_definition,
};

#[inline]
pub(super) fn parse_interface_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![interface]);

    parse_name(p).or_add_diagnostic(p, expected_name);

    // implements interface is optional
    parse_implements_interface(p).ok();

    DirectiveList.parse_list(p);

    // fields definition is optional
    parse_fields_definition(p).ok();

    Present(m.complete(p, GRAPHQL_INTERFACE_TYPE_DEFINITION))
}

#[inline]
pub(super) fn parse_implements_interface(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_implements_interface(p) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![implements]);

    ImplementsInterfaceList.parse_list(p);

    Present(m.complete(p, GRAPHQL_IMPLEMENTS_INTERFACES))
}

#[derive(Default)]
struct ImplementsInterfaceList;

impl ParseSeparatedList for ImplementsInterfaceList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_IMPLEMENTS_INTERFACE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_named_type(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_implements_interface_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ImplementsInterfaceListParseRecovery,
            expected_named_type,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![&]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn allow_leading_seperating_element(&self) -> bool {
        true
    }
}

struct ImplementsInterfaceListParseRecovery;

impl ParseRecovery for ImplementsInterfaceListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_nth_at_name(p, 0) || p.at(T![&]) || is_at_implements_interface_end(p)
    }
}

#[inline]
fn is_at_implements_interface(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![implements])
}

#[inline]
fn is_at_implements_interface_end(p: &mut GraphqlParser<'_>) -> bool {
    is_at_directive(p) || is_at_fields(p) || is_at_fields_end(p) || is_at_definition(p)
}

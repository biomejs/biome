use crate::parser::{
    directive::DirectiveList,
    is_at_name, parse_description,
    parse_error::{expected_name, expected_named_type},
    parse_name,
    r#type::parse_named_type,
    value::is_at_string,
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::{ParseNodeList, ParseSeparatedList},
    parse_recovery::ParseRecovery,
    parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*,
    token_source::TokenSource,
    Parser,
};

use super::is_at_definition;

#[inline]
pub(crate) fn parse_union_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_union_type_definition(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![union]);

    parse_name(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    // union member types are optional
    parse_union_member_types(p).ok();

    Present(m.complete(p, GRAPHQL_UNION_TYPE_DEFINITION))
}

#[inline]
fn parse_union_member_types(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_union_member_types(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![=]);

    if p.at(T![|]) {
        p.bump(T![|]);
    }

    let position = p.source().position();
    UnionMemberTypeList.parse_list(p);

    // has not progressed, meaning no union member types were parsed
    if position == p.source().position() {
        p.error(expected_named_type(p, p.cur_range()));
    }
    Present(m.complete(p, GRAPHQL_UNION_MEMBER_TYPES))
}

#[derive(Default)]
struct UnionMemberTypeList;

impl ParseSeparatedList for UnionMemberTypeList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_UNION_MEMBER_TYPE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_named_type(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_union_member_types_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &UnionMemberListParseRecovery, expected_named_type)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}

struct UnionMemberListParseRecovery;

impl ParseRecovery for UnionMemberListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![|])
        // After a union definition is a new type definition so it's safe to
        // assume any name we see before a new type definition is a union
        // member type
        || is_at_name(p)
        || is_at_union_member_types_end(p)
    }
}

#[inline]
pub(crate) fn is_at_union_type_definition(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![union]) || (is_at_string(p) && p.nth_at(1, T![union]))
}

#[inline]
fn is_at_union_member_types(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![=])
    // missing =
    || p.at(T![|])
    // missing both = and |. After a union definition is a new type definition
    // so it's safe to assume any name we see before a new type definition is
    // a union member type
    || is_at_name(p)
}

#[inline]
fn is_at_union_member_types_end(p: &mut GraphqlParser<'_>) -> bool {
    is_at_definition(p)
}

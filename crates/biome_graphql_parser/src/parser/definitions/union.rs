use crate::parser::{
    directive::DirectiveList,
    is_nth_at_name, is_nth_at_non_kw_name, parse_binding, parse_description,
    parse_error::{expected_name, expected_named_type, expected_union_extension},
    parse_reference,
    r#type::parse_named_type,
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
    Parser,
};

use super::is_at_definition;

#[inline]
pub(crate) fn parse_union_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![union]);

    parse_binding(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    // union member types are optional
    parse_union_member_types(p).ok();

    Present(m.complete(p, GRAPHQL_UNION_TYPE_DEFINITION))
}

/// Must only be called if the next 2 token is `extend` and `union`, otherwise it will panic.
#[inline]
pub(super) fn parse_union_type_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.bump(T![union]);

    parse_reference(p).or_add_diagnostic(p, expected_name);

    let directive_list = DirectiveList.parse_list(p);
    let directive_empty = directive_list.range(p).is_empty();

    let union_members_empty = parse_union_member_types(p).is_absent();

    if directive_empty && union_members_empty {
        p.error(expected_union_extension(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_UNION_TYPE_EXTENSION))
}

#[inline]
fn parse_union_member_types(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_union_member_types(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![=]);
    p.eat(T![|]); // leading pipe separator is optional

    UnionMemberTypeList.parse_list(p);

    Present(m.complete(p, GRAPHQL_UNION_MEMBER_TYPES))
}

#[derive(Default)]
struct UnionMemberTypeList;

impl ParseSeparatedList for UnionMemberTypeList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_UNION_MEMBER_TYPE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_union_member(p)
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

    fn allow_empty(&self) -> bool {
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
        // We should only recover at a non kw name token, as a kw name token
        // could be the start of a new type definition
        || is_nth_at_non_kw_name(p, 0)
        || is_at_union_member_types_end(p)
    }
}

#[inline]
fn parse_union_member(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_union_member(p) {
        return Absent;
    }

    parse_named_type(p)
}

/// We must enforce either a `=`, `|`, or a non kw name token to be present, as
/// a union member can be keyword, which could be the start of a new type
/// definition
/// ```graphql
/// union FirstUnion
/// union MyUnion = String
/// ```
#[inline]
fn is_at_union_member_types(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![=]) || p.at(T![|]) || is_nth_at_non_kw_name(p, 0)
}

#[inline]
fn is_at_union_member(p: &mut GraphqlParser<'_>) -> bool {
    is_nth_at_non_kw_name(p, 0)
    // if this token is a keyword and the next token is a non keyword name,
    // it's moke likely that this is a new type definition
    // 2 consecutive kw is allowed, as one kw would be a valid type name and
    // the other the start of a new type definition
    || (is_nth_at_name(p, 0) && !is_nth_at_non_kw_name(p, 1))
}

#[inline]
fn is_at_union_member_types_end(p: &mut GraphqlParser<'_>) -> bool {
    is_at_definition(p)
}

/// Parse all inpul value
/// https://spec.graphql.org/October2021/#sec-Input-Values
use crate::parser::GraphqlParser;
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, token_set, Parser, TokenSet,
};

use super::{
    argument::is_at_argument_list_end,
    is_nth_at_name,
    parse_error::{expected_object_field, expected_value},
    parse_literal_name,
    variable::{is_at_variable, parse_variable_reference},
};

const BOOLEAN_VALUE_SET: TokenSet<GraphqlSyntaxKind> = token_set![TRUE_KW, FALSE_KW];

struct ListValueElementListParseRecovery;

impl ParseRecovery for ListValueElementListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS_VALUE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_value(p) || is_at_list_end(p)
    }
}

#[derive(Default)]
struct ListValueElementList;

impl ParseNodeList for ListValueElementList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_LIST_VALUE_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_list_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &ListValueElementListParseRecovery, expected_value)
    }
}

struct ObjectValueMemberListParseRecovery;

impl ParseRecovery for ObjectValueMemberListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_OBJECT_FIELD;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_nth_at_name(p, 0) || is_at_object_end(p)
    }
}

#[derive(Default)]
struct ObjectValueMemberList;

impl ParseNodeList for ObjectValueMemberList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_OBJECT_VALUE_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_object_field(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_object_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ObjectValueMemberListParseRecovery,
            expected_object_field,
        )
    }
}

#[inline]
pub(crate) fn parse_default_value(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![=]);
    parse_value(p).or_add_diagnostic(p, expected_value);
    Present(m.complete(p, GRAPHQL_DEFAULT_VALUE))
}

#[inline]
pub(crate) fn parse_value(p: &mut GraphqlParser) -> ParsedSyntax {
    if is_at_variable(p) {
        parse_variable_reference(p)
    } else if is_at_int(p) {
        parse_int(p)
    } else if is_at_float(p) {
        parse_float(p)
    } else if is_at_string(p) {
        parse_string(p)
    } else if is_at_boolean(p) {
        parse_boolean(p)
    } else if is_at_null(p) {
        parse_null(p)
    } else if is_at_enum(p) {
        parse_enum_value(p)
    } else if is_at_list(p) {
        parse_list(p)
    } else if is_at_object(p) {
        parse_object(p)
    } else {
        return Absent;
    }
}

#[inline]
fn parse_int(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_int(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(GRAPHQL_INT_LITERAL);
    Present(m.complete(p, GRAPHQL_INT_VALUE))
}

#[inline]
fn parse_float(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_float(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(GRAPHQL_FLOAT_LITERAL);
    Present(m.complete(p, GRAPHQL_FLOAT_VALUE))
}

#[inline]
pub(crate) fn parse_string(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_string(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(GRAPHQL_STRING_LITERAL);
    Present(m.complete(p, GRAPHQL_STRING_VALUE))
}

#[inline]
fn parse_boolean(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_boolean(p) {
        return Absent;
    }
    let m = p.start();
    p.bump_ts(BOOLEAN_VALUE_SET);
    Present(m.complete(p, GRAPHQL_BOOLEAN_VALUE))
}

#[inline]
fn parse_null(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_null(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![null]);
    Present(m.complete(p, GRAPHQL_NULL_VALUE))
}

#[inline]
fn parse_enum_value(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum(p) {
        return Absent;
    }
    let m = p.start();
    parse_literal_name(p).ok();
    Present(m.complete(p, GRAPHQL_ENUM_VALUE))
}

#[inline]
fn parse_list(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_list(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['[']);
    ListValueElementList.parse_list(p);
    p.expect(T![']']);
    Present(m.complete(p, GRAPHQL_LIST_VALUE))
}

#[inline]
fn parse_object(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_object(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    ObjectValueMemberList.parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, GRAPHQL_OBJECT_VALUE))
}

#[inline]
fn parse_object_field(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_object_field(p) {
        return Absent;
    }
    let m = p.start();
    parse_literal_name(p).ok();
    p.expect(T![:]);
    parse_value(p).or_add_diagnostic(p, expected_value);
    Present(m.complete(p, GRAPHQL_OBJECT_FIELD))
}

#[inline]
fn is_at_value(p: &mut GraphqlParser) -> bool {
    is_at_variable(p)
        || is_at_int(p)
        || is_at_float(p)
        || is_at_string(p)
        || is_at_boolean(p)
        || is_at_null(p)
        || is_at_enum(p)
        || is_at_list(p)
        || is_at_object(p)
}

#[inline]
fn is_at_int(p: &GraphqlParser) -> bool {
    p.at(GRAPHQL_INT_LITERAL)
}

#[inline]
fn is_at_float(p: &GraphqlParser) -> bool {
    p.at(GRAPHQL_FLOAT_LITERAL)
}

#[inline]
pub(crate) fn is_at_string(p: &GraphqlParser) -> bool {
    p.at(GRAPHQL_STRING_LITERAL)
}

#[inline]
fn is_at_boolean(p: &GraphqlParser) -> bool {
    p.at_ts(BOOLEAN_VALUE_SET)
}

#[inline]
fn is_at_null(p: &GraphqlParser) -> bool {
    p.at(T![null])
}

/// https://spec.graphql.org/October2021/#EnumValue
#[inline]
fn is_at_enum(p: &mut GraphqlParser) -> bool {
    is_nth_at_name(p, 0) && !p.at(TRUE_KW) && !p.at(FALSE_KW) && !p.at(T![null])
}

#[inline]
fn is_at_list(p: &GraphqlParser) -> bool {
    p.at(T!['['])
}

#[inline]
fn is_at_list_end(p: &mut GraphqlParser) -> bool {
    p.at(T![']'])
    // at next argument
    || p.nth_at(1, T![:])
    // value is only used in argument
    || is_at_argument_list_end(p)
}

#[inline]
fn is_at_object(p: &GraphqlParser) -> bool {
    p.at(T!['{'])
}

#[inline]
fn is_at_object_field(p: &mut GraphqlParser) -> bool {
    is_nth_at_name(p, 0)
}

#[inline]
fn is_at_object_end(p: &mut GraphqlParser) -> bool {
    p.at(T!['}'])
    // value is only used in argument
    || is_at_argument_list_end(p)
}

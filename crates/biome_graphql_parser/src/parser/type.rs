use crate::parser::{parse_name, GraphqlParser};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, Parser};

use super::{
    is_at_name,
    parse_error::{expected_named_or_list_type, expected_type},
};

#[inline]
pub(crate) fn parse_type(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    let node = if is_at_list_type(p) {
        parse_list_type(p)
    } else {
        parse_named_type(p)
    };
    if p.at(T![!]) {
        p.bump(T![!]);
        // case like `a: !`
        node.or_add_diagnostic(p, expected_named_or_list_type);
        Present(m.complete(p, GRAPHQL_NON_NULL_TYPE))
    } else {
        m.abandon(p);
        node
    }
}

#[inline]
fn parse_list_type(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['[']);
    parse_type(p).or_add_diagnostic(p, expected_type);
    p.expect(T![']']);

    Present(m.complete(p, GRAPHQL_LIST_TYPE))
}

#[inline]
pub(crate) fn parse_named_type(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_name(p) {
        return Absent;
    }
    let m = p.start();
    parse_name(p).ok();

    Present(m.complete(p, GRAPHQL_NAMED_TYPE))
}

#[inline]
fn is_at_list_type(p: &GraphqlParser) -> bool {
    p.at(T!['['])
}

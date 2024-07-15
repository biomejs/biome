use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, CompletedMarker, Parser,
};

use super::{
    parse_error::{expected_named_or_list_type, expected_type},
    parse_reference, GraphqlParser,
};

#[inline]
pub(crate) fn parse_type(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    let node = if is_at_list_type(p) {
        Present(parse_list_type(p))
    } else {
        parse_named_type(p)
    };
    if p.at(T![!]) {
        p.bump(T![!]);
        // cases like `a: !`, having `!` without a type is invalid
        node.or_add_diagnostic(p, expected_named_or_list_type);
        Present(m.complete(p, GRAPHQL_NON_NULL_TYPE))
    } else {
        m.abandon(p);
        node
    }
}

#[inline]
fn parse_list_type(p: &mut GraphqlParser) -> CompletedMarker {
    // without '[' this type will be a named type
    // so we expect '[' to be present for list type
    let m = p.start();
    p.expect(T!['[']);
    parse_type(p).or_add_diagnostic(p, expected_type);
    p.expect(T![']']);

    m.complete(p, GRAPHQL_LIST_TYPE)
}

#[inline]
pub(crate) fn parse_named_type(p: &mut GraphqlParser) -> ParsedSyntax {
    parse_reference(p)
}

#[inline]
fn is_at_list_type(p: &GraphqlParser) -> bool {
    p.at(T!['['])
}

use crate::parser::{parse_name, GraphqlParser};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::{
    is_at_name,
    parse_error::{expected_argument, expected_value},
    value::parse_value,
};

struct ArgumentListParseRecovery;

impl ParseRecovery for ArgumentListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_ARGUMENT;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_name(p)
    }
}

#[derive(Default)]
struct ArgumentList;

impl ParseNodeList for ArgumentList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_ARGUMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_argument(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &ArgumentListParseRecovery, expected_argument)
    }
}

#[inline]
pub(crate) fn parse_arguments(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    ArgumentList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, GRAPHQL_ARGUMENTS))
}

#[inline]
fn parse_argument(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_name(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).ok();
    p.expect(T![:]);
    parse_value(p).or_add_diagnostic(p, expected_value);

    Present(m.complete(p, GRAPHQL_ARGUMENT))
}

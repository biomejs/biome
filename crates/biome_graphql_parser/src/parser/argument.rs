use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::{
    directive::is_at_directive,
    is_nth_at_name,
    parse_error::{expected_argument, expected_value},
    parse_literal_name,
    value::parse_value,
    GraphqlParser,
};

struct ArgumentListParseRecovery;

impl ParseRecovery for ArgumentListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_ARGUMENT;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_nth_at_name(p, 0) || is_at_argument_list_end(p)
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
        is_at_argument_list_end(p)
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
    if !is_nth_at_name(p, 0) {
        return Absent;
    }

    let m = p.start();

    // name is checked for in `is_at_name`
    parse_literal_name(p).ok();
    p.expect(T![:]);
    parse_value(p).or_add_diagnostic(p, expected_value);

    Present(m.complete(p, GRAPHQL_ARGUMENT))
}

/// Arguments are only allowed in the following cases:
/// - Inside a selection set
/// - In a directive
#[inline]
pub(crate) fn is_at_argument_list_end(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![')'])
    // at the start af a new arguments list
    || p.at(T!['('])
    // at the start of a new directive
    || is_at_directive(p)
    // if we can't find any of the above, we can't be sure if we're outside of
    // an argument list
}

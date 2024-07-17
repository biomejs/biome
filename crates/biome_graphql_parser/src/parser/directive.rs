use crate::parser::{parse_error::expected_name, GraphqlParser};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::{argument::parse_arguments, parse_error::expected_directive, parse_reference};
struct DirectiveListParseRecovery;

impl ParseRecovery for DirectiveListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_DIRECTIVE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_directive(p)
    }
}

#[derive(Default)]
pub(crate) struct DirectiveList;

impl ParseNodeList for DirectiveList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_DIRECTIVE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_directive(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_directive(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &DirectiveListParseRecovery, expected_directive)
    }
}

#[inline]
pub(crate) fn parse_directive(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![@]);
    parse_reference(p).or_add_diagnostic(p, expected_name);

    // arguments are optional
    parse_arguments(p).ok();

    Present(m.complete(p, GRAPHQL_DIRECTIVE))
}

#[inline]
pub(crate) fn is_at_directive(p: &GraphqlParser<'_>) -> bool {
    p.at(T![@])
}

use crate::parser::GraphqlParser;
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, Parser};

use super::{parse_error::expected_name, parse_name};

#[inline]
pub(crate) fn parse_variable(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_variable(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![$]);
    parse_name(p).or_add_diagnostic(p, expected_name);

    Present(m.complete(p, GRAPHQL_VARIABLE))
}

#[inline]
pub(crate) fn is_at_variable(p: &GraphqlParser) -> bool {
    p.at(T![$])
}

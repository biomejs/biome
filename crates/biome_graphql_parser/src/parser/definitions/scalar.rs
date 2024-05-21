use crate::parser::{
    directive::DirectiveList,
    parse_description,
    parse_error::{expected_directive, expected_name},
    parse_name, GraphqlParser,
};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parse_lists::ParseNodeList, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*,
    token_source::TokenSource, Parser,
};

#[inline]
pub(crate) fn parse_scalar_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    // description is optional
    parse_description(p).ok();

    p.bump(T![scalar]);

    parse_name(p).or_add_diagnostic(p, expected_name);
    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_SCALAR_TYPE_DEFINITION))
}

#[inline]
pub(crate) fn parse_scalar_type_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.bump(T![scalar]);

    parse_name(p).or_add_diagnostic(p, expected_name);
    let pos = p.source().position();
    DirectiveList.parse_list(p);
    let directive_empty = p.source().position() == pos;
    if directive_empty {
        p.error(expected_directive(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_SCALAR_TYPE_EXTENSION))
}

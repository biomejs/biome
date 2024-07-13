use crate::parser::{
    directive::DirectiveList,
    parse_binding,
    parse_error::{expected_name, expected_named_type, fragment_name_must_not_be_on},
    parse_reference, GraphqlParser,
};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parse_lists::ParseNodeList, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*,
    CompletedMarker, Parser,
};

use super::operation::parse_selection_set;

#[inline]
pub(crate) fn parse_fragment_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![fragment]);

    if p.at(T![on]) {
        p.error(fragment_name_must_not_be_on(p, p.cur_range()));
    }
    parse_binding(p).or_add_diagnostic(p, expected_name);
    parse_type_condition(p);

    DirectiveList.parse_list(p);
    parse_selection_set(p).ok();

    Present(m.complete(p, GRAPHQL_FRAGMENT_DEFINITION))
}

#[inline]
pub(crate) fn parse_type_condition(p: &mut GraphqlParser) -> CompletedMarker {
    let m = p.start();
    p.expect(T![on]);
    parse_reference(p).or_add_diagnostic(p, expected_named_type);
    m.complete(p, GRAPHQL_TYPE_CONDITION)
}

#[inline]
pub(crate) fn is_at_type_condition(p: &GraphqlParser<'_>) -> bool {
    p.at(T![on])
}

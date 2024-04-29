use crate::parser::{
    directive::DirectiveList, parse_description, parse_error::expected_name, parse_name,
    value::is_at_string, GraphqlParser,
};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parse_lists::ParseNodeList, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, Parser,
};

use super::{field::parse_fields_definition, interface::parse_implements_interface};

#[inline]
pub(crate) fn parse_object_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_object_type_definition(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![type]);

    parse_name(p).or_add_diagnostic(p, expected_name);

    // implements interface is optional
    parse_implements_interface(p).ok();
    DirectiveList.parse_list(p);

    // fields definition is optional
    parse_fields_definition(p).ok();

    Present(m.complete(p, GRAPHQL_OBJECT_TYPE_DEFINITION))
}

#[inline]
pub(crate) fn is_at_object_type_definition(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![type]) || (is_at_string(p) && p.nth_at(1, T![type]))
}

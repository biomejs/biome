use crate::parser::{
    directive::DirectiveList,
    parse_binding, parse_description,
    parse_error::{expected_name, expected_object_extension},
    parse_reference, GraphqlParser,
};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parse_lists::ParseNodeList, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*,
    token_source::TokenSource, Parser,
};

use super::{field::parse_fields_definition, interface::parse_implements_interface};

#[inline]
pub(crate) fn parse_object_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![type]);

    parse_binding(p).or_add_diagnostic(p, expected_name);

    // implements interface is optional
    parse_implements_interface(p).ok();
    DirectiveList.parse_list(p);

    // fields definition is optional
    parse_fields_definition(p).ok();

    Present(m.complete(p, GRAPHQL_OBJECT_TYPE_DEFINITION))
}

/// Must only be called if the next 2 token is `extend` and `type`, otherwise it will panic.
#[inline]
pub(crate) fn parse_object_type_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.bump(T![type]);

    parse_reference(p).or_add_diagnostic(p, expected_name);

    let implements_interface_empty = parse_implements_interface(p).is_absent();

    let pos = p.source().position();
    DirectiveList.parse_list(p);
    let directive_empty = p.source().position() == pos;

    let fields_definition_empty = parse_fields_definition(p).is_absent();

    if directive_empty && implements_interface_empty && fields_definition_empty {
        p.error(expected_object_extension(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_OBJECT_TYPE_EXTENSION))
}

use crate::parser::{
    directive::DirectiveList,
    parse_binding, parse_description,
    parse_error::{expected_input_object_extension, expected_name},
    parse_reference, GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::field::{is_at_input_value_definition, parse_input_value_definition};

#[inline]
pub(crate) fn parse_input_object_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![input]);

    parse_binding(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    // input fields are optional
    parse_input_fields_definition(p).ok();

    Present(m.complete(p, GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION))
}

/// Must only be called if the next 2 token is `extend` and `input`, otherwise it will panic.
#[inline]
pub(crate) fn parse_input_object_type_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.expect(T![input]);

    parse_reference(p).or_add_diagnostic(p, expected_name);

    let directive_list = DirectiveList.parse_list(p);
    let directive_empty = directive_list.range(p).is_empty();

    let input_fields_empty = parse_input_fields_definition(p).is_absent();

    if directive_empty && input_fields_empty {
        p.error(expected_input_object_extension(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION))
}

#[inline]
fn parse_input_fields_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_input_fields_definition(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);

    InputFieldList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, GRAPHQL_INPUT_FIELDS_DEFINITION))
}

#[derive(Default)]
struct InputFieldList;

impl ParseNodeList for InputFieldList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_INPUT_FIELD_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_input_value_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_input_fields_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &InputFieldListParseRecovery, expected_name)
    }
}

struct InputFieldListParseRecovery;

impl ParseRecovery for InputFieldListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_input_value_definition(p) || is_input_fields_end(p)
    }
}

#[inline]
fn is_at_input_fields_definition(p: &mut GraphqlParser) -> bool {
    p.at(T!['{'])
    // missing opening brace
    || is_at_input_value_definition(p)
}

#[inline]
fn is_input_fields_end(p: &mut GraphqlParser) -> bool {
    p.at(T!['}'])
    // start of next definition body, since input fields can't be nested
    || p.at(T!['{'])
}

use crate::parser::{
    directive::DirectiveList,
    is_at_name, parse_description,
    parse_error::{expected_field_definition, expected_name, expected_type},
    parse_name,
    r#type::parse_type,
    value::{is_at_string, parse_default_value},
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, Parser,
};

use super::is_at_definition;

#[inline]
pub(super) fn parse_fields_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_fields(p) {
        return Absent;
    }
    let m = p.start();

    p.expect(T!['{']);
    FieldDefinitionList.parse_list(p);

    p.expect(T!['}']);
    Present(m.complete(p, GRAPHQL_FIELDS_DEFINITION))
}

#[derive(Default)]
struct FieldDefinitionList;

impl ParseNodeList for FieldDefinitionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_FIELD_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_field_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_fields_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &FieldDefinitionListParseRecovery,
            expected_field_definition,
        )
    }
}

struct FieldDefinitionListParseRecovery;

impl ParseRecovery for FieldDefinitionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_field(p) || is_at_fields_end(p)
    }
}

#[inline]
fn parse_field_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_field(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();
    parse_name(p).or_add_diagnostic(p, expected_name);

    // arguments are optional
    parse_arguments_definition(p).ok();
    p.expect(T![:]);

    parse_type(p).or_add_diagnostic(p, expected_type);

    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_FIELD_DEFINITION))
}

#[inline]
fn parse_arguments_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_arguments_definition(p) {
        return Absent;
    }
    let m = p.start();

    p.expect(T!['(']);
    ArgumentDefinitionList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, GRAPHQL_ARGUMENTS_DEFINITION))
}

#[derive(Default)]
struct ArgumentDefinitionList;
impl ParseNodeList for ArgumentDefinitionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_ARGUMENT_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_input_value_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_arguments_definition_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ArgumentDefinitionListParseRecovery,
            expected_field_definition,
        )
    }
}

struct ArgumentDefinitionListParseRecovery;
impl ParseRecovery for ArgumentDefinitionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_field(p) || is_at_fields_end(p)
    }
}

#[inline]
pub(super) fn parse_input_value_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_input_value_definition(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();
    parse_name(p).or_add_diagnostic(p, expected_name);

    p.expect(T![:]);

    parse_type(p).or_add_diagnostic(p, expected_type);

    // default value is optional
    parse_default_value(p).ok();

    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_INPUT_VALUE_DEFINITION))
}

#[inline]
pub(super) fn is_at_fields(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T!['{'])
    // missing opening brace
    || is_at_field(p)
}

#[inline]
pub(super) fn is_at_fields_end(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T!['}']) || is_at_definition(p)
}

#[inline]
fn is_at_field(p: &mut GraphqlParser<'_>) -> bool {
    (is_at_name(p) && p.nth_at(1, T![:]))
    // with arguments
    || (is_at_name(p) && p.nth_at(1, T!['(']))
    || (is_at_string(p) && p.nth_at(1, GRAPHQL_NAME)) && p.nth_at(2, T![:])
    // missing name
    || p.at(T![:])
}

#[inline]
pub(super) fn is_at_input_value_definition(p: &mut GraphqlParser<'_>) -> bool {
    (is_at_name(p) && p.nth_at(1, T![:]))
    || (is_at_string(p) && p.nth_at(1, GRAPHQL_NAME) && p.nth_at(2, T![:]))
    // missing name
    || p.at(T![:])
    || (is_at_string(p) && p.nth_at(1, T![:]))
    // missing colon: `name String`
    || (is_at_name(p) && p.nth_at(1, GRAPHQL_NAME))
}

/// We must enforce that the arguments definition is always opened with a `(` token.
/// Otherwise, we might end up parsing a field definition as an input value definition.
/// For example:
/// ```graphql
/// name : String = "") : String
/// ```
/// In this case, the opening parenthesis is missing, the name token of an input value definition
/// is also missing. It would be to complex to disambiguate input value definitions from field.
#[inline]
fn is_at_arguments_definition(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T!['('])
}

#[inline]
fn is_at_arguments_definition_end(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T![')']) || is_at_fields_end(p)
}

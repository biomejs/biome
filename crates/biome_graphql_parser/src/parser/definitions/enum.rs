use crate::parser::{
    directive::{is_at_directive, DirectiveList},
    is_nth_at_name, is_nth_at_non_kw_name, parse_binding, parse_description,
    parse_error::{expected_enum_extension, expected_name},
    parse_literal_name, parse_reference,
    value::is_at_string,
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

#[inline]
pub(crate) fn parse_enum_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![enum]);

    parse_binding(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    // enum values are optional
    parse_enum_values(p).ok();

    Present(m.complete(p, GRAPHQL_ENUM_TYPE_DEFINITION))
}

/// Must only be called if the next 2 token is `extend` and `enum`, otherwise it will panic.
#[inline]
pub(crate) fn parse_enum_type_extension(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![extend]);
    p.expect(T![enum]);

    parse_reference(p).or_add_diagnostic(p, expected_name);

    let directive_list = DirectiveList.parse_list(p);
    let directive_empty = directive_list.range(p).is_empty();

    let enum_values_empty = parse_enum_values(p).is_absent();

    if directive_empty && enum_values_empty {
        p.error(expected_enum_extension(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_ENUM_TYPE_EXTENSION))
}

#[inline]
fn parse_enum_values(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum_values(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);

    EnumValueList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, GRAPHQL_ENUM_VALUES_DEFINITION))
}

#[derive(Default)]
struct EnumValueList;

impl ParseNodeList for EnumValueList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_ENUM_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_enum_value_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_enum_values_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &EnumValueListParseRecovery, expected_name)
    }
}

struct EnumValueListParseRecovery;

impl ParseRecovery for EnumValueListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_nth_at_name(p, 0) || is_at_enum_values_end(p)
    }
}

#[inline]
pub(crate) fn parse_enum_value_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_enum_value(p) {
        return Absent;
    }
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    parse_literal_name(p).or_add_diagnostic(p, expected_name);

    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_ENUM_VALUE_DEFINITION))
}

/// Either a `{`, `|`, or a non kw name token must be present, else this is
/// likely the start of a new type definition
#[inline]
fn is_at_enum_values(p: &mut GraphqlParser) -> bool {
    p.at(T!['{'])
        || is_nth_at_non_kw_name(p, 0)
        || (is_at_string(p) && is_nth_at_non_kw_name(p, 1))
        || is_at_directive(p)
}

#[inline]
fn is_at_enum_value(p: &mut GraphqlParser) -> bool {
    is_nth_at_name(p, 0) || (is_at_string(p) && is_nth_at_name(p, 1)) || is_at_directive(p)
}

/// Any name except `true`, `false`, `null`, is a valid enum value
/// so if the current token is still a name, it's still a valid enum value
#[inline]
fn is_at_enum_values_end(p: &mut GraphqlParser) -> bool {
    if p.at(T!['}']) {
        return true;
    }
    // whether the current token is a description
    // the directive check is for missing enum value case
    if is_at_string(p) {
        !is_nth_at_name(p, 1) && !p.nth_at(1, T![@])
    } else {
        !is_nth_at_name(p, 0) && !is_at_directive(p)
    }
}

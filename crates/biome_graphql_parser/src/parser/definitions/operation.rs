use crate::parser::{
    argument::parse_arguments,
    directive::{is_at_directive, DirectiveList},
    is_at_name,
    parse_error::{
        expected_any_selection, expected_name, expected_type, expected_value,
        expected_variable_definition,
    },
    parse_name,
    r#type::parse_type,
    value::parse_value,
    variable::{is_at_variable, parse_variable},
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, token_set, Parser, TokenSet,
};

use super::{
    fragment::{is_at_type_condition, parse_type_condition},
    is_at_definition,
};

pub(crate) const OPERATION_TYPE: TokenSet<GraphqlSyntaxKind> =
    token_set![T![query], T![mutation], T![subscription]];

#[derive(Default)]
struct SelectionList;

impl ParseNodeList for SelectionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_SELECTION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_selection(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_selection_set_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &SelectionListParseRecovery, expected_any_selection)
    }
}

struct SelectionListParseRecovery;

impl ParseRecovery for SelectionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS_SELECTION;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_selection(p) || is_at_selection_set_end(p)
    }
}

#[derive(Default)]
struct VariableDefinitionList;

impl ParseNodeList for VariableDefinitionList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_VARIABLE_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_variable_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_variable_definitions_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &VariableDefinitionListParseRecovery,
            expected_variable_definition,
        )
    }
}

struct VariableDefinitionListParseRecovery;

impl ParseRecovery for VariableDefinitionListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_variable(p) || is_at_variable_definitions_end(p)
    }
}

/// https://spec.graphql.org/October2021/#sec-Language.Operations.Query-shorthand
#[inline]
pub(crate) fn parse_operation_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_operation(p) {
        return Absent;
    }

    if is_at_selection_set(p) {
        return parse_selection_set(p);
    }

    let m = p.start();
    {
        let m = p.start();
        p.bump_ts(OPERATION_TYPE);
        m.complete(p, GRAPHQL_OPERATION_TYPE);
    }

    // we don't need diagnostic here, because name is optional
    parse_name(p).ok();
    // we don't need diagnostic here, because variable definitions are optional
    parse_variable_definitions(p).ok();

    DirectiveList.parse_list(p);
    parse_selection_set(p).ok();

    Present(m.complete(p, GRAPHQL_OPERATION_DEFINITION))
}

#[inline]
pub(crate) fn parse_selection_set(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['{']);
    SelectionList.parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, GRAPHQL_SELECTION_SET))
}

#[inline]
fn parse_selection(p: &mut GraphqlParser) -> ParsedSyntax {
    if is_at_field(p) {
        parse_field(p)
    } else if is_at_fragment(p) {
        parse_fragment(p)
    } else {
        Absent
    }
}

#[inline]
fn parse_field(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_field(p) {
        return Absent;
    }
    let m = p.start();

    // alias is optional, so if there is a colon, we parse it as an alias
    // otherwise we parse it as a normal field name
    if p.nth_at(1, T![:]) {
        let m = p.start();

        // name is checked for in `is_at_field`
        parse_name(p).ok();
        p.bump(T![:]);
        m.complete(p, GRAPHQL_ALIAS);

        parse_name(p).or_add_diagnostic(p, expected_name);
    } else {
        // name is checked for in `is_at_field`
        parse_name(p).ok();
    }

    // arguments are optional
    parse_arguments(p).ok();
    DirectiveList.parse_list(p);

    if is_at_selection_set(p) {
        parse_selection_set(p).ok();
    }
    Present(m.complete(p, GRAPHQL_FIELD))
}

#[inline]
fn parse_fragment(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_fragment(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(DOT3);
    if is_at_name(p) {
        // name is checked for in `is_at_name`
        parse_name(p).ok();
        DirectiveList.parse_list(p);
        Present(m.complete(p, GRAPHQL_FRAGMENT_SPREAD))
    } else {
        if is_at_type_condition(p) {
            parse_type_condition(p);
        }
        DirectiveList.parse_list(p);
        parse_selection_set(p).ok();
        Present(m.complete(p, GRAPHQL_INLINE_FRAGMENT))
    }
}

#[inline]
fn parse_variable_definitions(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    VariableDefinitionList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, GRAPHQL_VARIABLE_DEFINITIONS))
}

#[inline]
fn parse_variable_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_variable(p) {
        return Absent;
    }

    let m = p.start();

    // variable is checked for in `is_at_variable`
    parse_variable(p).ok();
    p.expect(T![:]);
    parse_type(p).or_add_diagnostic(p, expected_type);

    // default value is optional
    parse_default_value(p).ok();
    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_VARIABLE_DEFINITION))
}

#[inline]
fn parse_default_value(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![=]);
    parse_value(p).or_add_diagnostic(p, expected_value);
    Present(m.complete(p, GRAPHQL_DEFAULT_VALUE))
}

#[inline]
pub(crate) fn is_at_operation(p: &GraphqlParser<'_>) -> bool {
    p.at_ts(OPERATION_TYPE) || is_at_selection_set(p)
}

#[inline]
fn is_at_variable_definitions_end(p: &GraphqlParser) -> bool {
    p.at(T![')']) || is_at_directive(p) || is_at_selection_set(p)
}

#[inline]
fn is_at_selection_set(p: &GraphqlParser) -> bool {
    p.at(T!['{'])
}

#[inline]
pub(crate) fn is_at_selection_set_end(p: &mut GraphqlParser) -> bool {
    // stop at closing brace or at the start of a new definition
    p.at(T!['}']) || is_at_definition(p)
}

#[inline]
fn is_at_selection(p: &GraphqlParser) -> bool {
    is_at_field(p) || is_at_fragment(p)
}

#[inline]
fn is_at_field(p: &GraphqlParser) -> bool {
    is_at_name(p)
}

#[inline]
fn is_at_fragment(p: &GraphqlParser) -> bool {
    p.at(DOT3)
}

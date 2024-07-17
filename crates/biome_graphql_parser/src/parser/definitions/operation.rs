use crate::parser::{
    argument::parse_arguments,
    directive::{is_at_directive, DirectiveList},
    is_nth_at_name, parse_binding,
    parse_error::{
        expected_any_selection, expected_name, expected_type, expected_variable,
        expected_variable_definition,
    },
    parse_literal_name, parse_reference,
    r#type::parse_type,
    value::parse_default_value,
    variable::{is_at_variable, parse_variable_binding},
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

use super::fragment::{is_at_type_condition, parse_type_condition};

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
        is_at_variable_definition(p) || is_at_variable_definitions_end(p)
    }
}

/// https://spec.graphql.org/October2021/#sec-Language.Operations.Query-shorthand
#[inline]
pub(crate) fn parse_operation_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    {
        let m = p.start();
        p.bump_ts(OPERATION_TYPE);
        m.complete(p, GRAPHQL_OPERATION_TYPE);
    }

    // we don't need diagnostic here, because name is optional
    parse_binding(p).ok();
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
    if is_at_alias(p) {
        let m = p.start();
        if p.at(T![:]) {
            p.error(expected_name(p, p.cur_range()));
        } else if is_nth_at_name(p, 0) {
            // parse alias
            parse_literal_name(p).ok();
        } else {
            p.error(expected_name(p, p.cur_range()));
            p.bump_any();
        }

        p.bump(T![:]);
        m.complete(p, GRAPHQL_ALIAS);

        parse_literal_name(p).or_add_diagnostic(p, expected_name);
    } else {
        parse_literal_name(p).or_add_diagnostic(p, expected_name);
    }

    // arguments are optional
    parse_arguments(p).ok();
    DirectiveList.parse_list(p);

    if p.at(T!['{']) {
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
    p.expect(DOT3);
    if is_nth_at_name(p, 0) && !p.nth_at(0, T![on]) {
        // name is checked for in `is_at_name`
        parse_reference(p).ok();
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
    if !is_at_variable_definitions(p) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['(']);
    VariableDefinitionList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, GRAPHQL_VARIABLE_DEFINITIONS))
}

#[inline]
fn parse_variable_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_variable_definition(p) {
        return Absent;
    }

    let m = p.start();

    // Malformed variable
    if !is_at_variable(p) && p.nth_at(1, T![:]) {
        p.error(expected_variable(p, p.cur_range()));
        p.bump_any()
    } else {
        parse_variable_binding(p).or_add_diagnostic(p, expected_variable);
    }
    p.expect(T![:]);
    parse_type(p).or_add_diagnostic(p, expected_type);

    // default value is optional
    parse_default_value(p).ok();
    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_VARIABLE_DEFINITION))
}

#[inline]
fn is_at_variable_definitions(p: &mut GraphqlParser) -> bool {
    p.at(T!['('])
    // missing opening parenthesis
    || is_at_variable_definition(p)
}

#[inline]
fn is_at_variable_definitions_end(p: &GraphqlParser) -> bool {
    p.at(T![')'])
    || is_at_directive(p)
    // At the start of a selection set
    || p.at(T!('{'))
}

#[inline]
fn is_at_variable_definition(p: &mut GraphqlParser) -> bool {
    is_at_variable(p)
    // malformed variable
    || (is_nth_at_name(p, 0) && p.nth_at(1, T![:]))
    // malformed variable,but not inside selection set
    || (p.nth_at(1, T![:]) && !p.at(T!['{']))
    // missing entire variable
    || p.at(T![:])
}

// Since keywords are valid names, we could only be sure that we are at the end
// of a selection set if we are at a closing curly brace
#[inline]
fn is_at_selection_set_end(p: &mut GraphqlParser) -> bool {
    p.at(T!['}'])
}

#[inline]
fn is_at_selection(p: &mut GraphqlParser) -> bool {
    is_at_field(p) || is_at_fragment(p)
}

#[inline]
fn is_at_field(p: &mut GraphqlParser) -> bool {
    is_nth_at_name(p, 0) || is_at_alias(p)
}

#[inline]
fn is_at_fragment(p: &GraphqlParser) -> bool {
    p.at(DOT3) || p.at(T![on])
}

#[inline]
fn is_at_alias(p: &mut GraphqlParser) -> bool {
    p.nth_at(1, T![:])
    // an alias, but missing alias name
    || p.at(T![:])
}

use crate::parser::{
    argument::parse_arguments,
    directive::DirectiveList,
    is_at_name,
    parse_error::{
        expected_any_selection, expected_named_type, expected_selection_set, expected_type,
        expected_value, expected_variable_definition,
    },
    parse_name,
    r#type::{parse_named_type, parse_type},
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

const OPERATION_TYPE: TokenSet<GraphqlSyntaxKind> =
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
        p.at(T!['}'])
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
        is_at_selection(p)
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
        p.at(T![')'])
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
    const RECOVERED_KIND: Self::Kind = GRAPHQL_VARIABLE_DEFINITION;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_variable(p)
    }
}

/// https://spec.graphql.org/October2021/#sec-Language.Operations.Query-shorthand
#[inline]
pub(crate) fn parse_operation_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_operation(p) {
        return Absent;
    }

    if !p.at_ts(OPERATION_TYPE) {
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
    parse_selection_set(p).or_add_diagnostic(p, expected_selection_set);

    Present(m.complete(p, GRAPHQL_OPERATION_DEFINITION))
}

#[inline]
fn parse_selection_set(p: &mut GraphqlParser) -> ParsedSyntax {
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
    let next_token = p.lookahead();
    if next_token == T![:] {
        let m = p.start();
        parse_name(p).ok();
        p.bump(T![:]);
        m.complete(p, GRAPHQL_ALIAS);
    }
    parse_name(p).ok();

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
    if !p.at(DOT3) {
        return Absent;
    }
    let m = p.start();
    p.bump(DOT3);
    if is_at_name(p) {
        parse_name(p).ok();
        DirectiveList.parse_list(p);
        Present(m.complete(p, GRAPHQL_FRAGMENT_SPREAD))
    } else {
        if p.at(T![on]) {
            let m = p.start();
            p.bump(T![on]);
            parse_named_type(p).or_add_diagnostic(p, expected_named_type);
            m.complete(p, GRAPHQL_TYPE_CONDITION);
        }
        DirectiveList.parse_list(p);
        parse_selection_set(p).or_add_diagnostic(p, expected_selection_set);
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
pub(crate) fn is_at_operation(p: &mut GraphqlParser<'_>) -> bool {
    p.at_ts(OPERATION_TYPE) || is_at_selection_set(p)
}

#[inline]
fn is_at_selection_set(p: &GraphqlParser) -> bool {
    p.at(T!['{'])
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

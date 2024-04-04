use crate::parser::{
    directive::DirectiveList,
    parse_error::{expected_any_selection, expected_name, expected_selection_set},
    parse_name, GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, token_set, Parser, TokenSet,
};

pub(crate) const OPERATION_TYPE: TokenSet<GraphqlSyntaxKind> =
    token_set![T![query], T![mutation], T![subscription]];

/// https://spec.graphql.org/October2021/#sec-Language.Operations.Query-shorthand
#[inline]
pub(crate) fn parse_operation_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    if !is_at_operation(p) {
        return Absent;
    }

    if p.at_ts(OPERATION_TYPE) {
        // TODO: parse variables
        let m = p.start();
        {
            let m = p.start();
            p.bump_ts(OPERATION_TYPE);
            m.complete(p, GRAPHQL_OPERATION_TYPE);
        }

        // we don't need diagnostic here, because name is optional
        parse_name(p).ok();

        DirectiveList::new().parse_list(p);
        parse_selection_set(p).or_add_diagnostic(p, expected_selection_set);

        Present(m.complete(p, GRAPHQL_OPERATION_DEFINITION))
    } else {
        parse_selection_set(p)
    }
}

#[inline]
pub(crate) fn is_at_operation(p: &mut GraphqlParser<'_>) -> bool {
    p.at_ts(OPERATION_TYPE) || is_at_selection_set(p)
}

#[inline]
fn parse_selection_set(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['{']);
    SelectionList::new().parse_list(p);
    p.bump(T!['}']);
    Present(m.complete(p, GRAPHQL_SELECTION_SET))
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

pub(crate) struct SelectionList;

impl SelectionList {
    pub(crate) fn new() -> Self {
        Self
    }
}

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

#[inline]
fn parse_selection(p: &mut GraphqlParser) -> ParsedSyntax {
    // TODO: parse any selection
    match p.cur() {
        DOT3 => todo!(),
        _ if is_at_field(p) => parse_field(p),
        _ => Absent,
    }
}

#[inline]
fn parse_field(p: &mut GraphqlParser) -> ParsedSyntax {
    // TODO: parse alias, arguments, nested selection set
    let m = p.start();
    parse_name(p).or_add_diagnostic(p, expected_name);
    DirectiveList::new().parse_list(p);
    Present(m.complete(p, GRAPHQL_FIELD))
}

#[inline]
pub(crate) fn is_at_selection_set(p: &mut GraphqlParser<'_>) -> bool {
    p.at(T!['{'])
}

#[inline]
pub(crate) fn is_at_selection(p: &mut GraphqlParser<'_>) -> bool {
    // TODO: any selection
    is_at_field(p)
}

#[inline]
pub(crate) fn is_at_field(p: &mut GraphqlParser<'_>) -> bool {
    // TODO: handle arguments
    p.at(GRAPHQL_NAME)
}

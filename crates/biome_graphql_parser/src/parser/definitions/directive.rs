use crate::parser::{
    parse_binding, parse_description,
    parse_error::{expected_directive_location, expected_name},
    GraphqlParser,
};
use biome_graphql_syntax::{
    GraphqlSyntaxKind::{self, *},
    T,
};
use biome_parser::prelude::TokenSource;
use biome_parser::{
    parse_lists::ParseSeparatedList, parse_recovery::ParseRecovery, parsed_syntax::ParsedSyntax,
    prelude::ParsedSyntax::*, token_set, Parser, TokenSet,
};

use super::{field::parse_arguments_definition, is_at_definition};

const DIRECTIVE_LOCATION_SET: TokenSet<GraphqlSyntaxKind> = token_set!(
    T![UPPER_QUERY],
    T![UPPER_MUTATION],
    T![UPPER_SUBSCRIPTION],
    T![UPPER_FIELD],
    T![FRAGMENT_DEFINITION],
    T![FRAGMENT_SPREAD],
    T![INLINE_FRAGMENT],
    T![VARIABLE_DEFINITION],
    T![UPPER_SCHEMA],
    T![UPPER_SCALAR],
    T![UPPER_OBJECT],
    T![FIELD_DEFINITION],
    T![ARGUMENT_DEFINITION],
    T![UPPER_INTERFACE],
    T![UPPER_UNION],
    T![UPPER_ENUM],
    T![ENUM_VALUE],
    T![INPUT_OBJECT],
    T![INPUT_FIELD_DEFINITION]
);

#[inline]
pub(crate) fn parse_directive_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();

    // description is optional
    parse_description(p).ok();

    p.bump(T![directive]);
    p.expect(T![@]);
    parse_binding(p).or_add_diagnostic(p, expected_name);

    // arguments are optional
    parse_arguments_definition(p).ok();

    // repeatable is optional
    p.eat(T![repeatable]);
    p.expect(T![on]);

    // | is optional
    p.eat(T![|]);

    let position = p.source().position();
    DirectiveLocationList.parse_list(p);

    // has not progressed, meaning no directive locations were parsed
    if position == p.source().position() {
        p.error(expected_directive_location(p, p.cur_range()));
    }

    Present(m.complete(p, GRAPHQL_DIRECTIVE_DEFINITION))
}

#[derive(Default)]
struct DirectiveLocationList;

impl ParseSeparatedList for DirectiveLocationList {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;

    const LIST_KIND: Self::Kind = GRAPHQL_DIRECTIVE_LOCATION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_directive_location(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_definition(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &DirectiveLocationListParseRecovery,
            expected_directive_location,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}

struct DirectiveLocationListParseRecovery;

impl ParseRecovery for DirectiveLocationListParseRecovery {
    type Kind = GraphqlSyntaxKind;
    type Parser<'source> = GraphqlParser<'source>;
    const RECOVERED_KIND: Self::Kind = GRAPHQL_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(DIRECTIVE_LOCATION_SET) || is_at_definition(p)
    }
}

#[inline]
fn parse_directive_location(p: &mut GraphqlParser) -> ParsedSyntax {
    if !p.at_ts(DIRECTIVE_LOCATION_SET) {
        return Absent;
    }
    let m = p.start();
    p.bump_ts(DIRECTIVE_LOCATION_SET);
    Present(m.complete(p, GRAPHQL_DIRECTIVE_LOCATION))
}

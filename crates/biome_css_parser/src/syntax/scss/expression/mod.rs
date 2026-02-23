use crate::parser::CssParser;
use crate::syntax::parse_error::expected_scss_expression;
use crate::syntax::property::parse_generic_component_value;
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_PROPERTY_VALUE, EOF, SCSS_EXPRESSION, SCSS_EXPRESSION_ITEM_LIST,
    SCSS_LIST_EXPRESSION, SCSS_LIST_EXPRESSION_ELEMENT, SCSS_LIST_EXPRESSION_ELEMENT_LIST,
    SCSS_MAP_EXPRESSION, SCSS_MAP_EXPRESSION_PAIR, SCSS_MAP_EXPRESSION_PAIR_LIST,
    SCSS_PARENTHESIZED_EXPRESSION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress, TokenSet, token_set};

const SCSS_BINARY_OPERATOR_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![+], T![-], T![*], T![/]];

const SCSS_MAP_EXPRESSION_KEY_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![:], T![')']];
const SCSS_MAP_EXPRESSION_VALUE_END_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')']];
const SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')']];

pub(crate) const END_OF_SCSS_EXPRESSION_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')'], T![;], T!['}']].union(SCSS_BINARY_OPERATOR_TOKEN_SET);

#[inline]
pub(crate) fn parse_scss_expression(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_until(p, END_OF_SCSS_EXPRESSION_TOKEN_SET)
}

#[inline]
fn parse_scss_expression_until(p: &mut CssParser, end_ts: TokenSet<CssSyntaxKind>) -> ParsedSyntax {
    parse_scss_expression_with_options(p, end_ts.union(SCSS_BINARY_OPERATOR_TOKEN_SET), false)
}

#[inline]
fn parse_scss_inner_expression_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, end_ts, false)
}

#[inline]
fn parse_scss_expression_with_options(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
    allow_empty: bool,
) -> ParsedSyntax {
    if !allow_empty && (p.at_ts(end_ts) || p.at(EOF)) {
        return Absent;
    }

    let expression = p.start();
    let expression_items = p.start();
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && !p.at_ts(end_ts) {
        progress.assert_progressing(p);

        if parse_scss_expression_item(p)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, end_ts)
                    .enable_recovery_on_line_break(),
                expected_scss_expression,
            )
            .is_err()
        {
            break;
        }
    }

    expression_items.complete(p, SCSS_EXPRESSION_ITEM_LIST);
    Present(expression.complete(p, SCSS_EXPRESSION))
}

#[inline]
fn parse_scss_expression_item(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        parse_scss_parenthesized_or_map_expression(p)
    } else {
        parse_generic_component_value(p)
    }
}

#[inline]
fn parse_scss_parenthesized_or_map_expression(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    if p.at(T![')']) {
        let empty_pairs = p.start();
        empty_pairs.complete(p, SCSS_MAP_EXPRESSION_PAIR_LIST);
        p.bump(T![')']);
        return Present(m.complete(p, SCSS_MAP_EXPRESSION));
    }

    let first_expression =
        parse_scss_inner_expression_until(p, SCSS_MAP_EXPRESSION_KEY_END_TOKEN_SET)
            .or_add_diagnostic(p, expected_scss_expression);

    let Some(first_expression) = first_expression else {
        p.expect(T![')']);
        return Present(m.complete(p, SCSS_PARENTHESIZED_EXPRESSION));
    };

    if p.at(T![:]) {
        let first_pair = parse_scss_map_expression_pair_with_key(p, first_expression);
        complete_scss_map_expression_pair_list(p, first_pair);
        p.expect(T![')']);
        return Present(m.complete(p, SCSS_MAP_EXPRESSION));
    }

    if p.at(T![,]) {
        let first_element = complete_scss_list_expression_element(p, first_expression);
        complete_scss_list_expression(p, first_element);
        p.expect(T![')']);
        return Present(m.complete(p, SCSS_PARENTHESIZED_EXPRESSION));
    }

    p.expect(T![')']);
    Present(m.complete(p, SCSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
fn parse_scss_map_expression_pair_with_key(
    p: &mut CssParser,
    key: CompletedMarker,
) -> CompletedMarker {
    let pair_marker = key.precede(p);
    p.expect(T![:]);
    parse_scss_inner_expression_until(p, SCSS_MAP_EXPRESSION_VALUE_END_TOKEN_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    pair_marker.complete(p, SCSS_MAP_EXPRESSION_PAIR)
}

#[inline]
fn complete_scss_map_expression_pair_list(p: &mut CssParser, first_pair: CompletedMarker) {
    let pairs_marker = first_pair.precede(p);
    let mut progress = ParserProgress::default();

    while p.at(T![,]) {
        p.bump(T![,]);

        if p.at(T![')']) {
            break;
        }

        progress.assert_progressing(p);

        let pair = match parse_scss_inner_expression_until(p, SCSS_MAP_EXPRESSION_KEY_END_TOKEN_SET)
        {
            Present(key) => Present(parse_scss_map_expression_pair_with_key(p, key)),
            Absent => Absent,
        };

        if pair
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    CSS_BOGUS_PROPERTY_VALUE,
                    SCSS_MAP_EXPRESSION_VALUE_END_TOKEN_SET,
                )
                .enable_recovery_on_line_break(),
                expected_scss_expression,
            )
            .is_err()
        {
            break;
        }
    }

    pairs_marker.complete(p, SCSS_MAP_EXPRESSION_PAIR_LIST);
}

#[inline]
fn complete_scss_list_expression(p: &mut CssParser, first_element: CompletedMarker) {
    let list_elements = first_element.precede(p);
    let mut progress = ParserProgress::default();

    while p.at(T![,]) {
        p.bump(T![,]);

        if p.at(T![')']) {
            break;
        }

        progress.assert_progressing(p);

        if parse_scss_inner_expression_until(p, SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET)
            .map(|expression| complete_scss_list_expression_element(p, expression))
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    CSS_BOGUS_PROPERTY_VALUE,
                    SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET,
                )
                .enable_recovery_on_line_break(),
                expected_scss_expression,
            )
            .is_err()
        {
            break;
        }
    }

    list_elements
        .complete(p, SCSS_LIST_EXPRESSION_ELEMENT_LIST)
        .precede(p)
        .complete(p, SCSS_LIST_EXPRESSION);
}

#[inline]
fn complete_scss_list_expression_element(
    p: &mut CssParser,
    expression: CompletedMarker,
) -> CompletedMarker {
    expression
        .precede(p)
        .complete(p, SCSS_LIST_EXPRESSION_ELEMENT)
}

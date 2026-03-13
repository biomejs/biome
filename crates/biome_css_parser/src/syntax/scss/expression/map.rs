use crate::parser::CssParser;
use crate::syntax::scss::expected_scss_expression;
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_PROPERTY_VALUE, SCSS_MAP_EXPRESSION, SCSS_MAP_EXPRESSION_PAIR,
    SCSS_MAP_EXPRESSION_PAIR_LIST, SCSS_PARENTHESIZED_EXPRESSION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress, TokenSet, token_set};

use super::ScssExpressionOptions;
use super::list::{
    SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET, complete_scss_list_expression,
    complete_scss_list_expression_element, parse_scss_inner_expression_until,
};

const SCSS_MAP_EXPRESSION_KEY_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![:], T![')']];
const SCSS_MAP_EXPRESSION_VALUE_END_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')']];

/// Parses a parenthesized SCSS expression and upgrades it to a map when a `:`
/// appears after the first item.
///
/// Example:
/// ```scss
/// $map: (a: 1, b: 2);
/// ```
///
/// Docs: https://sass-lang.com/documentation/values/maps
#[inline]
pub(super) fn parse_scss_parenthesized_or_map_expression(p: &mut CssParser) -> ParsedSyntax {
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
        complete_scss_list_expression(
            p,
            first_element,
            ScssExpressionOptions::value(SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET),
        );
        p.expect(T![')']);
        return Present(m.complete(p, SCSS_PARENTHESIZED_EXPRESSION));
    }

    p.expect(T![')']);
    Present(m.complete(p, SCSS_PARENTHESIZED_EXPRESSION))
}

/// Parses a single `key: value` pair after the key has already been parsed as an
/// expression.
///
/// Example:
/// ```scss
/// $colors: ("primary": #c00);
/// ```
///
/// Docs: https://sass-lang.com/documentation/values/maps
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

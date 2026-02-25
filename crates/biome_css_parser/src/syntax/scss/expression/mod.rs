use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_component_value, expected_scss_expression, scss_ellipsis_not_allowed,
};
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::scss::{is_at_scss_identifier, parse_scss_identifier};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_PROPERTY_VALUE, EOF, SCSS_ARBITRARY_ARGUMENT, SCSS_BINARY_EXPRESSION,
    SCSS_EXPRESSION, SCSS_EXPRESSION_ITEM_LIST, SCSS_KEYWORD_ARGUMENT, SCSS_LIST_EXPRESSION,
    SCSS_LIST_EXPRESSION_ELEMENT, SCSS_LIST_EXPRESSION_ELEMENT_LIST, SCSS_MAP_EXPRESSION,
    SCSS_MAP_EXPRESSION_PAIR, SCSS_MAP_EXPRESSION_PAIR_LIST, SCSS_PARENTHESIZED_EXPRESSION,
    SCSS_UNARY_EXPRESSION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress, TokenSet, token_set};

const SCSS_BINARY_OPERATOR_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![
    T![*],
    T![/],
    T![%],
    T![+],
    T![-],
    T![>],
    T![>=],
    T![<],
    T![<=],
    T![==],
    T![!=],
    T![and],
    T![or],
];
pub(crate) const SCSS_UNARY_OPERATOR_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![+], T![-], T![not]];

const SCSS_MAP_EXPRESSION_KEY_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![:], T![')']];
const SCSS_MAP_EXPRESSION_VALUE_END_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')']];
const SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')']];

pub(crate) const END_OF_SCSS_EXPRESSION_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')'], T![;], T!['}']];

#[derive(Clone, Copy)]
struct ScssExpressionOptions {
    end_ts: TokenSet<CssSyntaxKind>,
    allows_keyword_arguments: bool,
    allows_ellipsis: bool,
}

impl ScssExpressionOptions {
    fn value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
        }
    }

    fn args(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_keyword_arguments: true,
            allows_ellipsis: true,
        }
    }
}

#[inline]
pub(crate) fn parse_scss_expression(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_until(p, END_OF_SCSS_EXPRESSION_TOKEN_SET)
}

#[inline]
pub(crate) fn parse_scss_expression_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::value(end_ts))
}

#[inline]
pub(crate) fn parse_scss_expression_in_args_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::args(end_ts))
}

#[inline]
fn parse_scss_inner_expression_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::value(end_ts))
}

#[inline]
fn parse_scss_expression_with_options(
    p: &mut CssParser,
    options: ScssExpressionOptions,
) -> ParsedSyntax {
    if p.at_ts(options.end_ts) || p.at(EOF) {
        return Absent;
    }

    let expression = p.start();
    let expression_items = p.start();
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && !p.at_ts(options.end_ts) {
        progress.assert_progressing(p);

        let parsed_item = parse_scss_expression_item(p, options);
        if parsed_item
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, options.end_ts)
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
fn parse_scss_expression_item(p: &mut CssParser, options: ScssExpressionOptions) -> ParsedSyntax {
    if is_at_scss_keyword_argument(p, options) {
        return parse_scss_keyword_argument(p, options);
    }

    let expression = parse_scss_binary_expression(p, 0);
    let expression = match expression {
        Present(expression) => expression,
        Absent => {
            if p.at(T![...]) {
                let range = p.cur_range();
                p.error(scss_ellipsis_not_allowed(p, range));
                p.bump(T![...]);
            }
            return Absent;
        }
    };

    if !p.at(T![...]) {
        return Present(expression);
    }

    if !options.allows_ellipsis {
        let range = p.cur_range();
        p.error(scss_ellipsis_not_allowed(p, range));
        p.bump(T![...]);
        return Present(expression);
    }

    let m = expression.precede(p);
    p.bump(T![...]);
    Present(m.complete(p, SCSS_ARBITRARY_ARGUMENT))
}

#[inline]
fn is_at_scss_keyword_argument(p: &mut CssParser, options: ScssExpressionOptions) -> bool {
    options.allows_keyword_arguments
        && !options.end_ts.contains(T![:])
        && is_at_scss_identifier(p)
        && p.nth_at(2, T![:])
}

#[inline]
fn parse_scss_keyword_argument(p: &mut CssParser, options: ScssExpressionOptions) -> ParsedSyntax {
    if !is_at_scss_keyword_argument(p, options) {
        return Absent;
    }

    let m = p.start();
    parse_scss_identifier(p).ok();
    p.expect(T![:]);

    parse_scss_expression_with_options(
        p,
        ScssExpressionOptions {
            end_ts: options.end_ts,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
        },
    )
    .or_add_diagnostic(p, expected_component_value);

    Present(m.complete(p, SCSS_KEYWORD_ARGUMENT))
}

#[inline]
fn parse_scss_binary_expression(p: &mut CssParser, min_prec: u8) -> ParsedSyntax {
    let mut left = match parse_scss_unary_expression(p) {
        Present(left) => left,
        Absent => return Absent,
    };

    loop {
        let Some(prec) = scss_binary_precedence(p) else {
            break;
        };

        if prec < min_prec {
            break;
        }

        let m = left.precede(p);
        p.bump_ts(SCSS_BINARY_OPERATOR_TOKEN_SET);

        parse_scss_binary_expression(p, prec + 1).or_add_diagnostic(p, expected_component_value);

        left = m.complete(p, SCSS_BINARY_EXPRESSION);
    }

    Present(left)
}

#[inline]
fn parse_scss_unary_expression(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_unary_operator(p) {
        let m = p.start();
        p.bump_ts(SCSS_UNARY_OPERATOR_TOKEN_SET);
        parse_scss_unary_expression(p).or_add_diagnostic(p, expected_component_value);
        return Present(m.complete(p, SCSS_UNARY_EXPRESSION));
    }

    parse_scss_primary_expression(p)
}

#[inline]
fn parse_scss_primary_expression(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        parse_scss_parenthesized_or_map_expression(p)
    } else {
        parse_generic_component_value(p)
    }
}

/// Returns the precedence level for the current SCSS binary operator token.
///
/// Docs: https://sass-lang.com/documentation/operators/#order-of-operations
#[inline]
fn scss_binary_precedence(p: &mut CssParser) -> Option<u8> {
    if !p.at_ts(SCSS_BINARY_OPERATOR_TOKEN_SET) {
        return None;
    }

    Some(match p.cur() {
        T![or] => 1,
        T![and] => 2,
        T![==] | T![!=] => 3,
        T![<] | T![<=] | T![>] | T![>=] => 4,
        T![+] | T![-] => 5,
        T![*] | T![/] | T![%] => 6,
        _ => return None,
    })
}

#[inline]
fn is_at_scss_unary_operator(p: &mut CssParser) -> bool {
    p.at_ts(SCSS_UNARY_OPERATOR_TOKEN_SET)
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

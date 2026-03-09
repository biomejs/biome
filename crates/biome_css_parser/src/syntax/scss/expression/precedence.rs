use crate::lexer::CssReLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::value::dimension::is_at_any_dimension;
use biome_css_syntax::CssSyntaxKind::{
    CSS_NUMBER_LITERAL, SCSS_BINARY_EXPRESSION, SCSS_UNARY_EXPRESSION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

use super::primary::parse_scss_primary_expression;

pub(super) const SCSS_BINARY_OPERATOR_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![
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

/// Parses SCSS expressions with Sass operator precedence using precedence
/// climbing.
///
/// Example:
/// ```scss
/// $x: 1 + 2 * 3;
/// ```
///
/// Docs: https://sass-lang.com/documentation/operators
#[inline]
pub(super) fn parse_scss_binary_expression(p: &mut CssParser, min_prec: u8) -> ParsedSyntax {
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

/// Parses chained unary operators (`-`, `+`, `not`) before the primary
/// expression.
///
/// Example:
/// ```scss
/// $x: not -$y;
/// ```
///
/// Docs: https://sass-lang.com/documentation/operators
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

/// Re-lexes signed numeric tokens in SCSS expression context.
///
/// If the current token is `CSS_NUMBER_LITERAL` or any dimension starting with `+` or `-`,
/// this mutates parser state via `CssParser::re_lex(CssReLexContext::ScssExpression)`.
#[inline]
fn re_lex_signed_numeric_as_scss_operator(p: &mut CssParser) {
    if !(p.at(CSS_NUMBER_LITERAL) || is_at_any_dimension(p)) {
        return;
    }

    let text = p.cur_text();
    if matches!(text.as_bytes().first(), Some(b'+' | b'-')) {
        p.re_lex(CssReLexContext::ScssExpression);
    }
}

/// Returns the precedence level for the current SCSS binary operator token.
///
/// Docs: https://sass-lang.com/documentation/operators/#order-of-operations
#[inline]
fn scss_binary_precedence(p: &mut CssParser) -> Option<u8> {
    re_lex_signed_numeric_as_scss_operator(p);

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

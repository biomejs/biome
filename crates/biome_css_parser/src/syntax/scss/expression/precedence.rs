use crate::lexer::CssReLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::scss::is_at_scss_interpolated_dashed_identifier;
use crate::syntax::value::dimension::is_at_any_dimension;
use biome_css_syntax::CssSyntaxKind::{
    CSS_NUMBER_LITERAL, SCSS_BINARY_EXPRESSION, SCSS_UNARY_EXPRESSION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, TokenSet, token_set};

use super::ScssExpressionOptions;
use super::operand::parse_scss_expression_operand;

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
pub(super) fn parse_scss_binary_expression(
    p: &mut CssParser,
    min_prec: u8,
    options: ScssExpressionOptions,
) -> ParsedSyntax {
    let left = match parse_scss_unary_expression(p, options) {
        Present(left) => left,
        Absent => return Absent,
    };

    Present(parse_scss_binary_tail(p, left, min_prec, options))
}

/// Parses a binary-operator tail after the caller has already parsed the left
/// operand.
///
/// Example: `+ 1px` after `$width` in `$width + 1px`.
#[inline]
pub(super) fn parse_scss_binary_tail(
    p: &mut CssParser,
    mut left: CompletedMarker,
    min_prec: u8,
    options: ScssExpressionOptions,
) -> CompletedMarker {
    // `[a / 1 / b]` uses `/` as the bracketed-list separator. Honor caller
    // delimiters before treating the same token as an infix operator.
    while !p.at_ts(options.end_ts) {
        let Some(prec) = scss_binary_precedence(p) else {
            break;
        };

        if prec < min_prec {
            break;
        }

        let m = left.precede(p);
        p.bump_ts(SCSS_BINARY_OPERATOR_TOKEN_SET);
        parse_scss_binary_expression(p, prec + 1, options)
            .or_add_diagnostic(p, expected_component_value);
        left = m.complete(p, SCSS_BINARY_EXPRESSION);
    }

    left
}

/// Parses chained unary operators (`-`, `+`, `not`) before an operand.
///
/// Example:
/// ```scss
/// $x: not -$y;
/// ```
///
/// Docs: https://sass-lang.com/documentation/operators
#[inline]
fn parse_scss_unary_expression(p: &mut CssParser, options: ScssExpressionOptions) -> ParsedSyntax {
    if is_at_scss_unary_operator(p) {
        let m = p.start();
        p.bump_ts(SCSS_UNARY_OPERATOR_TOKEN_SET);
        parse_scss_unary_expression(p, options).or_add_diagnostic(p, expected_component_value);
        return Present(m.complete(p, SCSS_UNARY_EXPRESSION));
    }

    parse_scss_expression_operand(p, options)
}

#[inline]
fn is_at_scss_unary_operator(p: &mut CssParser) -> bool {
    // `var(--#{$name})` starts with `-`, but the pair belongs to one
    // custom-property identifier, not chained unary operators.
    if is_at_scss_interpolated_dashed_identifier(p) {
        return false;
    }

    p.at_ts(SCSS_UNARY_OPERATOR_TOKEN_SET)
}

/// Returns the precedence level for the current SCSS binary operator token.
///
/// Docs: https://sass-lang.com/documentation/operators/#order-of-operations
#[inline]
pub(super) fn scss_binary_precedence(p: &mut CssParser) -> Option<u8> {
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

/// Returns whether the current token is a SCSS binary operator, re-lexing tight
/// signed numeric tokens the same way the expression parser does.
///
/// Example: the `+10px` token after `10px` in `10px+10px`.
#[inline]
pub(crate) fn is_at_scss_binary_operator(p: &mut CssParser) -> bool {
    scss_binary_precedence(p).is_some()
}

/// Re-lexes signed numeric tokens that Sass treats as binary operators.
///
/// `1 - 2` already has a `-` token. Re-lex `1-2`, but keep `1 -2` as a list.
#[inline]
fn re_lex_signed_numeric_as_scss_operator(p: &mut CssParser) {
    if !(p.at(CSS_NUMBER_LITERAL) || is_at_any_dimension(p)) {
        return;
    }

    if should_re_lex_signed_numeric(p.cur_text(), p) {
        p.re_lex(CssReLexContext::ScssExpression);
    }
}

#[inline]
fn should_re_lex_signed_numeric(text: &str, p: &CssParser) -> bool {
    match text.as_bytes().first().copied() {
        Some(b'+') => true,
        Some(b'-') => !p.has_preceding_whitespace() && !p.has_preceding_line_break(),
        _ => false,
    }
}

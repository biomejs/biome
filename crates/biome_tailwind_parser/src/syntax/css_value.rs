use crate::parser::TailwindParser;
use crate::syntax::atoms::{css_token, url_raw_value};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, TokenSet, token_set};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_unicode_table::Dispatch::{PNC, QOT, WHS};

/// Returns the kind and length in atoms of the CSS token at the current atom.
#[inline]
fn css(p: &TailwindParser) -> (TailwindSyntaxKind, usize) {
    p.css_token()
}

/// Whether the CSS token at the current atom is of one of `kinds`.
#[inline]
fn css_at_ts(p: &TailwindParser, kinds: TokenSet<TailwindSyntaxKind>) -> bool {
    kinds.contains(css(p).0)
}

pub(crate) fn parse_css_generic_component_value_list(p: &mut TailwindParser) -> bool {
    let start = p.source().position();
    CssGenericComponentValueList.parse_list(p);
    p.source().position() != start
}

/// Parses the comma-separated arguments of a CSS function call. The list is
/// always created, so a required list slot is never missing; an empty list
/// has an empty range.
pub(crate) fn parse_css_parameter_list(p: &mut TailwindParser) -> CompletedMarker {
    ParameterList.parse_list(p)
}

struct CssGenericComponentValueList;

impl ParseNodeList for CssGenericComponentValueList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_css_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![']']) || is_at_class_separator(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PROPERTY_VALUE,
                token_set![T![']'], WHITESPACE, EOF],
            ),
            crate::syntax::parse_error::expected_value,
        )
    }
}

fn parse_css_generic_component_value(p: &mut TailwindParser) -> ParsedSyntax {
    let (kind, _) = css(p);
    if token_set![T![,], T![/], T![:], T![=], T![.]].contains(kind) {
        let m = p.start();
        p.bump_css(kind);
        Present(m.complete(p, CSS_GENERIC_DELIMITER))
    } else {
        parse_css_value(p)
    }
}

fn parse_css_value(p: &mut TailwindParser) -> ParsedSyntax {
    if css_at_ts(p, token_set![T![+], T![-]]) {
        parse_unary_expression(p)
    } else if p.at(T!['(']) {
        parse_parenthesized_expression(p)
    } else {
        parse_any_value(p)
    }
}

/// Parses a single value that is not an expression: math operators and
/// parentheses are left to the caller.
fn parse_any_value(p: &mut TailwindParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        return parse_any_function(p);
    }
    let (kind, atoms) = css(p);
    if is_identifier(kind) {
        return parse_css_identifier(p);
    }
    match kind {
        CSS_STRING_LITERAL => parse_css_string(p),
        CSS_DIMENSION_VALUE => {
            let m = p.start();
            p.bump_css_atoms(CSS_NUMBER_LITERAL, atoms);
            // The lexer splits a number from its unit, the ASCII letters after it, and gives the
            // unit the kind of its keyword.
            let node = if is_unit(p.cur()) {
                CSS_REGULAR_DIMENSION
            } else {
                CSS_UNKNOWN_DIMENSION
            };
            p.bump_css_atoms(T![ident], 1);
            Present(m.complete(p, node))
        }
        CSS_PERCENTAGE_VALUE => {
            let m = p.start();
            p.bump_css_atoms(CSS_NUMBER_LITERAL, atoms);
            p.expect_css(T![%]);
            Present(m.complete(p, CSS_PERCENTAGE))
        }
        CSS_NUMBER_LITERAL => parse_css_number_or_ratio(p),
        CSS_COLOR_LITERAL => {
            let m = p.start();
            p.bump_css_atoms(CSS_COLOR_LITERAL, atoms);
            Present(m.complete(p, CSS_COLOR))
        }
        _ => Absent,
    }
}

/// Parses an identifier, which is dashed if it starts with `--`.
fn parse_css_identifier(p: &mut TailwindParser) -> ParsedSyntax {
    let (kind, atoms) = css(p);
    if !is_identifier(kind) {
        return Absent;
    }
    let node = if is_at_dashed_identifier(p) {
        CSS_DASHED_IDENTIFIER
    } else {
        CSS_IDENTIFIER
    };
    let m = p.start();
    p.bump_css_atoms(T![ident], atoms);
    Present(m.complete(p, node))
}

fn parse_css_string(p: &mut TailwindParser) -> ParsedSyntax {
    let (_, atoms) = css(p);
    let m = p.start();
    p.bump_css_atoms(CSS_STRING_LITERAL, atoms);
    Present(m.complete(p, CSS_STRING))
}

fn parse_css_number(p: &mut TailwindParser) -> ParsedSyntax {
    let (kind, atoms) = css(p);
    if kind != CSS_NUMBER_LITERAL {
        return Absent;
    }
    let m = p.start();
    p.bump_css_atoms(CSS_NUMBER_LITERAL, atoms);
    Present(m.complete(p, CSS_NUMBER))
}

fn parse_css_number_or_ratio(p: &mut TailwindParser) -> ParsedSyntax {
    let number = parse_css_number(p);
    if number.is_absent() || !current_slash_is_followed_by_css_number(p) {
        return number;
    }

    p.bump_css(T![/]);
    let ratio = number.precede(p);
    parse_css_number(p).ok();

    Present(ratio.complete(p, CSS_RATIO))
}

/// Whether the current atom is a `/` and the CSS token after it, past any `_`s, is a number.
fn current_slash_is_followed_by_css_number(p: &TailwindParser) -> bool {
    if !p.at(T![/]) {
        return false;
    }
    let mut n = 1;
    while p.source().nth_kind(n) == CSS_WHITESPACE {
        n += 1;
    }
    css_token(p.source(), n).0 == CSS_NUMBER_LITERAL
}

#[inline]
fn is_at_any_function(p: &TailwindParser) -> bool {
    is_at_url_function(p) || is_at_function(p)
}

#[inline]
fn parse_any_function(p: &mut TailwindParser) -> ParsedSyntax {
    if is_at_url_function(p) {
        parse_url_function(p)
    } else if is_at_function(p) {
        parse_function(p)
    } else {
        Absent
    }
}

/// Whether the current atom starts an identifier with a `(` right after it.
#[inline]
fn is_at_identifier_and_paren(p: &TailwindParser) -> bool {
    let (kind, atoms) = css(p);
    is_identifier(kind) && p.source().nth_kind(atoms) == T!['(']
}

#[inline]
fn is_at_url_function(p: &TailwindParser) -> bool {
    css(p).0 == T![url] && is_at_identifier_and_paren(p)
}

#[inline]
fn is_at_function(p: &TailwindParser) -> bool {
    is_at_identifier_and_paren(p) && css(p).0 != T![url]
}

fn parse_url_function(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    let (_, atoms) = css(p);
    p.bump_css_atoms(T![url], atoms);
    p.expect_css(T!['(']);
    if is_at_any_function(p) {
        ParameterList.parse_list(p);
    } else {
        parse_url_value(p).ok();
    }
    p.expect_css(T![')']);
    Present(m.complete(p, CSS_URL_FUNCTION))
}

/// Parses the argument of `url(...)` when it isn't a function: a string, or a raw URL, which
/// runs to the next `)` or whitespace.
fn parse_url_value(p: &mut TailwindParser) -> ParsedSyntax {
    match p.source().nth_dispatch(0) {
        Some(QOT) => parse_css_string(p),
        None | Some(PNC | WHS) => Absent,
        Some(_) => {
            let m = p.start();
            let atoms = url_raw_value(p.source());
            p.bump_atoms(CSS_URL_VALUE_RAW_LITERAL, atoms);
            Present(m.complete(p, CSS_URL_VALUE_RAW))
        }
    }
}

fn parse_function(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    parse_css_identifier(p).ok();
    p.expect_css(T!['(']);
    ParameterList.parse_list(p);
    p.expect_css(T![')']);
    Present(m.complete(p, CSS_FUNCTION))
}

struct ParameterList;

impl ParseSeparatedList for ParameterList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T![')'], T![']']]) || is_at_class_separator(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PROPERTY_VALUE,
                token_set![T![,], T![')'], T![']'], WHITESPACE],
            ),
            crate::syntax::parse_error::expected_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_css(T![,])
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn is_at_any_expression(p: &TailwindParser) -> bool {
    is_at_unary_operator(p) || p.at(T!['(']) || is_at_any_value(p)
}

#[inline]
fn parse_any_expression(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_any_expression(p) {
        return Absent;
    }

    let param = parse_unary_expression_operand(p);

    let (kind, _) = css(p);
    if BINARY_OPERATION_TOKEN.contains(kind) {
        let binary_expression = param.precede(p);
        p.bump_css(kind);
        parse_any_expression(p).or_add_diagnostic(p, crate::syntax::parse_error::expected_value);
        Present(binary_expression.complete(p, CSS_BINARY_EXPRESSION))
    } else {
        param
    }
}

const BINARY_OPERATION_TOKEN: TokenSet<TailwindSyntaxKind> = token_set![T![+], T![-], T![*], T![/]];
const UNARY_OPERATION_TOKEN: TokenSet<TailwindSyntaxKind> = token_set![T![+], T![-], T![*]];
const COMPONENT_VALUE_EXPRESSION_RECOVERY_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![')'], T![,], T![']'], WHITESPACE].union(BINARY_OPERATION_TOKEN);

#[inline]
fn is_at_unary_operator(p: &TailwindParser) -> bool {
    css_at_ts(p, UNARY_OPERATION_TOKEN)
}

#[inline]
fn parse_unary_expression(p: &mut TailwindParser) -> ParsedSyntax {
    let (kind, _) = css(p);
    if !UNARY_OPERATION_TOKEN.contains(kind) {
        return Absent;
    }

    let m = p.start();
    p.bump_css(kind);
    parse_unary_expression_operand(p)
        .or_add_diagnostic(p, crate::syntax::parse_error::expected_value);
    Present(m.complete(p, CSS_UNARY_EXPRESSION))
}

#[inline]
fn parse_unary_expression_operand(p: &mut TailwindParser) -> ParsedSyntax {
    if is_at_unary_operator(p) {
        parse_unary_expression(p)
    } else if p.at(T!['(']) {
        parse_parenthesized_expression(p)
    } else {
        parse_list_of_component_values_expression(p)
    }
}

fn parse_parenthesized_expression(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.expect_css(T!['(']);
    parse_any_expression(p).ok();
    p.expect_css(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
fn parse_list_of_component_values_expression(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_any_value(p) {
        return Absent;
    }

    let m = p.start();
    ComponentValueExpressionList.parse_list(p);
    Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION))
}

struct ComponentValueExpressionList;

impl ParseNodeList for ComponentValueExpressionList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        css_at_ts(
            p,
            BINARY_OPERATION_TOKEN.union(token_set![T![,], T![')'], T![']']]),
        ) || is_at_class_separator(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PROPERTY_VALUE,
                COMPONENT_VALUE_EXPRESSION_RECOVERY_SET,
            ),
            crate::syntax::parse_error::expected_value,
        )
    }
}

#[inline]
fn is_at_any_value(p: &TailwindParser) -> bool {
    let (kind, _) = css(p);
    is_at_any_function(p)
        || is_identifier(kind)
        || token_set![
            CSS_STRING_LITERAL,
            CSS_DIMENSION_VALUE,
            CSS_PERCENTAGE_VALUE,
            CSS_NUMBER_LITERAL,
            CSS_COLOR_LITERAL
        ]
        .contains(kind)
}

fn is_at_class_separator(p: &TailwindParser) -> bool {
    p.at(WHITESPACE)
}

/// Whether a CSS token of `kind` is an identifier: `IDENT`, or a keyword such as `var` or a
/// unit.
#[inline]
pub(crate) fn is_identifier(kind: TailwindSyntaxKind) -> bool {
    kind == IDENT || CSS_VALUE_KEYWORD_SET.contains(kind)
}

/// Whether the CSS token at the current atom is a dashed identifier: one that starts with `--`.
#[inline]
pub(crate) fn is_at_dashed_identifier(p: &TailwindParser) -> bool {
    css(p).0 == IDENT && p.source().text()[p.cur_range().start().into()..].starts_with("--")
}

/// Whether a token of `kind` is a unit keyword.
#[inline]
fn is_unit(kind: TailwindSyntaxKind) -> bool {
    ALL_UNIT_SET.contains(kind)
}

const CSS_VALUE_KEYWORD_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![var], T![url]].union(ALL_UNIT_SET);

const ALL_UNIT_SET: TokenSet<TailwindSyntaxKind> = LENGTH_UNIT_SET
    .union(CONTAINER_LENGTHS_UNIT_SET)
    .union(ANGLE_UNIT_SET)
    .union(TIME_UNIT_SET)
    .union(FREQUENCY_UNIT_SET)
    .union(RESOLUTION_UNIT_SET)
    .union(FLEX_UNIT_SET);

const LENGTH_UNIT_SET: TokenSet<TailwindSyntaxKind> = token_set![
    T![em],
    T![rem],
    T![ex],
    T![rex],
    T![cap],
    T![rcap],
    T![ch],
    T![rch],
    T![ic],
    T![ric],
    T![lh],
    T![rlh],
    T![vw],
    T![svw],
    T![lvw],
    T![dvw],
    T![vh],
    T![svh],
    T![lvh],
    T![dvh],
    T![vi],
    T![svi],
    T![lvi],
    T![dvi],
    T![vb],
    T![svb],
    T![lvb],
    T![dvb],
    T![vmin],
    T![svmin],
    T![lvmin],
    T![dvmin],
    T![vmax],
    T![svmax],
    T![lvmax],
    T![dvmax],
    T![cm],
    T![mm],
    T![q],
    T![in],
    T![pc],
    T![pt],
    T![px],
    T![mozmm],
    T![rpx]
];

const CONTAINER_LENGTHS_UNIT_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![cqw], T![cqh], T![cqi], T![cqb], T![cqmin], T![cqmax]];

const ANGLE_UNIT_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![deg], T![grad], T![rad], T![turn]];

const TIME_UNIT_SET: TokenSet<TailwindSyntaxKind> = token_set![T![s], T![ms]];

const FREQUENCY_UNIT_SET: TokenSet<TailwindSyntaxKind> = token_set![T![hz], T![khz]];

const RESOLUTION_UNIT_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![dpi], T![dpcm], T![dppx], T![x]];

const FLEX_UNIT_SET: TokenSet<TailwindSyntaxKind> = token_set![T![fr]];

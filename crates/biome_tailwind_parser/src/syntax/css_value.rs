use crate::parser::TailwindParser;
use crate::token_source::TailwindLexContext;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, TokenSet, token_set};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

pub(crate) fn parse_css_generic_component_value_list(p: &mut TailwindParser) -> bool {
    let start = p.source().position();
    CssGenericComponentValueList.parse_list(p);
    p.source().position() != start
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
        p.at(T![']'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, token_set![T![']'], EOF]),
            crate::syntax::parse_error::expected_value,
        )
    }
}

fn parse_css_generic_component_value(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at_ts(token_set![T![,], T![/], T![:], T![=], T![.]]) {
        let m = p.start();
        p.bump_with_context(p.cur(), TailwindLexContext::CssValue);
        Present(m.complete(p, CSS_GENERIC_DELIMITER))
    } else {
        parse_css_value(p)
    }
}

fn parse_css_value(p: &mut TailwindParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        parse_any_function(p)
    } else if is_at_identifier(p) {
        if is_at_dashed_identifier(p) {
            parse_css_dashed_identifier(p)
        } else {
            parse_css_identifier(p)
        }
    } else if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p)
    } else if p.at(CSS_DIMENSION_VALUE) {
        parse_css_dimension(p)
    } else if p.at(CSS_PERCENTAGE_VALUE) {
        parse_css_percentage(p)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_css_number_or_ratio(p)
    } else if p.at(CSS_COLOR_LITERAL) {
        parse_css_color(p)
    } else if p.at_ts(token_set![T![+], T![-]]) {
        parse_css_unary_expression(p)
    } else if p.at(T!['(']) {
        parse_css_parenthesized_expression(p)
    } else {
        Absent
    }
}

fn parse_css_identifier(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_IDENTIFIER))
}

fn parse_css_dashed_identifier(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_DASHED_IDENTIFIER))
}

fn parse_css_string(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_with_context(CSS_STRING_LITERAL, TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_STRING))
}

fn parse_css_number(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_with_context(CSS_NUMBER_LITERAL, TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_NUMBER))
}

fn parse_css_number_or_ratio(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let number = parse_css_number(p);

    if !current_slash_is_followed_by_css_number(p) {
        return number;
    }

    p.bump_with_context(T![/], TailwindLexContext::CssValue);
    let ratio = number.precede(p);
    parse_css_number(p).ok();

    Present(ratio.complete(p, CSS_RATIO))
}

fn current_slash_is_followed_by_css_number(p: &mut TailwindParser) -> bool {
    if !p.at(T![/]) {
        return false;
    }

    // because p.nth_at() does not preserve the current lex context
    let checkpoint = p.checkpoint();
    p.bump_with_context(T![/], TailwindLexContext::CssValue);
    let is_followed_by_number = p.at(CSS_NUMBER_LITERAL);
    p.rewind(checkpoint);

    is_followed_by_number
}

fn parse_css_dimension(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_remap_with_context(CSS_NUMBER_LITERAL, TailwindLexContext::CssValue);
    let kind = if is_nth_at_unit(p, 0) {
        CSS_REGULAR_DIMENSION
    } else {
        CSS_UNKNOWN_DIMENSION
    };
    p.bump_remap_with_context(T![ident], TailwindLexContext::CssValue);
    Present(m.complete(p, kind))
}

fn parse_css_percentage(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_remap_with_context(CSS_NUMBER_LITERAL, TailwindLexContext::CssValue);
    p.expect_with_context(T![%], TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_PERCENTAGE))
}

fn parse_css_color(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_with_context(CSS_COLOR_LITERAL, TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_COLOR))
}

#[inline]
fn is_at_any_function(p: &mut TailwindParser) -> bool {
    is_at_url_function(p) || is_at_function(p)
}

#[inline]
fn parse_any_function(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function(p)
    } else {
        parse_function(p)
    }
}

#[inline]
fn is_at_url_function(p: &mut TailwindParser) -> bool {
    p.at(T![url]) && p.nth_at(1, T!['('])
}

fn parse_url_function(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_url_function(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![url], TailwindLexContext::CssValue);
    p.expect_with_context(T!['('], TailwindLexContext::CssValue);
    if is_at_any_function(p) {
        ParameterList.parse_list(p);
    } else {
        p.source_mut()
            .re_lex_current_in_context(TailwindLexContext::CssUrlRawValue);
        parse_url_value(p).ok();
    }
    p.expect_with_context(T![')'], TailwindLexContext::CssValue);
    Present(m.complete(p, CSS_URL_FUNCTION))
}

#[inline]
fn parse_url_value_raw(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(CSS_URL_VALUE_RAW_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        CSS_URL_VALUE_RAW_LITERAL,
        TailwindLexContext::CssUrlRawValue,
    );
    Present(m.complete(p, CSS_URL_VALUE_RAW))
}

#[inline]
fn parse_url_value(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p)
    } else {
        parse_url_value_raw(p)
    }
}

#[inline]
fn is_at_function(p: &mut TailwindParser) -> bool {
    is_nth_at_function(p, 0) && !is_at_url_function(p)
}

#[inline]
fn is_nth_at_function(p: &mut TailwindParser, n: usize) -> bool {
    is_nth_at_identifier(p, n) && p.nth_at(n + 1, T!['('])
}

fn parse_function(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    parse_css_identifier(p).ok();
    p.expect_with_context(T!['('], TailwindLexContext::CssValue);
    ParameterList.parse_list(p);
    p.expect_with_context(T![')'], TailwindLexContext::CssValue);
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
        p.at_ts(token_set![T![')'], T![']']])
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
                token_set![T![,], T![')'], T![']']],
            ),
            crate::syntax::parse_error::expected_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(T![,], TailwindLexContext::CssValue)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_css_parenthesized_expression(p: &mut TailwindParser) -> ParsedSyntax {
    parse_parenthesized_expression(p)
}

#[inline]
fn is_at_any_expression(p: &mut TailwindParser) -> bool {
    is_at_unary_operator(p) || p.at(T!['(']) || is_at_any_value(p)
}

#[inline]
fn parse_any_expression(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_any_expression(p) {
        return Absent;
    }

    let param = parse_unary_expression_operand(p);

    if is_at_binary_operator(p) {
        let binary_expression = param.precede(p);
        p.eat_ts_with_context(BINARY_OPERATION_TOKEN, TailwindLexContext::CssValue);
        parse_any_expression(p).or_add_diagnostic(p, crate::syntax::parse_error::expected_value);
        Present(binary_expression.complete(p, CSS_BINARY_EXPRESSION))
    } else {
        param
    }
}

const BINARY_OPERATION_TOKEN: TokenSet<TailwindSyntaxKind> = token_set![T![+], T![-], T![*], T![/]];
const UNARY_OPERATION_TOKEN: TokenSet<TailwindSyntaxKind> = token_set![T![+], T![-], T![*]];
const COMPONENT_VALUE_EXPRESSION_RECOVERY_SET: TokenSet<TailwindSyntaxKind> =
    token_set![T![')'], T![,], T![']']].union(BINARY_OPERATION_TOKEN);

#[inline]
fn is_at_binary_operator(p: &mut TailwindParser) -> bool {
    p.at_ts(BINARY_OPERATION_TOKEN)
}

#[inline]
fn is_at_unary_operator(p: &mut TailwindParser) -> bool {
    p.at_ts(UNARY_OPERATION_TOKEN)
}

#[inline]
fn parse_unary_expression(p: &mut TailwindParser) -> ParsedSyntax {
    if !is_at_unary_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.eat_ts_with_context(UNARY_OPERATION_TOKEN, TailwindLexContext::CssValue);
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
    p.expect_with_context(T!['('], TailwindLexContext::CssValue);
    parse_any_expression(p).ok();
    p.expect_with_context(T![')'], TailwindLexContext::CssValue);
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
        parse_css_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(BINARY_OPERATION_TOKEN.union(token_set![T![,], T![')'], T![']']]))
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
fn parse_css_unary_expression(p: &mut TailwindParser) -> ParsedSyntax {
    parse_unary_expression(p)
}

#[inline]
fn is_at_any_value(p: &mut TailwindParser) -> bool {
    is_at_any_function(p)
        || is_at_identifier(p)
        || p.at_ts(token_set![
            CSS_STRING_LITERAL,
            CSS_DIMENSION_VALUE,
            CSS_PERCENTAGE_VALUE,
            CSS_NUMBER_LITERAL,
            CSS_COLOR_LITERAL,
            T!['(']
        ])
}

#[inline]
fn is_at_identifier(p: &mut TailwindParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
fn is_nth_at_identifier(p: &mut TailwindParser, n: usize) -> bool {
    p.nth_at(n, IDENT) || p.nth_at_ts(n, CSS_VALUE_KEYWORD_SET)
}

#[inline]
fn is_at_dashed_identifier(p: &mut TailwindParser) -> bool {
    p.at(IDENT) && p.cur_text().starts_with("--")
}

#[inline]
fn is_nth_at_unit(p: &mut TailwindParser, n: usize) -> bool {
    p.nth_at_ts(n, ALL_UNIT_SET)
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

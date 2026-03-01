use super::r#if::is_at_if_function;
use super::parse_error::expected_expression;
use super::url::{is_at_url_function, parse_url_function_with_context};
use crate::parser::CssParser;
use crate::syntax::css_modules::v_bind_not_allowed;
use crate::syntax::parse_error::{
    expected_component_value, expected_declaration_item, expected_identifier,
    scss_only_syntax_error,
};
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::scss::{
    SCSS_UNARY_OPERATOR_TOKEN_SET, is_at_scss_qualified_name, is_nth_at_scss_qualified_name,
    parse_scss_expression, parse_scss_expression_in_args_until, parse_scss_function_name,
};
use crate::syntax::value::attr::{is_at_attr_function, parse_attr_function};
use crate::syntax::value::r#if::parse_if_function;
use crate::syntax::{
    CssSyntaxFeatures, ValueParsingContext, ValueParsingMode, is_at_any_value_with_context,
    is_at_dashed_identifier, is_nth_at_identifier, parse_any_value_with_context,
    parse_dashed_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, TokenSet, token_set};

/// Checks if the current position is at any function recognized by CSS-only value parsing.
#[inline]
pub(crate) fn is_at_any_css_function(p: &mut CssParser) -> bool {
    is_at_any_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
pub(crate) fn is_at_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> bool {
    is_at_url_function(p)
        || is_at_if_function(p)
        || is_at_attr_function(p)
        || is_at_vue_v_bind_function(p)
        || is_at_function_with_context(p, context)
}

#[inline]
pub(crate) fn parse_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_function_with_context(p, context) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function_with_context(p, context)
    } else if is_at_if_function(p) {
        parse_if_function(p)
    } else if is_at_attr_function(p) {
        parse_attr_function(p)
    } else if is_at_vue_v_bind_function(p) {
        CssSyntaxFeatures::CssModulesWithVue.parse_exclusive_syntax(
            p,
            |p| parse_function_with_context(p, context),
            |p, marker| v_bind_not_allowed(p, marker.range(p)),
        )
    } else {
        parse_function_with_context(p, context)
    }
}

/// Checks if the current position in the `CssParser` is at the start of a simple CSS function.
///
/// This function determines if the parser's current position is at the start of a simple CSS function,
/// excluding URL functions (since URL functions are also considered simple functions but are handled separately).
#[inline]
pub(crate) fn is_at_function(p: &mut CssParser) -> bool {
    is_at_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::ScssAware))
}

/// Checks if the current position is at a simple function head in CSS-only mode.
#[inline]
pub(crate) fn is_at_css_function(p: &mut CssParser) -> bool {
    is_at_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn is_at_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_nth_at_function_with_context(p, 0, context) && !is_at_url_function(p)
}

#[inline]
pub(crate) fn is_nth_at_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(
        p,
        n,
        ValueParsingContext::new(p, ValueParsingMode::ScssAware),
    )
}

/// Checks if the `n`th token starts a simple function head in CSS-only mode.
#[inline]
pub(crate) fn is_nth_at_css_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(p, n, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn is_nth_at_function_with_context(
    p: &mut CssParser,
    n: usize,
    context: ValueParsingContext,
) -> bool {
    (is_nth_at_identifier(p, n) && p.nth_at(n + 1, T!['(']))
        || (context.is_scss_syntax_allowed()
            && is_nth_at_scss_qualified_name(p, n)
            && p.nth_at(n + 3, T!['(']))
}

#[inline]
fn is_at_vue_v_bind_function(p: &mut CssParser) -> bool {
    if !is_nth_at_css_function(p, 0) {
        return false;
    }

    let is_v_bind = p.cur_text() == "v-bind";
    if !is_v_bind {
        return false;
    }

    true
}

/// Parses a simple CSS function at the current position in the `CssParser`.
///
/// This function is responsible for parsing simple CSS functions (excluding URL functions).
/// # See Also
///
/// * [CSS Function Token Specification](https://www.w3.org/TR/css-syntax-3/#function-token-diagram)
/// * [CSS Functions Specification](https://www.w3.org/TR/css-syntax-3/#consume-function)
///
#[inline]
pub(crate) fn parse_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::ScssAware))
}

/// Parses a simple function using CSS-only branches.
#[inline]
pub(crate) fn parse_css_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn parse_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    if !is_at_function_with_context(p, context) {
        return Absent;
    }

    let m = p.start();

    if context.is_scss_syntax_allowed() && is_at_scss_qualified_name(p) {
        CssSyntaxFeatures::Scss
            .parse_exclusive_syntax(p, parse_scss_function_name, |p, marker| {
                scss_only_syntax_error(p, "SCSS qualified function names", marker.range(p))
            })
            .or_add_diagnostic(p, expected_identifier);
    } else {
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    }
    p.bump(T!['(']);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}

#[derive(Debug, Copy, Clone)]
struct ParameterListParseRecovery {
    context: ValueParsingContext,
}

impl ParseRecovery for ParameterListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_PARAMETER;

    /// Determines if the parser has reached a point where it can recover from an error
    /// while parsing a parameter list.
    ///
    /// This function checks if the parser is at a position where it can safely resume parsing
    /// after encountering an error in a parameter list. The recovery points are:
    /// - A comma ',', indicating the list separator.
    /// - The next parameter, indicating the start of a new parameter.
    /// - A closing parenthesis ')', indicating the end of a parameter list.
    /// - A ';', indicating the end of a declaration.
    /// # Examples
    /// Basic usage in CSS:
    ///
    /// ```css
    /// transform: rotate(30deg,, /* Error in parameter, recover here */)
    /// ```
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set!(T![,], T![')'], T![;])) || is_at_parameter_with_context(p, self.context)
    }
}

pub(crate) struct ParameterList {
    context: ValueParsingContext,
}

impl ParameterList {
    #[inline]
    fn new(context: ValueParsingContext) -> Self {
        Self { context }
    }
}

impl ParseSeparatedList for ParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_parameter_with_context(p, self.context)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParameterListParseRecovery {
                context: self.context,
            },
            expected_declaration_item,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn is_at_parameter_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_at_any_expression_with_context(p, context)
}

#[inline]
fn parse_parameter_with_context(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    if !is_at_parameter_with_context(p, context) {
        return Absent;
    }

    if context.is_scss_parsing_allowed() {
        parse_scss_expression_in_args_until(p, token_set![T![,], T![')'], T![;], T!['}']])
    } else {
        parse_any_expression_with_context(p, context)
    }
}

#[inline]
fn is_at_any_expression_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_at_unary_operator(p)
        || is_at_parenthesized(p)
        || is_at_any_value_with_context(p, context)
        || is_at_comma_separated_value(p)
}

#[inline]
fn parse_any_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_expression_with_context(p, context) {
        return Absent;
    }

    if context.is_scss_parsing_allowed()
        && (is_at_parenthesized(p)
            || is_at_any_value_with_context(p, context)
            || p.at_ts(SCSS_UNARY_OPERATOR_TOKEN_SET))
    {
        return parse_scss_expression(p);
    }

    let param = parse_unary_expression_operand_with_context(p, context);

    if is_at_binary_operator(p) {
        let binary_expression = param.precede(p);

        p.bump_ts(BINARY_OPERATION_TOKEN);
        parse_any_expression_with_context(p, context).or_add_diagnostic(p, expected_expression);

        Present(binary_expression.complete(p, CSS_BINARY_EXPRESSION))
    } else {
        param
    }
}

pub(crate) const BINARY_OPERATION_TOKEN: TokenSet<CssSyntaxKind> =
    token_set![T![+], T![-], T![*], T![/]];
const UNARY_OPERATION_TOKEN: TokenSet<CssSyntaxKind> = token_set![T![+], T![-], T![*]];

/// Checks if the current position in the CSS parser is at a binary operator.
///
/// This function determines whether the parser's current position is at the start
/// of a binary operation in a CSS expression. Binary operations include operators like
/// '+', '-', '*', '/', etc., used in CSS calculations or similar expressions.
#[inline]
pub(crate) fn is_at_binary_operator(p: &mut CssParser) -> bool {
    p.at_ts(BINARY_OPERATION_TOKEN)
}

#[inline]
pub(crate) fn is_at_unary_operator(p: &mut CssParser) -> bool {
    p.at_ts(UNARY_OPERATION_TOKEN)
}

#[inline]
fn parse_unary_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_unary_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(UNARY_OPERATION_TOKEN);
    parse_unary_expression_operand_with_context(p, context)
        .or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, CSS_UNARY_EXPRESSION))
}

#[inline]
fn parse_unary_expression_operand_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if is_at_unary_operator(p) {
        parse_unary_expression_with_context(p, context)
    } else if is_at_parenthesized(p) {
        parse_parenthesized_expression_with_context(p, context)
    } else if is_at_comma_separated_value(p) {
        parse_comma_separated_value(p)
    } else {
        parse_list_of_component_values_expression_with_context(p, context)
    }
}

/// Determines if the current position in the CSS parser is at the start of a parenthesized expression.
///
/// This function checks if the parser is currently positioned at an opening parenthesis '(',
/// which typically indicates the beginning of a parenthesized expression in CSS.
#[inline]
pub(crate) fn is_at_parenthesized(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_parenthesized_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_parenthesized(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['(']);
    parse_any_expression_with_context(p, context).ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
fn parse_list_of_component_values_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_value_with_context(p, context) {
        return Absent;
    }

    let m = p.start();
    ComponentValueExpressionList::new(context).parse_list(p);
    Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION))
}

#[derive(Debug, Copy, Clone)]
struct ComponentValueExpressionList {
    context: ValueParsingContext,
}

impl ComponentValueExpressionList {
    #[inline]
    fn new(context: ValueParsingContext) -> Self {
        Self { context }
    }
}

impl ParseNodeList for ComponentValueExpressionList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_value_with_context(p, self.context)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at_ts(BINARY_OPERATION_TOKEN)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set!(T![')'], T![;])),
            expected_component_value,
        )
    }
}

/// Parses theme references: --tab-size-*
pub(crate) fn parse_tailwind_value_theme_reference(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_dashed_identifier(p).ok();
    p.expect(T![-]);
    p.expect(T![*]);

    Present(m.complete(p, TW_VALUE_THEME_REFERENCE))
}

#[inline]
fn is_at_comma_separated_value(p: &mut CssParser) -> bool {
    p.at(T!['{'])
}

#[inline]
fn parse_comma_separated_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_comma_separated_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['{']);
    CommaSeparatedValueValueList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_COMMA_SEPARATED_VALUE))
}

struct CommaSeparatedValueValueList;

impl ParseNodeList for CommaSeparatedValueValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, token_set![T!['}']])
                .enable_recovery_on_line_break(),
            expected_component_value,
        )
    }
}

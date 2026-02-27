use super::r#if::is_at_if_function;
use super::parse_error::expected_expression;
use super::url::{is_at_url_function, parse_url_function};
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
    CssComponentValueList, CssSyntaxFeatures, is_at_any_value, is_at_dashed_identifier,
    is_nth_at_identifier, parse_dashed_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, TokenSet, token_set};

/// Checks if the current position in the `CssParser` is at the start of any recognized CSS function.
///
/// This function combines checks for specific CSS functions like `url()` and simple functions.
/// It's used to quickly determine if the parser is positioned at a relevant function.
#[inline]
pub(crate) fn is_at_any_function(p: &mut CssParser) -> bool {
    is_at_url_function(p)
        || is_at_if_function(p)
        || is_at_attr_function(p)
        || is_at_vue_v_bind_function(p)
        || is_at_function(p)
}

/// Parses any recognized CSS function at the current position in the `CssParser`.
///
/// This function first checks if the parser is positioned at a valid function.
/// If it is, the function will parse either a URL function or a simple function,
/// based on what is detected.
#[inline]
pub(crate) fn parse_any_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function(p)
    } else if is_at_if_function(p) {
        parse_if_function(p)
    } else if is_at_attr_function(p) {
        parse_attr_function(p)
    } else if is_at_vue_v_bind_function(p) {
        CssSyntaxFeatures::CssModulesWithVue.parse_exclusive_syntax(
            p,
            parse_function,
            |p, marker| v_bind_not_allowed(p, marker.range(p)),
        )
    } else {
        parse_function(p)
    }
}

/// Checks if the current position in the `CssParser` is at the start of a simple CSS function.
///
/// This function determines if the parser's current position is at the start of a simple CSS function,
/// excluding URL functions (since URL functions are also considered simple functions but are handled separately).
#[inline]
pub(crate) fn is_at_function(p: &mut CssParser) -> bool {
    is_nth_at_function(p, 0) && !is_at_url_function(p)
}

#[inline]
pub(crate) fn is_nth_at_function(p: &mut CssParser, n: usize) -> bool {
    (is_nth_at_identifier(p, n) && p.nth_at(n + 1, T!['(']))
        || (is_nth_at_scss_qualified_name(p, n) && p.nth_at(n + 3, T!['(']))
}

#[inline]
fn is_at_vue_v_bind_function(p: &mut CssParser) -> bool {
    if !is_nth_at_function(p, 0) {
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
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();

    if is_at_scss_qualified_name(p) {
        CssSyntaxFeatures::Scss
            .parse_exclusive_syntax(p, parse_scss_function_name, |p, marker| {
                scss_only_syntax_error(p, "SCSS qualified function names", marker.range(p))
            })
            .or_add_diagnostic(p, expected_identifier);
    } else {
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    }
    p.bump(T!['(']);
    ParameterList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}

struct ParameterListParseRecovery;

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
        p.at_ts(token_set!(T![,], T![')'], T![;])) || is_at_parameter(p)
    }
}

pub(crate) struct ParameterList;

impl ParseSeparatedList for ParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_parameter(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &ParameterListParseRecovery, expected_declaration_item)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// The function first checks whether the current position in the parser is at
/// the start of a valid parameter
#[inline]
pub(crate) fn is_at_parameter(p: &mut CssParser) -> bool {
    is_at_any_expression(p)
}

/// Parses a single CSS parameter.
///
/// This function attempts to parse a single parameter from the current position
/// in the CSS parser.
///
/// # Examples
///
/// Imagine parsing a CSS transform function like `rotate(45deg)`. When the parser
/// reaches `45deg`, `parse_parameter` would be invoked to parse and capture this
/// value as a parameter of the `rotate` function.
///
#[inline]
pub(crate) fn parse_parameter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_parameter(p) {
        return Absent;
    }

    let param = p.start();
    if CssSyntaxFeatures::Scss.is_supported(p) {
        parse_scss_expression_in_args_until(p, token_set![T![,], T![')'], T![;], T!['}']]).ok();
    } else {
        parse_any_expression(p).ok();
    }
    Present(param.complete(p, CSS_PARAMETER))
}

/// Determines if the current position in the CSS parser is at the start of any CSS expression.
#[inline]
pub(crate) fn is_at_any_expression(p: &mut CssParser) -> bool {
    is_at_unary_operator(p)
        || is_at_parenthesized(p)
        || is_at_any_value(p)
        || is_at_comma_separated_value(p)
}

/// Parses any CSS expression from the current position in the CSS parser.
#[inline]
pub(crate) fn parse_any_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_expression(p) {
        return Absent;
    }

    if CssSyntaxFeatures::Scss.is_supported(p)
        && (is_at_parenthesized(p) || is_at_any_value(p) || p.at_ts(SCSS_UNARY_OPERATOR_TOKEN_SET))
    {
        return parse_scss_expression(p);
    }

    let param = parse_unary_expression_operand(p);

    if is_at_binary_operator(p) {
        let binary_expression = param.precede(p);

        p.bump_ts(BINARY_OPERATION_TOKEN);
        parse_any_expression(p).or_add_diagnostic(p, expected_expression);

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
pub(crate) fn parse_unary_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unary_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(UNARY_OPERATION_TOKEN);
    parse_unary_expression_operand(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, CSS_UNARY_EXPRESSION))
}

#[inline]
fn parse_unary_expression_operand(p: &mut CssParser) -> ParsedSyntax {
    if is_at_unary_operator(p) {
        parse_unary_expression(p)
    } else if is_at_parenthesized(p) {
        parse_parenthesized_expression(p)
    } else if is_at_comma_separated_value(p) {
        parse_comma_separated_value(p)
    } else {
        parse_list_of_component_values_expression(p)
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

/// Parses a parenthesized expression from the current position in the CSS parser.
///
/// This function is invoked when a parenthesized expression is identified. It handles
/// the parsing of the entire expression enclosed within the parentheses.
#[inline]
pub(crate) fn parse_parenthesized_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_parenthesized(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['(']);
    parse_any_expression(p).ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

/// Parses a list of component values from the current position in the CSS parser.
///
/// This function is used to parse a sequence of CSS component values, typically found
/// in various CSS properties. It is called when the parser is at the start of such a list.
#[inline]
pub(crate) fn parse_list_of_component_values_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_value(p) {
        return Absent;
    }

    let m = p.start();
    CssComponentValueList.parse_list(p);
    Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION))
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

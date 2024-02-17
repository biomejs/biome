use super::parse_error::expected_expression;
use super::url::{is_at_url_function, parse_url_function};
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_declaration_item;
use crate::syntax::{
    is_at_any_value, is_nth_at_identifier, parse_regular_identifier, CssComponentValueList,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

/// Checks if the current position in the `CssParser` is at the start of any recognized CSS function.
///
/// This function combines checks for specific CSS functions like `url()` and simple functions.
/// It's used to quickly determine if the parser is positioned at a relevant function.
#[inline]
pub(crate) fn is_at_any_function(p: &mut CssParser) -> bool {
    is_at_url_function(p) || is_at_function(p)
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
    is_nth_at_identifier(p, n) && p.nth_at(n + 1, T!['('])
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

    parse_regular_identifier(p).ok();
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
    parse_any_expression(p).ok();
    Present(param.complete(p, CSS_PARAMETER))
}

/// Determines if the current position in the CSS parser is at the start of any CSS expression.
///
/// This function checks whether the parser's current position is at the beginning of
/// either a parenthesized expression or any CSS value. It's a preliminary check used
/// to decide if parsing should proceed for a general CSS expression.
#[inline]
pub(crate) fn is_at_any_expression(p: &mut CssParser) -> bool {
    is_at_parenthesized(p) || is_at_any_value(p)
}

/// Parses any CSS expression from the current position in the CSS parser.
///
/// Depending on the current position, it either parses a parenthesized expression
/// or a list of component values. If a binary operator is encountered after parsing
/// the expression, it continues to parse as a binary expression.
#[inline]
pub(crate) fn parse_any_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_expression(p) {
        return Absent;
    }

    let param = if is_at_parenthesized(p) {
        parse_parenthesized_expression(p)
    } else {
        parse_list_of_component_values_expression(p)
    };

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

/// Checks if the current position in the CSS parser is at a binary operator.
///
/// This function determines whether the parser's current position is at the start
/// of a binary operation in a CSS expression. Binary operations include operators like
/// '+', '-', '*', '/', etc., used in CSS calculations or similar expressions.
#[inline]
pub(crate) fn is_at_binary_operator(p: &mut CssParser) -> bool {
    p.at_ts(BINARY_OPERATION_TOKEN)
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

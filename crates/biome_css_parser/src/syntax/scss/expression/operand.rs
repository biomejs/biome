use crate::parser::CssParser;
use crate::syntax::scss::{
    add_scss_variable_member_function_name_diagnostic, is_at_scss_interpolation,
    is_at_scss_namespaced_variable, is_at_scss_variable, parse_scss_function_call_from_name,
    parse_scss_interpolated_function_or_value_until, parse_scss_interpolated_value,
    parse_scss_variable,
};
use crate::syntax::value::dimension::{is_at_any_dimension, parse_any_dimension};
use crate::syntax::{is_at_ratio, parse_ratio, parse_regular_identifier, parse_regular_number};
use biome_css_syntax::CssSyntaxKind::{
    CSS_NUMBER_LITERAL, SCSS_MODULE_MEMBER_ACCESS, SCSS_NAMESPACED_VARIABLE,
};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

use super::precedence::scss_binary_precedence;
use super::regular_expression_operand::parse_scss_regular_expression_operand;

/// Parses an SCSS expression operand such as `#{fn}(arg)`, `10#{unit}`,
/// `module.$name#{suffix}`, or `(1 + 2)`.
///
/// Examples:
/// ```scss
/// #{fn}(arg)
/// #{1}0
/// 10#{unit}
/// (1 + 2)
/// ```
///
/// Docs:
/// - https://sass-lang.com/documentation/operators
/// - https://sass-lang.com/documentation/interpolation
#[inline]
pub(super) fn parse_scss_expression_operand(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        return parse_scss_interpolated_function_or_value_until(
            p,
            is_at_scss_expression_interpolated_value_boundary,
        );
    }

    if is_at_scss_namespaced_variable(p) {
        return parse_scss_module_variable_operand(p);
    }

    if is_at_scss_interpolatable_value(p) {
        return parse_scss_interpolatable_value(p);
    }

    parse_scss_regular_expression_operand(p)
}

/// Parses `module.$name` as either a standalone module-member operand or an
/// interpolatable namespaced variable.
///
/// Example:
/// ```scss
/// module.$name#{suffix}
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
fn parse_scss_module_variable_operand(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_namespaced_variable(p) {
        return Absent;
    }

    let head = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![.]);
    parse_scss_variable(p).ok();

    if p.at(T!['(']) && !p.has_preceding_whitespace() && !p.has_preceding_line_break() {
        // `module.$name(` is an invalid module function name; recover by
        // parsing the call and reporting the `$` member diagnostic.
        let name = head.complete(p, SCSS_MODULE_MEMBER_ACCESS);
        let name = add_scss_variable_member_function_name_diagnostic(p, true, name);
        return parse_scss_function_call_from_name(p, name);
    }

    if !is_at_scss_expression_interpolated_value_suffix(p) {
        return Present(head.complete(p, SCSS_MODULE_MEMBER_ACCESS));
    }

    let head = head.complete(p, SCSS_NAMESPACED_VARIABLE);
    Present(parse_scss_interpolated_value(
        p,
        head,
        is_at_scss_expression_interpolated_value_boundary,
    ))
}

/// Parses an interpolatable value such as `$value#{suffix}`, `10px#{suffix}`,
/// `10#{unit}`, or `10/10`.
///
/// Examples:
/// ```scss
/// $value#{suffix}
/// 10px#{suffix}
/// 10#{unit}
/// 10/10
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
fn parse_scss_interpolatable_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_ratio(p) {
        // `10/10`: parse the ratio before `/` can become an SCSS binary operator.
        return parse_ratio(p);
    }

    let first_part = match parse_scss_interpolatable_value_first_part(p) {
        Present(first_part) => first_part,
        Absent => return Absent,
    };

    // Return only the parsed head for `$value`, `$value #{suffix}`, `$value / 2`,
    // and `3%4`; only interpolation suffixes like `$value#{suffix}` continue.
    if !is_at_scss_expression_interpolated_value_suffix(p) {
        return Present(first_part);
    }

    Present(parse_scss_interpolated_value(
        p,
        first_part,
        is_at_scss_expression_interpolated_value_boundary,
    ))
}

/// Parses the first part that can still accept adjacent interpolation.
///
/// Examples:
/// ```scss
/// $value#{suffix}
/// 10px#{suffix}
/// 10#{unit}
/// ```
///
/// `10/10` is not returned here because it is already a `CssRatio`.
#[inline]
fn parse_scss_interpolatable_value_first_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_variable(p) {
        parse_scss_variable(p)
    } else if is_at_any_dimension(p) {
        parse_any_dimension(p)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_regular_number(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_scss_interpolatable_value(p: &mut CssParser) -> bool {
    is_at_scss_variable(p) || is_at_any_dimension(p) || p.at(CSS_NUMBER_LITERAL)
}

#[inline]
fn is_at_scss_expression_interpolated_value_suffix(p: &mut CssParser) -> bool {
    !is_at_scss_expression_interpolated_value_boundary(p) && is_at_scss_interpolation(p)
}

/// Stops expression value chains before a space, line break, or operator.
///
/// Examples:
/// ```scss
/// $value #{suffix}
/// $value
/// #{suffix}
/// $value / 2
/// ```
#[inline]
fn is_at_scss_expression_interpolated_value_boundary(p: &mut CssParser) -> bool {
    p.has_preceding_whitespace()
        || p.has_preceding_line_break()
        || scss_binary_precedence(p).is_some()
}

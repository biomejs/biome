use crate::parser::CssParser;
use crate::syntax::scss::{
    is_at_scss_interpolation, is_at_scss_namespaced_variable, is_at_scss_variable,
    is_nth_at_scss_interpolation, parse_scss_identifier_or_interpolation,
    parse_scss_namespaced_variable, parse_scss_regular_interpolation, parse_scss_variable,
};
use crate::syntax::value::dimension::{is_at_any_dimension, parse_any_dimension};
use crate::syntax::{
    is_at_identifier, is_nth_at_identifier, parse_regular_identifier, parse_regular_number,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_NUMBER_LITERAL, SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST,
    SCSS_INTERPOLATED_VALUE, SCSS_INTERPOLATED_VALUE_PART_LIST, SCSS_INTERPOLATION,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress};

use super::function::parse_scss_function_call_from_name;

#[inline]
pub(crate) fn is_at_scss_interpolated_function_or_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || is_at_identifier_with_interpolation_suffix(p)
}

#[inline]
fn is_at_identifier_with_interpolation_suffix(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && is_nth_at_scss_interpolation(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

/// Parses an SCSS interpolation-led value and upgrades it to a function call
/// when the interpolation-shaped name is followed by `(`.
///
/// Examples:
///
/// ```scss
/// #{foo}(arg)
/// foo#{1 + 1}(arg)
/// #{$value}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_function_or_value(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_interpolated_function_or_value_until(p, |p| p.has_preceding_whitespace())
}

/// Parses `#{fn}(arg)`, `#{$value}`, or `#{x}0` with a caller-owned value
/// boundary.
///
/// The stop rule is checked before a bare interpolation grows into an
/// interpolated value.
///
/// Examples:
/// ```scss
/// #{fn}(arg)
/// #{$value}
/// #{x}0
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_function_or_value_until(
    p: &mut CssParser,
    should_stop: impl Fn(&mut CssParser) -> bool,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_function_or_value(p) {
        return Absent;
    }

    let head = match parse_scss_identifier_or_interpolation(p) {
        Present(head) => head,
        Absent => return Absent,
    };

    match head.kind(p) {
        SCSS_INTERPOLATED_IDENTIFIER => {
            // Adjacent identifier fragments are already part of the name; an
            // immediate `(` is the only remaining function-call signal.
            if p.at(T!['(']) {
                parse_scss_function_call_from_name(p, head)
            } else {
                Present(head)
            }
        }
        SCSS_INTERPOLATION => {
            // A bare interpolation needs one more decision point because it
            // can stand alone, become a function name, or start a value chain.
            if p.at(T!['(']) {
                // `#{fn}(` needs an interpolated identifier as the function name.
                let list = head
                    .precede(p)
                    .complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
                let name = list.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER);
                parse_scss_function_call_from_name(p, name)
            } else if !should_stop(p) && is_at_scss_interpolated_value_suffix(p) {
                Present(parse_scss_interpolated_value(p, head, should_stop))
            } else {
                Present(head)
            }
        }
        _ => Present(head),
    }
}

/// Returns true when the current token can continue an interpolated SCSS value
/// after a head has already been parsed.
///
/// This intentionally does not check whitespace or expression boundaries. The
/// caller owns those stop rules because expression parsing and regular value
/// parsing stop in different places.
///
/// Examples:
/// ```scss
/// 10#{unit}
/// $value#{suffix}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn is_at_scss_interpolated_value_suffix(p: &mut CssParser) -> bool {
    is_at_scss_namespaced_variable(p)
        || is_at_scss_variable(p)
        || is_at_scss_interpolation(p)
        || is_at_any_dimension(p)
        || p.at(CSS_NUMBER_LITERAL)
        || is_at_identifier(p)
}

/// Parses a `ScssInterpolatedValue` from an already parsed first part by
/// consuming adjacent suffix parts until a caller-defined stop condition.
///
/// Example:
/// ```scss
/// 10#{foo}px
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_value(
    p: &mut CssParser,
    first_part: CompletedMarker,
    should_stop: impl Fn(&mut CssParser) -> bool,
) -> CompletedMarker {
    let list = first_part.precede(p);
    let mut progress = ParserProgress::default();

    // The caller owns the stop condition because expression parsing must stop
    // before binary operators, while regular value parsing only cares about
    // adjacency/trivia boundaries.
    while !should_stop(p) && is_at_scss_interpolated_value_suffix(p) {
        progress.assert_progressing(p);
        parse_scss_interpolated_value_suffix_part(p).ok();
    }

    list.complete(p, SCSS_INTERPOLATED_VALUE_PART_LIST)
        .precede(p)
        .complete(p, SCSS_INTERPOLATED_VALUE)
}

#[inline]
fn parse_scss_interpolated_value_suffix_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_namespaced_variable(p) {
        parse_scss_namespaced_variable(p)
    } else if is_at_scss_variable(p) {
        parse_scss_variable(p)
    } else if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else if is_at_any_dimension(p) {
        parse_any_dimension(p)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_regular_number(p)
    } else if is_at_identifier(p) {
        parse_regular_identifier(p)
    } else {
        Absent
    }
}

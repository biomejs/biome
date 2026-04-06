use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::property::{is_at_generic_delimiter, parse_generic_component_value};
use crate::syntax::scss::expression::precedence::parse_scss_binary_expression;
use crate::syntax::scss::{
    END_OF_SCSS_EXPRESSION_TOKEN_SET, expected_scss_expression, is_at_scss_identifier,
    parse_scss_identifier, scss_ellipsis_not_allowed,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_PROPERTY_VALUE, EOF, SCSS_ARBITRARY_ARGUMENT, SCSS_EXPRESSION,
    SCSS_EXPRESSION_ITEM_LIST, SCSS_KEYWORD_ARGUMENT, SCSS_LIST_EXPRESSION,
    SCSS_LIST_EXPRESSION_ELEMENT, SCSS_LIST_EXPRESSION_ELEMENT_LIST, SCSS_STRING_QUOTE,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress, TokenSet, token_set};

use super::{ScssExpressionOptions, is_at_scss_expression_end};

pub(super) const SCSS_LIST_EXPRESSION_ELEMENT_END_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')']];

/// Parses a SCSS expression, deferring the list-vs-expression decision until a
/// comma is seen.
///
/// Example:
/// ```scss
/// margin: 1px 2px, 3px 4px;
/// ```
///
/// Docs: https://sass-lang.com/documentation/values/lists
#[inline]
pub(crate) fn parse_scss_expression(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_until(p, END_OF_SCSS_EXPRESSION_TOKEN_SET)
}

/// Parses a SCSS value that may be empty, returning `Absent` when no expression
/// content was produced.
#[inline]
pub(crate) fn parse_scss_optional_value_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    let options = ScssExpressionOptions::optional_value(end_ts);

    if is_at_scss_expression_sequence_end(p, options) {
        return Absent;
    }

    parse_scss_expression_with_options(p, options)
}

/// Parses a required SCSS value and recovers missing content as an empty
/// `ScssExpression` node plus a diagnostic.
///
/// Example:
/// ```scss
/// color: ;
/// ```
#[inline]
pub(crate) fn parse_required_scss_value_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> CompletedMarker {
    match parse_scss_optional_value_until(p, end_ts) {
        Present(value) => value,
        Absent => {
            let empty_expression = complete_empty_scss_expression(p);
            p.error(expected_component_value(p, p.cur_range()));
            empty_expression
        }
    }
}

/// Parses a SCSS expression until a caller-provided terminator, used by map
/// pairs and other contexts that embed expressions in larger constructs.
///
/// Example:
/// ```scss
/// $map: (a: 1);
/// ```
///
/// Docs: https://sass-lang.com/documentation/syntax/structure
#[inline]
pub(crate) fn parse_scss_expression_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::value(end_ts))
}

/// Parses arguments where `...` may terminate or expand the list and keyword
/// arguments are legal.
///
/// Example:
/// ```scss
/// @include foo($args..., $kw: 1);
/// ```
///
/// Docs:
/// - https://sass-lang.com/documentation/at-rules/mixin/
/// - https://sass-lang.com/documentation/at-rules/function/
#[inline]
pub(crate) fn parse_scss_expression_in_args_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::args(end_ts))
}

/// Parses a SCSS variable value, allowing `!important` to remain in the value
/// while still stopping before trailing variable modifiers like `!default`.
#[inline]
pub(crate) fn parse_scss_expression_in_variable_value_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::variable_value(end_ts))
}

#[inline]
pub(super) fn parse_scss_inner_expression_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::value(end_ts))
}

#[inline]
pub(crate) fn parse_scss_inner_expression_in_string_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_scss_expression_with_options(p, ScssExpressionOptions::value_in_string(end_ts))
}

#[inline]
pub(crate) fn complete_empty_scss_expression(p: &mut CssParser) -> CompletedMarker {
    let expression = p.start();
    let expression_items = p.start();
    expression_items.complete(p, SCSS_EXPRESSION_ITEM_LIST);
    expression.complete(p, SCSS_EXPRESSION)
}

/// Shared entrypoint so map pairs, arglists, and declarations can reuse the
/// same list/spacing ambiguity logic without speculative parsing.
///
/// Example:
/// ```scss
/// $map: (a: 1, b: 2);
/// ```
///
/// Docs: https://sass-lang.com/documentation/syntax/structure
#[inline]
fn parse_scss_expression_with_options(
    p: &mut CssParser,
    options: ScssExpressionOptions,
) -> ParsedSyntax {
    if is_at_scss_expression_sequence_end(p, options) {
        return if options.allows_empty_value {
            Present(complete_empty_scss_expression(p))
        } else {
            Absent
        };
    }

    let Present(first_expression) = parse_scss_expression_sequence(p, options) else {
        return Absent;
    };

    if !options.comma_separates_list() || !p.at(T![,]) {
        return Present(first_expression);
    }

    let first_element = complete_scss_list_expression_element(p, first_expression);
    let list_expression = complete_scss_list_expression(p, first_element, options);
    Present(complete_scss_expression_from_list(p, list_expression))
}

#[inline]
fn parse_scss_expression_sequence(
    p: &mut CssParser,
    options: ScssExpressionOptions,
) -> ParsedSyntax {
    if is_at_scss_expression_sequence_end(p, options) {
        return Absent;
    }

    let expression = p.start();
    let expression_items = p.start();
    let mut progress = ParserProgress::default();

    while !is_at_scss_expression_sequence_end(p, options) {
        progress.assert_progressing(p);

        let parsed_item = parse_scss_expression_item(p, options);
        if parsed_item
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, options.recovery_end_ts())
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
fn is_at_scss_expression_sequence_end(p: &mut CssParser, options: ScssExpressionOptions) -> bool {
    p.at(EOF)
        || is_at_scss_expression_end(p, options)
        || (options.comma_separates_list() && p.at(T![,]))
        || (options.stops_at_string_quote
            && p.at(SCSS_STRING_QUOTE)
            && !p.is_at_scss_interpolated_string())
}

#[inline]
fn parse_scss_expression_item(p: &mut CssParser, options: ScssExpressionOptions) -> ParsedSyntax {
    if is_at_scss_keyword_argument(p, options) {
        return parse_scss_keyword_argument(p, options);
    }

    if is_at_generic_delimiter(p) {
        return parse_generic_component_value(p);
    }

    let expression = parse_scss_binary_expression(p, 0).or_else(|| {
        if p.at(T![...]) {
            report_and_bump_scss_ellipsis(p);
        }

        Absent
    });
    let expression = match expression {
        Present(expression) => expression,
        Absent => return Absent,
    };

    if !p.at(T![...]) {
        return Present(expression);
    }

    if !options.allows_ellipsis {
        report_and_bump_scss_ellipsis(p);
        return Present(expression);
    }

    let m = expression.precede(p);
    p.bump(T![...]);
    Present(m.complete(p, SCSS_ARBITRARY_ARGUMENT))
}

#[inline]
fn report_and_bump_scss_ellipsis(p: &mut CssParser) {
    let range = p.cur_range();
    p.error(scss_ellipsis_not_allowed(p, range));
    p.bump(T![...]);
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
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
            stops_at_string_quote: false,
        },
    )
    .or_add_diagnostic(p, expected_component_value);

    Present(m.complete(p, SCSS_KEYWORD_ARGUMENT))
}

#[inline]
pub(super) fn complete_scss_list_expression(
    p: &mut CssParser,
    first_element: CompletedMarker,
    options: ScssExpressionOptions,
) -> CompletedMarker {
    let list_elements = first_element.precede(p);
    let mut progress = ParserProgress::default();

    while p.at(T![,]) {
        p.bump(T![,]);

        if p.at(T![,]) {
            let empty_expression = complete_empty_scss_expression(p);
            complete_scss_list_expression_element(p, empty_expression);
            continue;
        }

        if p.at(EOF) || is_at_scss_expression_end(p, options) {
            break;
        }

        progress.assert_progressing(p);

        if parse_scss_list_expression_element(p, options)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, options.recovery_end_ts())
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
        .complete(p, SCSS_LIST_EXPRESSION)
}

#[inline]
fn complete_scss_expression_from_list(
    p: &mut CssParser,
    list_expression: CompletedMarker,
) -> CompletedMarker {
    let expression_items = list_expression
        .precede(p)
        .complete(p, SCSS_EXPRESSION_ITEM_LIST);
    expression_items.precede(p).complete(p, SCSS_EXPRESSION)
}

#[inline]
fn parse_scss_list_expression_element(
    p: &mut CssParser,
    options: ScssExpressionOptions,
) -> ParsedSyntax {
    parse_scss_expression_sequence(p, options)
        .map(|expression| complete_scss_list_expression_element(p, expression))
}

#[inline]
pub(super) fn complete_scss_list_expression_element(
    p: &mut CssParser,
    expression: CompletedMarker,
) -> CompletedMarker {
    expression
        .precede(p)
        .complete(p, SCSS_LIST_EXPRESSION_ELEMENT)
}

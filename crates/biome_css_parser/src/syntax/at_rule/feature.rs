use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::{
    is_at_scss_binary_operator, is_at_scss_interpolated_identifier, is_at_scss_interpolation,
    parse_scss_expression_from_head, parse_scss_interpolated_name,
    parse_scss_interpolated_query_feature, parse_scss_interpolation_or_identifier,
};
use crate::syntax::{CssSyntaxFeatures, is_at_any_value, is_at_identifier, parse_any_value};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T, TextRange};
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Marker, Parser, SyntaxFeature, TokenSet, token_set};

/// Parses a CSS query feature used by `@media` and `@container`.
#[inline]
pub fn parse_any_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_interpolated_query_feature(p)
    } else if is_at_query_feature_name(p) {
        parse_named_query_feature(p)
    } else if is_at_any_query_feature_value(p) {
        parse_value_prefixed_query_feature(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_query_feature_name(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_identifier(p) || is_at_identifier(p)
}

/// Parses a query feature that starts with a feature name.
///
/// This covers boolean features such as `color`, plain features such as
/// `(width: 500px)`, and name-first range features such as `(width <= 500px)`.
#[inline]
fn parse_named_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_name(p) {
        return Absent;
    }

    let m = p.start();
    parse_query_feature_name(p).or_add_diagnostic(p, expected_identifier);

    parse_query_feature_from_name(p, m)
}

#[inline]
pub(crate) fn parse_query_feature_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_name(p) {
        return Absent;
    }

    parse_scss_interpolated_name(p)
}

/// Parses a query feature after its name is already parsed.
///
/// Examples: `(color)`, `(width: 500px)`, or `(width <= #{$value})`.
#[inline]
pub(crate) fn parse_query_feature_from_name(p: &mut CssParser, m: Marker) -> ParsedSyntax {
    let kind = if is_at_query_feature_range_comparison(p) {
        // Checked by `is_at_query_feature_range_comparison` above.
        parse_query_feature_range_comparison(p).ok();
        parse_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);
        CSS_QUERY_FEATURE_RANGE
    } else if p.at(T![:]) {
        p.bump(T![:]);
        parse_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);
        CSS_QUERY_FEATURE_PLAIN
    } else {
        CSS_QUERY_FEATURE_BOOLEAN
    };

    Present(m.complete(p, kind))
}

/// Parses a query feature that starts with a feature value.
///
/// This covers reverse ranges such as `(500px <= width)` and interval ranges
/// such as `(500px <= width <= 1000px)`.
#[inline]
fn parse_value_prefixed_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_query_feature_value(p) {
        return Absent;
    }

    let m = p.start();

    // Guarded by `is_at_any_query_feature_value` above.
    parse_query_feature_value_until(p, QUERY_FEATURE_RANGE_VALUE_END_SET).ok();
    parse_query_feature_range_comparison(p)
        .or_add_diagnostic(p, expected_query_feature_range_comparison);
    parse_query_feature_name(p).or_add_diagnostic(p, expected_identifier);

    parse_query_feature_interval_end(p, m)
}

/// Parses the optional upper bound after `value <= name`.
///
/// Examples: `@media (#{$min} <= width) {}` or
/// `@media (#{$min} <= width <= #{$max}) {}`.
#[inline]
pub(crate) fn parse_query_feature_interval_end(p: &mut CssParser, m: Marker) -> ParsedSyntax {
    if is_at_query_feature_range_comparison(p) {
        // Checked by `is_at_query_feature_range_comparison` above.
        parse_query_feature_range_comparison(p).ok();
        parse_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);

        Present(m.complete(p, CSS_QUERY_FEATURE_RANGE_INTERVAL))
    } else {
        Present(m.complete(p, CSS_QUERY_FEATURE_REVERSE_RANGE))
    }
}

#[inline]
pub(crate) fn is_at_query_feature_range_comparison(p: &mut CssParser) -> bool {
    p.at_ts(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
}

#[inline]
pub(crate) fn parse_query_feature_range_comparison(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_range_comparison(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET);

    Present(m.complete(p, CSS_QUERY_FEATURE_RANGE_COMPARISON))
}

const QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![<], T![>=], T![<=], T![=]];
const QUERY_FEATURE_VALUE_END_SET: TokenSet<CssSyntaxKind> = token_set![T![')']];
const QUERY_FEATURE_RANGE_VALUE_END_SET: TokenSet<CssSyntaxKind> =
    QUERY_FEATURE_VALUE_END_SET.union(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET);
const QUERY_FEATURE_VALUE_MISSING_RHS_SET: TokenSet<CssSyntaxKind> =
    token_set![T![')'], T![']'], T!['}'], T![,], EOF];

/// Returns whether the current token can start a query-feature value.
///
/// Example: `$breakpoint` in `@media (max-width: $breakpoint) {}`.
#[inline]
pub(crate) fn is_at_any_query_feature_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || is_at_any_value(p)
}

/// Parses the CSS-compatible head of a query-feature value.
///
/// Example: `#{$width}` in `@container (max-width: #{$width} + 1px) {}`.
#[inline]
fn parse_any_query_feature_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_interpolation_or_identifier(p)
    } else {
        // The concrete query-feature value grammar is narrower than `AnyCssValue`,
        // but this shared value parser gives us the right recovery for the current
        // media/container feature branches.
        parse_any_value(p)
    }
}

/// Parses a query-feature value up to the closing parenthesis.
///
/// Example: `#{$width} + 1px` in `@media (max-width: #{$width} + 1px) {}`.
#[inline]
fn parse_query_feature_value(p: &mut CssParser) -> ParsedSyntax {
    parse_query_feature_value_until(p, QUERY_FEATURE_VALUE_END_SET)
}

/// Parses a query-feature value, using the CSS-compatible head first and
/// switching to a SassScript tail only when a Sass operator follows.
///
/// Example: `500px + 100px` in `@media (500px + 100px < width) {}`.
#[inline]
fn parse_query_feature_value_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    let Present(head) = parse_any_query_feature_value(p) else {
        return Absent;
    };

    if CssSyntaxFeatures::Scss.is_unsupported(p)
        || !is_at_scss_query_feature_value_tail(p, &head, end_ts)
    {
        return Present(head);
    }

    parse_scss_expression_from_head(p, head, end_ts)
}

/// Returns whether a parsed query-feature value head has a SassScript tail.
///
/// Example: `+ 1px` after `$width` in `@media (max-width: $width + 1px) {}`.
#[inline]
fn is_at_scss_query_feature_value_tail(
    p: &mut CssParser,
    head: &CompletedMarker,
    end_ts: TokenSet<CssSyntaxKind>,
) -> bool {
    // Keep complete CSS values like `aspect-ratio: 16 / 9` on the CSS path.
    // If the parsed head stops on a Sass operator such as `+`, continue from
    // that head as SassScript without reparsing it.
    if p.at_ts(end_ts) || !is_at_scss_binary_operator(p) {
        return false;
    }

    if p.at(T![/]) && !is_scss_query_feature_value_head(head.kind(p)) {
        return false;
    }

    // Keep dangling operators such as `scroll-state(scrolled: bottom and )`
    // on the CSS/container recovery path instead of parsing them as Sass.
    !p.nth_at_ts(1, QUERY_FEATURE_VALUE_MISSING_RHS_SET.union(end_ts))
}

#[inline]
fn is_scss_query_feature_value_head(kind: CssSyntaxKind) -> bool {
    matches!(
        kind,
        SCSS_INTERPOLATED_IDENTIFIER | SCSS_INTERPOLATION | SCSS_VARIABLE
    )
}

pub(crate) fn expected_any_query_feature(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "<mf-plain>",
            "<mf-boolean>",
            "<mf-range>",
            "<query-in-parens> or <query-in-parens>",
        ],
        range,
    )
    .into_diagnostic(p)
}

fn expected_query_feature_range_comparison(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["<", "<=", "=", ">=", ">"], range).into_diagnostic(p)
}

fn expected_query_feature_value(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_component_value(p, range)
}

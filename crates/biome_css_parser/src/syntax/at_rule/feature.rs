use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_error::scss_only_syntax_error;
use crate::syntax::scss::{
    complete_scss_expression_from_item, complete_scss_interpolated_identifier,
    is_at_scss_binary_operator, is_at_scss_interpolation, is_at_scss_namespaced_variable,
    is_at_scss_variable, is_nth_at_scss_interpolation, parse_scss_expression_from_head,
    parse_scss_expression_until, parse_scss_interpolated_name,
    parse_scss_interpolated_query_feature, parse_scss_interpolation_or_identifier,
    parse_scss_variable,
};
use crate::syntax::value::function::is_nth_at_function;
use crate::syntax::{
    CssSyntaxFeatures, is_at_any_value, is_at_dashed_identifier, is_at_identifier, parse_any_value,
    parse_dashed_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{
    AnyCssQueryFeatureName, AnyCssQueryFeatureValue, CssSyntaxKind, T, TextRange,
};
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Marker, Parser, SyntaxFeature, TokenSet, token_set};
use biome_rowan::{AstNode, SyntaxKind};

/// Parses a query feature without computed SCSS names.
///
/// ```css
/// @container (width > 40rem) {}
/// ```
#[inline]
pub fn parse_any_query_feature(p: &mut CssParser) -> ParsedSyntax {
    parse_query_feature(p, false)
}

/// Allows computed names in media features and CSS-import media queries.
///
/// ```scss
/// @media ($bound + "-width": 40rem) {}
/// ```
#[inline]
pub(crate) fn parse_media_query_feature(p: &mut CssParser) -> ParsedSyntax {
    let is_computed_name_allowed = !p.state().is_in_if_media_test;
    parse_query_feature(p, is_computed_name_allowed)
}

#[inline]
fn parse_query_feature(p: &mut CssParser, is_computed_name_allowed: bool) -> ParsedSyntax {
    if !is_at_query_feature(p) {
        return Absent;
    }

    if is_computed_name_allowed && is_at_scss_variable_query_feature(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_variable_query_feature,
            expected_scss_computed_query_feature,
        )
    } else if is_at_scss_interpolation(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_interpolated_query_feature,
            |p, marker| {
                scss_only_syntax_error(p, "SCSS interpolated query features", marker.range(p))
            },
        )
    } else if is_at_scss_namespaced_variable(p)
        || (is_computed_name_allowed && is_nth_at_function(p, 0))
    {
        parse_value_prefixed_query_feature(p, is_computed_name_allowed)
    } else if is_at_query_feature_name(p) {
        parse_named_query_feature(p, is_computed_name_allowed)
    } else {
        parse_value_prefixed_query_feature(p, is_computed_name_allowed)
    }
}

/// Recognizes a feature name or a lower range bound.
///
/// ```scss
/// @media (width: 40rem), (20rem < $feature) {}
/// ```
#[inline]
fn is_at_query_feature(p: &mut CssParser) -> bool {
    is_at_query_feature_name(p) || is_at_any_query_feature_value(p)
}

#[inline]
fn is_at_query_feature_name(p: &mut CssParser) -> bool {
    is_at_scss_variable(p) || is_at_identifier(p) || is_at_scss_interpolation(p)
}

/// Parses a query feature that starts with an identifier or variable.
///
/// This covers boolean features such as `color`, plain features such as
/// `(width: 500px)`, and name-first range features such as `(width <= 500px)`.
/// Variables can also start reverse ranges and intervals such as `($min < width)`.
#[inline]
fn parse_named_query_feature(p: &mut CssParser, is_computed_name_allowed: bool) -> ParsedSyntax {
    if !is_at_query_feature_name(p) {
        return Absent;
    }

    let m = p.start();
    let Present(name) = parse_query_feature_name(p) else {
        m.abandon(p);
        return Absent;
    };

    if is_computed_name_allowed
        && is_at_scss_query_feature_expression_tail(p)
        // `screen and (color)` must retain CSS media-condition recovery.
        && (!p.at_ts(token_set![T![and], T![or]]) || CssSyntaxFeatures::Scss.is_supported(p))
    {
        m.abandon(p);
        return CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| parse_scss_query_feature_from_head(p, name),
            expected_scss_computed_query_feature,
        );
    }

    if name.kind(p) != SCSS_VARIABLE {
        return parse_query_feature_from_name(p, m);
    }

    if !is_at_query_feature_range_comparison(p) {
        if !is_at_scss_binary_operator(p) {
            return parse_query_feature_from_name(p, m);
        }

        parse_query_feature_expression_from_head(p, name, QUERY_FEATURE_RANGE_VALUE_END_SET).ok();
        return parse_query_feature_after_value(p, m);
    }

    // The non-comparison branch returns above, so this always returns `Present`.
    parse_query_feature_range_comparison(p).ok();
    let value = parse_query_feature_value_until(p, QUERY_FEATURE_RANGE_VALUE_END_SET)
        .or_add_diagnostic(p, expected_query_feature_value);
    if let Some(value) = value {
        match value.kind(p) {
            CSS_IDENTIFIER | SCSS_INTERPOLATED_IDENTIFIER => {
                return parse_query_feature_interval_end(p, m);
            }
            SCSS_VARIABLE | SCSS_INTERPOLATION if is_at_query_feature_range_comparison(p) => {
                if value.kind(p) == SCSS_INTERPOLATION {
                    complete_scss_interpolated_identifier(p, value);
                }
                return parse_query_feature_interval_end(p, m);
            }
            _ => {}
        }
    }

    // Two variables alone remain ambiguous, so retain the name-first shape.
    Present(m.complete(p, CSS_QUERY_FEATURE_RANGE))
}

#[inline]
pub(crate) fn parse_query_feature_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_name(p) {
        return Absent;
    }

    if is_at_scss_variable(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(p, parse_scss_variable, |p, m| {
            scss_only_syntax_error(p, "SCSS variables", m.range(p))
        })
    } else if is_at_scss_interpolated_query_feature_name(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_interpolated_name,
            |p, marker| {
                scss_only_syntax_error(p, "SCSS interpolated query feature names", marker.range(p))
            },
        )
    } else if is_at_dashed_identifier(p) {
        parse_dashed_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
}

/// Parses a query feature after its name is already parsed.
///
/// Examples: `(color)`, `(width: 500px)`, or `(width <= #{$value})`.
#[inline]
pub(crate) fn parse_query_feature_from_name(p: &mut CssParser, m: Marker) -> ParsedSyntax {
    let kind = if is_at_query_feature_range_comparison(p) {
        // Checked by `is_at_query_feature_range_comparison` above.
        parse_query_feature_range_comparison(p).ok();
        parse_query_feature_value_until(p, QUERY_FEATURE_RANGE_VALUE_END_SET)
            .or_add_diagnostic(p, expected_query_feature_value);
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
fn parse_value_prefixed_query_feature(
    p: &mut CssParser,
    is_computed_name_allowed: bool,
) -> ParsedSyntax {
    if !is_at_any_query_feature_value(p) {
        return Absent;
    }

    let m = p.start();

    if !is_computed_name_allowed {
        parse_query_feature_value_until(p, QUERY_FEATURE_RANGE_VALUE_END_SET).ok();
        return parse_query_feature_after_value(p, m);
    }

    let Present(head) = parse_query_feature_value_until(p, QUERY_FEATURE_HEAD_END_SET) else {
        return parse_query_feature_after_value(p, m);
    };

    if p.at_ts(token_set![T![:], T![')']]) {
        return CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| {
                complete_query_feature_name(p, head);
                parse_query_feature_from_name(p, m)
            },
            expected_scss_computed_query_feature,
        );
    }

    if matches!(head.kind(p), CSS_STRING | SCSS_INTERPOLATED_STRING) {
        return CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| {
                let head = complete_scss_expression_from_item(p, head);
                parse_media_query_feature_range_from_head(p, m, head)
            },
            expected_scss_computed_query_feature,
        );
    }

    if !AnyCssQueryFeatureValue::can_cast(head.kind(p)) && !head.kind(p).is_bogus() {
        return parse_query_feature_after_value(p, m);
    }

    parse_media_query_feature_range_from_head(p, m, head)
}

/// Recognizes a local or namespaced variable at the start of a computed feature.
///
/// ```scss
/// @media ($feature: 40rem), (breakpoints.$feature: 40rem) {}
/// ```
#[inline]
fn is_at_scss_variable_query_feature(p: &mut CssParser) -> bool {
    is_at_scss_variable(p) || is_at_scss_namespaced_variable(p)
}

/// Parses a variable-led feature without committing to a name or bound.
///
/// ```scss
/// @media ($feature + "" >= 40rem) {}
/// @media ($min + 1rem < width) {}
/// ```
fn parse_scss_variable_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_variable_query_feature(p) {
        return Absent;
    }

    if is_at_scss_variable(p) {
        parse_scss_variable(p).and_then(|head| parse_scss_query_feature_from_head(p, head))
    } else {
        parse_value_prefixed_query_feature(p, true)
    }
}

/// Continues a computed media name or bound up to its query delimiter.
///
/// ```scss
/// @media ($bound + "-width": 40rem) {}
/// @media ($feature + "" >= 40rem) {}
/// ```
pub(crate) fn parse_scss_query_feature_from_head(
    p: &mut CssParser,
    head: CompletedMarker,
) -> ParsedSyntax {
    let head = if is_at_scss_query_feature_expression_tail(p) {
        parse_scss_expression_from_head(
            p,
            head,
            QUERY_FEATURE_HEAD_END_SET.union(token_set![T!['{']]),
        )
    } else {
        Present(head)
    };

    head.and_then(|head| {
        let m = head.clone().precede(p);
        if !is_at_query_feature_range_comparison(p) {
            complete_query_feature_name(p, head);
            parse_query_feature_from_name(p, m)
        } else {
            parse_media_query_feature_range_from_head(p, m, head)
        }
    })
}

/// Recognizes a Sass operator within an operand, excluding range comparisons.
///
/// ```scss
/// @media ($feature + "" >= 40rem) {}
/// ```
#[inline]
fn is_at_scss_query_feature_expression_tail(p: &mut CssParser) -> bool {
    is_at_scss_binary_operator(p) && !is_at_query_feature_range_comparison(p)
}

/// Parses the right operand after a consumed head, then chooses the range shape.
/// A missing comparison or operand is diagnosed without abandoning the head.
///
/// ```scss
/// @media ($feature + "" >= 40rem) {}
/// @media (20rem < $feature + "" <= 80rem) {}
/// ```
fn parse_media_query_feature_range_from_head(
    p: &mut CssParser,
    m: Marker,
    left: CompletedMarker,
) -> ParsedSyntax {
    parse_query_feature_range_comparison(p)
        .or_add_diagnostic(p, expected_query_feature_range_comparison);
    let right = parse_query_feature_value_until(p, QUERY_FEATURE_HEAD_END_SET)
        .or_add_diagnostic(p, expected_query_feature_value);
    let Some(right) = right else {
        return Present(m.complete(p, CSS_QUERY_FEATURE_RANGE));
    };

    let is_reverse = is_at_query_feature_range_comparison(p)
        || !AnyCssQueryFeatureName::can_cast(left.kind(p))
        || matches!(
            right.kind(p),
            CSS_IDENTIFIER | CSS_DASHED_IDENTIFIER | SCSS_INTERPOLATED_IDENTIFIER
        );

    let name = if is_reverse {
        complete_query_feature_range_name(p, right)
    } else {
        complete_query_feature_range_value(p, right);
        // Dynamic operands cannot be evaluated here; retain the name-first shape.
        left
    };
    let has_scss_name =
        !matches!(name.kind(p), CSS_IDENTIFIER | CSS_DASHED_IDENTIFIER) && !name.kind(p).is_bogus();

    if has_scss_name {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| parse_query_feature_range_end(p, m, is_reverse),
            expected_scss_computed_query_feature,
        )
    } else {
        parse_query_feature_range_end(p, m, is_reverse)
    }
}

/// Validates the name in a reverse range or interval, retaining invalid syntax.
///
/// ```scss
/// @media (20rem < "#{$feature}" <= 80rem) {}
/// ```
fn complete_query_feature_range_name(
    p: &mut CssParser,
    mut name: CompletedMarker,
) -> CompletedMarker {
    if !AnyCssQueryFeatureName::can_cast(name.kind(p))
        && !matches!(name.kind(p), CSS_STRING | SCSS_INTERPOLATED_STRING)
        && !name.kind(p).is_bogus()
    {
        p.error(expected_identifier(p, name.range(p)));
        name.change_to_bogus(p);
    }
    complete_query_feature_name(p, name)
}

/// Wraps string bounds as Sass expressions and diagnoses invalid range values.
///
/// ```scss
/// @media ($feature >= "#{$width}") {}
/// ```
fn complete_query_feature_range_value(p: &mut CssParser, mut value: CompletedMarker) {
    if matches!(value.kind(p), CSS_STRING | SCSS_INTERPOLATED_STRING) {
        complete_scss_expression_from_item(p, value);
    } else if !AnyCssQueryFeatureValue::can_cast(value.kind(p)) && !value.kind(p).is_bogus() {
        p.error(expected_query_feature_value(p, value.range(p)));
        value.change_to_bogus(p);
    }
}

fn parse_query_feature_range_end(p: &mut CssParser, m: Marker, is_reverse: bool) -> ParsedSyntax {
    if is_reverse {
        parse_query_feature_interval_end(p, m)
    } else {
        Present(m.complete(p, CSS_QUERY_FEATURE_RANGE))
    }
}

/// Gives a computed operand the shared query-name shape.
///
/// ```scss
/// @media (20rem < #{$feature} <= 80rem) {}
/// ```
fn complete_query_feature_name(p: &mut CssParser, value: CompletedMarker) -> CompletedMarker {
    if AnyCssQueryFeatureName::can_cast(value.kind(p)) || value.kind(p).is_bogus() {
        value
    } else {
        complete_scss_expression_from_item(p, value)
    }
}

fn expected_scss_computed_query_feature(
    p: &CssParser,
    marker: &CompletedMarker,
) -> ParseDiagnostic {
    scss_only_syntax_error(p, "SCSS computed media features", marker.range(p))
}

/// Parses the comparison and feature name after a range's lower bound.
#[inline]
fn parse_query_feature_after_value(p: &mut CssParser, m: Marker) -> ParsedSyntax {
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
        parse_query_feature_value_until(p, QUERY_FEATURE_RANGE_VALUE_END_SET)
            .or_add_diagnostic(p, expected_query_feature_value);

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
const QUERY_FEATURE_HEAD_END_SET: TokenSet<CssSyntaxKind> =
    QUERY_FEATURE_RANGE_VALUE_END_SET.union(token_set![T![:], T![,], T![;], T!['}']]);
const QUERY_FEATURE_VALUE_MISSING_RHS_SET: TokenSet<CssSyntaxKind> =
    token_set![T![')'], T![']'], T!['}'], T![,], EOF];

/// Returns whether the current token can start a query-feature value.
///
/// Example: `$breakpoint` in `@media (max-width: $breakpoint) {}`.
#[inline]
pub(crate) fn is_at_any_query_feature_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || is_at_any_value(p) || p.at_ts(token_set![T![+], T![-]])
}

/// Parses the CSS-compatible head of a query-feature value.
///
/// Example: `#{$width}` in `@container (max-width: #{$width} + 1px) {}`.
#[inline]
fn parse_any_query_feature_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_interpolation_or_identifier,
            |p, marker| {
                scss_only_syntax_error(p, "SCSS interpolated query feature values", marker.range(p))
            },
        )
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

/// Parses a query-feature value, allowing parenthesized Sass expressions and
/// leading unary `+` or `-` in SCSS. Other values retain their CSS-compatible
/// head and switch to a SassScript tail only when a Sass operator follows.
///
/// Example: `500px + 100px` in `@media (500px + 100px < width) {}`.
#[inline]
fn parse_query_feature_value_until(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    if p.at(T!['(']) {
        return CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| parse_scss_expression_until(p, end_ts.union(token_set![T!['{']])),
            |p, marker| {
                scss_only_syntax_error(p, "SCSS parenthesized query values", marker.range(p))
            },
        );
    }

    if p.at_ts(token_set![T![+], T![-]]) && !is_at_any_value(p) {
        return CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| {
                parse_scss_expression_until(
                    p,
                    end_ts
                        .union(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
                        .union(token_set![T!['{']]),
                )
            },
            |p, marker| scss_only_syntax_error(p, "SCSS unary query values", marker.range(p)),
        );
    }

    let Present(head) = parse_any_query_feature_value(p) else {
        return Absent;
    };

    if !is_at_scss_query_feature_value_tail(p, &head, end_ts) {
        if head.kind(p) == SCSS_MODULE_MEMBER_ACCESS {
            // Module accesses are expression operands, not direct query-feature values.
            return Present(complete_scss_expression_from_item(p, head));
        }
        return Present(head);
    }

    parse_query_feature_expression_from_head(p, head, end_ts)
}

/// Continues a query value as a Sass expression up to the caller's delimiters.
#[inline]
fn parse_query_feature_expression_from_head(
    p: &mut CssParser,
    head: CompletedMarker,
    end_ts: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    CssSyntaxFeatures::Scss.parse_exclusive_syntax(
        p,
        |p| parse_scss_expression_from_head(p, head, end_ts.union(token_set![T!['{']])),
        |p, marker| scss_only_syntax_error(p, "SCSS query expressions", marker.range(p)),
    )
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

#[inline]
fn is_at_scss_interpolated_query_feature_name(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
        || is_at_identifier(p)
            && is_nth_at_scss_interpolation(p, 1)
            && !p.has_nth_preceding_whitespace(1)
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

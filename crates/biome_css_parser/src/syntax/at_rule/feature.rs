use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::{
    is_at_scss_interpolated_identifier, is_at_scss_interpolation,
    parse_scss_interpolated_identifier, parse_scss_regular_interpolation,
};
use crate::syntax::{
    CssSyntaxFeatures, is_at_any_value, is_at_identifier, parse_any_value, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T, TextRange};
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, TokenSet, token_set};

/// Parses a CSS query feature used by `@media` and `@container`.
#[inline]
pub fn parse_any_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_interpolated_query_feature(p)
    } else if is_at_query_feature_name(p) {
        parse_named_query_feature(p)
    } else if is_at_value_prefixed_query_feature(p) {
        parse_value_prefixed_query_feature(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_query_feature_name(p: &mut CssParser) -> bool {
    if CssSyntaxFeatures::Scss.is_supported(p) {
        is_at_scss_interpolated_identifier(p)
    } else {
        is_at_identifier(p)
    }
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

    let kind = if is_at_query_feature_range_comparison(p) {
        // Checked by `is_at_query_feature_range_comparison` above.
        parse_query_feature_range_comparison(p).ok();
        parse_any_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);
        CSS_QUERY_FEATURE_RANGE
    } else if p.at(T![:]) {
        p.bump(T![:]);
        parse_any_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);
        CSS_QUERY_FEATURE_PLAIN
    } else {
        CSS_QUERY_FEATURE_BOOLEAN
    };

    Present(m.complete(p, kind))
}

#[inline]
fn parse_query_feature_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_name(p) {
        return Absent;
    }

    if CssSyntaxFeatures::Scss.is_supported(p) {
        parse_scss_interpolated_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
}

#[inline]
fn parse_interpolated_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let feature_start = p.checkpoint();

    // Interpolation can start either a feature name or a feature value.
    // Classify from what follows the interpolation so we only speculate over
    // the ambiguous prefix instead of the whole feature.
    // Guarded by `is_at_scss_interpolation` above.
    parse_scss_regular_interpolation(p).ok();

    if p.at(T![:]) || !is_at_query_feature_range_comparison(p) {
        p.rewind(feature_start);
        return parse_named_query_feature(p);
    }

    // Checked by `is_at_query_feature_range_comparison` above.
    parse_query_feature_range_comparison(p).ok();

    if !is_at_query_feature_name(p) {
        p.rewind(feature_start);
        return parse_named_query_feature(p);
    }

    // Checked by `is_at_query_feature_name` above.
    parse_query_feature_name(p).ok();
    p.rewind(feature_start);

    // For interpolation-first forms like `#{$min} <= width`, prefer the
    // value-first interpretation so the canonical identifier in the middle
    // becomes the feature name.
    parse_value_prefixed_query_feature(p)
}

#[inline]
fn is_at_value_prefixed_query_feature(p: &mut CssParser) -> bool {
    is_at_any_query_feature_value(p)
}

/// Parses a query feature that starts with a feature value.
///
/// This covers reverse ranges such as `(500px <= width)` and interval ranges
/// such as `(500px <= width <= 1000px)`.
#[inline]
fn parse_value_prefixed_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_value_prefixed_query_feature(p) {
        return Absent;
    }

    let m = p.start();

    // Guarded by `is_at_value_prefixed_query_feature` above.
    parse_any_query_feature_value(p).ok();
    parse_query_feature_range_comparison(p)
        .or_add_diagnostic(p, expected_query_feature_range_comparison);
    parse_query_feature_name(p).or_add_diagnostic(p, expected_identifier);

    if is_at_query_feature_range_comparison(p) {
        // Checked by `is_at_query_feature_range_comparison` above.
        parse_query_feature_range_comparison(p).ok();
        parse_any_query_feature_value(p).or_add_diagnostic(p, expected_query_feature_value);

        Present(m.complete(p, CSS_QUERY_FEATURE_RANGE_INTERVAL))
    } else {
        Present(m.complete(p, CSS_QUERY_FEATURE_REVERSE_RANGE))
    }
}

#[inline]
fn is_at_query_feature_range_comparison(p: &mut CssParser) -> bool {
    p.at_ts(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
}

#[inline]
fn parse_query_feature_range_comparison(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_range_comparison(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET);

    Present(m.complete(p, CSS_QUERY_FEATURE_RANGE_COMPARISON))
}

const QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![<], T![>=], T![<=], T![=]];

#[inline]
pub(crate) fn is_at_any_query_feature_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || is_at_any_value(p)
}

#[inline]
fn parse_any_query_feature_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolated_query_feature_value(p) {
        parse_scss_interpolated_identifier(p)
    } else if is_at_scss_interpolation(p) {
        parse_scss_regular_interpolation(p)
    } else {
        // The concrete query-feature value grammar is narrower than `AnyCssValue`,
        // but this shared value parser gives us the right recovery for the current
        // media/container feature branches.
        parse_any_value(p)
    }
}

#[inline]
fn is_at_scss_interpolated_query_feature_value(p: &mut CssParser) -> bool {
    if !CssSyntaxFeatures::Scss.is_supported(p) || !is_at_scss_interpolation(p) {
        return false;
    }

    let checkpoint = p.checkpoint();
    parse_scss_regular_interpolation(p).ok();
    let continues_as_identifier = is_at_scss_interpolated_identifier(p);
    p.rewind(checkpoint);

    continues_as_identifier
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

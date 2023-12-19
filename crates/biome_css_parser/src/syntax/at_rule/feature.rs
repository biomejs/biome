use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{is_at_any_value, is_at_identifier, parse_any_value, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub fn parse_any_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if is_at_query_feature_range(p) {
        parse_query_feature_range(p)
    } else if is_at_query_feature_plain(p) {
        parse_query_feature_plain(p)
    } else if is_at_query_feature_boolean(p) {
        parse_query_feature_boolean(p)
    } else if is_at_query_feature_reverse_or_interval_range(p) {
        parse_query_feature_reverse_or_interval_range(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_query_feature_boolean(p: &mut CssParser) -> bool {
    is_at_identifier(p)
}

#[inline]
fn parse_query_feature_boolean(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_boolean(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();

    Present(m.complete(p, CSS_QUERY_FEATURE_BOOLEAN))
}

#[inline]
fn is_at_query_feature_range(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at_ts(1, QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
}

#[inline]
fn parse_query_feature_range(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_range(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    parse_query_feature_range_comparison(p).ok();
    parse_any_query_feature_value(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_QUERY_FEATURE_RANGE))
}

#[inline]
fn is_at_query_feature_reverse_or_interval_range(p: &mut CssParser) -> bool {
    is_at_any_query_feature_value(p)
}

#[inline]
fn parse_query_feature_reverse_or_interval_range(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_reverse_or_interval_range(p) {
        return Absent;
    }

    let m = p.start();

    parse_any_query_feature_value(p).ok();
    parse_query_feature_range_comparison(p).ok();
    parse_regular_identifier(p).ok(); // TODO handle error

    if is_at_query_feature_range_comparison(p) {
        parse_query_feature_range_comparison(p).ok();
        parse_any_query_feature_value(p).ok(); // TODO handle error

        Present(m.complete(p, CSS_QUERY_FEATURE_RANGE_INTERVAL))
    } else {
        Present(m.complete(p, CSS_QUERY_FEATURE_REVERSE_RANGE))
    }
}

const QUERY_FEATURE_RANGE_COMPARISON_OPERATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![<], T![>=], T![<=], T![=]];

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

#[inline]
fn is_at_query_feature_plain(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

#[inline]
fn parse_query_feature_plain(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_query_feature_plain(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T![:]);
    parse_any_query_feature_value(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_QUERY_FEATURE_PLAIN))
}

#[inline]
pub(crate) fn is_at_any_query_feature_value(p: &mut CssParser) -> bool {
    is_at_any_value(p)
}

#[inline]
fn parse_any_query_feature_value(p: &mut CssParser) -> ParsedSyntax {
    // TODO add diagnostics if the any value is different with gramma
    parse_any_value(p)
}

use super::super::{
    is_at_scss_interpolation, is_nth_at_scss_interpolation, parse_scss_interpolation_or_identifier,
    parse_scss_regular_interpolation,
};
use super::query_feature::parse_scss_interpolated_query_feature_from_head;
use crate::parser::CssParser;
use crate::syntax::at_rule::error::AnyInParensChainParseRecovery;
use crate::syntax::at_rule::feature::expected_any_query_feature;
use crate::syntax::at_rule::media::{
    expected_any_media_in_parens, parse_any_media_condition_operand, parse_media_and_condition,
    parse_media_not_condition, parse_media_or_condition, recover_missing_and_rhs,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Returns true when Sass interpolation can start a media query operand.
#[inline]
pub(crate) fn is_at_scss_media_query(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
}

/// Parses a Sass interpolation that stands in for a media query operand.
///
/// Examples:
/// ```scss
/// @media #{$query} {}
/// @media (color) and #{$query} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/css/#media
#[inline]
pub(crate) fn parse_scss_media_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_media_query(p) {
        return Absent;
    }

    let m = p.start();
    // Guarded by `is_at_scss_media_query` above.
    parse_scss_regular_interpolation(p).ok();

    Present(m.complete(p, SCSS_MEDIA_QUERY))
}

/// Returns true when Sass interpolation starts a parenthesized media branch.
///
/// The parser still decides after the shared head whether this is a nested
/// media condition or a query feature.
#[inline]
pub(crate) fn is_at_scss_interpolated_media_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['(']) && is_nth_at_scss_interpolation(p, 1)
}

/// Parses an interpolation-led parenthesized media branch.
///
/// The shared interpolation head is parsed once, then completed as either a
/// nested media condition or a query feature after the following token is known.
///
/// Examples:
/// ```scss
/// @media (#{$query} and not (color)) {}
/// @media (#{$feature}: #{$value}) {}
/// ```
#[inline]
pub(crate) fn parse_scss_interpolated_media_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_media_in_parens(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let Some(head) =
        parse_scss_interpolation_or_identifier(p).or_add_diagnostic(p, expected_any_query_feature)
    else {
        p.expect(T![')']);
        return Present(m.complete(p, CSS_MEDIA_FEATURE_IN_PARENS));
    };

    let kind = if head.kind(p) == SCSS_INTERPOLATION && is_at_media_condition_operator(p) {
        let query = head.precede(p).complete(p, SCSS_MEDIA_QUERY);
        parse_scss_media_condition_from_query(p, query);
        CSS_MEDIA_CONDITION_IN_PARENS
    } else {
        parse_scss_interpolated_query_feature_from_head(p, head)
            .or_add_diagnostic(p, expected_any_query_feature);
        CSS_MEDIA_FEATURE_IN_PARENS
    };

    p.expect(T![')']);

    Present(m.complete(p, kind))
}

/// Parses a Sass interpolation-led media condition.
///
/// Examples:
/// ```scss
/// @media (#{$query} and not (color)) {}
/// @media (#{$query} or (color)) {}
/// ```
#[inline]
pub(crate) fn parse_scss_media_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_media_condition(p) {
        return Absent;
    }

    parse_scss_media_query(p).map(|query| parse_scss_media_condition_from_query(p, query))
}

#[inline]
pub(crate) fn is_at_scss_media_condition(p: &mut CssParser) -> bool {
    is_at_scss_media_query(p)
}

/// Parses a media query list item that starts with Sass interpolation.
///
/// Examples:
/// ```scss
/// @media #{$query} {}
/// @media #{$query} and not (color) {}
/// @media #{$query} or (color) {}
/// ```
#[inline]
pub(crate) fn parse_scss_media_query_or_condition_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_media_query(p) {
        return Absent;
    }

    parse_scss_media_query(p).map(|query| {
        if is_at_media_condition_operator(p) {
            parse_scss_media_condition_from_query(p, query)
                .precede(p)
                .complete(p, CSS_MEDIA_CONDITION_QUERY)
        } else {
            query
        }
    })
}

#[inline]
fn parse_scss_media_condition_from_query(
    p: &mut CssParser,
    query: CompletedMarker,
) -> CompletedMarker {
    match p.cur() {
        T![and] => parse_scss_media_and_condition(p, query),
        T![or] => parse_media_or_condition(p, query),
        _ => query,
    }
}

#[inline]
fn is_at_media_condition_operator(p: &mut CssParser) -> bool {
    p.at(T![and]) || p.at(T![or])
}

/// Parses an `and` chain after an interpolation-led media query.
///
/// Sass accepts `@media #{a} and not (b)`, but plain media conditions such as
/// `@media (a) and not (b)` must keep recovering as invalid. This helper is the
/// only place where `not` is accepted as an `and` RHS outside the media-type
/// query path.
#[inline]
fn parse_scss_media_and_condition(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![and]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_any_scss_media_and_combinable_condition(p)
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![and]).with_stop_kind(T![')']),
            expected_any_media_in_parens,
        )
        .map(|rhs| parse_media_and_condition(p, rhs));

    recover_missing_and_rhs(p, recovery_result);

    m.complete(p, CSS_MEDIA_AND_CONDITION)
}

/// Parses the RHS of an interpolation-led `and` chain.
///
/// This is wider than a normal CSS `and` RHS because Sass accepts `not` after
/// an interpolation-led media query.
#[inline]
fn parse_any_scss_media_and_combinable_condition(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![not]) {
        parse_media_not_condition(p)
    } else {
        parse_any_media_condition_operand(p).map(|rhs| parse_media_and_condition(p, rhs))
    }
}

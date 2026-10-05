use super::super::{
    complete_scss_interpolated_identifier, is_at_scss_interpolation, is_nth_at_scss_interpolation,
    is_nth_source_tight, parse_scss_interpolation_or_identifier, parse_scss_regular_interpolation,
};
use crate::parser::CssParser;
use crate::syntax::at_rule::container::{complete_any_container_query, is_at_any_container_query};
use crate::syntax::{CssSyntaxFeatures, is_at_identifier};
use biome_css_syntax::CssSyntaxKind::{SCSS_CONTAINER_INTERPOLATED_QUERY, SCSS_INTERPOLATION};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Returns whether a Sass-interpolated container name or query starts the
/// `@container` condition.
///
/// Examples:
/// ```scss
/// @container #{$name} (width > 400px) {}
/// @container sidebar-#{$name} (width > 400px) {}
/// @container #{$query} {}
/// ```
#[inline]
pub(crate) fn is_at_scss_container_name_or_query(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_at_scss_interpolation(p)
            || (is_at_identifier(p)
                && is_nth_at_scss_interpolation(p, 1)
                && is_nth_source_tight(p, 1)))
}

/// Parses the Sass-interpolated head of an `@container` condition, which is
/// either the container name or the whole container query.
///
/// A standalone interpolation is a container name only when a container query
/// follows it. Otherwise, it stands in for the query itself, because the name
/// is optional but the query is required.
///
/// Returns a `ScssInterpolatedIdentifier` for a name, and a container query
/// node otherwise.
///
/// Examples:
/// ```scss
/// @container #{$name} (width > 400px) {}
/// @container #{$query} {}
/// @container #{$query} and (width > 400px) {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/css/
#[inline]
pub(crate) fn parse_scss_container_name_or_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_container_name_or_query(p) {
        return Absent;
    }

    let Present(head) = parse_scss_interpolation_or_identifier(p) else {
        return Absent;
    };

    if head.kind(p) != SCSS_INTERPOLATION {
        return Present(head);
    }

    if is_at_any_container_query(p) {
        return Present(complete_scss_interpolated_identifier(p, head));
    }

    let query = head
        .precede(p)
        .complete(p, SCSS_CONTAINER_INTERPOLATED_QUERY);

    Present(complete_any_container_query(p, query))
}

/// Returns whether Sass interpolation can stand in for a container query operand.
#[inline]
pub(crate) fn is_at_scss_container_interpolated_query(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
}

/// Parses Sass interpolation standing in for a complete container query operand.
///
/// Examples:
/// ```scss
/// @container not #{$query} {}
/// @container (width > 400px) or #{$query} {}
/// ```
#[inline]
pub(crate) fn parse_scss_container_interpolated_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_container_interpolated_query(p) {
        return Absent;
    }

    let m = p.start();
    parse_scss_regular_interpolation(p).ok();

    Present(m.complete(p, SCSS_CONTAINER_INTERPOLATED_QUERY))
}

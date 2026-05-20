use crate::parser::CssParser;
use crate::syntax::at_rule::feature::{
    is_at_query_feature_range_comparison, parse_query_feature_from_name,
    parse_query_feature_interval_end, parse_query_feature_name,
    parse_query_feature_range_comparison,
};
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolated_identifier,
    parse_scss_interpolation_or_identifier,
};
use biome_css_syntax::CssSyntaxKind::{
    SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST, SCSS_INTERPOLATION,
};
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};

/// Parses an interpolation-led query feature.
///
/// Examples:
/// ```scss
/// @media (#{$feature}: #{$value}) {}
/// @media (#{$min} <= width <= #{$max}) {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_query_feature(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let m = p.start();

    let Present(head) = parse_scss_interpolation_or_identifier(p) else {
        return Absent;
    };

    if is_at_query_feature_name_after_comparison(p) {
        // `#{$min}px <= width`: keep the canonical identifier as the feature name.
        // Safe: `is_at_query_feature_name_after_comparison` checks both tokens.
        parse_query_feature_range_comparison(p).ok();
        parse_query_feature_name(p).ok();
        return parse_query_feature_interval_end(p, m);
    }

    // `#{$feature}: value` and `#{$feature} <= 10px`: the interpolation is the name.
    if head.kind(p) == SCSS_INTERPOLATION {
        let parts = head
            .precede(p)
            .complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
        parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER);
    }

    parse_query_feature_from_name(p, m)
}

#[inline]
fn is_at_query_feature_name_after_comparison(p: &mut CssParser) -> bool {
    is_at_query_feature_range_comparison(p) && is_nth_at_scss_interpolated_identifier(p, 1)
}

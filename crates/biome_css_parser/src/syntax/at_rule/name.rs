use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::scss_only_syntax_error;
use crate::syntax::scss::{
    complete_scss_interpolated_identifier, is_at_scss_interpolated_dashed_identifier,
    is_at_scss_interpolation, is_nth_at_scss_hyphen_interpolated_identifier,
    is_nth_at_scss_interpolation, is_nth_source_tight, parse_scss_interpolated_at_rule_name,
};
use crate::syntax::{
    CssSyntaxFeatures, is_at_identifier, parse_custom_identifier, parse_dashed_identifier,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_CUSTOM_IDENTIFIER, SCSS_INTERPOLATED_DASHED_IDENTIFIER, SCSS_INTERPOLATION,
};
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax::{self, Absent};

/// Recognizes interpolation within one name, not a following name in a list.
///
/// ```scss
/// @font-feature-values Font-#{$name}, Font #{$name} {}
/// ```
#[inline]
pub(super) fn is_at_scss_at_rule_prelude_name(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
        || is_at_scss_interpolated_dashed_identifier(p)
        || is_nth_at_scss_hyphen_interpolated_identifier(p, 0)
        || (is_at_identifier(p) && is_nth_at_scss_interpolation(p, 1) && is_nth_source_tight(p, 1))
}

/// Parses an interpolated name after an at-rule keyword, rejecting it in CSS.
///
/// ```scss
/// @counter-style counter-#{$name} { symbols: "*"; }
/// @page :#{$pseudo} { margin: 1in; }
/// ```
#[inline]
pub(super) fn parse_scss_at_rule_prelude_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_at_rule_prelude_name(p) {
        return Absent;
    }

    CssSyntaxFeatures::Scss.parse_exclusive_syntax_with_kind(
        p,
        |p| {
            parse_scss_interpolated_at_rule_name(p).map(|name| {
                if name.kind(p) == SCSS_INTERPOLATION {
                    complete_scss_interpolated_identifier(p, name)
                } else {
                    name
                }
            })
        },
        |p, marker| {
            scss_only_syntax_error(
                p,
                "SCSS interpolated at-rule prelude names",
                marker.range(p),
            )
        },
        Some(CSS_BOGUS_CUSTOM_IDENTIFIER),
    )
}

/// Keeps CSS custom-name validation for names without interpolation.
///
/// ```scss
/// @counter-style #{$name} { symbols: "*"; }
/// @counter-style markers { symbols: "*"; }
/// ```
#[inline]
pub(super) fn parse_custom_at_rule_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_at_rule_prelude_name(p) {
        parse_scss_at_rule_prelude_name(p)
    } else {
        parse_custom_identifier(p, CssLexContext::Regular)
    }
}

/// Requires `--` in literal names; SCSS interpolation can supply the whole name.
///
/// ```scss
/// @font-palette-values --palette-#{$name} { font-family: "Bixa"; }
/// @property --#{$name} { syntax: "*"; inherits: false; }
/// @property #{$name} { syntax: "*"; inherits: false; }
/// ```
#[inline]
pub(super) fn parse_dashed_at_rule_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_at_rule_prelude_name(p)
    } else if is_at_scss_interpolated_dashed_identifier(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax_with_kind(
            p,
            |p| {
                parse_scss_interpolated_at_rule_name(p).map(|mut name| {
                    name.change_kind(p, SCSS_INTERPOLATED_DASHED_IDENTIFIER);
                    name
                })
            },
            |p, marker| {
                scss_only_syntax_error(p, "SCSS interpolated dashed identifiers", marker.range(p))
            },
            Some(CSS_BOGUS_CUSTOM_IDENTIFIER),
        )
    } else {
        parse_dashed_identifier(p)
    }
}

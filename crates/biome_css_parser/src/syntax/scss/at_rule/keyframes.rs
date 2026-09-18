use crate::parser::CssParser;
use crate::syntax::scss::{
    is_at_scss_interpolation, is_at_scss_variable, is_nth_at_scss_interpolation,
    parse_scss_interpolated_name, parse_scss_regular_interpolation, parse_scss_variable,
};
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier};
use biome_css_syntax::CssSyntaxKind::{SCSS_KEYFRAMES_NAME, SCSS_KEYFRAMES_SELECTOR};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::SyntaxFeature;

#[inline]
pub(crate) fn is_at_scss_keyframes_selector(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p) && is_at_scss_interpolation(p)
}

/// Parses an interpolated keyframe selector.
///
/// Example:
/// ```scss
/// @keyframes loader {
///   #{50% - $duration} {}
///   #{$i}% {}
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_keyframes_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_keyframes_selector(p) {
        return Absent;
    }

    let m = p.start();
    // `is_at_scss_keyframes_selector` guarantees the `#{...}` selector body.
    parse_scss_regular_interpolation(p).ok();
    // `#{$i}%`: the literal percent suffix belongs to the selector only when adjacent.
    if !p.has_preceding_whitespace() {
        p.eat(T![%]);
    }

    Present(m.complete(p, SCSS_KEYFRAMES_SELECTOR))
}

#[inline]
pub(crate) fn is_at_scss_keyframes_name(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_at_scss_variable(p)
            || is_at_scss_interpolation(p)
            || is_at_identifier_with_interpolation_suffix(p))
}

/// Parses a dynamic SCSS keyframes name.
///
/// Examples:
/// ```scss
/// @keyframes $name {}
/// @keyframes #{$name} {}
/// @keyframes fade-#{$name} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_keyframes_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_keyframes_name(p) {
        return Absent;
    }

    let m = p.start();

    if is_at_scss_variable(p) {
        parse_scss_variable(p).ok();
    } else {
        parse_scss_interpolated_name(p).ok();
    }

    Present(m.complete(p, SCSS_KEYFRAMES_NAME))
}

#[inline]
fn is_at_identifier_with_interpolation_suffix(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && is_nth_at_scss_interpolation(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

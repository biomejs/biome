use crate::parser::CssParser;
use crate::syntax::scss::{is_at_scss_interpolation, parse_scss_regular_interpolation};
use biome_css_syntax::{CssSyntaxKind::SCSS_KEYFRAMES_SELECTOR, T};
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

#[inline]
pub(crate) fn is_at_scss_keyframes_selector(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p)
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

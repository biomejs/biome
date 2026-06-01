use crate::parser::CssParser;
use crate::syntax::scss::expression::parse_scss_selector_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    is_at_identifier_continuation, is_at_scss_interpolated_identifier,
    parse_scss_interpolated_identifier_parts,
};
use crate::syntax::scss::is_at_scss_interpolation;
use crate::syntax::selector::{
    parse_selector_custom_identifier_fragment, parse_selector_identifier_fragment,
};
use biome_css_syntax::CssSyntaxKind::{SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATION};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Parses SCSS-interpolated selector name slots.
///
/// This selector-specific variant keeps the closing `}` of an interpolation in
/// selector lexing mode so following whitespace is tokenized as a selector
/// combinator (`CSS_SPACE_LITERAL`) instead of being skipped as trivia.
///
/// Examples:
/// ```scss
/// .icon-#{$name} {}
/// button-#{$variant} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_selector_identifier_with_fragment(p, parse_selector_identifier_fragment)
}

/// Parses SCSS-interpolated selector custom identifiers.
///
/// Example:
/// ```scss
/// .button-#{$variant} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_custom_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_selector_identifier_with_fragment(p, parse_selector_custom_identifier_fragment)
}

/// Parses selector identifier grammar with caller-owned non-interpolation parts.
///
/// Examples: `button-#{$variant}` and `.button-#{$variant}`.
fn parse_scss_selector_identifier_with_fragment(
    p: &mut CssParser,
    parse_selector_fragment: fn(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_identifier(p) {
        return Absent;
    }

    let Present(first_fragment) = parse_selector_identifier_part(p, parse_selector_fragment) else {
        return Absent;
    };

    // A plain selector identifier only becomes an interpolated identifier when
    // another selector identifier fragment follows with no separating trivia.
    if first_fragment.kind(p) != SCSS_INTERPOLATION && !is_at_identifier_continuation(p) {
        return Present(first_fragment);
    }

    let parts = parse_scss_interpolated_identifier_parts(p, first_fragment, |p| {
        parse_selector_identifier_part(p, parse_selector_fragment)
    });

    Present(parts.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}

#[inline]
fn parse_selector_identifier_part(
    p: &mut CssParser,
    parse_selector_fragment: fn(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_selector_interpolation(p)
    } else {
        parse_selector_fragment(p)
    }
}

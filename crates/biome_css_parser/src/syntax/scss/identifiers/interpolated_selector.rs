use crate::parser::CssParser;
use crate::syntax::scss::expression::parse_scss_selector_interpolation;
use crate::syntax::scss::identifiers::interpolated_identifier::{
    complete_scss_interpolated_identifier, is_at_identifier_continuation,
    is_at_scss_interpolated_identifier,
};
use crate::syntax::scss::is_at_scss_interpolation;
use crate::syntax::selector::{
    parse_selector_custom_identifier_fragment, parse_selector_identifier_fragment,
};
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATION;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Parses selector identifier grammar that may contain interpolation parts.
///
/// This is different from [`parse_scss_selector_interpolation`], which parses
/// exactly one standalone selector interpolation such as `#{$type}`. This
/// helper parses selector identifier grammar, so it can merge adjacent selector
/// identifier fragments and interpolation fragments into one identifier-shaped
/// node.
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
pub(crate) fn parse_scss_selector_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_selector_interpolated_identifier(p, parse_selector_identifier_fragment)
}

/// Parses selector custom identifiers such as `.foo-#{$name}` and preserves the
/// selector-specific custom-identifier rules used by class and id selectors.
///
/// Like `parse_scss_selector_interpolated_identifier`, this keeps selector
/// whitespace after `#{$...}` visible as a combinator rather than treating it
/// as trivia.
///
/// Example:
/// ```scss
/// .button-#{$variant} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_custom_interpolated_identifier(
    p: &mut CssParser,
) -> ParsedSyntax {
    parse_selector_interpolated_identifier(p, parse_selector_custom_identifier_fragment)
}

/// Parses selector identifier grammar using `parse_selector_fragment` for the
/// non-interpolation fragments.
///
/// `parse_selector_fragment` keeps the selector-specific identifier rules at
/// the call site. Class/id custom identifiers pass
/// [`parse_selector_custom_identifier_fragment`], while regular selector
/// identifiers pass [`parse_selector_identifier_fragment`].
fn parse_selector_interpolated_identifier(
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

    Present(complete_scss_interpolated_identifier(
        p,
        first_fragment,
        |p| parse_selector_identifier_part(p, parse_selector_fragment),
    ))
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

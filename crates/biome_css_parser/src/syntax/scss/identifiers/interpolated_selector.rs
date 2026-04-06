use crate::parser::CssParser;
use crate::syntax::scss::expression::{ScssInterpolationMode, parse_scss_interpolation_with_mode};
use crate::syntax::scss::identifiers::interpolated_identifier::parse_scss_interpolated_identifier_with;
use crate::syntax::scss::is_at_scss_interpolation;
use crate::syntax::selector::{
    parse_selector_custom_identifier_fragment, parse_selector_identifier_fragment,
};
use biome_parser::prelude::ParsedSyntax;

/// Parses selector identifiers such as `.icon-#{$name}` or `#{$type}` as a
/// single SCSS interpolated identifier node.
///
/// This selector-specific variant keeps the closing `}` of an interpolation in
/// selector lexing mode so following whitespace is tokenized as a selector
/// combinator (`CSS_SPACE_LITERAL`) instead of being skipped as trivia.
///
/// Example:
/// ```scss
/// .icon-#{$name} {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_selector_interpolated_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_interpolated_identifier_with(p, parse_scss_selector_identifier_any_fragment)
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
    parse_scss_interpolated_identifier_with(p, parse_scss_selector_custom_identifier_any_fragment)
}

#[inline]
fn parse_scss_selector_identifier_any_fragment(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_selector_interpolation(p)
    } else {
        parse_selector_identifier_fragment(p)
    }
}

#[inline]
fn parse_scss_selector_custom_identifier_any_fragment(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_selector_interpolation(p)
    } else {
        parse_selector_custom_identifier_fragment(p)
    }
}

/// Parses an interpolation inside a selector-aware context.
///
/// This keeps the closing `}` in selector lexing mode so whitespace after
/// `#{$...}` becomes a selector combinator instead of trivia.
#[inline]
pub(crate) fn parse_scss_selector_interpolation(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_interpolation_with_mode(p, ScssInterpolationMode::Selector)
}

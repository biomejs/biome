use crate::parser::CssParser;
use crate::syntax::scss::expression::parse_scss_selector_interpolation as parse_selector_interpolation_expression;
use crate::syntax::scss::identifiers::interpolated_identifier::parse_scss_interpolated_identifier_with;
use crate::syntax::scss::is_at_scss_interpolation;
use crate::syntax::selector::{
    parse_selector_custom_identifier_fragment, parse_selector_identifier_fragment,
};
use biome_parser::prelude::ParsedSyntax;

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

/// Parses one standalone interpolation inside a selector-aware context.
///
/// This is different from [`parse_scss_selector_interpolated_identifier`],
/// which parses selector identifier grammar that may include interpolation
/// parts. This helper only parses the single interpolation expression at the
/// current position.
///
/// This keeps the closing `}` in selector lexing mode so whitespace after
/// `#{$...}` becomes a selector combinator instead of trivia.
///
/// Example:
/// ```scss
/// #{$type} > .child {}
/// ```
#[inline]
pub(crate) fn parse_scss_selector_interpolation(p: &mut CssParser) -> ParsedSyntax {
    parse_selector_interpolation_expression(p)
}

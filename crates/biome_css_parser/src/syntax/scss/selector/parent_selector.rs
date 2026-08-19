use crate::parser::CssParser;
use crate::syntax::is_nth_at_identifier;
use crate::syntax::scss::expression::parse_scss_selector_interpolation;
use crate::syntax::scss::{is_at_scss_interpolation, is_nth_at_scss_interpolation};
use crate::syntax::selector::{parse_selector_identifier_fragment, selector_lex_context};
use biome_css_syntax::CssSyntaxKind::{
    CSS_DIMENSION_VALUE, CSS_NUMBER, CSS_NUMBER_LITERAL, SCSS_PARENT_SELECTOR,
    SCSS_PARENT_SELECTOR_SUFFIX, SCSS_PARENT_SELECTOR_SUFFIX_HYPHEN,
    SCSS_PARENT_SELECTOR_SUFFIX_PART_LIST,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Parses an SCSS parent selector with a source-tight suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/style-rules/parent-selector/
#[inline]
pub(crate) fn parse_scss_parent_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector(p) {
        return Absent;
    }

    let parent = p.start();
    let context = selector_lex_context(p);
    p.bump_with_context(T![&], context);

    let suffix = p.start();
    ScssParentSelectorSuffixPartList.parse_list(p);
    suffix.complete(p, SCSS_PARENT_SELECTOR_SUFFIX);

    Present(parent.complete(p, SCSS_PARENT_SELECTOR))
}

/// Returns whether the current token starts an SCSS parent selector with a
/// source-tight suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
#[inline]
pub(crate) fn is_at_scss_parent_selector(p: &mut CssParser) -> bool {
    p.at(T![&])
        && !p.has_nth_preceding_whitespace(1)
        && is_nth_at_scss_parent_selector_suffix_part(p, 1)
}

/// Parses adjacent suffix parts until whitespace or selector syntax ends the
/// parent suffix.
///
/// Example:
/// ```scss
/// .button {
///   &-#{$state} {}
/// }
/// ```
struct ScssParentSelectorSuffixPartList;
impl ParseNodeList for ScssParentSelectorSuffixPartList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_PARENT_SELECTOR_SUFFIX_PART_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_scss_parent_selector_suffix_part(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.has_preceding_whitespace() || !is_at_scss_parent_selector_suffix_part(p)
    }

    fn recover(&mut self, _p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element {
            Absent => Err(RecoveryError::AlreadyRecovered),
            Present(m) => Ok(m),
        }
    }
}

/// Parses one suffix part in an SCSS parent selector.
///
/// Examples:
/// ```scss
/// .button {
///   &--active {}
///   &-100\.200 {}
///   &#{$state} {}
///   &-#{$state} {}
/// }
/// ```
#[inline]
fn parse_scss_parent_selector_suffix_part(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector_suffix_part(p) {
        return Absent;
    }

    if is_at_scss_interpolation(p) {
        parse_scss_selector_interpolation(p)
    } else if is_at_scss_parent_selector_suffix_hyphen(p) {
        parse_scss_parent_selector_suffix_hyphen(p)
    } else if is_at_scss_parent_selector_suffix_number(p) {
        parse_scss_parent_selector_suffix_number(p)
    } else {
        parse_selector_identifier_fragment(p)
    }
}

/// Returns whether the current token is an identifier, interpolation, number,
/// dimension, or raw hyphen accepted by a parent-selector suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
#[inline]
fn is_at_scss_parent_selector_suffix_part(p: &mut CssParser) -> bool {
    is_nth_at_scss_parent_selector_suffix_part(p, 0)
}

/// Returns whether the token at `n` is an identifier, interpolation, number,
/// dimension, or raw hyphen accepted by a parent-selector suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
#[inline]
fn is_nth_at_scss_parent_selector_suffix_part(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_scss_parent_selector_suffix_value(p, n) || p.nth_at(n, T![-])
}

#[inline]
fn is_nth_at_scss_parent_selector_suffix_value(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
        || is_nth_at_scss_interpolation(p, n)
        || p.nth_at(n, CSS_DIMENSION_VALUE)
        || p.nth_at(n, CSS_NUMBER_LITERAL)
}

/// Returns whether the current token is a raw hyphen in a parent-selector
/// suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
#[inline]
fn is_at_scss_parent_selector_suffix_hyphen(p: &mut CssParser) -> bool {
    p.at(T![-])
}

/// Parses one raw hyphen in a parent-selector suffix.
///
/// Example:
/// ```scss
/// .button {
///   &--#{$state} {}
/// }
/// ```
#[inline]
fn parse_scss_parent_selector_suffix_hyphen(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector_suffix_hyphen(p) {
        return Absent;
    }

    let m = p.start();
    let context = selector_lex_context(p);
    p.bump_with_context(T![-], context);
    Present(m.complete(p, SCSS_PARENT_SELECTOR_SUFFIX_HYPHEN))
}

/// Parses a numeric parent-selector suffix part.
///
/// Example:
/// ```scss
/// .button {
///   &-100\.200 {}
/// }
/// ```
#[inline]
fn parse_scss_parent_selector_suffix_number(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector_suffix_number(p) {
        return Absent;
    }

    let m = p.start();
    let context = selector_lex_context(p);
    // `&-100\.200`: `-100` can lex as a dimension head, but the suffix
    // owns it as a number and leaves `\.200` as the next suffix part.
    p.bump_remap_with_context(CSS_NUMBER_LITERAL, context);
    Present(m.complete(p, CSS_NUMBER))
}

#[inline]
fn is_at_scss_parent_selector_suffix_number(p: &mut CssParser) -> bool {
    p.at(CSS_DIMENSION_VALUE) || p.at(CSS_NUMBER_LITERAL)
}

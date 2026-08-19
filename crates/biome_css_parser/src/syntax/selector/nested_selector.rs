use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::scss::{is_at_scss_parent_selector, parse_scss_parent_selector};
use crate::syntax::selector::selector_lex_context;
use biome_css_syntax::CssSyntaxKind::{CSS_NESTED_SELECTOR, CSS_NESTED_SELECTOR_LIST};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature};

pub(crate) struct NestedSelectorList;
impl ParseNodeList for NestedSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_NESTED_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_nested_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !p.at(T![&])
    }

    fn recover(&mut self, _p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element {
            Absent => Err(RecoveryError::AlreadyRecovered),
            Present(m) => Ok(m),
        }
    }
}

/// Returns whether the current token starts a nested selector.
///
/// Example:
/// ```scss
/// .card {
///   &:hover {}
/// }
/// ```
#[inline]
fn is_at_nested_selector(p: &mut CssParser) -> bool {
    p.at(T![&])
}

/// Parses `&` as an SCSS parent selector when a source-tight suffix follows;
/// otherwise, parses a CSS nested selector.
///
/// Example:
/// ```scss
/// .card {
///   &:hover {}
///   &--#{$state} {}
/// }
/// ```
#[inline]
fn parse_nested_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_nested_selector(p) {
        return Absent;
    }

    if CssSyntaxFeatures::Scss.is_supported(p) && is_at_scss_parent_selector(p) {
        return parse_scss_parent_selector(p);
    }

    let m = p.start();
    let context = selector_lex_context(p);
    p.bump_with_context(T![&], context);

    Present(m.complete(p, CSS_NESTED_SELECTOR))
}

/// Parses `&` as a CSS nesting sub-selector. Unlike [parse_nested_selector],
/// this never produces a `ScssParentSelector`, since `AnyCssSubSelector` only
/// allows `CssNestedSelector`.
///
/// CSS example:
/// ```css
/// [data-smth="true"] {
///   h1& {}
/// }
/// ```
///
/// SCSS parent-selector suffixes are only meaningful when `&` opens a compound
/// selector:
/// ```scss
/// .button {
///   &-100 {}
///   &#{$state} {}
/// }
/// ```
#[inline]
pub(crate) fn parse_nested_selector_as_sub_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_nested_selector(p) {
        return Absent;
    }

    let m = p.start();
    let context = selector_lex_context(p);
    p.bump_with_context(T![&], context);

    Present(m.complete(p, CSS_NESTED_SELECTOR))
}

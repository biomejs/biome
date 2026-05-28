use crate::parser::CssParser;
use crate::syntax::css_modules::{
    CSS_MODULES_SCOPE_SET, CSS_MODULES_VUE_ENHANCED_SET, local_or_global_not_allowed,
    slotted_or_deep_not_allowed,
};
use crate::syntax::parse_error::expected_selector;
use crate::syntax::selector::{SelectorList, eat_or_recover_selector_function_close_token};
use crate::syntax::{CssSyntaxFeatures, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, token_set};

/// Checks if the current parser position is at a pseudo-class function selector for CSS Modules and SFC Vue.
///
/// This function determines if the parser is currently positioned at the start of a `:local` or `:global`.
#[inline]
pub(crate) fn is_at_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    p.at_ts(CSS_MODULES_SCOPE_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn is_at_vue_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    p.at_ts(CSS_MODULES_VUE_ENHANCED_SET) && p.nth_at(1, T!['('])
}

/// Parses a pseudo-class function selector for CSS Modules.
///
/// This function parses a pseudo-class function selector, specifically `:local` or `:global`, in CSS Modules.
/// If the `css.parser.cssModules` option is not enabled, it generates a diagnostic error and skips the selector.
/// ```css
/// :local(.className) {
///     color: red;
/// }
/// :global(.globalClass) .nestedClass {
///     padding: 10px;
/// }
/// :local(.className) > :global(.globalClass) {
///     margin: 0;
/// }
/// ```
#[inline]
pub(crate) fn parse_pseudo_class_function_selector(p: &mut CssParser) -> ParsedSyntax {
    if is_at_pseudo_class_function_selector(p) {
        CssSyntaxFeatures::CssModules.parse_exclusive_syntax(
            p,
            parse_pseudo_selector,
            |p, marker| local_or_global_not_allowed(p, marker.range(p)),
        )
    } else if is_at_vue_pseudo_class_function_selector(p) {
        CssSyntaxFeatures::CssModulesWithVue.parse_exclusive_syntax(
            p,
            parse_pseudo_selector,
            |p, marker| slotted_or_deep_not_allowed(p, marker.range(p)),
        )
    } else {
        Absent
    }
}

fn parse_pseudo_selector(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let list = SelectorList::default()
        .with_end_kind_ts(token_set!(T![')']))
        .disable_recovery()
        .parse_list(p);
    let list_range = list.range(p);

    if list_range.is_empty() && p.at(T![')']) {
        let diagnostic = expected_selector(p, list_range);
        p.error(diagnostic);
    }

    let kind = if eat_or_recover_selector_function_close_token(p, list, expected_selector)
        && !list_range.is_empty()
    {
        CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

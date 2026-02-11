use crate::parser::CssParser;
use crate::syntax::css_modules::{CSS_MODULES_SCOPE_SET, local_or_global_not_allowed};
use crate::syntax::parse_error::expected_selector;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_selector,
    recover_selector_function_parameter,
};
use crate::syntax::vue_scoped_css::{VUE_SCOPED_CSS_SET, vue_scoped_css_not_allowed};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::CssSyntaxKind::{self, CSS_PSEUDO_CLASS_FUNCTION_SELECTOR};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};

/// Checks if the current parser position is at a pseudo-class function selector
/// (`:local()`, `:global()` for CSS Modules; `:deep()`, `:slotted()` for Vue SFC).
#[inline]
pub(crate) fn is_at_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    (p.at_ts(CSS_MODULES_SCOPE_SET) || p.at_ts(VUE_SCOPED_CSS_SET)) && p.nth_at(1, T!['('])
}

/// Parses a pseudo-class function selector for CSS Modules (`:local()`, `:global()`)
/// or Vue SFC scoped CSS (`:deep()`, `:slotted()`).
///
/// If the corresponding parser option is not enabled, emits a diagnostic and skips the selector.
#[inline]
pub(crate) fn parse_pseudo_class_function_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_selector(p) {
        return Absent;
    }

    // Determine which feature this selector belongs to and whether it's enabled.
    let disabled_error = if p.at_ts(VUE_SCOPED_CSS_SET) {
        (!p.options().is_vue_scoped_css_enabled())
            .then(|| vue_scoped_css_not_allowed(p, p.cur_range()))
    } else {
        p.options()
            .is_css_modules_disabled()
            .then(|| local_or_global_not_allowed(p, p.cur_range()))
    };

    if let Some(error) = disabled_error {
        p.error(error);

        while !p.eat(T![')']) && !p.at(CssSyntaxKind::EOF) {
            p.bump_any();
        }

        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = match parse_selector(p) {
        Present(selector) => {
            if eat_or_recover_selector_function_close_token(p, selector, expected_selector) {
                CSS_PSEUDO_CLASS_FUNCTION_SELECTOR
            } else {
                CSS_BOGUS_PSEUDO_CLASS
            }
        }
        Absent => {
            recover_selector_function_parameter(p, expected_selector);
            p.expect(T![')']);
            CSS_BOGUS_PSEUDO_CLASS
        }
    };

    Present(m.complete(p, kind))
}

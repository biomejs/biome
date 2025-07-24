use crate::parser::CssParser;
use crate::syntax::css_modules::{CSS_MODULES_SCOPE_SET, local_or_global_not_allowed};
use crate::syntax::parse_error::expected_selector;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_selector,
    recover_selector_function_parameter,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::CssSyntaxKind::{self, CSS_PSEUDO_CLASS_FUNCTION_SELECTOR};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};

/// Checks if the current parser position is at a pseudo-class function selector for CSS Modules.
///
/// This function determines if the parser is currently positioned at the start of a `:local` or `:global`
/// pseudo-class function selector, which is part of the CSS Modules syntax.
#[inline]
pub(crate) fn is_at_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    p.at_ts(CSS_MODULES_SCOPE_SET) && p.nth_at(1, T!['('])
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
    if !is_at_pseudo_class_function_selector(p) {
        return Absent;
    }

    if p.options().is_css_modules_disabled() {
        // :local and :global are not standard CSS features
        // provide a hint on how to enable parsing of these pseudo-classes
        p.error(local_or_global_not_allowed(p, p.cur_range()));

        // Skip the entire pseudo-class function selector
        // Skip until the next closing parenthesis
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

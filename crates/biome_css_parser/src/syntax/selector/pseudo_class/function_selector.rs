use crate::parser::CssParser;
use crate::syntax::parse_error::expected_selector;
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_selector,
    recover_selector_function_parameter,
};
use biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_SELECTOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![global], T![local]];

#[inline]
pub(crate) fn is_at_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_SELECTOR_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(PSEUDO_CLASS_FUNCTION_SELECTOR_SET);
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

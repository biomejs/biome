use crate::parser::CssParser;
use crate::syntax::parse_error::expected_compound_selector;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_compound_selector,
    recover_selector_function_parameter,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![host], T![host_context]];

#[inline]
pub(crate) fn is_at_pseudo_class_function_compound_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_compound_selector(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = match parse_compound_selector(p) {
        Present(selector) => {
            if eat_or_recover_selector_function_close_token(p, selector, expected_compound_selector)
            {
                CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
            } else {
                CSS_BOGUS_PSEUDO_CLASS
            }
        }
        Absent => {
            recover_selector_function_parameter(p, expected_compound_selector);
            p.expect(T![')']);
            CSS_BOGUS_PSEUDO_CLASS
        }
    };

    Present(m.complete(p, kind))
}

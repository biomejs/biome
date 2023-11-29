use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_any_pseudo_element, expected_identifier, expected_selector,
};
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_selector, parse_selector_identifier,
    recover_selector_function_parameter,
};
use crate::syntax::{is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn parse_pseudo_element_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![::]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![::]);
    parse_pseudo_element(p).or_add_diagnostic(p, expected_any_pseudo_element);

    Present(m.complete(p, CSS_PSEUDO_ELEMENT_SELECTOR))
}
#[inline]
pub(crate) fn parse_pseudo_element(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    if is_at_pseudo_element_function_identifier(p) {
        parse_pseudo_element_function_identifier(p)
    } else if is_at_pseudo_element_function_selector(p) {
        parse_pseudo_element_function_selector(p)
    } else {
        parse_pseudo_element_identifier(p)
    }
}

const PSEUDO_ELEMENT_FUNCTION_IDENTIFIER_SET: TokenSet<CssSyntaxKind> =
    token_set![HIGHLIGHT_KW, PART_KW];
#[inline]
pub(crate) fn is_at_pseudo_element_function_identifier(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_ELEMENT_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}
#[inline]
pub(crate) fn parse_pseudo_element_function_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_element_function_identifier(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(PSEUDO_ELEMENT_FUNCTION_IDENTIFIER_SET);
    p.bump(T!['(']);

    let kind = match parse_regular_identifier(p) {
        Present(selector) => {
            if eat_or_recover_selector_function_close_token(p, selector, expected_identifier) {
                CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
            } else {
                CSS_BOGUS_PSEUDO_ELEMENT
            }
        }
        Absent => {
            recover_selector_function_parameter(p, expected_identifier);
            p.expect(T![')']);
            CSS_BOGUS_PSEUDO_ELEMENT
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn is_at_pseudo_element_function_selector(p: &mut CssParser) -> bool {
    is_at_identifier(p) && !p.at_ts(PSEUDO_ELEMENT_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_element_function_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_element_function_selector(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need to check if the identifier is valid, because we already did that
    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = match parse_selector(p) {
        Present(selector) => {
            if eat_or_recover_selector_function_close_token(p, selector, expected_selector) {
                CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
            } else {
                CSS_BOGUS_PSEUDO_ELEMENT
            }
        }
        Absent => {
            recover_selector_function_parameter(p, expected_selector);
            p.expect(T![')']);
            CSS_BOGUS_PSEUDO_ELEMENT
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn parse_pseudo_element_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_PSEUDO_ELEMENT_IDENTIFIER))
}

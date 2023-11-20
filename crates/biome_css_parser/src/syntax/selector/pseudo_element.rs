use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expect_any_pseudo_element, expect_any_selector, expected_identifier,
};
use crate::syntax::selector::{
    parse_selector, parse_selector_function_close_token, parse_selector_identifier,
    selector_lex_context, SELECTOR_FUNCTION_RECOVERY_SET,
};
use crate::syntax::{is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};
use biome_rowan::TextRange;

#[inline]
pub(crate) fn parse_pseudo_element_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![::]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![::]);
    parse_pseudo_element(p).or_add_diagnostic(p, expect_any_pseudo_element);

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

    p.eat_ts(PSEUDO_ELEMENT_FUNCTION_IDENTIFIER_SET);
    p.bump(T!['(']);

    let start_identifier = p.cur_range().start();
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);

    let context = selector_lex_context(p);
    if !p.eat_with_context(T![')'], context) {
        match ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET).recover(p) {
            Ok(marker) => {
                p.error(expected_identifier(
                    p,
                    TextRange::new(start_identifier, marker.range(p).end()),
                ));
            }
            Err(_) => {
                p.error(expected_token(T![')']));
            }
        }
    }

    Present(m.complete(p, CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER))
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

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_selector(p).or_add_diagnostic(p, expect_any_selector);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR))
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

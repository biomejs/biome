use crate::parser::CssParser;
use crate::syntax::parse_error::{expected_identifier, expected_relative_selector};
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, is_at_compound_selector, parse_selector,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::CssSyntaxKind::{
    CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST, CSS_RELATIVE_SELECTOR,
    CSS_RELATIVE_SELECTOR_LIST,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![HAS_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_relative_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_relative_selector_list(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_pseudo_class_function_relative_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);

    let list = CssRelativeSelectorList.parse_list(p);
    let list_range = list.range(p);

    if list_range.is_empty() && p.at(T![')']) {
        let diagnostic = expected_relative_selector(p, list_range);
        p.error(diagnostic);
    }

    let kind = if eat_or_recover_selector_function_close_token(p, list, expected_relative_selector)
        && !list_range.is_empty()
    {
        CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

struct CssRelativeSelectorList;

impl ParseSeparatedList for CssRelativeSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_RELATIVE_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_relative_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element.or_add_diagnostic(p, expected_relative_selector) {
            Some(m) => Ok(m),
            // we don't need to recover here, because we have a better diagnostic message in a close token
            None => Err(RecoveryError::RecoveryDisabled),
        }
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}

const RELATIVE_SELECTOR_COMBINATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![+], T![~], T![||]];

#[inline]
fn is_at_relative_selector_combinator(p: &mut CssParser) -> bool {
    p.at_ts(RELATIVE_SELECTOR_COMBINATOR_SET)
}

#[inline]
fn is_at_relative_selector(p: &mut CssParser) -> bool {
    is_at_relative_selector_combinator(p) || is_at_compound_selector(p)
}

#[inline]
fn parse_relative_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_relative_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.eat_ts(RELATIVE_SELECTOR_COMBINATOR_SET);
    parse_selector(p).or_add_diagnostic(p, expected_relative_selector);

    Present(m.complete(p, CSS_RELATIVE_SELECTOR))
}

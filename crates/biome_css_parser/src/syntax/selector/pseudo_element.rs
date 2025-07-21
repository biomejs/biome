use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_any_pseudo_element, expected_identifier, expected_selector,
};
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, parse_selector, parse_selector_identifier,
    recover_selector_function_parameter,
};
use crate::syntax::{is_at_identifier, parse_custom_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

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

    if is_at_pseudo_element_function_custom_identifier(p) {
        parse_pseudo_element_function_custom_identifier(p)
    } else if is_at_pseudo_element_function(p) {
        parse_pseudo_element_function(p)
    } else if is_at_pseudo_element_function_selector(p) {
        parse_pseudo_element_function_selector(p)
    } else {
        parse_pseudo_element_identifier(p)
    }
}

#[inline]
pub(crate) fn is_at_pseudo_element_function_custom_identifier(p: &mut CssParser) -> bool {
    p.at(HIGHLIGHT_KW) && p.nth_at(1, T!['('])
}
#[inline]
pub(crate) fn parse_pseudo_element_function_custom_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_element_function_custom_identifier(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need to check if the identifier is valid, because we already did that
    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = match parse_custom_identifier(p, CssLexContext::Regular) {
        Present(selector) => {
            if eat_or_recover_selector_function_close_token(p, selector, expected_identifier) {
                CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER
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
pub(crate) fn is_at_pseudo_element_function(p: &mut CssParser) -> bool {
    p.at(PART_KW) && p.nth_at(1, T!['('])
}
#[inline]
pub(crate) fn parse_pseudo_element_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_element_function(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need to check if the identifier is valid, because we already did that
    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let list = PseudoElementFunctionParameterList.parse_list(p);
    let kind = if eat_or_recover_selector_function_close_token(p, list, expected_identifier) {
        CSS_PSEUDO_ELEMENT_FUNCTION
    } else {
        CSS_BOGUS_PSEUDO_ELEMENT
    };

    Present(m.complete(p, kind))
}

struct PseudoElementFunctionParameterList;
impl ParseNodeList for PseudoElementFunctionParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PSEUDO_ELEMENT_FUNCTION_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_regular_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &PseudoElementFunctionParameterListParseRecovery,
            expected_identifier,
        )
    }
}

struct PseudoElementFunctionParameterListParseRecovery;
impl ParseRecovery for PseudoElementFunctionParameterListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // 1. We are at the end of the parameter list
        // 2. We are at an identifier (which is a valid parameter)
        // 3. We have a preceding line break
        p.at(T![')']) || is_at_identifier(p) || p.has_preceding_line_break()
    }
}

#[inline]
pub(crate) fn is_at_pseudo_element_function_selector(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T!['('])
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

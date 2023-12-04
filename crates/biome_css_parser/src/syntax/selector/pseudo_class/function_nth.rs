use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{expected_any_pseudo_class_nth, expected_number};
use crate::syntax::selector::{
    eat_or_recover_selector_function_close_token, recover_selector_function_parameter,
    CssSelectorList,
};
use crate::syntax::{parse_number, parse_regular_number};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::CssSyntaxKind::{
    CSS_NTH_OFFSET, CSS_PSEUDO_CLASS_FUNCTION_NTH, CSS_PSEUDO_CLASS_NTH,
    CSS_PSEUDO_CLASS_NTH_IDENTIFIER, CSS_PSEUDO_CLASS_NTH_NUMBER, CSS_PSEUDO_CLASS_NTH_SELECTOR,
    CSS_PSEUDO_CLASS_OF_NTH_SELECTOR, OF_KW,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_NTH_SET: TokenSet<CssSyntaxKind> = token_set![
    T![nth_child],
    T![nth_last_child],
    T![nth_of_type],
    T![nth_last_of_type],
    T![nth_col],
    T![nth_last_col],
];

#[inline]
pub(crate) fn is_at_pseudo_class_function_nth(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_NTH_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_nth(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_nth(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(PSEUDO_CLASS_FUNCTION_NTH_SET);
    p.bump_with_context(T!['('], CssLexContext::PseudoNthSelector);

    let kind = if is_at_pseudo_class_nth_selector(p) {
        // SAFETY: we know that the next token is a nth parameter
        let selector = parse_pseudo_class_nth_selector(p).unwrap();

        if eat_or_recover_selector_function_close_token(p, selector, expected_any_pseudo_class_nth)
        {
            CSS_PSEUDO_CLASS_FUNCTION_NTH
        } else {
            CSS_BOGUS_PSEUDO_CLASS
        }
    } else {
        recover_selector_function_parameter(p, expected_any_pseudo_class_nth);
        p.expect(T![')']);
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

const PSEUDO_CLASS_FUNCTION_NTH_CLASS_IDENTIFIER_SET: TokenSet<CssSyntaxKind> =
    token_set![T![odd], T![even]];

const PSEUDO_CLASS_FUNCTION_NTH_CLASS_SIGN_SET: TokenSet<CssSyntaxKind> = token_set![T![+], T![-]];

const PSEUDO_CLASS_FUNCTION_NTH_CLASS_SET: TokenSet<CssSyntaxKind> =
    PSEUDO_CLASS_FUNCTION_NTH_CLASS_IDENTIFIER_SET
        .union(PSEUDO_CLASS_FUNCTION_NTH_CLASS_SIGN_SET)
        .union(token_set![T![n], CSS_NUMBER_LITERAL,]);

#[inline]
fn is_at_pseudo_class_nth_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_NTH_CLASS_SET)
}

#[inline]
fn parse_pseudo_class_nth_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_nth_selector(p) {
        return Absent;
    }

    let m = p.start();

    parse_pseudo_class_nth(p).ok();
    parse_pseudo_class_of_nth_selector(p).ok();

    Present(m.complete(p, CSS_PSEUDO_CLASS_NTH_SELECTOR))
}

#[inline]
fn parse_pseudo_class_nth(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_nth_selector(p) {
        return Absent;
    }

    let m = p.start();

    let kind = if p.eat_ts(PSEUDO_CLASS_FUNCTION_NTH_CLASS_IDENTIFIER_SET) {
        CSS_PSEUDO_CLASS_NTH_IDENTIFIER
    } else {
        p.eat_ts_with_context(
            PSEUDO_CLASS_FUNCTION_NTH_CLASS_SIGN_SET,
            CssLexContext::PseudoNthSelector,
        );

        parse_number(p, CssLexContext::PseudoNthSelector).ok();

        if p.eat_with_context(T![n], CssLexContext::PseudoNthSelector) {
            parse_nth_offset(p).ok();

            CSS_PSEUDO_CLASS_NTH
        } else {
            CSS_PSEUDO_CLASS_NTH_NUMBER
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
fn parse_nth_offset(p: &mut CssParser) -> ParsedSyntax {
    if !p.at_ts(PSEUDO_CLASS_FUNCTION_NTH_CLASS_SIGN_SET) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(PSEUDO_CLASS_FUNCTION_NTH_CLASS_SIGN_SET);
    parse_regular_number(p).or_add_diagnostic(p, expected_number);

    Present(m.complete(p, CSS_NTH_OFFSET))
}

#[inline]
fn parse_pseudo_class_of_nth_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(OF_KW) {
        return Absent;
    }

    let m = p.start();

    p.bump(OF_KW);

    CssSelectorList::default()
        .with_end_kind(T![')'])
        .parse_list(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_OF_NTH_SELECTOR))
}

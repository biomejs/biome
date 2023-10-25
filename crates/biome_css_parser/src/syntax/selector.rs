use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expect_any_attribute_matcher_name, expect_any_attribute_modifier, expect_any_pseudo_element,
    expect_any_selector, expected_identifier,
};
use crate::syntax::{
    is_at_identifier, parse_css_string, parse_error, parse_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};

#[inline]
pub(crate) fn parse_selector(p: &mut CssParser) -> ParsedSyntax {
    // In CSS, we have compound selectors and complex selectors.
    // Compound selectors are simple, unseparated chains of selectors,
    // while complex selectors are compound selectors separated by combinators.
    // After parsing the compound selector, it then checks if this compound selector is a part of a complex selector.
    parse_compound_selector(p).and_then(|selector| parse_complex_selector(p, selector))
}

#[inline]
pub(crate) fn parse_complex_selector(p: &mut CssParser, mut left: CompletedMarker) -> ParsedSyntax {
    let mut progress = ParserProgress::default();

    loop {
        progress.assert_progressing(p);

        if is_at_complex_selector_combinator(p) {
            let complex_selector = left.precede(p);
            // bump combinator
            p.bump(p.cur());
            parse_compound_selector(p).or_add_diagnostic(p, expect_any_selector);
            left = complex_selector.complete(p, CSS_COMPLEX_SELECTOR)
        } else {
            return Present(left);
        }
    }
}

const COMBINATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![+], T![~], T![||], CSS_SPACE_LITERAL];
#[inline]
pub(crate) fn is_at_complex_selector_combinator(p: &mut CssParser) -> bool {
    p.at_ts(COMBINATOR_SET)
}
#[inline]
pub(crate) fn parse_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_compound_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.eat(T![&]);
    parse_simple_selector(p).ok();
    CssSubSelectorList.parse_list(p);

    Present(m.complete(p, CSS_COMPOUND_SELECTOR))
}

#[inline]
pub(crate) fn is_at_compound_selector(p: &mut CssParser) -> bool {
    p.at(T![&]) || is_at_simple_selector(p) || p.at_ts(CssSubSelectorList::START_SET)
}

#[inline]
pub(crate) fn parse_simple_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_simple_selector(p) {
        return Absent;
    }

    match p.cur() {
        T![*] => parse_universal_selector(p),
        _ if is_at_identifier(p) => parse_type_selector(p),
        _ => Absent,
    }
}

#[inline]
pub(crate) fn is_at_simple_selector(p: &mut CssParser) -> bool {
    p.at(T![*]) || is_at_identifier(p)
}

struct CssSubSelectorList;
impl CssSubSelectorList {
    const START_SET: TokenSet<CssSyntaxKind> = token_set![T![#], T![.], T![:], T![::], T!['[']];
}
impl ParseNodeList for CssSubSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_SUB_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_sub_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !p.at_ts(Self::START_SET)
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SUB_SELECTOR, Self::START_SET),
            parse_error::expect_any_sub_selector,
        )
    }
}

#[inline]
pub(crate) fn parse_sub_selector(p: &mut CssParser) -> ParsedSyntax {
    match p.cur() {
        T![.] => parse_class_selector(p),
        T![#] => parse_id_selector(p),
        T!['['] => parse_attribute_selector(p),
        T![::] => parse_pseudo_element_selector(p),
        _ => Absent,
    }
}

#[inline]
pub(crate) fn parse_class_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![.]);
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_CLASS_SELECTOR))
}

#[inline]
pub(crate) fn parse_id_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![#]);
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_ID_SELECTOR))
}

#[inline]
pub(crate) fn parse_universal_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();

    let context = selector_lex_context(p);
    p.eat_with_context(T![*], context);

    Present(m.complete(p, CSS_UNIVERSAL_SELECTOR))
}

#[inline]
pub(crate) fn parse_type_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_TYPE_SELECTOR))
}

#[inline]
pub(crate) fn parse_selector_identifier(p: &mut CssParser) -> ParsedSyntax {
    let context = selector_lex_context(p);
    parse_identifier(p, context)
}

const SELECTOR_LEX_SET: TokenSet<CssSyntaxKind> =
    COMBINATOR_SET.union(token_set![T!['{'], T![,], T![')']]);
#[inline]
pub(crate) fn selector_lex_context(p: &mut CssParser) -> CssLexContext {
    if SELECTOR_LEX_SET.contains(p.nth(1)) {
        CssLexContext::Regular
    } else {
        CssLexContext::Selector
    }
}

#[inline]
pub(crate) fn parse_attribute_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();

    p.bump(T!['[']);
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);

    // `parse_attribute_matcher` method is invoked with `ok()`,
    // which turns an `Err` into an `Ok` variant, because attribute matcher in a CSS attribute selector
    // is optional. If it isn't present or fails to parse correctly, parsing can continue.
    parse_attribute_matcher(p).ok();

    let context = selector_lex_context(p);
    p.expect_with_context(T![']'], context);

    Present(m.complete(p, CSS_ATTRIBUTE_SELECTOR))
}

#[inline]
pub(crate) fn parse_attribute_matcher(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_matcher(p) {
        return Absent;
    }

    let m = p.start();

    // bump attribute matcher type
    p.bump(p.cur());
    parse_attribute_matcher_value(p).or_add_diagnostic(p, expect_any_attribute_matcher_name);

    let modifier = p.cur();
    if modifier.is_attribute_modifier_keyword() {
        p.bump(modifier);
    } else if modifier != T![']'] {
        // if we have an invalid modifier, we should add a diagnostic and bump it
        let diagnostic = expect_any_attribute_modifier(p, p.cur_range());
        p.error(diagnostic);
        p.bump_any();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER))
}

#[inline]
pub(crate) fn parse_attribute_matcher_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_matcher_value(p) {
        return Absent;
    }

    let m = p.start();

    if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER_VALUE))
}

#[inline]
pub(crate) fn is_at_attribute_matcher_value(p: &mut CssParser) -> bool {
    is_at_identifier(p) || p.at(CSS_STRING_LITERAL)
}

const ATTRIBUTE_MATCHER_SET: TokenSet<CssSyntaxKind> =
    token_set![T![~=], T![|=], T![^=], T!["$="], T![*=], T![=]];
#[inline]
pub(crate) fn is_at_attribute_matcher(p: &mut CssParser) -> bool {
    p.at_ts(ATTRIBUTE_MATCHER_SET)
}

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

    let m = p.start();

    let is_identifier_parameter = is_at_pseudo_identifier_parameter(p);

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);

    if p.eat(T!['(']) {
        if is_identifier_parameter {
            parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        } else {
            parse_selector(p).or_add_diagnostic(p, expect_any_selector);
        };

        let context = selector_lex_context(p);
        p.expect_with_context(T![')'], context);

        Present(m.complete(p, CSS_PSEUDO_ELEMENT_FUNCTION))
    } else {
        Present(m.complete(p, CSS_PSEUDO_ELEMENT_IDENTIFIER))
    }
}

const PSEUDO_IDENTIFIER_PARAMETER_SET: TokenSet<CssSyntaxKind> =
    token_set![PART_KW, HIGHLIGHT_KW];
#[inline]
pub(crate) fn is_at_pseudo_identifier_parameter(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_IDENTIFIER_PARAMETER_SET) && p.nth_at(1, T!['('])
}

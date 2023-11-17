use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expect_any_pseudo_class, expect_any_selector, expected_identifier, expected_number,
    expected_pseudo_class_nth,
};
use crate::syntax::selector::{
    is_at_compound_selector, parse_compound_selector, parse_selector,
    parse_selector_function_close_token, parse_selector_identifier, CssSelectorList,
};
use crate::syntax::{is_at_identifier, is_nth_at_identifier, parse_css_string, parse_number, parse_regular_identifier, parse_regular_number};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn parse_pseudo_class_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);
    parse_pseudo_class(p).or_add_diagnostic(p, expect_any_pseudo_class);

    Present(m.complete(p, CSS_PSEUDO_CLASS_SELECTOR))
}

#[inline]
fn parse_pseudo_class(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    if is_at_pseudo_class_function_identifier(p) {
        parse_pseudo_class_function_identifier(p)
    } else if is_at_pseudo_class_function_selector(p) {
        parse_pseudo_class_function_selector(p)
    } else if is_at_pseudo_class_function_selector_list(p) {
        parse_pseudo_class_function_selector_list(p)
    } else if is_at_pseudo_class_function_compound_selector(p) {
        parse_pseudo_class_function_compound_selector(p)
    } else if is_at_pseudo_class_function_compound_selector_list(p) {
        parse_pseudo_class_function_compound_selector_list(p)
    } else if is_at_pseudo_class_function_relative_selector_list(p) {
        parse_pseudo_class_function_relative_selector_list(p)
    } else if is_at_pseudo_class_function_value_list(p) {
        parse_pseudo_class_function_value_list(p)
    } else if is_at_pseudo_class_function_nth(p) {
        parse_pseudo_class_function_nth(p)
    } else {
        parse_pseudo_class_identifier(p)
    }
}

const PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET: TokenSet<CssSyntaxKind> = token_set![DIR_KW];

#[inline]
fn is_at_pseudo_class_function_identifier(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER))
}

const PSEUDO_CLASS_FUNCTION_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![MATCHES_KW, NOT_KW, IS_KW, WHERE_KW];

#[inline]
fn is_at_pseudo_class_function_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_selector_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssSelectorList::default()
        .with_end_kind(T![')'])
        .parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST))
}

const PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![MOZANY_KW, WEBKITANY_KW, PAST_KW, CURRENT_KW, FUTURE_KW];

#[inline]
fn is_at_pseudo_class_function_compound_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_compound_selector_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_compound_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssCompoundSelectorList.parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST))
}

struct CssCompoundSelectorList;

impl CssCompoundSelectorList {
    const RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T![')']];
}
impl ParseSeparatedList for CssCompoundSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_COMPOUND_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_compound_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SELECTOR, Self::RECOVERY_SET),
            expect_any_selector,
        )
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}

const PSEUDO_CLASS_FUNCTION_SELECTOR_SET: TokenSet<CssSyntaxKind> = token_set![GLOBAL_KW, LOCAL_KW];
#[inline]
fn is_at_pseudo_class_function_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_SELECTOR_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_selector(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_selector(p).or_add_diagnostic(p, expect_any_selector);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_SELECTOR))
}

const PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET: TokenSet<CssSyntaxKind> =
    token_set![HOST_KW, HOSTCONTEXT_KW];
#[inline]
fn is_at_pseudo_class_function_compound_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_compound_selector(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_compound_selector(p).or_add_diagnostic(p, expect_any_selector);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR))
}

const PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![HAS_KW];

#[inline]
fn is_at_pseudo_class_function_relative_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_relative_selector_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_relative_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssRelativeSelectorList.parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST))
}

struct CssRelativeSelectorList;

impl CssRelativeSelectorList {
    const RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T![')']];
}
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
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SELECTOR, Self::RECOVERY_SET),
            expect_any_selector,
        )
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

    if is_at_relative_selector_combinator(p) {
        p.bump_any();
    }

    parse_selector(p).or_add_diagnostic(p, expect_any_selector);

    Present(m.complete(p, CSS_RELATIVE_SELECTOR))
}

const PSEUDO_CLASS_FUNCTION_VALUE_LIST_SET: TokenSet<CssSyntaxKind> = token_set![LANG_KW];

#[inline]
fn is_at_pseudo_class_function_value_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_VALUE_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_value_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_value_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssPseudoValueList.parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST))
}

struct CssPseudoValueList;

impl CssPseudoValueList {
    const RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T![')']];
}
impl ParseSeparatedList for CssPseudoValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_PSEUDO_VALUE_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_pseudo_value(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, Self::RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}
#[inline]
fn is_at_pseudo_value(p: &mut CssParser) -> bool {
    is_at_identifier(p) || p.at(CSS_STRING_LITERAL)
}

#[inline]
fn parse_pseudo_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_value(p) {
        return Absent;
    }

    if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p)
    } else {
        parse_regular_identifier(p)
    }
}

const PSEUDO_CLASS_FUNCTION_NTH_SET: TokenSet<CssSyntaxKind> = token_set![
    NTHCHILD_KW,
    NTHLASTCHILD_KW,
    NTHOFTYPE_KW,
    NTHLASTOFTYPE_KW,
    NTHCOL_KW,
    NTHLASTCOL_KW
];

#[inline]
fn is_at_pseudo_class_function_nth(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_NTH_SET) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_pseudo_class_function_nth(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_nth(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);

    p.bump_with_context(T!['('], CssLexContext::PseudoNthSelector);
    parse_pseudo_class_nth_selector(p).or_add_diagnostic(p, expect_any_selector);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_NTH))
}

const PSEUDO_CLASS_FUNCTION_NTH_CLASS_SET: TokenSet<CssSyntaxKind> =
    token_set![T![odd], T![even], T![n], CSS_NUMBER_LITERAL, T![+], T![-],];
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

    parse_pseudo_class_nth(p).or_add_diagnostic(p, expected_pseudo_class_nth);
    parse_pseudo_class_of_nth_selector(p).ok();

    Present(m.complete(p, CSS_PSEUDO_CLASS_NTH_SELECTOR))
}

#[inline]
fn parse_pseudo_class_nth(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_nth_selector(p) {
        return Absent;
    }

    let m = p.start();

    let kind = match p.cur() {
        T![odd] | T![even] => {
            p.bump_any();
            CSS_PSEUDO_CLASS_NTH_IDENTIFIER
        }
        // +123
        T![+] | T![-] if p.nth_at(1, CSS_NUMBER_LITERAL) && !is_nth_at_identifier(p, 2)  => {
            p.bump_any();
            parse_regular_number(p).ok();

            CSS_PSEUDO_CLASS_NTH_NUMBER
        }
        // +n + 3
        // -n + 3
        //  n + 3
        // -2n + 1
        // +2n- 1
        T![+] | T![-] | T![n] => {
            parse_nth_multiplier(p).ok();
            p.expect_with_context(T![n], CssLexContext::PseudoNthSelector);
            parse_nth_offset(p).ok();

            CSS_PSEUDO_CLASS_NTH
        }
        // 123
        // 2n + 4
        // 2n- 4
        _ => {
            parse_number(p, CssLexContext::PseudoNthSelector).ok();

            if p.eat_with_context(T![n], CssLexContext::PseudoNthSelector) {
                parse_nth_offset(p).ok();

                CSS_PSEUDO_CLASS_NTH
            } else {
                CSS_PSEUDO_CLASS_NTH_NUMBER
            }
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
fn parse_nth_multiplier(p: &mut CssParser) -> ParsedSyntax {
    if !matches!(p.cur(), T![+] | T![-]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(p.cur(), CssLexContext::PseudoNthSelector);
    parse_number(p, CssLexContext::PseudoNthSelector).ok();

    Present(m.complete(p, CSS_NTH_MULTIPLIER))
}

#[inline]
fn parse_nth_offset(p: &mut CssParser) -> ParsedSyntax {
    if !matches!(p.cur(), T![+] | T![-]) {
        return Absent;
    }

    let m = p.start();

    p.bump_any();
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

#[inline]
fn parse_pseudo_class_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_PSEUDO_CLASS_IDENTIFIER))
}

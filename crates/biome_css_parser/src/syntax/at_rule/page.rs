use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::{
    expected_any_page_at_rule_item, expected_page_selector, expected_page_selector_pseudo,
};
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::blocks::{parse_block_body, parse_or_recover_declaration_or_at_rule_list_block};
use crate::syntax::parse_error::expected_block;
use crate::syntax::{
    is_at_identifier, parse_custom_identifier_with_keywords, parse_declaration_with_semicolon,
    BODY_RECOVERY_SET,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_page_at_rule(p: &mut CssParser) -> bool {
    p.at(T![page])
}

#[inline]
pub(crate) fn parse_page_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_page_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![page]);

    PageSelectorList.parse_list(p);

    if parse_page_block(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_block,
        )
        .is_err()
    {
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    Present(m.complete(p, CSS_PAGE_AT_RULE))
}

const PAGE_SELECTOR_SET: TokenSet<CssSyntaxKind> = token_set!(T![ident], T![:]);
const PAGE_SELECTOR_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    PAGE_SELECTOR_SET.union(token_set!(T!['{']));
struct PageSelectorList;

impl ParseSeparatedList for PageSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PAGE_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_page_selector(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_SELECTOR, PAGE_SELECTOR_LIST_RECOVERY_SET),
            expected_page_selector,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

#[inline]
fn is_at_page_selector(p: &mut CssParser) -> bool {
    is_at_identifier(p) || p.at(T![:])
}

#[inline]
pub(crate) fn parse_page_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_page_selector(p) {
        return Absent;
    }

    let m = p.start();

    // it's optional
    //
    // Because the selector is formally defined as `<ident>` _and_ also case-
    // sensitive, we use a `<custom-ident>` instead to preserve the casing, but
    // need to allow the CSS-wide keywords that would otherwise be disallowed.
    parse_custom_identifier_with_keywords(p, CssLexContext::Regular, true).ok();
    PageSelectorPseudoList.parse_list(p);

    Present(m.complete(p, CSS_PAGE_SELECTOR))
}

const PAGE_SELECTOR_PSEUDO_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set!(T!['{'], T![,]);
const PAGE_SELECTOR_PSEUDO_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    PAGE_SELECTOR_PSEUDO_LIST_END_SET.union(token_set!(T![:]));

struct PageSelectorPseudoList;

impl ParseNodeList for PageSelectorPseudoList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PAGE_SELECTOR_PSEUDO_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_page_selector_pseudo(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(PAGE_SELECTOR_PSEUDO_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PSEUDO_CLASS,
                PAGE_SELECTOR_PSEUDO_LIST_RECOVERY_SET,
            ),
            expected_page_selector_pseudo,
        )
    }
}

const PAGE_SELECTOR_PSEUDO_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![left], T![right], T![first], T![blank]);

#[inline]
fn is_at_page_selector_pseudo(p: &mut CssParser) -> bool {
    p.at(T![:])
}

#[inline]
pub(crate) fn parse_page_selector_pseudo(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_page_selector_pseudo(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);

    let kind = if p.eat_ts(PAGE_SELECTOR_PSEUDO_SET) {
        CSS_PAGE_SELECTOR_PSEUDO
    } else {
        CSS_BOGUS_PAGE_SELECTOR_PSEUDO
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn parse_page_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = parse_block_body(p, |p| {
        PageAtRuleItemList.parse_list(p);
    });

    Present(m.complete(p, CSS_PAGE_AT_RULE_BLOCK))
}

const CSS_PAGE_AT_RULE_ITEM_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![@], T![ident]);
struct PageAtRuleItemList;
impl ParseNodeList for PageAtRuleItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PAGE_AT_RULE_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if at_margin_rule(p) {
            parse_margin_at_rule(p)
        } else if is_at_at_rule(p) {
            parse_at_rule(p)
        } else {
            parse_declaration_with_semicolon(p)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, CSS_PAGE_AT_RULE_ITEM_LIST_RECOVERY_SET),
            expected_any_page_at_rule_item,
        )
    }
}

const PAGE_MARGIN_AT_RULE_NAME_SET: TokenSet<CssSyntaxKind> = token_set!(
    T![top_left_corner],
    T![top_left],
    T![top_center],
    T![top_right],
    T![top_right_corner],
    T![bottom_left_corner],
    T![bottom_left],
    T![bottom_center],
    T![bottom_right],
    T![bottom_right_corner],
    T![left_top],
    T![left_middle],
    T![left_bottom],
    T![right_top],
    T![right_middle],
    T![right_bottom],
);

#[inline]
pub(crate) fn at_margin_rule(p: &mut CssParser) -> bool {
    p.at(T![@]) && p.nth_at_ts(1, PAGE_MARGIN_AT_RULE_NAME_SET)
}
#[inline]
pub(crate) fn parse_margin_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !at_margin_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![@]);
    p.bump_ts(PAGE_MARGIN_AT_RULE_NAME_SET);

    if parse_or_recover_declaration_or_at_rule_list_block(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    };

    Present(m.complete(p, CSS_MARGIN_AT_RULE))
}

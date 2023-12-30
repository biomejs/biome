use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::{
    expected_keyframes_item, expected_keyframes_item_selector,
};
use crate::syntax::blocks::parse_declaration_list_block;
use crate::syntax::css_dimension::{is_at_percentage_dimension, parse_percentage_dimension};
use crate::syntax::parse_error::{expected_block, expected_non_css_wide_keyword_identifier};
use crate::syntax::{is_at_identifier, parse_custom_identifier, parse_string, BODY_RECOVERY_SET};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_keyframes_at_rule(p: &mut CssParser) -> bool {
    p.at(T![keyframes])
}

#[inline]
pub(crate) fn parse_keyframes_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![keyframes]);

    let name = if is_at_identifier(p) {
        parse_custom_identifier(p, CssLexContext::Regular)
    } else {
        parse_string(p)
    };

    let kind = if name
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, KEYFRAMES_NAME_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_non_css_wide_keyword_identifier,
        )
        .is_ok()
    {
        CSS_KEYFRAMES_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    if parse_keyframes_block(p)
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

    Present(m.complete(p, kind))
}
const KEYFRAMES_NAME_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

#[inline]
fn parse_keyframes_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['{']);
    KeyframesItemList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_KEYFRAMES_BLOCK))
}

const KEYFRAMES_ITEM_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    KEYFRAMES_ITEM_SELECTOR_SET.union(token_set!(T!['}']));

struct KeyframesItemList;

impl ParseNodeList for KeyframesItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_KEYFRAMES_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_keyframes_item(p)
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
            &ParseRecoveryTokenSet::new(CSS_BOGUS_KEYFRAMES_ITEM, KEYFRAMES_ITEM_LIST_RECOVERY_SET),
            expected_keyframes_item,
        )
    }
}

#[inline]
fn parse_keyframes_item(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    KeyframesSelectorList.parse_list(p);
    // `parse_list` will take care of recovering invalid selectors, but if
    // _none_ are present, we still want to add a diagnostic to explain the
    // error while continuing the rest of the parse, since we know that the
    // following content should still be a declaration list block.
    if p.cur_range().start() == m.start() {
        p.error(expected_keyframes_item_selector(p, p.cur_range()))
    }

    if parse_declaration_list_block(p)
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

    Present(m.complete(p, CSS_KEYFRAMES_ITEM))
}

struct KeyframesSelectorListParseRecovery;

impl ParseRecovery for KeyframesSelectorListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SELECTOR;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{']) || is_at_keyframes_item_selector(p)
    }
}

struct KeyframesSelectorList;

impl ParseSeparatedList for KeyframesSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_KEYFRAMES_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_keyframes_item_selector(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &KeyframesSelectorListParseRecovery,
            expected_keyframes_item_selector,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

const KEYFRAMES_ITEM_SELECTOR_IDENT_SET: TokenSet<CssSyntaxKind> = token_set!(T![from], T![to]);
const KEYFRAMES_ITEM_SELECTOR_SET: TokenSet<CssSyntaxKind> =
    KEYFRAMES_ITEM_SELECTOR_IDENT_SET.union(token_set!(CSS_PERCENTAGE_VALUE));

fn is_at_keyframes_item_selector(p: &mut CssParser) -> bool {
    p.at_ts(KEYFRAMES_ITEM_SELECTOR_IDENT_SET) || is_at_percentage_dimension(p)
}
#[inline]
fn parse_keyframes_item_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_item_selector(p) {
        return Absent;
    }

    let m = p.start();

    let kind = if is_at_percentage_dimension(p) {
        parse_percentage_dimension(p).ok();
        CSS_KEYFRAMES_PERCENTAGE_SELECTOR
    } else {
        p.bump_ts(KEYFRAMES_ITEM_SELECTOR_IDENT_SET);
        CSS_KEYFRAMES_IDENT_SELECTOR
    };

    Present(m.complete(p, kind))
}

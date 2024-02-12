use super::parse_error::{expected_any_font_feature_value_item, expected_font_feature_values_item};
use crate::syntax::blocks::parse_or_recover_declaration_list_block;
use crate::{
    lexer::CssLexContext,
    parser::CssParser,
    syntax::{
        is_at_identifier, parse_custom_identifier,
        parse_error::{expected_block, expected_non_css_wide_keyword_identifier},
        parse_string, BODY_RECOVERY_SET,
    },
};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecoveryTokenSet, RecoveryResult},
    parsed_syntax::ParsedSyntax::{self, Absent, Present},
    prelude::*,
    Parser, TokenSet,
};

#[inline]
pub(crate) fn is_at_font_feature_values_at_rule(p: &mut CssParser) -> bool {
    p.at(T![font_feature_values])
}

#[inline]
pub(crate) fn parse_font_feature_values_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_font_feature_values_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![font_feature_values]);

    // TODO: handle font family e.g. `Font One`
    let name = if is_at_identifier(p) {
        parse_custom_identifier(p, CssLexContext::Regular)
    } else {
        parse_string(p)
    };

    let kind = if name
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, FONT_FEATURE_VALUES_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_non_css_wide_keyword_identifier,
        )
        .is_ok()
    {
        CSS_FONT_FEATURE_VALUES_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    if parse_font_feature_values_block(p)
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

#[inline]
fn parse_font_feature_values_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    FontFeatureValuesItemList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_FONT_FEATURE_VALUES_BLOCK))
}

struct FontFeatureValuesItemList;

impl ParseNodeList for FontFeatureValuesItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_FONT_FEATURE_VALUES_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_font_feature_values_item(p)
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
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_FONT_FEATURE_VALUES_ITEM,
                FONT_FEATURE_VALUES_ITEM_LIST_RECOVERY_SET,
            ),
            expected_font_feature_values_item,
        )
    }
}

#[inline]
fn parse_font_feature_values_item(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);

    match p.cur() {
        T![stylistic]
        | T![historical_forms]
        | T![styleset]
        | T![character_variant]
        | T![swash]
        | T![ornaments]
        | T![annotation] => p.bump_any(),
        _ => p.error(expected_any_font_feature_value_item(p, p.cur_range())),
    };

    if parse_or_recover_declaration_list_block(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_BLOCK));
    }

    Present(m.complete(p, CSS_FONT_FEATURE_VALUES_ITEM))
}

const FONT_FEATURE_VALUES_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];
const FONT_FEATURE_VALUES_ITEM_SET: TokenSet<CssSyntaxKind> = token_set![
    T![stylistic],
    T![historical_forms],
    T![styleset],
    T![character_variant],
    T![swash],
    T![ornaments],
    T![annotation]
];
const FONT_FEATURE_VALUES_ITEM_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    FONT_FEATURE_VALUES_ITEM_SET.union(token_set![T!['}']]);

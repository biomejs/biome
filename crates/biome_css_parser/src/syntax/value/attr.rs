use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::TokenSet;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::token_set;
use biome_parser::{Parser, prelude::ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_regular_identifier;
use crate::syntax::property::GenericComponentValueList;
use crate::syntax::value::r#type::is_at_type_function;
use crate::syntax::value::r#type::parse_type_function;

/// https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Values_and_units/Numeric_data_types#distance_units
const CSS_DISTANCE_UNIT_SET: TokenSet<CssSyntaxKind> = token_set![
    // local font-relative units
    T![cap],
    T![ch],
    T![em],
    T![ex],
    T![ic],
    T![lh],
    // root font-relative units
    T![rcap],
    T![rch],
    T![rem],
    T![rex],
    T![ric],
    T![rlh],
    // viewport units
    T![dvh],
    T![dvw],
    T![lvh],
    T![lvw],
    T![svh],
    T![svw],
    T![vb],
    T![vh],
    T![vi],
    T![vmax],
    T![vmin],
    T![vw],
    // container units
    T![cqb],
    T![cqh],
    T![cqi],
    T![cqmax],
    T![cqmin],
    T![cqw],
    // absolute length units
    T![cm],
    T![in],
    T![mm],
    T![pc],
    T![pt],
    T![px],
    T![q],
    // angle units
    T![deg],
    T![grad],
    T![rad],
    T![turn],
    // time units
    T![ms],
    T![s],
    T![hz],
    T![khz],
    // flex units
    T![fr],
    // resolution units
    T![dpcm],
    T![dpi],
    T![dppx],
];

#[inline]
pub(crate) fn is_at_attr_function(p: &mut CssParser) -> bool {
    p.at(T![attr]) && p.nth_at(1, T!['('])
}

/// Parses an attribute function.
#[inline]
pub(crate) fn parse_attr_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_function(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![attr]);
    p.bump(T!['(']);

    AttrNameList.parse_list(p);
    // todo: bogus for this
    parse_attr_type(p).ok();
    // todo: bogus for this
    parse_attr_fallback_value(p).ok();

    p.expect(T![')']);

    Present(m.complete(p, CSS_ATTR_FUNCTION))
}

#[inline]
fn is_at_attr_type(p: &mut CssParser) -> bool {
    p.at(T![raw_string]) || p.at(T![number]) || is_at_type_function(p) || is_at_attr_unit(p)
}

#[inline]
fn parse_attr_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_type(p) {
        return Absent;
    }

    if is_at_type_function(p) {
        return parse_type_function(p);
    }

    if p.at(T![raw_string]) {
        let m = p.start();
        p.bump(T![raw_string]);
        return Present(m.complete(p, CSS_RAW_STRING_DECLARATOR));
    }

    if p.at(T![number]) {
        let m = p.start();
        p.bump(T![number]);
        return Present(m.complete(p, CSS_NUMBER_DECLARATOR));
    }

    if is_at_attr_unit(p) {
        return parse_attr_unit(p);
    }

    Absent
}

#[inline]
fn is_at_attr_unit(p: &mut CssParser) -> bool {
    p.at(T![%]) || p.at_ts(CSS_DISTANCE_UNIT_SET)
}

#[inline]
fn parse_attr_unit(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_unit(p) {
        return Absent;
    }

    let m = p.start();

    if p.at(T![%]) {
        p.bump(T![%]);
        return Present(m.complete(p, CSS_PERCENT_SIGN));
    }

    p.bump_ts(CSS_DISTANCE_UNIT_SET);
    Present(m.complete(p, CSS_DISTANCE_UNIT))
}

#[inline]
fn is_at_attr_fallback_value(p: &mut CssParser) -> bool {
    p.at(T![,])
}

#[inline]
fn parse_attr_fallback_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attr_fallback_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![,]);
    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_ATTR_FALLBACK_VALUE))
}

struct AttrNameListParseRecovery;

impl ParseRecovery for AttrNameListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // 1. At a new attr name
        // 2. At fallback value
        // 3. At end of attr() or maybe type()
        // 4. At the end of the declaration
        p.at(T![|]) || p.at(T![,]) || p.at(T![')']) || p.at(T![;])
    }
}

struct AttrNameList;

impl ParseSeparatedList for AttrNameList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_ATTR_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_regular_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_attr_type(p) || p.at(T![,]) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &AttrNameListParseRecovery, expected_identifier)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

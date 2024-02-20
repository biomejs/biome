use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::{is_at_any_value, is_at_identifier, parse_any_value, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

pub(crate) fn is_at_any_property(p: &mut CssParser) -> bool {
    is_at_generic_property(p)
}

pub(crate) fn parse_any_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_property(p) {
        return Absent;
    }

    parse_generic_property(p)
}

#[inline]
pub(crate) fn is_at_generic_property(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

#[inline]
pub(crate) fn parse_generic_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_property(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();

    p.expect(T![:]);

    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_GENERIC_PROPERTY))
}
const CSS_END_OF_PROPERTY_VALUE_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set!(T!['}'], T![;]);

pub(crate) struct GenericComponentValueList;

impl ParseNodeList for GenericComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(CSS_END_OF_PROPERTY_VALUE_TOKEN_SET) || p.at(T![')']) || /* !token is !important */ p.at(T![!])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PROPERTY_VALUE,
                CSS_END_OF_PROPERTY_VALUE_TOKEN_SET,
            ),
            expected_component_value,
        )
    }
}

#[inline]
pub(crate) fn is_at_generic_component_value(p: &mut CssParser) -> bool {
    is_at_any_value(p) || is_at_generic_delimiter(p)
}

#[inline]
pub(crate) fn parse_generic_component_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_component_value(p) {
        return Absent;
    }

    if is_at_generic_delimiter(p) {
        parse_generic_delimiter(p)
    } else {
        parse_any_value(p)
    }
}

const GENERIC_DELIMITER_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![/]];
#[inline]
pub(crate) fn is_at_generic_delimiter(p: &mut CssParser) -> bool {
    p.at_ts(GENERIC_DELIMITER_SET)
}

#[inline]
pub(crate) fn parse_generic_delimiter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_delimiter(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(GENERIC_DELIMITER_SET);
    Present(m.complete(p, CSS_GENERIC_DELIMITER))
}

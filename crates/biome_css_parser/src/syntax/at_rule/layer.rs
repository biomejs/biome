use crate::parser::CssParser;

use crate::syntax::blocks::parse_or_recover_rule_list_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_layer_at_rule(p: &mut CssParser) -> bool {
    p.at(T![layer])
}

#[inline]
pub(crate) fn parse_layer_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_layer_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![layer]);

    parse_any_layer(p);

    Present(m.complete(p, CSS_LAYER_AT_RULE))
}

#[inline]
pub(crate) fn parse_any_layer(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    CssLayerReferenceList.parse_list(p);

    if p.at(T!['{']) {
        let kind = if parse_or_recover_rule_list_block(p).is_ok() {
            CSS_LAYER_DECLARATION
        } else {
            CSS_BOGUS_LAYER
        };

        m.complete(p, kind)
    } else {
        let kind = if p.expect(T![;]) {
            CSS_LAYER_REFERENCE
        } else {
            CSS_BOGUS_LAYER
        };

        m.complete(p, kind)
    }
}

const LAYER_REFERENCE_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set!(T!['{'], T![;]);
const LAYER_REFERENCE_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    LAYER_REFERENCE_LIST_END_SET.union(token_set!(T![,]));

struct CssLayerReferenceList;

impl ParseSeparatedList for CssLayerReferenceList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_LAYER_REFERENCE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        Present(CssLayerNameList.parse_list(p))
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        (!is_at_identifier(p) && !p.at(T![,])) || p.at_ts(LAYER_REFERENCE_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, LAYER_REFERENCE_LIST_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

const LAYER_NAME_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set!(T![,], T!['{'], T![;]);
const LAYER_NAME_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    LAYER_NAME_LIST_END_SET.union(token_set!(T![.]));

struct CssLayerNameList;

impl ParseSeparatedList for CssLayerNameList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_LAYER_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_regular_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        (!is_at_identifier(p) && !p.at(T![.])) || p.at_ts(LAYER_NAME_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, LAYER_NAME_LIST_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![.]
    }
}

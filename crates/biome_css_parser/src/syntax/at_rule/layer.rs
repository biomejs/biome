use crate::parser::CssParser;

use crate::syntax::block::parse_conditional_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_regular_identifier;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
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

    LayerReferenceList.parse_list(p);

    let kind = if p.at(T!['{']) {
        parse_conditional_block(p);
        CSS_LAYER_DECLARATION
    } else if p.expect(T![;]) {
        CSS_LAYER_REFERENCE
    } else {
        CSS_BOGUS_LAYER
    };
    m.complete(p, kind)
}

const LAYER_REFERENCE_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set!(T!['{'], T![;]);
const LAYER_REFERENCE_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    LAYER_REFERENCE_LIST_END_SET.union(token_set!(T![,]));

struct LayerReferenceList;

impl ParseSeparatedList for LayerReferenceList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_LAYER_REFERENCE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        LayerNameList.parse_list(p).into()
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(LAYER_REFERENCE_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, LAYER_REFERENCE_LIST_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

const LAYER_NAME_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set!(T![')'], T![,], T!['{'], T![;]);
const LAYER_NAME_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    LAYER_NAME_LIST_END_SET.union(token_set!(T![.]));

pub(crate) struct LayerNameList;

impl ParseSeparatedList for LayerNameList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_LAYER_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // The Spec for `<layer-name>` technically adds that "The CSS-wide
        // keywords are reserved for future use, and cause the rule to be
        // invalid at parse time"...but it's unclear if that means it's a
        // _parse error_, or just not a valid layer. The examples in the
        // document use `default`, which was an _old_ keyword (renamed to
        // `revert` in Level 4), and all of the validators I've found allow
        // keywords as identifiers here, so for now we will continue to
        // allow them as well.
        //
        // https://drafts.csswg.org/css-cascade-5/#typedef-layer-name
        parse_regular_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(LAYER_NAME_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, LAYER_NAME_LIST_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![.]
    }
}

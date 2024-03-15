use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::token_set;
use biome_parser::TokenSet;
use biome_parser::{parsed_syntax::ParsedSyntax, Parser};

use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use crate::syntax::parse_dashed_identifier;
use crate::syntax::parse_error::expected_dashed_identifier;

#[inline]
pub(crate) fn is_at_property_at_rule(p: &mut CssParser) -> bool {
    p.at(T![property])
}

#[inline]
pub(crate) fn parse_property_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_property_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![property]);

    let kind = if parse_dashed_identifier(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, PROPERTY_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_dashed_identifier,
        )
        .is_ok()
    {
        CSS_PROPERTY_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    parse_declaration_block(p);

    Present(m.complete(p, kind))
}

const PROPERTY_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set!(T!['{']);

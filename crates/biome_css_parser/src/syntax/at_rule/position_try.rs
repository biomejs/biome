use crate::syntax::block::parse_declaration_block;
use crate::syntax::parse_dashed_identifier;
use crate::{parser::CssParser, syntax::parse_error::expected_dashed_identifier};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T,
};
use biome_parser::{
    parse_recovery::ParseRecoveryTokenSet,
    parsed_syntax::ParsedSyntax::{self, Present},
    prelude::ParsedSyntax::Absent,
    token_set, Parser, TokenSet,
};

#[inline]
pub(crate) fn is_at_position_try_at_rule(p: &mut CssParser) -> bool {
    p.at(T![position_try])
}

#[inline]
pub(crate) fn parse_position_try_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_position_try_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![position_try]);

    let kind = if parse_dashed_identifier(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, POSITION_TRY_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_dashed_identifier,
        )
        .is_ok()
    {
        CSS_POSITION_TRY_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    parse_declaration_block(p);

    Present(m.complete(p, kind))
}

const POSITION_TRY_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

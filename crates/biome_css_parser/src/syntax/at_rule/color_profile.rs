use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use crate::syntax::parse_custom_identifier;
use crate::syntax::parse_error::expected_non_css_wide_keyword_identifier;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_color_profile_at_rule(p: &mut CssParser) -> bool {
    p.at(T![color_profile])
}

#[inline]
pub(crate) fn parse_color_profile_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_color_profile_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![color_profile]);

    // TODO: This should actually be `<dashed-ident> | device-cmyk`.
    let kind = if parse_custom_identifier(p, CssLexContext::Regular)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, COLOR_PROFILE_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_non_css_wide_keyword_identifier,
        )
        .is_ok()
    {
        CSS_COLOR_PROFILE_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    parse_declaration_block(p);

    Present(m.complete(p, kind))
}

const COLOR_PROFILE_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

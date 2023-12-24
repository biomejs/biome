use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{parse_or_recover_declaration_list_block, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_counter_style_at_rule(p: &mut CssParser) -> bool {
    p.at(T![counter_style])
}

#[inline]
pub(crate) fn parse_counter_style_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_counter_style_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![counter_style]);

    let kind = if parse_regular_identifier(p)
        .or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, COUNTER_STYLE_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_identifier,
        )
        .is_ok()
    {
        CSS_COUNTER_STYLE_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    if parse_or_recover_declaration_list_block(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    Present(m.complete(p, kind))
}

const COUNTER_STYLE_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

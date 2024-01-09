use crate::parser::CssParser;
use crate::syntax::parse_error::expected_string;
use crate::syntax::parse_string;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, TextRange, T};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_rowan::SyntaxKind;

#[inline]
pub(crate) fn is_at_charset_at_rule(p: &mut CssParser) -> bool {
    p.at(T![charset])
}

#[inline]
pub(crate) fn parse_charset_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_charset_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![charset]);

    let kind = match parse_string(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS, CHARTSET_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_string,
    ) {
        Ok(encoding) if !encoding.kind(p).is_bogus() => {
            if eat_or_recover_close_token(p, encoding) {
                CSS_CHARSET_AT_RULE
            } else {
                CSS_BOGUS_AT_RULE
            }
        }
        _ => {
            p.expect(T![;]);
            CSS_BOGUS_AT_RULE
        }
    };

    Present(m.complete(p, kind))
}

const CHARTSET_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![;]];

#[inline]
fn eat_or_recover_close_token(p: &mut CssParser, encoding: CompletedMarker) -> bool {
    if p.eat(T![;]) {
        true
    } else {
        if let Ok(m) = ParseRecoveryTokenSet::new(CSS_BOGUS, CHARTSET_RECOVERY_SET)
            .enable_recovery_on_line_break()
            .recover(p)
        {
            let diagnostic = expected_string(
                p,
                TextRange::new(encoding.range(p).start(), m.range(p).end()),
            );
            p.error(diagnostic);
        }

        p.expect(T![;]);

        false
    }
}

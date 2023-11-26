use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::SELECTOR_FUNCTION_RECOVERY_SET;
use biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, TextRange, T};
use biome_parser::diagnostic::{expected_any, ParseDiagnostic};
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET: TokenSet<CssSyntaxKind> = token_set![DIR_KW];
#[inline]
pub(crate) fn is_at_pseudo_class_function_identifier(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_identifier(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need to check if the identifier is valid, because we already did that
    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = if is_at_dir_parameter_identifier(p) {
        let identifier = parse_regular_identifier(p);

        if eat_or_recover_function_close_token(p, identifier) {
            CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
        } else {
            CSS_BOGUS_PSEUDO_CLASS
        }
    } else {
        recover_invalid_dir_parameter_identifier(p);
        p.expect(T![')']);
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

#[inline]
fn eat_or_recover_function_close_token(p: &mut CssParser, identifier: ParsedSyntax) -> bool {
    if p.eat(T![')']) {
        true
    } else {
        let start = identifier
            .ok()
            .map(|m| m.range(p).start())
            .unwrap_or_else(|| p.cur_range().start());

        if let Ok(marker) = ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET).recover(p)
        {
            p.error(expected_dir_parameter_identifier(
                p,
                TextRange::new(start, marker.range(p).end()),
            ));
        }

        p.expect(T![')']);

        false
    }
}
const DIR_PARAMETER_IDENTIFIER_SET: TokenSet<CssSyntaxKind> = token_set![LTR_KW, RTL_KW];
#[inline]
fn is_at_dir_parameter_identifier(p: &mut CssParser) -> bool {
    p.at_ts(DIR_PARAMETER_IDENTIFIER_SET)
}

#[inline]
fn recover_invalid_dir_parameter_identifier(p: &mut CssParser) {
    let start = p.cur_range().start();

    match ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET).recover(p) {
        Ok(marker) => {
            p.error(expected_dir_parameter_identifier(
                p,
                TextRange::new(start, marker.range(p).end()),
            ));
        }
        Err(_) => {
            p.error(expected_dir_parameter_identifier(p, p.cur_range()));
        }
    }
}

#[inline]
fn expected_dir_parameter_identifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["ltr", "rtl"], range, p)
}

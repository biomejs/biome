use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_any;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::{ParseDiagnostic, ParsedSyntax, ToDiagnostic};
use biome_parser::{token_set, Parser, ParserProgress, TokenSet};
use biome_rowan::TextRange;

const SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T!['}'],];
const BODY_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SELECTOR_RECOVERY_SET.union(token_set![T![.], T![*],]);

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();

    parse_rules_list(p).expect("Parse rule list, handle this case");

    m.complete(p, CSS_ROOT);
}

pub(crate) fn parse_rules_list(p: &mut CssParser) -> ParsedSyntax {
    let rules = p.start();
    while !p.at(EOF) {
        match p.cur() {
            T![.] => {
                parse_rule(p).expect("Parse rule, handle this case properly");
            }
            _ => return Absent,
        }
    }

    let completed = rules.complete(p, CSS_RULE_LIST);

    Present(completed)
}

pub(crate) fn parse_rule(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    if parse_selector_list(p)
        .or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_PATTERN, SELECTOR_RECOVERY_SET),
            expect_pattern,
        )
        .is_err()
    {
        m.abandon(p);
        return Absent;
    }

    if parse_css_block(p)
        .or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_BODY, BODY_RECOVERY_SET),
            expect_block,
        )
        .is_err()
    {
        m.abandon(p);
        return Absent;
    }

    let completed = m.complete(p, CSS_RULE);
    Present(completed)
}

pub(crate) fn parse_selector_list(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && !p.at(T!['{']) {
        progress.assert_progressing(p);

        match p.cur() {
            T![.] => {
                parse_css_selector_pattern(p).expect("Handle this case");
            }

            _ => {
                return Absent;
            }
        }
    }
    if p.at(EOF) {
        m.abandon(p);
        return Absent;
    }

    Present(m.complete(p, CSS_SELECTOR_LIST))
}

#[inline]
pub(crate) fn parse_css_selector_pattern(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![.]);

    match p.cur() {
        IDENT => {
            let m = p.start();
            p.bump(IDENT);
            m.complete(p, CSS_IDENTIFIER);
        }

        _ => {
            m.abandon(p);
            return Absent;
        }
    }

    Present(m.complete(p, CSS_CLASS_SELECTOR_PATTERN))
}

pub(crate) fn parse_css_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    let list = p.start();
    list.complete(p, CSS_DECLARATION_LIST);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_BLOCK))
}

fn expect_pattern(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["selector pattern"], range).into_diagnostic(p)
}

fn expect_block(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["body"], range).into_diagnostic(p)
}

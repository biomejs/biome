mod parse_error;
mod pattern;

use crate::parser::CssParser;
use crate::syntax::parse_error::{expect_any_pattern, expect_block};
use crate::syntax::pattern::{
    parse_class_selector_pattern, parse_id_selector_pattern, parse_type_selector_pattern,
    parse_universal_selector_pattern,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, ParserProgress, TokenSet};

const RULE_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![T![#], T![.], T![*], T![ident], T![:], T!['{']];
const SELECTOR_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T!['}'],];
const BODY_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SELECTOR_LIST_RECOVERY_SET.union(RULE_RECOVERY_SET);

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();

    parse_rule_list(p);

    m.complete(p, CSS_ROOT);
}

pub(crate) fn parse_rule_list(p: &mut CssParser) {
    let mut progress = ParserProgress::default();

    let rules = p.start();
    while !p.at(EOF) {
        progress.assert_progressing(p);

        if parse_rule(p)
            .or_recover(
                p,
                &ParseRecovery::new(CSS_BOGUS_RULE, RULE_RECOVERY_SET),
                expect_any_pattern,
            )
            .is_err()
        {
            break;
        }
    }

    rules.complete(p, CSS_RULE_LIST);
}

pub(crate) fn parse_rule(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();

    CssSelectorList.parse_list(p);

    if parse_rule_block(p)
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

pub(crate) struct CssSelectorList;

impl ParseSeparatedList for CssSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![.] => parse_class_selector_pattern(p),
            T![#] => parse_id_selector_pattern(p),
            T![*] => parse_universal_selector_pattern(p),
            _ if is_at_identifier(p) => parse_type_selector_pattern(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                CSS_BOGUS_PATTERN,
                RULE_RECOVERY_SET.union(token_set![T![,]]),
            )
            .enable_recovery_on_line_break(),
            expect_any_pattern,
        )
    }
    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

#[inline]
pub(crate) fn parse_rule_block(p: &mut CssParser) -> ParsedSyntax {
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

#[inline]
pub(super) fn parse_identifier(p: &mut CssParser, kind: CssSyntaxKind) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![ident]);
    let identifier = m.complete(p, kind);

    Present(identifier)
}
#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    matches!(p.cur(), T![ident]) || p.cur().is_contextual_keyword()
}

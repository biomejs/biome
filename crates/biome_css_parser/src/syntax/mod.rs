mod parse_error;
mod selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{expect_any_selector, expect_block};
use crate::syntax::selector::parse_selector;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, ParserProgress, TokenSet};

const RULE_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![T![#], T![.], T![*], T![ident], T![:], T![::], T!['{']];
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
                expect_any_selector,
            )
            .is_err()
        {
            break;
        }
    }

    rules.complete(p, CSS_RULE_LIST);
}

#[inline]
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

const SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = RULE_RECOVERY_SET.union(token_set![T![,]]);

pub(crate) struct CssSelectorList;

impl ParseSeparatedList for CssSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_selector(p)
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
            &ParseRecovery::new(CSS_BOGUS_SELECTOR, SELECTOR_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expect_any_selector,
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
pub(super) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}
#[inline]
pub(super) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}
#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}

#[inline]
pub(crate) fn parse_css_string(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(CSS_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

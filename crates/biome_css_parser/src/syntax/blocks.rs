use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::parse_error::{expected_any_declaration_or_at_rule, expected_block};
use crate::syntax::{
    parse_declaration_with_semicolon, DeclarationList, RuleList, BODY_RECOVERY_SET,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn parse_or_recover_declaration_list_block(p: &mut CssParser) -> RecoveryResult {
    parse_declaration_list_block(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_declaration_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['{']);
    DeclarationList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_DECLARATION_LIST_BLOCK))
}

#[inline]
pub(crate) fn parse_or_recover_rule_list_block(p: &mut CssParser) -> RecoveryResult {
    parse_rule_list_block(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_rule_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['{']);
    RuleList::new(T!['}']).parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_RULE_LIST_BLOCK))
}

#[inline]
pub(crate) fn parse_or_recover_declaration_or_rule_list_block(p: &mut CssParser) -> RecoveryResult {
    parse_declaration_or_rule_list_block(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_declaration_or_rule_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['{']);
    DeclarationOrAtRuleList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_DECLARATION_OR_AT_RULE_BLOCK))
}

const CSS_DECLARATION_OR_AT_RULE_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![@], T![ident]);
struct DeclarationOrAtRuleList;
impl ParseNodeList for DeclarationOrAtRuleList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_OR_AT_RULE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_rule(p) {
            parse_at_rule(p)
        } else {
            parse_declaration_with_semicolon(p)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, CSS_DECLARATION_OR_AT_RULE_LIST_RECOVERY_SET),
            expected_any_declaration_or_at_rule,
        )
    }
}

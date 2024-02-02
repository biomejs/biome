use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::parse_error::{expected_any_declaration_or_at_rule, expected_block};
use crate::syntax::{
    is_at_declaration, is_at_nested_qualified_rule, parse_declaration_with_semicolon,
    parse_nested_qualified_rule, DeclarationList, RuleList, BODY_RECOVERY_SET,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser};

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

    let m = parse_block_body(p, |p| {
        DeclarationList.parse_list(p);
    });

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

    let m = parse_block_body(p, |p| {
        RuleList::new(T!['}']).parse_list(p);
    });

    Present(m.complete(p, CSS_RULE_LIST_BLOCK))
}

#[inline]
pub(crate) fn parse_or_recover_declaration_or_at_rule_list_block(
    p: &mut CssParser,
) -> RecoveryResult {
    parse_declaration_or_at_rule_list_block(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_declaration_or_at_rule_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = parse_block_body(p, |p| {
        DeclarationOrAtRuleList.parse_list(p);
    });

    Present(m.complete(p, CSS_DECLARATION_OR_AT_RULE_BLOCK))
}

struct DeclarationOrAtRuleListParseRecovery;
impl ParseRecovery for DeclarationOrAtRuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || is_at_at_rule(p) || is_at_declaration(p)
    }
}

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
        parsed_element.or_recover(
            p,
            &DeclarationOrAtRuleListParseRecovery,
            expected_any_declaration_or_at_rule,
        )
    }
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

    let m = parse_block_body(p, |p| {
        DeclarationOrRuleList.parse_list(p);
    });

    Present(m.complete(p, CSS_DECLARATION_OR_RULE_BLOCK))
}

struct DeclarationOrRuleListParseRecovery;
impl ParseRecovery for DeclarationOrRuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || is_at_nested_qualified_rule(p) || is_at_at_rule(p) || is_at_declaration(p)
    }
}

struct DeclarationOrRuleList;
impl ParseNodeList for DeclarationOrRuleList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_OR_RULE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_rule(p) {
            parse_at_rule(p)
        } else if is_at_declaration(p) {
            parse_declaration_with_semicolon(p)
        } else {
            parse_nested_qualified_rule(p)
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
        parsed_element.or_recover(
            p,
            &DeclarationOrRuleListParseRecovery,
            expected_any_declaration_or_at_rule,
        )
    }
}

/// Parses the body of a block in CSS.
///
/// This function handles the parsing of a block's content, delimited by curly braces `{}`.
/// It temporarily sets the parser's state to indicate it is within a nesting block and then
/// processes the content of the block using the provided callback function.
pub(crate) fn parse_block_body(p: &mut CssParser, func: impl FnOnce(&mut CssParser)) -> Marker {
    let old_nesting_block = std::mem::replace(&mut p.state_mut().is_nesting_block, true);

    let m = p.start();
    p.bump(T!['{']);
    func(p);
    p.expect(T!['}']);

    p.state_mut().is_nesting_block = old_nesting_block;
    m
}

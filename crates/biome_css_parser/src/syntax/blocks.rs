use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::parse_error::expected_any_declaration_or_at_rule;
use crate::syntax::{
    is_at_declaration, is_at_nested_qualified_rule, parse_declaration_with_semicolon,
    parse_nested_qualified_rule, DeclarationList, RuleList,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::{CompletedMarker, Marker, Parser};

#[inline]
pub(crate) fn parse_declaration_list_block(p: &mut CssParser) -> CompletedMarker {
    let m = parse_block_body(p, |p| {
        DeclarationList.parse_list(p);
    });

    m.complete(p, CSS_DECLARATION_LIST_BLOCK)
}

#[inline]
pub(crate) fn parse_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    let m = parse_block_body(p, |p| {
        RuleList::new(T!['}']).parse_list(p);
    });

    m.complete(p, CSS_RULE_LIST_BLOCK)
}

#[inline]
fn is_at_declaration_or_at_rule_item(p: &mut CssParser) -> bool {
    is_at_at_rule(p) || is_at_declaration(p)
}

#[inline]
pub(crate) fn parse_declaration_or_at_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    let m = parse_block_body(p, |p| {
        DeclarationOrAtRuleList.parse_list(p);
    });

    m.complete(p, CSS_DECLARATION_OR_AT_RULE_BLOCK)
}

struct DeclarationOrAtRuleListParseRecovery;
impl ParseRecovery for DeclarationOrAtRuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || is_at_declaration_or_at_rule_item(p)
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
        } else if is_at_declaration(p) {
            parse_declaration_with_semicolon(p)
        } else {
            Absent
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
fn is_at_declaration_or_rule_item(p: &mut CssParser) -> bool {
    is_at_nested_qualified_rule(p) || is_at_declaration_or_at_rule_item(p)
}

#[inline]
pub(crate) fn parse_declaration_or_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    let m = parse_block_body(p, |p| {
        DeclarationOrRuleList.parse_list(p);
    });

    m.complete(p, CSS_DECLARATION_OR_RULE_BLOCK)
}

struct DeclarationOrRuleListParseRecovery;
impl ParseRecovery for DeclarationOrRuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || is_at_declaration_or_rule_item(p)
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
        } else if is_at_nested_qualified_rule(p) {
            parse_nested_qualified_rule(p)
        } else {
            Absent
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
    p.expect(T!['{']);
    func(p);
    p.expect(T!['}']);

    p.state_mut().is_nesting_block = old_nesting_block;
    m
}

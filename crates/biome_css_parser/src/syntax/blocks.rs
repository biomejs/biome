use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::parse_error::expected_any_declaration_or_at_rule;
use crate::syntax::{
    is_at_declaration, is_at_nested_qualified_rule, is_at_rule_list_element,
    parse_declaration_with_semicolon, parse_nested_qualified_rule, DeclarationList, RuleList,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::{CompletedMarker, Parser};
use biome_parser::diagnostic::{expected_node, ParseDiagnostic};
use biome_rowan::TextRange;

struct DeclarationListBlock;

impl ParseBlockBody for DeclarationListBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_DECLARATION_LIST_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_declaration(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        DeclarationList.parse_list(p);
    }
}

#[inline]
pub(crate) fn parse_declaration_list_block(p: &mut CssParser) -> CompletedMarker {
    DeclarationListBlock.parse_block_body(p)
}

struct RuleListBlock;

impl ParseBlockBody for RuleListBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_RULE_LIST_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_rule_list_element(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        RuleList::new(T!['}']).parse_list(p);
    }
}

#[inline]
pub(crate) fn parse_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    RuleListBlock.parse_block_body(p)
}

#[inline]
pub(crate) fn parse_declaration_or_at_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    DeclarationOrAtRuleListBlock.parse_block_body(p)
}

struct DeclarationOrAtRuleListBlock;

impl ParseBlockBody for DeclarationOrAtRuleListBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_DECLARATION_OR_AT_RULE_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_declaration_or_at_rule_item(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        DeclarationOrAtRuleList.parse_list(p);
    }
}

#[inline]
fn is_at_declaration_or_at_rule_item(p: &mut CssParser) -> bool {
    is_at_at_rule(p) || is_at_declaration(p)
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
pub(crate) fn parse_declaration_or_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    DeclarationOrRuleListBlock.parse_block_body(p)
}

struct DeclarationOrRuleListBlock;

impl ParseBlockBody for DeclarationOrRuleListBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_DECLARATION_OR_RULE_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_declaration_or_rule_item(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        DeclarationOrRuleList.parse_list(p);
    }
}

#[inline]
fn is_at_declaration_or_rule_item(p: &mut CssParser) -> bool {
    is_at_at_rule(p) || is_at_nested_qualified_rule(p) || is_at_declaration_or_at_rule_item(p)
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

pub(crate) trait ParseBlockBody {
    const BLOCK_KIND: CssSyntaxKind;

    /// If the '{' is missing try we need to recover:
    ///  - Try to check if the next item is an item of a list. If it is, we can parse the list.
    ///  - If the next item is not a list item, we return bogus block and skip the list parsing.
    fn is_at_element(&self, p: &mut CssParser) -> bool;

    fn parse_list(&mut self, p: &mut CssParser);

    /// Parses the body of a block in CSS.
    ///
    /// This function handles the parsing of a block's content, delimited by curly braces `{}`.
    /// It temporarily sets the parser's state to indicate it is within a nesting block and then
    /// processes the content of the block using the provided callback function.
    fn parse_block_body(&mut self, p: &mut CssParser) -> CompletedMarker {
        let m = p.start();

        if !p.eat(T!['{']) && !self.is_at_element(p) {
            p.error(expected_block(p, p.cur_range()));
            return m.complete(p, CSS_BOGUS_BLOCK);
        }

        let old_nesting_block = std::mem::replace(&mut p.state_mut().is_nesting_block, true);

        self.parse_list(p);
        p.expect(T!['}']);

        p.state_mut().is_nesting_block = old_nesting_block;
        m.complete(p, Self::BLOCK_KIND)
    }
}

pub(crate) fn expected_block(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("body", range, p)
}

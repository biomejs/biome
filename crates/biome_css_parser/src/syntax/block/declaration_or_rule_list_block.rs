use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::block::ParseBlockBody;
use crate::syntax::parse_error::expected_any_declaration_or_at_rule;
use crate::syntax::{
    is_at_declaration, is_at_nested_qualified_rule, parse_declaration_with_semicolon,
    parse_nested_qualified_rule,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::{CompletedMarker, Parser};

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
    is_at_at_rule(p) || is_at_nested_qualified_rule(p) || is_at_declaration(p)
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

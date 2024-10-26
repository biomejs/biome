use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::block::ParseBlockBody;
use crate::syntax::parse_error::expected_any_declaration_or_at_rule;
use crate::syntax::{
    is_at_declaration, is_at_declaration_semicolon, is_at_metavariable,
    is_at_nested_qualified_rule, parse_declaration_with_semicolon, parse_empty_declaration,
    parse_metavariable, parse_nested_qualified_rule, try_parse,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;
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
    is_at_at_rule(p)
        || is_at_nested_qualified_rule(p)
        || is_at_declaration(p)
        || is_at_metavariable(p)
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
            // if we are at a declaration,
            // we still can have a nested qualified rule or a declaration
            // E.g.
            // main {
            //     label:hover {  <---
            // it looks like a declaration but it is a nested qualified rule
            //         font-weight: 500;
            //     }
            // }
            // Attempt to parse the current block as a declaration.
            let declaration = try_parse(p, |p| {
                let declaration = parse_declaration_with_semicolon(p);
                // Check if the *last* token parsed is a semicolon
                // (;) or if the parser is at a closing brace (}).
                // ; - Indicates the end of a declaration.
                // Indicates the end of the last declaration because `;` is optional.
                // E.g
                // .class {
                //     color: red; <----
                // The semicolon indicates the end of the declaration.
                //     font-size: 16px
                // } <---
                // The closing brace indicates the end of the declaration block.
                // If either condition is true, the declaration is considered valid.
                if matches!(p.last(), Some(T![;])) || p.at(T!['}']) {
                    Ok(declaration)
                } else {
                    Err(())
                }
            });

            // If parsing as a declaration was successful, return the parsed declaration.
            if let Ok(declaration) = declaration {
                return declaration;
            }

            // If parsing as a declaration failed,
            // attempt to parse the current block as a nested qualified rule.
            let rule = try_parse(p, |p| {
                // Parse the block as a nested qualified rule.
                let rule = parse_nested_qualified_rule(p);
                // Check if the *last* token parsed is a closing brace (}).
                // Indicates the end of a rule block.
                // If true, the nested qualified rule is considered valid.
                if matches!(p.last(), Some(T!['}'])) {
                    Ok(rule)
                } else {
                    // If the condition is not met, return an error to indicate parsing failure.
                    Err(())
                }
            });

            // If parsing as a nested qualified rule was successful, return the parsed rule.
            if let Ok(rule) = rule {
                return rule;
            }

            // If both parsing attempts fail,
            // fall back to parsing the block as a declaration,
            // because declaration error is more relevant.
            parse_declaration_with_semicolon(p)
        } else if is_at_nested_qualified_rule(p) {
            parse_nested_qualified_rule(p)
        } else if is_at_metavariable(p) {
            parse_metavariable(p)
        } else if is_at_declaration_semicolon(p) {
            parse_empty_declaration(p)
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

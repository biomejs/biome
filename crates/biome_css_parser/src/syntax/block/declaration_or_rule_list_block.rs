use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::block::ParseBlockBody;
use crate::syntax::parse_error::{expected_any_declaration_or_at_rule, scss_only_syntax_error};
use crate::syntax::scss::{
    is_at_scss_declaration, is_at_scss_nesting_declaration, parse_scss_declaration,
    parse_scss_nesting_declaration,
};
use crate::syntax::{
    CssSyntaxFeatures, is_at_any_declaration_with_semicolon, is_at_metavariable,
    is_at_nested_qualified_rule, is_nth_at_identifier, parse_any_declaration_with_semicolon,
    parse_metavariable, parse_nested_qualified_rule, try_parse,
    try_parse_nested_qualified_rule_without_selector_recovery,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::token_source::TokenSourceWithBufferedLexer;
use biome_parser::{CompletedMarker, Parser, SyntaxFeature};

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
        DeclarationOrRuleList::new(T!['}']).parse_list(p);
    }
}

#[inline]
fn is_at_declaration_or_rule_item(p: &mut CssParser) -> bool {
    is_at_at_rule(p)
        || is_at_nested_qualified_rule(p)
        || is_at_scss_nesting_declaration(p)
        || is_at_scss_declaration(p)
        || is_at_any_declaration_with_semicolon(p)
        || is_at_metavariable(p)
}

#[inline]
fn has_whitespace_after_scss_property_colon(p: &mut CssParser) -> bool {
    // We enter this helper at `ident` in `ident:...`.
    // `nth_non_trivia(1)` is the `:` token, so `nth_non_trivia(2)` is the first token
    // after `:`. Its preceding flags tell us whether there was spacing after the colon.
    let Some(after_colon) = p.source_mut().lexer().nth_non_trivia(2) else {
        return false;
    };

    after_colon.has_preceding_whitespace() || after_colon.has_preceding_line_break()
}

#[inline]
fn is_at_ambiguous_scss_nesting_item(p: &mut CssParser) -> bool {
    // Match Sass's ambiguity gate: only no-spacing `name:ident` and `name::...`
    // forms can be nested selectors.
    !has_whitespace_after_scss_property_colon(p)
        && (is_nth_at_identifier(p, 2) || p.nth_at(2, T![:]))
}

struct DeclarationOrRuleListParseRecovery {
    end_kind: CssSyntaxKind,
}

impl DeclarationOrRuleListParseRecovery {
    fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

impl ParseRecovery for DeclarationOrRuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind) || is_at_declaration_or_rule_item(p)
    }
}

pub(crate) struct DeclarationOrRuleList {
    end_kind: CssSyntaxKind,
}

impl DeclarationOrRuleList {
    pub(crate) fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

impl ParseNodeList for DeclarationOrRuleList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_OR_RULE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_rule(p) {
            parse_at_rule(p)
        } else if is_at_scss_nesting_declaration(p) {
            let is_ambiguous = is_at_ambiguous_scss_nesting_item(p);

            if is_ambiguous {
                // Match Sass's declaration-first strategy for ambiguous `name:ident` and
                // `name::...` forms. Parse as declaration first, then backtrack to selector
                // parsing when the result is declaration-like but selector-ambiguous.
                let declaration = try_parse(p, |p| {
                    let declaration = parse_scss_nesting_declaration(p);

                    match declaration.kind(p) {
                        Some(SCSS_NESTING_DECLARATION) => Err(()),
                        Some(CSS_DECLARATION_WITH_SEMICOLON)
                            if matches!(p.last(), Some(T![;])) || p.at(self.end_kind) =>
                        {
                            Ok(declaration)
                        }
                        _ => Err(()),
                    }
                });

                if let Ok(declaration) = declaration {
                    return declaration;
                }

                if let Ok(rule) =
                    try_parse_nested_qualified_rule_without_selector_recovery(p, self.end_kind)
                {
                    return rule;
                }
            }

            parse_scss_nesting_declaration(p)
        } else if is_at_scss_declaration(p) {
            CssSyntaxFeatures::Scss.parse_exclusive_syntax(
                p,
                parse_scss_declaration,
                |p, marker| {
                    scss_only_syntax_error(p, "SCSS variable declarations", marker.range(p))
                },
            )
        } else if is_at_any_declaration_with_semicolon(p) {
            // if we are at a declaration,
            // we still can have a nested qualified rule or a declaration
            // E.g.
            // main {
            //     label:hover {  <---
            // it looks like a declaration but it is a nested qualified rule
            //         font-weight: 500;
            //     }
            // }

            // Reset the if-function flag before parsing. This flag is used to detect
            // if we encountered an if() function during speculative parsing.
            p.state_mut().encountered_if_function = false;

            // Attempt to parse the current block as a declaration.
            let declaration = try_parse(p, |p| {
                let declaration = parse_any_declaration_with_semicolon(p);

                // If we encountered an if() function, always fail speculative parsing.
                // The if() function uses semicolons as branch separators, which can cause
                // p.last() to be `;` even when the declaration is incomplete. By failing
                // here, we force a non-speculative retry where recovery is enabled.
                if p.state().encountered_if_function {
                    return Err(());
                }

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
                let valid = matches!(p.last(), Some(T![;])) || p.at(self.end_kind);
                if valid { Ok(declaration) } else { Err(()) }
            });

            // If parsing as a declaration was successful, return the parsed declaration.
            if let Ok(declaration) = declaration {
                return declaration;
            }

            // If the speculative parse failed and we encountered an if() function,
            // we know this is a declaration (not a rule) because if() can only appear
            // in declaration values. Parse again with recovery enabled.
            if std::mem::take(&mut p.state_mut().encountered_if_function) {
                return parse_any_declaration_with_semicolon(p);
            }

            // If parsing as a declaration failed,
            // attempt to parse the current block as a nested qualified rule.
            let rule = try_parse(p, |p| {
                // Parse the block as a nested qualified rule.
                let rule = parse_nested_qualified_rule(p);
                // Check if the *last* token parsed is a closing brace (}).
                // Indicates the end of a rule block.
                // If true, the nested qualified rule is considered valid.
                if p.last().is_some_and(|kind| kind == self.end_kind) {
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
            parse_any_declaration_with_semicolon(p)
        } else if is_at_nested_qualified_rule(p) {
            parse_nested_qualified_rule(p)
        } else if is_at_metavariable(p) {
            parse_metavariable(p)
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &DeclarationOrRuleListParseRecovery::new(self.end_kind),
            expected_any_declaration_or_at_rule,
        )
    }
}

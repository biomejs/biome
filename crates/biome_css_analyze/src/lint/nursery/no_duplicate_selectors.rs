use std::collections::HashSet;
use std::vec;

use biome_analyze::{AddVisitor, Ast, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, AnyCssSelector, CssAtRule, CssComplexSelector, CssCompoundSelector, CssDeclarationBlock, CssDeclarationOrAtRuleBlock, CssDeclarationOrRuleBlock, CssLanguage, CssNestedQualifiedRule, CssQualifiedRule, CssRelativeSelector, CssRuleBlock, CssRuleList, CssSelectorList, CssSyntaxList, CssSyntaxNode};
use biome_rowan::{AstNode, AstNodeList, Language, SyntaxList, SyntaxNode, SyntaxNodeCast, TextRange, WalkEvent};

declare_rule! {
    /// 
    /// 
    pub NoDuplicateSelectors {
        version: "next",
        name: "noDuplicateSelectors",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-duplicate-selectors")],
    }
}

impl Rule for NoDuplicateSelectors {
    type Query = Ast<CssRuleList>;
    type State = CssSyntaxNode;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let node = ctx.query();
        let mut resolved_list = HashSet::new();
        let mut output: Vec<CssSyntaxNode> = vec!();

        let selectors = node
            .syntax()
            .descendants()
            .filter_map(|x|x.cast::<AnyCssSelector>());
        
        for selector in selectors {
            let mut this_selctor_list = selector.clone().into_syntax().parent().unwrap();

            if let Some(parent_selector) = CssComplexSelector::cast_ref(&this_selctor_list) {
                // Don't handle the children of complex selectors
                this_selctor_list = parent_selector.into_syntax().parent().unwrap()
            } else if let Some(parent_selector) = CssRelativeSelector::cast_ref(&this_selctor_list) {
                // Don't handle the children of complex relative
                this_selctor_list = parent_selector.into_syntax().parent().unwrap();
            }

            let this_rule = this_selctor_list.parent().unwrap();
            let this_block = this_rule.grand_parent().unwrap();

            let resolved = resolve_nested_selectors(selector.clone().text(), this_block);
            for r in resolved {
                println!("resolved: {:?}", r);
                if !resolved_list.insert(r) {
                    output.push(selector.clone().into_syntax());
                }
            }
        }
        output
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.text_range(),
                markup! {
                    "Duplicate selector."
                },
            ).note(markup! {
                "Please fix it."
            }),
        )
    }
}

// TODO: need to handle AtRules etc.
fn resolve_nested_selectors(selector: String, block: CssSyntaxNode) -> Vec<String> {
    let mut parent_selectors: Vec<String> = vec!();

    let parent_node = get_parent_block(block);

    match &parent_node {
        None => {
            return vec!(selector)
        },
        Some(parent_block) => {
            if let Some(parent_node_parent) = parent_block.parent() {
                if let Some(parent_rule) = AnyCssRule::cast_ref(&parent_node_parent){
                    match parent_rule {
                        AnyCssRule::CssBogusRule(_) => todo!(),
                        AnyCssRule::CssAtRule(parent_rule) => {
                            // Treat the AtRule as a selector
                            let rule = parent_rule.rule().unwrap();
                            parent_selectors.push(rule.text());
                        },
                        AnyCssRule::CssNestedQualifiedRule(parent_rule) => {
                            for selector in parent_rule.prelude() {
                                if let Ok(selector) = selector {
                                    parent_selectors.push(selector.text());
                                }
                            }
                        },
                        AnyCssRule::CssQualifiedRule(parent_rule) => {
                            for selector in parent_rule.prelude() {
                                if let Ok(selector) = selector {
                                    parent_selectors.push(selector.text());
                                }
                            }
                        },
                    }

                }
            }

            let resolved_selectors: Vec<String> = parent_selectors.iter().fold(vec!(), |result: Vec<String>, parent_selector|{
                match &parent_node {
                    Some(parent_node) => {
                        if selector.contains("&") {
                            let resolved_parent_selectors = resolve_nested_selectors(parent_selector.to_string(), parent_node.clone());
                            let resolved = resolved_parent_selectors.into_iter().map(|newly_resolved|{
                                // TODO: Need to handle the case where an equivalent is the result of an &
                                // e.g. .a.c { &.b } == .a.b.c but the order will be a.c.b
                                return selector.replace("&", &newly_resolved)
                            }).collect();
                            return [result, resolved].concat();
                        } else {
                            let combined_selectors = parent_selector.to_owned()+ " " + &selector;
                            let resolved = resolve_nested_selectors(combined_selectors, parent_node.clone());
                            return [result, resolved].concat();
                        }
                    }
                    None => result,
                }
            });
            if resolved_selectors.len() > 0 {
                return resolved_selectors
            }
            return vec!(selector)
        },
    }
}

// This does not handle the highest level rules
fn get_parent_block(this_block: CssSyntaxNode) -> Option<CssSyntaxNode> {
    if let Some(parent_rule) = this_block.parent() {
        return parent_rule.grand_parent();
    }
    return None
}


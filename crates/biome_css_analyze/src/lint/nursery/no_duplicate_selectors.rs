use std::borrow::Borrow;
use std::hash::Hash;
use std::{collections::HashSet, io};
use std::{println, vec};

use biome_analyze::{AddVisitor, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext};
use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssDeclarationOrRule, AnyCssRelativeSelector, AnyCssRule, AnyCssSelector, AnyCssSubSelector, CssBogus, CssComplexSelector, CssCompoundSelector, CssCustomIdentifier, CssDeclarationOrRuleBlock, CssIdentifier, CssLanguage, CssNestedQualifiedRule, CssQualifiedRule, CssRelativeSelector, CssRelativeSelectorList, CssRoot, CssRuleList, CssSelectorList, CssSyntaxElement, CssSyntaxNode};
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};

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

#[derive(Default)]

struct CssRuleSelectorListVisitor {
    stack: (
        Vec<AnyCssSelector>, 
        Vec<Vec<AnyCssSelector>>, 
        Vec<Vec<Vec<AnyCssSelector>>>
    ), // Current group, total group, total for all rules
}

impl Visitor for CssRuleSelectorListVisitor {
    type Language = CssLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(node) = CssSelectorList::cast_ref(node) {
                    // Push to the current group
                    for selector in node {
                        match selector {
                            Ok(result) => {
                                self.stack.0.push(result);
                            },
                            Err(_) => todo!(),
                        }
                    }
                }
                if let Some(node) = CssRelativeSelector::cast_ref(node) {
                    let selector = node.selector();
                    match selector {
                        Ok(result) => self.stack.0.push(result),
                        Err(_) => todo!(),
                    }
                }
                // Instead of a NQR we use a DRB because otherwise we will have dangling stack0
                if let Some(_node) = CssDeclarationOrRuleBlock::cast_ref(node) {
                    // Push the current group to the total group
                    self.stack.1.push(self.stack.0.clone());
                    self.stack.0.clear();
                }
            },
            WalkEvent::Leave(node) => {

                // End of rule, clear the stack
                if let Some(_node) = CssQualifiedRule::cast_ref(node) {
                    self.stack.2.push(self.stack.1.clone());
                    self.stack.1.clear();
                    self.stack.0.clear();
                }
                if let Some(_node) = CssNestedQualifiedRule::cast_ref(node) {
                    self.stack.2.push(self.stack.1.clone());
                    // Clear the last group from the rule
                    self.stack.1.pop();
                }
                if let Some(_node) = CssRuleList::cast_ref(node) {
                    ctx.match_query(CssRuleSelectorList(self.stack.2.clone()));
                }
            },
        }
    }
    
}

pub struct CssRuleSelectorList(Vec<Vec<Vec<AnyCssSelector>>>);

impl QueryMatch for CssRuleSelectorList {
    fn text_range(&self) -> TextRange {
        todo!()
    }
}

impl Queryable for CssRuleSelectorList {
    type Input = Self;
    type Language = CssLanguage;
    type Output = Vec<Vec<Vec<AnyCssSelector>>>;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        // Register our custom visitor to run in the `Syntax` phase
        analyzer.add_visitor(Phases::Syntax, CssRuleSelectorListVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(services: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
    

}

impl Rule for NoDuplicateSelectors {
    type Query = CssRuleSelectorList;
    type State = Vec<AnyCssSelector>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        println!("Run result:");
        let mut temp_state: Vec<AnyCssSelector> = vec!();
        for rule in node {
            let mut paths:Vec<String> = vec!();

            for group in rule {
                let mut group = group.clone();
                group.sort_by_key(|sel| sel.text());

                if paths.len() == 0 {
                    for selector in group.clone().into_iter() {
                        // sort sub selectors
                        match selector {
                            AnyCssSelector::CssCompoundSelector(compound_selector) => {
                                let mut prefix = String::new();

                                if let Some(simple_selector) = compound_selector.simple_selector(){
                                    prefix = simple_selector.text();
                                }
                                let text_list = compound_selector
                                    .sub_selectors()
                                    .into_iter()
                                    .map(|sub_selector| { sub_selector.text() });
                                let mut text_list: Vec<String> = text_list.collect();
                                text_list.sort();
    
                                let final_path = prefix + &text_list.join("");
                                if paths.contains(&final_path) {
                                    temp_state.push(compound_selector.into());
                                }
                                paths.push(final_path)
                            },
                            AnyCssSelector::CssComplexSelector(complex_selector) => {
                                // TODO: this is a brute force approach
                                // We can do better to join based on whether there is a simple selector or not
                                let complex_as_string = complex_selector.text();
                                let mut complex_as_list: Vec<String> = complex_as_string.split(" ").map(|s| s.to_string()).collect();
                                
                                complex_as_list.sort();

                                let final_path = complex_as_list.join(" ");
                                if paths.contains(&final_path) {
                                    temp_state.push(complex_selector.into());
                                }
                                paths.push(final_path);
                            }, 
                            _ => {
                                paths.push(selector.text());
                            }
                        }
                    }
                } else {
                    let path_clone = paths.clone();
                    paths.clear();
                    for path in path_clone {
                        for selector in group.clone().into_iter() {   
                            if let Some(compound_selector) = CssCompoundSelector::cast_ref(selector.syntax()) {
                                // Handle sub selector sorting
                                let mut prefix = "".to_string();
                                if let Some(simple_selector) = compound_selector.simple_selector(){
                                    prefix = simple_selector.text();
                                }
                                let text_list = compound_selector
                                    .sub_selectors()
                                    .into_iter()
                                    .map(|sub_selector| { sub_selector.text() });
                                let mut text_list: Vec<String> = text_list.collect();
                                text_list.sort();
                                // Handle the ampersand
                                if let Some(_nesting_selector_token) = compound_selector.nesting_selector_token() {
                                    let new_path = path.clone() + &prefix + &text_list.join("");
                                    let final_path = new_path.replace("&","");
                                    if paths.contains(&final_path) {
                                        temp_state.push(selector.clone());
                                    }
                                    paths.push(final_path);
                                    continue;
                                }
                                let final_path = path.clone() + " " + &prefix + &text_list.join("");
                                if paths.contains(&final_path) {
                                    temp_state.push(selector.clone());
                                }
                                paths.push(final_path);

                                continue;
                            } 
                            // sort sub selectors

                            let final_path = path.clone() + " " + &selector.text();
                            if paths.contains(&final_path) {
                                temp_state.push(selector.clone());
                            }
                            paths.push(final_path);
                        }
                    }
                }
            }

            for path in paths {
                println!("{path}")
            }
        }

        if temp_state.len() != 0 {
            return Some(temp_state);
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        for n in node.into_iter() {
            let n_syn = n.clone().into_syntax();
            return Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    n_syn.text_range(),
                    markup! {
                        "Unexpected duplicate selector" <Emphasis>{n_syn.to_string()}</Emphasis>
                    },
                )
                .note(markup! {
                        "This note will give you more information."
                }),
            )
        }
        None
    }
}

use std::collections::HashSet;
use std::vec;

use biome_analyze::{AddVisitor, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, AnyCssSelector, CssComplexSelector, CssCompoundSelector, CssDeclarationOrRuleBlock, CssLanguage, CssNestedQualifiedRule, CssQualifiedRule, CssRelativeSelector, CssRuleList};
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
struct DeclarationOrRuleBlockListVisitor {
    stack: Vec<CssDeclarationOrRuleBlock>
}

impl Visitor for DeclarationOrRuleBlockListVisitor {
    type Language = CssLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(node) = CssDeclarationOrRuleBlock::cast_ref(node) {
                    self.stack.push(node);
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(_node) = CssRuleList::cast_ref(node) {
                    ctx.match_query(DeclarationOrRuleBlockList(self.stack.clone()));
                    self.stack.clear();
                }
            }
        }
    }
}

pub struct DeclarationOrRuleBlockList(Vec<CssDeclarationOrRuleBlock>);

impl QueryMatch for DeclarationOrRuleBlockList {
    fn text_range(&self) -> TextRange {
        todo!()
    }
}

impl Queryable for DeclarationOrRuleBlockList {
    type Input = Self;
    type Output = Vec<CssDeclarationOrRuleBlock>;
    type Language = CssLanguage;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _root: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, DeclarationOrRuleBlockListVisitor::default);
    }

    fn unwrap_match(_services: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for NoDuplicateSelectors {
    type Query = DeclarationOrRuleBlockList;
    type State = SyntaxNode<CssLanguage>;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let node_list = ctx.query();

        let mut select_list = HashSet::new();
        let mut output: Vec<SyntaxNode<CssLanguage>> = vec!();

        for node in node_list {
            if let Some(this_rule) = node.syntax().parent() {
                let handled_rule = handle_css_rule(this_rule);
                for (selector_node, selector) in handled_rule {
                    let resolved_selectors = resolve_nested_selectors(selector, node.clone());
                    for resolved in resolved_selectors {
                        if !select_list.insert(resolved) {
                            output.push(selector_node.clone());
                        }
                    }
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
fn resolve_nested_selectors(selector: String, node: CssDeclarationOrRuleBlock) -> Vec<String> {
    let mut parent_selectors: Vec<String> = vec!();

    if let Some(this_rule) = node.syntax().parent() {
        if let Some(_qualified_rule) = CssQualifiedRule::cast_ref(&this_rule) {
            // Highest Level
            return vec!(selector);
        }
        
        let parent_node = get_parent_block(this_rule);

        if let Some(parent_block) = parent_node.clone() {
            if let Some(parent_node_parent) = parent_block.into_syntax().parent() {
                if let Some(parent_rule) = CssQualifiedRule::cast_ref(&parent_node_parent) {
                    for selector in parent_rule.prelude() {
                        if let Ok(selector) = selector {
                            parent_selectors.push(handle_css_selector(selector.into()));
                        }
                    }
                }
                if let Some(parent_rule) = CssNestedQualifiedRule::cast_ref(&parent_node_parent) {
                    for selector in parent_rule.prelude() {
                        if let Ok(selector) = selector {
                            parent_selectors.push(handle_css_selector(selector.into()));
                        }
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
            return resolved_selectors
        }
    }
    vec!(selector)
}

// This does not handle the highest level rules
fn get_parent_block(this_rule: SyntaxNode<CssLanguage>) -> Option<CssDeclarationOrRuleBlock> {
    if let Some(nested_qualified_rule) = CssNestedQualifiedRule::cast_ref(&this_rule) {
        if let Some(rule_grand_parent) = nested_qualified_rule.syntax().grand_parent(){
            if let Some(css_declaration_block) = CssDeclarationOrRuleBlock::cast_ref(&rule_grand_parent) {
                return Some(css_declaration_block)
            }
        }
    }
    None
}

fn handle_css_rule(rule: SyntaxNode<CssLanguage>) -> Vec<(SyntaxNode<CssLanguage>, String)> {
    let mut selector_list = vec!();
    if let Some(any_css_rule) = AnyCssRule::cast_ref(&rule) {
        match any_css_rule {
            AnyCssRule::CssAtRule(_) => todo!(),
            AnyCssRule::CssBogusRule(_) => todo!(),
            AnyCssRule::CssNestedQualifiedRule(nested_qualified_rule) => {
                for selector in nested_qualified_rule.prelude() {
                    if let Ok(valid_selector) = selector {
                        let selector_syntax = valid_selector.into_syntax();
                        selector_list.push(
                            (selector_syntax.clone(), handle_css_selector(selector_syntax))
                        );
                    }
                }
            },
            AnyCssRule::CssQualifiedRule(qualified_rule) => {
                for selector in qualified_rule.prelude() {
                    if let Ok(valid_selector) = selector {
                        let selector_syntax = valid_selector.into_syntax();
                        selector_list.push(
                            (selector_syntax.clone(), handle_css_selector(selector_syntax))
                        );
                    }
                }
            },
        }
    }
    selector_list
}

fn handle_css_selector(selector: SyntaxNode<CssLanguage>) -> String {
    if let Some(complex_selector) = CssComplexSelector::cast_ref(&selector) {
        let mut resolved = complex_selector.text();
        // This is to handle the special case of an empty combinator
        // i.e. .foo .bar == .bar .foo
        let mut left_right :Vec<String> = vec!();

        if let Ok(left) = complex_selector.left() {
            left_right.push(handle_css_selector(left.into_syntax()));
        }
        if let Ok(right) = complex_selector.right() {
            left_right.push(handle_css_selector(right.into_syntax()));
        }
        if let Ok(combinator) = complex_selector.combinator() {
            if combinator.text() == " " {
                left_right.sort()
            }
            resolved = left_right.join(combinator.text());
        }

        return resolved
    }
    if let Some(relative_selector) = CssRelativeSelector::cast_ref(&selector) {
        let mut resolved = String::new();
        if let Some(combinator) = relative_selector.combinator() {
            resolved.push_str(combinator.text());
        }
        if let Ok(selector) = relative_selector.selector(){
            resolved.push_str(&handle_css_selector(selector.into()));
        }
        return resolved
    }
    if let Some(compound_selector) = CssCompoundSelector::cast_ref(&selector) {
        return format_compound_selector(compound_selector)
    }
    if let Some(any) = AnyCssSelector::cast_ref(&selector) {
        return any.text();
    }
    selector.to_string()
}

fn format_compound_selector (selector: CssCompoundSelector) -> String {
    let nesting_selector_token = selector.nesting_selector_token();
    let sub_selectors = selector.sub_selectors();
    let simple_selector = selector.simple_selector();

    let mut resolved = String::new();
    if let Some(token) = nesting_selector_token {
        resolved.push_str(&token.text().trim());
    }
    if let Some(selector) = simple_selector {
        resolved.push_str(&selector.text());
    }
    let mut sub_selector_string: Vec<String> = sub_selectors.into_iter().map(|s|{
        return s.text()
    }).collect();
    sub_selector_string.sort();
    if sub_selector_string.len() > 0 {
        resolved.push_str(&sub_selector_string.join(""));
    }
    return resolved
}
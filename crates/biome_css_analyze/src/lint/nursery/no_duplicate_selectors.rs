use std::collections::HashSet;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{  AnyCssSelector,  CssSelectorList};
use biome_rowan::AstNode;

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
    type Query = Ast<CssSelectorList>;
    type State = AnyCssSelector;
    type Signals = Option<Self::State>;
    type Options = ();

    // TODO: Should allow duplicate in list (option)
    // TODO: Traverse and compare with the entire sheet
    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut selector_list = HashSet::new();

        for selector in node {
            let valid_selector = selector.ok()?;
            if !selector_list.insert(valid_selector.text()) {
                return Some(valid_selector)
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    // TODO: Update this
                    "Unexpected duplicate selector:" <Emphasis>{node.text()}</Emphasis>
                },
            )
            .note(markup! {
                    "This note will give you more information."
            }),
        )
    }
}

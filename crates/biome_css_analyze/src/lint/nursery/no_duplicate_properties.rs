use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashSet;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "next",
        name: "noDuplicateProperties",
        language: "css",
        recommended: false,
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range()).unwrap();

        let mut duplicates = Vec::new();
        let mut seen = FxHashSet::default();

        for declaration in rule.declarations.iter() {
            let property = &declaration.property;
            let prop_name = property.name.to_lowercase();
            let is_custom_propety = prop_name.starts_with("--");

            if !seen.insert(prop_name) && !is_custom_propety {
                duplicates.push(property.range);
            }
        }

        duplicates
    }

    fn diagnostic(_: &RuleContext<Self>, span: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected empty block is not allowed"
                },
            )
            .note(markup! {
                    "This note will give you more information."
            }),
        )
    }
}

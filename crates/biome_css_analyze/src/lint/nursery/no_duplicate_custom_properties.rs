use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_semantic::model::CssProperty;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashSet;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate custom properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate custom properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { --custom-property: pink; --custom-property: orange;  }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { --custom-property: pink; background: orange; --custom-property: orange }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { --custom-property: pink; }
    /// ```
    ///
    /// ```css
    /// a { --custom-property: pink; --cUstOm-prOpErtY: orange; }
    /// ```
    ///
    pub NoDuplicateCustomProperties {
        version: "1.9.0",
        name: "noDuplicateCustomProperties",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-custom-properties")],
    }
}

impl Rule for NoDuplicateCustomProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range())?;

        let properties = rule
            .declarations
            .iter()
            .map(|d| d.property.clone())
            .collect::<Vec<_>>();

        if let Some(range) = check_duplicate_custom_properties(properties) {
            return Some(range);
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Duplicate custom properties are not allowed."
                },
            )
            .note(markup! {
                    "Consider removing the duplicate custom property."
            }),
        )
    }
}

fn check_duplicate_custom_properties(properties: Vec<CssProperty>) -> Option<TextRange> {
    let mut seen = FxHashSet::default();

    for property in properties {
        if !seen.insert(property.name) {
            return Some(property.range);
        }
    }

    None
}

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_rowan::{AstNode, TextRange};
use biome_string_case::StrOnlyExtension;
use rustc_hash::FxHashSet;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   color: pink;
    ///   color: orange;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: pink;
    ///   background: orange;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "next",
        name: "noDuplicateProperties",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties")],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range()).unwrap();

        let mut seen = FxHashSet::default();

        for declaration in rule.declarations.iter() {
            let property = &declaration.property;
            let prop_name = property.name.to_lowercase_cow();
            let is_custom_propety = prop_name.starts_with("--");

            if !is_custom_propety && !seen.insert(prop_name) {
                return Some(property.range);
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, span: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally."
                },
            )
            .note(markup! {
                    "Remove or rename the duplicate property to ensure styling."
            }),
        )
    }
}

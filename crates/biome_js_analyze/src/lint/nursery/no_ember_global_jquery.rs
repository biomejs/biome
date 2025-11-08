use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{global_identifier, AnyJsExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of global `$` or `jQuery` objects.
    ///
    /// In Ember applications, you should import jQuery from Ember instead of using global references.
    /// Using global `$` or `jQuery` can lead to maintenance issues and makes it harder to track dependencies.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// $('.selector').hide();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// jQuery('.selector').addClass('active');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import $ from 'jquery';
    ///
    /// $('.selector').hide();
    /// ```
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    ///
    /// export default class MyComponent extends Component {
    ///   // No jQuery usage
    /// }
    /// ```
    ///
    pub NoEmberGlobalJquery {
        version: "next",
        name: "noEmberGlobalJquery",
        language: "js",
        recommended: true,
    }
}

impl Rule for NoEmberGlobalJquery {
    type Query = Semantic<AnyJsExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        // Use global_identifier to check if this is a reference to $ or jQuery
        let (reference, name) = global_identifier(node)?;
        let name_text = name.text();

        // Check if it's $ or jQuery
        if name_text != "$" && name_text != "jQuery" {
            return None;
        }

        // Check if it's a global reference (no binding means it's global)
        if model.binding(&reference).is_some() {
            // It's locally bound, not global - this is OK
            return None;
        }

        // It's global - report it
        Some(name_text.to_string())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use global "<Emphasis>{state}</Emphasis>"."
                },
            )
            .note(markup! {
                "Import jQuery explicitly instead of relying on the global reference."
            })
            .note(markup! {
                "Using explicit imports makes dependencies clearer and helps with bundling and tree-shaking."
            })
        )
    }
}

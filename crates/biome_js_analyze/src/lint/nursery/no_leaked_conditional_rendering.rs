use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsIdentifierBinding, JsLogicalExpression, JsxExpressionChild,
    JsxTagExpression, jsx_ext::AnyJsxElement,
};
use biome_rowan::AstNode;
use biome_rule_options::no_leaked_conditional_rendering::NoLeakedConditionalRenderingOptions;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoLeakedConditionalRendering {
        version: "next",
        name: "noLeakedConditionalRendering",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoLeakedConditionalRendering {
    type Query = Ast<JsxExpressionChild>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoLeakedConditionalRenderingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let jsx_expression = ctx.query();
        if !jsx_expression.to_trimmed_text().contains("&&") {
            return None;
        }
        match jsx_expression.expression()? {
            AnyJsExpression::JsLogicalExpression(exp) => {
                let left = exp.left().ok()?;
                let right = exp.right().ok()?;

                match left {
                    AnyJsExpression::JsIdentifierExpression(ident) => {
                        let ident = ident.name().ok()?;
                        if ident.to_trimmed_text() == "NaN" {}
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use rome_js_syntax::JsArrowFunctionExpression;

declare_rule! {
    /// Disallow arrow functions where they could be confused with comparisons.
    ///
    /// Arrow functions (`=>`) are similar in syntax to some comparison operators (`>`, `<`, `<=`, and `>=`).
    /// This rule warns against using the arrow function syntax in places where it could be confused with a comparison operator.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-confusing-arrow
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var x = a => 1 ? 2 : 3;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var x = (a) => 1 ? 2 : 3;
    ///
    /// var x = a => (1 ? 2 : 3);
    ///
    /// var x = (a) => (1 ? 2 : 3);
    ///
    /// var x = a => { return 1 ? 2 : 3; };
    ///
    /// var x = (a) => { return 1 ? 2 : 3; };
    /// ```
    ///
    pub(crate) NoConfusingArrow {
        version: "1.0.0",
        name: "noConfusingArrow",
        recommended: false,
    }
}

impl Rule for NoConfusingArrow {
    type Query = Ast<JsArrowFunctionExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let arrow_fn = ctx.query();
        if arrow_fn.parameters().ok()?.as_js_parameters().is_some() {
            // Don't report arrow functions that enclose its parameters with parenthesis.
            return None;
        }
        arrow_fn
            .body()
            .ok()?
            .as_any_js_expression()?
            .as_js_conditional_expression()
            .is_some()
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().fat_arrow_token().ok()?.text_trimmed_range(),
            markup! {
                "Fat arrows can be confused with some comparison operators ("
                    <Emphasis>"<"</Emphasis>", "
                    <Emphasis>">"</Emphasis>", "
                    <Emphasis>"<="</Emphasis>", "
                    <Emphasis>">="</Emphasis>
                ")."
            },
        ))
    }
}

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsConditionalExpression};
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow nested ternary expressions.
    ///
    /// Nesting ternary expressions can make code more difficult to understand.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const thing = foo ? bar : baz === qux ? quxx : foobar;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo ? baz === qux ? quxx() : foobar() : bar();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const thing = foo ? bar : foobar;
    /// ```
    ///
    /// ```js
    /// let thing;
    ///
    /// if (foo) {
    ///     thing = bar;
    /// } else if (baz === qux) {
    ///     thing = quxx;
    /// } else {
    ///     thing = foobar;
    /// }
    /// ```
    ///
    pub NoNestedTernary {
        version: "1.9.3",
        name: "noNestedTernary",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-nested-ternary")],
    }
}

impl Rule for NoNestedTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let alternate = node.alternate().ok()?;
        let consequent = node.consequent().ok()?;

        if let AnyJsExpression::JsConditionalExpression(expr) = consequent {
            return Some(expr.range());
        }

        if let AnyJsExpression::JsConditionalExpression(expr) = alternate {
            return Some(expr.range());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Do not nest ternary expressions."
                },
            )
            .note(markup! {
                "Nesting ternary expressions can make code more difficult to understand."
            })
            .note(markup! {
                "Convert nested ternary expression into if-else statements or separate the conditions to make the logic easier to understand."
            }),
        )
    }
}

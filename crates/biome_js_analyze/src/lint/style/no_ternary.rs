use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsConditionalExpression;
use biome_rowan::AstNode;
use biome_rule_options::no_ternary::NoTernaryOptions;

declare_lint_rule! {
    /// Disallow ternary operators.
    ///
    /// The ternary operator is used to conditionally assign a value to a variable.
    /// Some believe that the use of ternary operators leads to unclear code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = isBar ? baz : qux;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo;
    ///
    /// if (isBar) {
    /// 	foo = baz;
    /// } else {
    /// 	foo = qux;
    /// }
    /// ```
    ///
    pub NoTernary {
        version: "2.3.8",
        name: "noTernary",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-ternary").same()],
    }
}

impl Rule for NoTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoTernaryOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected ternary operator."
                },
            )
            .note(markup! {
                "Ternary operators can lead to unclear code. Use if-else statement instead."
            }),
        )
    }
}

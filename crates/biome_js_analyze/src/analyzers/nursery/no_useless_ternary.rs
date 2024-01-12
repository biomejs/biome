use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsConditionalExpression, JsSyntaxKind};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow ternary operators when simpler alternatives exist.
    ///
    /// It’s a common mistake in JavaScript to use a conditional expression to select between two
    /// boolean values instead of using the logical NOT (`!`) or double NOT (`!!`) to convert the test to a boolean.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = x ? true : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo === 1 ? false : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo + 1 ? false : true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = foo + 1 ? true : false;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = x === 2 ? 'Yes' : 'No';
    /// ```
    ///
    /// ```js
    /// var a = x === 2 ? 'Yes' : false;
    /// ```
    ///
    /// ## Resources
    ///
    /// Logical NOT: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT
    ///
    pub(crate) NoUselessTernary {
        version: "1.5.0",
        name: "noUselessTernary",
        source: RuleSource::Eslint("no-unneeded-ternary"),
        recommended: true,
    }
}

impl Rule for NoUselessTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let alternate = node.alternate().ok()?;
        let consequent = node.consequent().ok()?;

        if is_boolean_literal(&alternate) && is_boolean_literal(&consequent) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unnecessary use of boolean literals in conditional expression."
                },
            )
            .note(markup! {
                "Simplify your code by directly assigning the result without using a ternary operator."
            })
            .note(markup! {
                "If your goal is negation, you may use the logical NOT (!) or double NOT (!!) operator for clearer and concise code.\n Check for more details about "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT">"NOT"</Hyperlink>" operator."
            }),
        )
    }
}

fn is_boolean_literal(expression: &AnyJsExpression) -> bool {
    let expr_kind = expression.syntax().kind();
    if expr_kind == JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION {
        return true;
    }
    false
}

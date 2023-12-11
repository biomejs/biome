use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsConditionalExpression, JsSyntaxKind};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow ternary operators when simpler alternatives exist
    ///
    /// It’s a common mistake in JavaScript to use a conditional expression to select between two
    /// Boolean values instead of using ! to convert the test to a Boolean.
    /// Here are some examples:
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-unneeded-ternary/
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
    pub(crate) NoUselessTernary {
        version: "next",
        name: "noUselessTernary",
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

        if is_boolean(&alternate) && is_boolean(&consequent) {
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
                    "Disallow ternary operators when simpler alternatives exist."
                },
            )
            .note(markup! {
                "Unnecessary use of boolean literals in conditional expression.\n Simplify your code by directly assigning the result without using a ternary operator."
            }),
        )
    }
}

fn is_boolean(expression: &AnyJsExpression) -> bool {
    let expr_kind = expression.syntax().kind();
    if expr_kind == JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION {
        return true;
    }
    false
}

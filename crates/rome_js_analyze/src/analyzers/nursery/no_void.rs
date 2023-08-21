use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsUnaryExpression;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of `void` operators, which is not a familiar operator.
    ///
    /// > The `void` operator is often used merely to obtain the undefined primitive value,
    /// > usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-void
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// void 0;
    /// ```
    ///
    pub(crate) NoVoid {
        version: "1.0.0",
        name: "noVoid",
        recommended: false,
    }
}

impl Rule for NoVoid {
    type Query = Ast<JsUnaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        if expression.is_void().ok()? {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The use of "<Emphasis>"void"</Emphasis>" is not allowed."
            },
        ).note(
			markup!{
				"If you use "<Emphasis>"void"</Emphasis>" to alter the return type of a function or return `undefined`, use the global `undefined` instead."
			}
		))
    }
}

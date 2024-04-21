use biome_analyze::{
    context::RuleContext, declare_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsExpressionStatement};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow Array constructors.
    ///
    /// The corresponding ESLint rule:
    /// https://eslint.org/docs/latest/rules/no-array-constructor#rule-details
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Array(0, 1, 1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Array(500);
    /// [0, 1, 2];
    /// ```
    ///
    pub NoArrayConstructor {
        version: "next",
        name: "noArrayConstructor",
        sources: &[RuleSource::Eslint("no-array-constructor")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoArrayConstructor {
    type Query = Ast<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression_statement = ctx.query();

        let expression = expression_statement.expression().ok()?;

        match expression {
            AnyJsExpression::JsCallExpression(call_expression) => {
                let callee = call_expression.callee().ok()?;
                let arguments = call_expression.arguments().ok()?;
                return validate(callee, arguments);
            }
            AnyJsExpression::JsNewExpression(new_expression) => {
                let callee = new_expression.callee().ok()?;
                let arguments = new_expression.arguments()?;
                return validate(callee, arguments);
            }
            _ => {
                return None;
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use Array constructors."
                },
            )
            .note(markup! {
                "The array literal notation [] is preferable."
            }),
        )
    }
}

fn validate(callee: AnyJsExpression, arguments: JsCallArguments) -> Option<()> {
    let len = arguments.args().into_iter().count();
    if callee.text() == "Array" {
        if len == 1
            && !matches!(
                arguments.args().into_iter().nth(0)?.ok()?,
                AnyJsCallArgument::JsSpread(_)
            )
        {
            return None;
        }
        return Some(());
    } else {
        return None;
    }
}

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsExpressionStatement};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow Array constructors.
    ///
    /// Use of the Array constructor to construct a new array is generally discouraged in favor of array literal notation because of the single-argument pitfall and because the Array global may be redefined.
    /// The exception is when the Array constructor intentionally creates sparse arrays of a specified size by giving the constructor a single numeric argument.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Array();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Array(...args);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Array(500);
    /// ```
    ///
    /// ```js
    /// [0, 1, 2];
    /// ```
    ///
    pub UseArrayLiterals {
        version: "1.7.2",
        name: "useArrayLiterals",
        language: "js",
        sources: &[RuleSource::Eslint("no-array-constructor")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseArrayLiterals {
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
                validate(&callee, &arguments)
            }
            AnyJsExpression::JsNewExpression(new_expression) => {
                let callee = new_expression.callee().ok()?;
                let arguments = new_expression.arguments()?;
                validate(&callee, &arguments)
            }
            _ => None,
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
                "Use of the Array constructor is not allowed except creating sparse arrays of a specified size by giving a single numeric argument."
            })
            .note(markup! {
                "The array literal notation [] is preferable."
            }),
        )
    }
}

fn validate(callee: &AnyJsExpression, arguments: &JsCallArguments) -> Option<()> {
    if callee.text() != "Array" {
        return None;
    }
    let mut args_iter = arguments.args().into_iter();
    let first_arg = args_iter.next();
    let second_arg = args_iter.next();
    if first_arg.is_some()
        && second_arg.is_none()
        && !matches!(first_arg?.ok()?, AnyJsCallArgument::JsSpread(_))
    {
        return None;
    }
    Some(())
}

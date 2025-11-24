use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsReturnStatement};
use biome_rowan::AstNode;
use biome_rule_options::no_return_assign::NoReturnAssignOptions;

declare_lint_rule! {
    /// Disallow assignments in return statements.
    ///
    /// In return statements, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
    /// Moreover, the use of assignments in a return statement is confusing.
    /// Return statements are often considered side-effect free.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(a) {
    ///     return a = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(a) {
    ///     a = 1;
    ///     return a;
    /// }
    /// ```
    ///
    /// ```js
    /// function f(a) {
    ///     return a == 1;
    /// }
    /// ```
    ///
    pub NoReturnAssign {
        version: "next",
        name: "noReturnAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-return-assign").same()],
        recommended: false,
        severity: Severity::Error,
    }
}

impl Rule for NoReturnAssign {
    type Query = Ast<JsReturnStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReturnAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let except_parenthesis = ctx.options().except_parenthesis();

        traverse_expression(&query.argument()?, except_parenthesis)?.then_some(())
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
                "The "<Emphasis>"assignment"</Emphasis>" should not be in a "<Emphasis>"return statement"</Emphasis>"."
                },
            )
            .note(markup! {
            "The use of assignments in return statements is confusing.\nReturn statements are often considered side-effect free."
            }),
        )
    }
}

fn traverse_expression(root: &AnyJsExpression, except_parenthesis: bool) -> Option<bool> {
    let mut stack = vec![root.clone()];
    while let Some(current_node) = stack.pop() {
        match current_node {
            AnyJsExpression::JsAssignmentExpression(_) => return Some(true),
            AnyJsExpression::JsParenthesizedExpression(expression) => {
                if except_parenthesis {
                    continue;
                }

                if let Ok(expression) = expression.expression() {
                    stack.push(expression);
                }
            }
            AnyJsExpression::JsBinaryExpression(expression) => {
                if let Ok(left) = expression.left() {
                    stack.push(left);
                }
                if let Ok(right) = expression.right() {
                    stack.push(right);
                }
            }
            AnyJsExpression::JsSequenceExpression(expression) => {
                if let Ok(left) = expression.left() {
                    stack.push(left);
                }
                if let Ok(right) = expression.right() {
                    stack.push(right);
                }
            }
            _ => {}
        }
    }
    Some(false)
}

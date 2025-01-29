use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, JsNewExpression, JsNewExpressionFields,
};
use biome_rowan::{AstNode, AstSeparatedList};

declare_lint_rule! {
    /// Disallows using an async function as a Promise executor.
    ///
    /// The executor function can also be an async function. However, this is usually a mistake, for a few reasons:
    /// 1. If an async executor function throws an error, the error will be lost and won't cause the newly-constructed `Promise` to reject. This could make it difficult to debug and handle some errors.
    /// 2. If a Promise executor function is using `await`, this is usually a sign that it is not actually necessary to use the `new Promise` constructor, or the scope of the `new Promise` constructor can be reduced.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Promise(async function foo(resolve, reject) {})
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   new Promise(async (resolve, reject) => {})
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   new Promise(((((async () => {})))))
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    ///   new Promise((resolve, reject) => {})
    ///   new Promise((resolve, reject) => {}, async function unrelated() {})
    ///   new Foo(async (resolve, reject) => {})
    ///   new Foo((( (resolve, reject) => {} )))
    /// ```
    pub NoAsyncPromiseExecutor {
        version: "1.0.0",
        name: "noAsyncPromiseExecutor",
        language: "js",
        sources: &[RuleSource::Eslint("no-async-promise-executor")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoAsyncPromiseExecutor {
    type Query = Ast<JsNewExpression>;
    type State = AnyJsFunction;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let JsNewExpressionFields {
            new_token: _,
            callee,
            type_arguments: _,
            arguments,
        } = node.as_fields();
        let callee = callee.ok()?;
        let is_promise_constructor = callee
            .as_js_identifier_expression()
            .and_then(|ident| ident.name().ok())
            .is_some_and(|name| name.syntax().text_trimmed() == "Promise");
        if !is_promise_constructor {
            return None;
        }

        // get first argument of the `Promise` constructor
        let first_arg = arguments?.args().iter().next()?.ok()?;
        if let AnyJsCallArgument::AnyJsExpression(expr) = first_arg {
            get_async_function_expression_like(expr)
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "Promise executor functions should not be `async`."
            },
        ))
    }
}

/// Check if the expression is async function expression like, include the edge case
///  ```js
/// ((((((async function () {}))))))
/// ```
fn get_async_function_expression_like(expr: AnyJsExpression) -> Option<AnyJsFunction> {
    match expr.omit_parentheses() {
        AnyJsExpression::JsFunctionExpression(func) => func
            .async_token()
            .map(|_| AnyJsFunction::JsFunctionExpression(func)),
        AnyJsExpression::JsArrowFunctionExpression(func) => func
            .async_token()
            .map(|_| AnyJsFunction::JsArrowFunctionExpression(func)),
        _ => None,
    }
}

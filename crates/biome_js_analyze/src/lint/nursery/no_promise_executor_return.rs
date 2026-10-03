use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, JsClassDeclaration, JsFunctionBody,
    JsFunctionDeclaration, JsLanguage, JsNewExpression, JsReturnStatement, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_promise_executor_return::NoPromiseExecutorReturnOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow returning values from the function passed to `new Promise()`.
    ///
    /// When you create a Promise with `new Promise()`, you pass it a function called the _executor_.
    /// The executor usually starts an asynchronous task, such as reading a file.
    /// When the task finishes, the executor calls `resolve` with the result, or `reject` with an error.
    ///
    /// `new Promise()` ignores the value that the executor returns, so returning a value has no effect on the Promise.
    /// This is usually a mistake: the code most likely meant to call `resolve` or `reject` instead.
    ///
    /// A `return` statement without a value is allowed, because it only stops the executor early.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Promise((resolve, reject) => {
    ///     if (someCondition) {
    ///         return defaultResult;
    ///     }
    ///     getSomething((err, result) => {
    ///         if (err) {
    ///             reject(err);
    ///         } else {
    ///             resolve(result);
    ///         }
    ///     });
    /// });
    /// ```
    ///
    /// An arrow function without braces returns the value of its body:
    ///
    /// ```js,expect_diagnostic
    /// new Promise((resolve, reject) => getSomething((err, data) => {
    ///     if (err) {
    ///         reject(err);
    ///     } else {
    ///         resolve(data);
    ///     }
    /// }));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Promise(() => {
    ///     return 1;
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Promise(r => r(1));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// new Promise((resolve, reject) => {
    ///     if (someCondition) {
    ///         resolve(defaultResult);
    ///         return;
    ///     }
    ///     getSomething((err, result) => {
    ///         if (err) {
    ///             reject(err);
    ///         } else {
    ///             resolve(result);
    ///         }
    ///     });
    /// });
    ///
    /// new Promise(r => { r(1) });
    ///
    /// Promise.resolve(1);
    /// ```
    ///
    /// ## See Also
    ///
    /// - To disallow `async` functions as the executor, see [`noAsyncPromiseExecutor`](https://biomejs.dev/linter/rules/no-async-promise-executor/).
    ///
    pub NoPromiseExecutorReturn {
        version: "next",
        name: "noPromiseExecutorReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-promise-executor-return").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

pub enum ExecutorReturn {
    /// The expression body of an arrow function executor.
    ArrowBody(AnyJsExpression),
    /// A `return` statement with a value inside the executor.
    Return(JsReturnStatement),
}

impl Rule for NoPromiseExecutorReturn {
    type Query = Semantic<JsNewExpression>;
    type State = ExecutorReturn;
    type Signals = Box<[Self::State]>;
    type Options = NoPromiseExecutorReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(executor) = find_promise_executor(ctx.query(), ctx.model()) else {
            return Box::default();
        };
        match executor {
            AnyJsFunctionBody::AnyJsExpression(body) => {
                Box::new([ExecutorReturn::ArrowBody(body)])
            }
            AnyJsFunctionBody::JsFunctionBody(body) => collect_returns(&body),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            ExecutorReturn::ArrowBody(body) => RuleDiagnostic::new(
                rule_category!(),
                body.range(),
                markup! {
                    "The function passed to "<Emphasis>"new Promise()"</Emphasis>" returns this value."
                },
            ),
            ExecutorReturn::Return(statement) => RuleDiagnostic::new(
                rule_category!(),
                statement.range(),
                markup! {
                    "The function passed to "<Emphasis>"new Promise()"</Emphasis>" returns a value here."
                },
            ),
        };
        Some(
            diagnostic
                .note(markup! {
                    <Emphasis>"new Promise()"</Emphasis>" ignores this returned value, so it has no effect on the Promise."
                })
                .note(markup! {
                    "Call "<Emphasis>"resolve()"</Emphasis>" or "<Emphasis>"reject()"</Emphasis>" to complete the Promise. To stop the function early, use "<Emphasis>"return"</Emphasis>" without a value."
                }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let ExecutorReturn::ArrowBody(body) = state else {
            return None;
        };
        // `{ function () {} }` and `{ class {} }` are syntax errors.
        if matches!(
            body,
            AnyJsExpression::JsFunctionExpression(function) if function.id().is_none()
        ) || matches!(
            body,
            AnyJsExpression::JsClassExpression(class) if class.id().is_none()
        ) {
            return None;
        }
        // The trivia of the body is transferred to the braces by `replace_node`.
        let statement = make::js_expression_statement(
            body.clone()
                .with_leading_trivia_pieces([])?
                .with_trailing_trivia_pieces([])?,
        )
        .build();
        let block = make::js_function_body(
            make::token(T!['{']),
            make::js_directive_list([]),
            make::js_statement_list([statement.into()]),
            make::token(T!['}']),
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsFunctionBody::AnyJsExpression(body.clone()),
            AnyJsFunctionBody::JsFunctionBody(block),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the expression in braces." }.to_owned(),
            mutation,
        ))
    }
}

/// Kinds of nodes that can't contain a `return` statement of the executor itself:
/// any `return` statement inside an expression or a function or class declaration
/// belongs to a nested function.
const NESTED_FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyJsExpression::KIND_SET
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsClassDeclaration::KIND_SET);

/// Returns the body of the executor if `new_expression` is `new Promise(executor)`
/// and `Promise` refers to the global.
fn find_promise_executor(
    new_expression: &JsNewExpression,
    model: &SemanticModel,
) -> Option<AnyJsFunctionBody> {
    let callee = new_expression.callee().ok()?.omit_parentheses();
    let reference = callee.as_js_identifier_expression()?.name().ok()?;
    if reference.value_token().ok()?.text_trimmed() != "Promise"
        || model.binding(&reference).is_some()
    {
        return None;
    }
    let first_argument = new_expression.arguments()?.args().iter().next()?.ok()?;
    match first_argument.as_any_js_expression()?.clone().omit_parentheses() {
        AnyJsExpression::JsFunctionExpression(function) => {
            Some(AnyJsFunctionBody::JsFunctionBody(function.body().ok()?))
        }
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok(),
        _ => None,
    }
}

/// Collects the `return` statements with a value that belong to the executor,
/// ignoring the ones inside nested functions and classes.
fn collect_returns(body: &JsFunctionBody) -> Box<[ExecutorReturn]> {
    let mut returns = Vec::new();
    let mut preorder = body.syntax().preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(statement) = JsReturnStatement::cast_ref(&node) {
            if statement.argument().is_some() {
                returns.push(ExecutorReturn::Return(statement));
            }
            preorder.skip_subtree();
        } else if NESTED_FUNCTION_KINDS.matches(node.kind()) {
            preorder.skip_subtree();
        }
    }
    returns.into_boxed_slice()
}

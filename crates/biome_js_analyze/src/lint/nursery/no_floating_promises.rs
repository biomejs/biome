use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsExpression, JsArrowFunctionExpression,
    JsCallExpression, JsExpressionStatement, JsFunctionDeclaration, JsMethodClassMember,
    JsMethodObjectMember, JsStaticMemberExpression, JsSyntaxKind, JsVariableDeclarator,
    TsReturnTypeAnnotation,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind};

use crate::{services::semantic::Semantic, JsRuleAction};

declare_lint_rule! {
    /// Require Promise-like statements to be handled appropriately.
    ///
    /// A "floating" `Promise` is one that is created without any code set up to handle any errors it might throw.
    /// Floating Promises can lead to several issues, including improperly sequenced operations, unhandled Promise rejections, and other unintended consequences.
    ///
    /// This rule will report Promise-valued statements that are not treated in one of the following ways:
    /// - Calling its `.then()` method with two arguments
    /// - Calling its `.catch()` method with one argument
    /// - `await`ing it
    /// - `return`ing it
    /// - `void`ing it
    ///
    /// :::caution
    /// ## Important notes
    ///
    /// This rule is a work in progress, and is only partially implemented.
    /// Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/3187
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// async function returnsPromise(): Promise<string> {
    ///   return 'value';
    /// }
    /// returnsPromise().then(() => {});
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const returnsPromise = async (): Promise<string> => {
    ///   return 'value';
    /// }
    /// async function returnsPromiseInAsyncFunction() {
    ///   returnsPromise().then(() => {});
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// async function returnsPromise(): Promise<string> {
    ///   return 'value';
    /// }
    ///
    /// await returnsPromise();
    ///
    /// void returnsPromise();
    ///
    /// // Calling .then() with two arguments
    /// returnsPromise().then(
    ///   () => {},
    ///   () => {},
    /// );
    ///
    /// // Calling .catch() with one argument
    /// returnsPromise().catch(() => {});
    /// ```
    ///
    pub NoFloatingPromises {
        version: "next",
        name: "noFloatingPromises",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-floating-promises")],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoFloatingPromises {
    type Query = Semantic<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let expression = node.expression().ok()?;
        if let AnyJsExpression::JsCallExpression(js_call_expression) = expression {
            let Ok(any_js_expression) = js_call_expression.callee() else {
                return None;
            };

            if !is_callee_a_promise(&any_js_expression, model)? {
                return None;
            }

            if is_handled_promise(&js_call_expression).unwrap_or(false) {
                return None;
            }

            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "A \"floating\" Promise was found, meaning it is not properly handled and could lead to ignored errors or unexpected behavior."
                },
            )
            .note(markup! {
                "This happens when a Promise is not awaited, lacks a `.catch` or `.then` rejection handler, or is not explicitly ignored using the `void` operator."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if !is_in_async_function(node) {
            return None;
        }

        let expression = node.expression().ok()?;
        let mut mutation = ctx.root().begin();
        let await_expression = AnyJsExpression::JsAwaitExpression(make::js_await_expression(
            make::token(JsSyntaxKind::AWAIT_KW)
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            expression.clone().trim_leading_trivia()?,
        ));

        mutation.replace_node(expression, await_expression);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add await operator." }.to_owned(),
            mutation,
        ))
    }
}

/// Checks if the callee of a JavaScript expression is a promise.
///
/// This function inspects the callee of a given JavaScript expression to determine
/// if it is a promise. It returns `true` if the callee is a promise, otherwise `false`.
///
/// The function works by finding the binding of the callee and checking if it is a promise.
///
/// # Arguments
///
/// * `callee` - A reference to an `AnyJsExpression` representing the callee to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `true` if the callee is a promise.
/// * `false` otherwise.
///
/// # Examples
///
/// Example JavaScript code that would return `true`:
/// ```typescript
/// async function returnsPromise(): Promise<string> {
///     return "value";
/// }
///
/// returnsPromise().then(() => {});
/// ```
///
/// Example JavaScript code that would return `false`:
/// ```typescript
/// function doesNotReturnPromise() {
///     return 42;
/// }
///
/// doesNotReturnPromise().then(() => {});
/// ```
fn is_callee_a_promise(callee: &AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    match callee {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let reference = ident_expr.name().ok()?;
            let binding = model.binding(&reference)?;
            let any_js_binding_decl = binding.tree().declaration()?;

            match any_js_binding_decl {
                AnyJsBindingDeclaration::JsFunctionDeclaration(func_decl) => {
                    Some(is_function_a_promise(&func_decl))
                }
                AnyJsBindingDeclaration::JsVariableDeclarator(js_var_decl) => Some(
                    is_variable_initializer_a_promise(&js_var_decl).unwrap_or(false)
                        || is_variable_annotation_a_promise(&js_var_decl).unwrap_or(false),
                ),
                _ => Some(false),
            }
        }
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            is_member_expression_callee_a_promise(static_member_expr, model)
        }
        _ => Some(false),
    }
}

fn is_function_a_promise(func_decl: &JsFunctionDeclaration) -> bool {
    func_decl.async_token().is_some()
        || is_return_type_a_promise(func_decl.return_type_annotation()).unwrap_or(false)
}

/// Checks if a TypeScript return type annotation is a `Promise`.
///
/// This function inspects the return type annotation of a TypeScript function to determine
/// if it is a `Promise`. It returns `Some(true)` if the return type annotation is `Promise`,
/// `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `return_type` - An optional `TsReturnTypeAnnotation` to check.
///
/// # Returns
///
/// * `Some(true)` if the return type annotation is `Promise`.
/// * `Some(false)` if the return type annotation is not `Promise`.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
/// ```typescript
/// async function returnsPromise(): Promise<void> {}
/// ```
///
/// Example TypeScript code that would return `false`:
/// ```typescript
/// function doesNotReturnPromise(): void {}
/// ```
fn is_return_type_a_promise(return_type: Option<TsReturnTypeAnnotation>) -> Option<bool> {
    let ts_return_type_anno = return_type?.ty().ok()?;
    let any_ts_type = ts_return_type_anno.as_any_ts_type()?;
    let reference_type = any_ts_type.as_ts_reference_type()?;
    let any_ts_name = reference_type.name().ok()?;
    let name = any_ts_name.as_js_reference_identifier()?;

    Some(name.has_name("Promise"))
}

/// Checks if a `JsCallExpression` is a handled Promise-like expression.
/// - Calling its .then() with two arguments
/// - Calling its .catch() with one argument
///
/// This function inspects a `JsCallExpression` to determine if it is a handled Promise-like expression.
/// It returns `Some(true)` if the expression is handled, `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `js_call_expression` - A reference to a `JsCallExpression` to check.
///
/// # Returns
///
/// * `Some(true)` if the expression is a handled Promise-like expression.
/// * `Some(false)` if the expression is not a handled Promise-like expression.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
/// ```typescript
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa", () => null).finally(() => null)
///
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa").catch(() => null).finally(() => null)
/// ```
fn is_handled_promise(js_call_expression: &JsCallExpression) -> Option<bool> {
    let expr = js_call_expression.callee().ok()?;
    let static_member_expr = expr.as_js_static_member_expression()?;
    let member = static_member_expr.member().ok()?;
    let js_name = member.as_js_name()?;
    let name = js_name.to_string();

    if name == "finally" {
        let expr = static_member_expr.object().ok()?;
        let callee = expr.as_js_call_expression()?;
        return is_handled_promise(callee);
    }
    if name == "catch" {
        let call_args = js_call_expression.arguments().ok()?;
        // just checking if there are any arguments, not if it's a function for simplicity
        return Some(call_args.args().len() > 0);
    }
    if name == "then" {
        let call_args = js_call_expression.arguments().ok()?;
        // just checking arguments have a reject function from length
        return Some(call_args.args().len() >= 2);
    }

    Some(false)
}

/// Checks if the callee of a `JsStaticMemberExpression` is a promise expression.
///
/// This function inspects the callee of a `JsStaticMemberExpression` to determine
/// if it is a promise expression. It returns `Some(true)` if the callee is a promise expression,
/// `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `static_member_expr` - A reference to a `JsStaticMemberExpression` to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `Some(true)` if the callee is a promise expression.
/// * `Some(false)` if the callee is not a promise expression.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `true`:
/// ```typescript
/// async function returnsPromise(): Promise<void> {}
///
/// returnsPromise().then(() => null).catch(() => {});
/// ```
///
/// Example TypeScript code that would return `false`:
/// ```typescript
/// function doesNotReturnPromise(): void {}
///
/// doesNotReturnPromise().then(() => null).catch(() => {});
/// ```
fn is_member_expression_callee_a_promise(
    static_member_expr: &JsStaticMemberExpression,
    model: &SemanticModel,
) -> Option<bool> {
    let expr = static_member_expr.object().ok()?;
    let js_call_expr = expr.as_js_call_expression()?;
    let callee = js_call_expr.callee().ok()?;

    is_callee_a_promise(&callee, model)
}

/// Checks if the given `JsExpressionStatement` is within an async function.
///
/// This function traverses up the syntax tree from the given expression node
/// to find the nearest function and checks if it is an async function. It
/// supports arrow functions, function declarations, class methods, and object
/// methods.
///
/// # Arguments
///
/// * `node` - A reference to a `JsExpressionStatement` to check.
///
/// # Returns
///
/// * `true` if the expression is within an async function.
/// * `false` otherwise.
fn is_in_async_function(node: &JsExpressionStatement) -> bool {
    node.syntax()
        .ancestors()
        .find_map(|ancestor| match ancestor.kind() {
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => ancestor
                .cast::<JsArrowFunctionExpression>()
                .and_then(|func| func.async_token()),
            JsSyntaxKind::JS_FUNCTION_DECLARATION => ancestor
                .cast::<JsFunctionDeclaration>()
                .and_then(|func| func.async_token()),
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER => ancestor
                .cast::<JsMethodClassMember>()
                .and_then(|method| method.async_token()),
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => ancestor
                .cast::<JsMethodObjectMember>()
                .and_then(|method| method.async_token()),
            _ => None,
        })
        .is_some()
}

/// Checks if the initializer of a `JsVariableDeclarator` is an async function or returns a promise.
///
/// This function inspects the initializer of a given `JsVariableDeclarator` to determine
/// if it is an async function or returns a promise. It returns `Some(true)` if the initializer
/// is an async function or returns a promise, `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `js_variable_declarator` - A reference to a `JsVariableDeclarator` to check.
///
/// # Returns
///
/// * `Some(true)` if the initializer is an async function or returns a promise.
/// * `Some(false)` if the initializer is not an async function and does not return a promise.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
///
/// ```typescript
/// const returnsPromise = async (): Promise<string> => {
///   return 'value';
/// }
///
/// const returnsPromise = async function (): Promise<string> {
///   return 'value'
/// }
/// ```
fn is_variable_initializer_a_promise(
    js_variable_declarator: &JsVariableDeclarator,
) -> Option<bool> {
    let initializer_clause = &js_variable_declarator.initializer()?;
    let expr = initializer_clause.expression().ok()?;
    match expr {
        AnyJsExpression::JsArrowFunctionExpression(arrow_func) => Some(
            arrow_func.async_token().is_some()
                || is_return_type_a_promise(arrow_func.return_type_annotation()).unwrap_or(false),
        ),
        AnyJsExpression::JsFunctionExpression(func_expr) => Some(
            func_expr.async_token().is_some()
                || is_return_type_a_promise(func_expr.return_type_annotation()).unwrap_or(false),
        ),
        _ => Some(false),
    }
}

/// Checks if a `JsVariableDeclarator` has a TypeScript type annotation of `Promise`.
///
/// This function inspects the type annotation of a given `JsVariableDeclarator` to determine
/// if it is a `Promise`. It returns `Some(true)` if the type annotation is `Promise`,
/// `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `js_variable_declarator` - A reference to a `JsVariableDeclarator` to check.
///
/// # Returns
///
/// * `Some(true)` if the type annotation is `Promise`.
/// * `Some(false)` if the type annotation is not `Promise`.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
/// ```typescript
/// const returnsPromise: () => Promise<string> = () => {
///   return Promise.resolve("value")
/// }
/// ```
fn is_variable_annotation_a_promise(js_variable_declarator: &JsVariableDeclarator) -> Option<bool> {
    let any_ts_var_anno = js_variable_declarator.variable_annotation()?;
    let ts_type_anno = any_ts_var_anno.as_ts_type_annotation()?;
    let any_ts_type = ts_type_anno.ty().ok()?;
    let func_type = any_ts_type.as_ts_function_type()?;
    let return_type = func_type.return_type().ok()?;
    let ref_type = return_type.as_any_ts_type()?.as_ts_reference_type()?;
    let name = ref_type.name().ok()?;
    let identifier = name.as_js_reference_identifier()?;

    Some(identifier.has_name("Promise"))
}

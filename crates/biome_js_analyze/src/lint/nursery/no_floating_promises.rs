use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsExpression, AnyJsName, AnyTsReturnType, AnyTsType,
    JsCallExpression, JsExpressionStatement, JsFileSource, JsFunctionDeclaration,
    JsStaticMemberExpression, JsSyntaxKind, TsReturnTypeAnnotation,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

use crate::{services::semantic::Semantic, JsRuleAction};

declare_lint_rule! {
    /// Require Promise-like statements to be handled appropriately.
    ///
    /// "floating" Promise is one that is created without any code set up to handle any errors it might throw.
    /// Floating Promises can lead to several issues, including improperly sequenced operations, unhandled Promise rejections, and other unintended consequences.
    ///
    /// This rule will report Promise-valued statements that are not treated in one of the following ways:
    /// - Calling its `.then()` with two arguments
    /// - Calling its `.catch()` with one argument
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
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoFloatingPromises {
    type Query = Semantic<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        if !source_type.is_typescript() || source_type.is_definition_file() {
            return None;
        }

        let node = ctx.query();
        let model = ctx.model();
        let expression = node.expression().ok()?;
        if let AnyJsExpression::JsCallExpression(js_call_expression) = expression {
            let Ok(any_js_expression) = js_call_expression.callee() else {
                return None;
            };

            if !is_callee_a_promise(&any_js_expression, model) {
                return None;
            }

            if is_handled_promise(&js_call_expression) {
                return None;
            }

            return Some(());
        }
        None
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
fn is_callee_a_promise(callee: &AnyJsExpression, model: &SemanticModel) -> bool {
    match callee {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let Some(reference) = ident_expr.name().ok() else {
                return false;
            };

            let Some(binding) = model.binding(&reference) else {
                return false;
            };

            let Some(any_js_binding_decl) = binding.tree().declaration() else {
                return false;
            };

            let AnyJsBindingDeclaration::JsFunctionDeclaration(func_decl) = any_js_binding_decl
            else {
                return false;
            };

            return is_function_a_promise(&func_decl);
        }
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            return is_member_expression_callee_a_promise(static_member_expr, model);
        }
        _ => {}
    }
    false
}

fn is_function_a_promise(func_decl: &JsFunctionDeclaration) -> bool {
    func_decl.async_token().is_some() || is_return_type_promise(func_decl.return_type_annotation())
}

/// Checks if a TypeScript return type annotation is a `Promise`.
///
/// This function inspects the return type annotation of a TypeScript function to determine
/// if it is a `Promise`. It returns `true` if the return type annotation is `Promise`, otherwise `false`.
///
/// # Arguments
///
/// * `return_type` - An optional `TsReturnTypeAnnotation` to check.
///
/// # Returns
///
/// * `true` if the return type annotation is `Promise`.
/// * `false` otherwise.
///
/// # Examples
///
/// Example TypeScript code that would return `true`:
/// ```typescript
/// async function returnsPromise(): Promise<void> {}
/// ```
///
/// Example TypeScript code that would return `false`:
/// ```typescript
/// function doesNotReturnPromise(): void {}
/// ```
fn is_return_type_promise(return_type: Option<TsReturnTypeAnnotation>) -> bool {
    return_type
        .and_then(|ts_return_type_anno| ts_return_type_anno.ty().ok())
        .and_then(|any_ts_return_type| match any_ts_return_type {
            AnyTsReturnType::AnyTsType(any_ts_type) => Some(any_ts_type),
            _ => None,
        })
        .and_then(|any_ts_type| match any_ts_type {
            AnyTsType::TsReferenceType(reference_type) => Some(reference_type),
            _ => None,
        })
        .and_then(|reference_type| reference_type.name().ok())
        .map_or(false, |name| name.text() == "Promise")
}

/// Checks if a `JsCallExpression` is a handled Promise-like expression.
/// - Calling its .then() with two arguments
/// - Calling its .catch() with one argument
///
/// Example TypeScript code that would return `true`:
///
/// ```typescript
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa", () => null).finally(() => null)
///
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa").catch(() => null).finally(() => null)
/// ```
fn is_handled_promise(js_call_expression: &JsCallExpression) -> bool {
    let Ok(expr) = js_call_expression.callee() else {
        return false;
    };

    let AnyJsExpression::JsStaticMemberExpression(static_member_expr) = expr else {
        return false;
    };

    let Ok(AnyJsName::JsName(name)) = static_member_expr.member() else {
        return false;
    };

    let name = name.text();

    if name == "finally" {
        if let Ok(expr) = static_member_expr.object() {
            if let Some(callee) = expr.as_js_call_expression() {
                return is_handled_promise(callee);
            }
        }
    }
    if name == "catch" {
        if let Ok(call_args) = js_call_expression.arguments() {
            // just checking if there are any arguments, not if it's a function for simplicity
            if call_args.args().len() > 0 {
                return true;
            }
        }
    }
    if name == "then" {
        if let Ok(call_args) = js_call_expression.arguments() {
            // just checking arguments have a reject function from length
            if call_args.args().len() >= 2 {
                return true;
            }
        }
    }
    false
}

/// Checks if the callee of a `JsStaticMemberExpression` is a promise expression.
///
/// This function inspects the callee of a `JsStaticMemberExpression` to determine
/// if it is a promise expression. It returns `true` if the callee is a promise expression,
/// otherwise `false`.
///
/// # Arguments
///
/// * `static_member_expr` - A reference to a `JsStaticMemberExpression` to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `true` if the callee is a promise expression.
/// * `false` otherwise.
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
) -> bool {
    let Ok(expr) = static_member_expr.object() else {
        return false;
    };

    let AnyJsExpression::JsCallExpression(js_call_expr) = expr else {
        return false;
    };

    let Ok(callee) = js_call_expr.callee() else {
        return false;
    };

    match callee {
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            return is_member_expression_callee_a_promise(&static_member_expr, model);
        }
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let Some(reference) = ident_expr.name().ok() else {
                return false;
            };
            let Some(binding) = model.binding(&reference) else {
                return false;
            };

            let Some(any_js_binding_decl) = binding.tree().declaration() else {
                return false;
            };

            let AnyJsBindingDeclaration::JsFunctionDeclaration(func_decl) = any_js_binding_decl
            else {
                return false;
            };
            return is_function_a_promise(&func_decl);
        }
        _ => {}
    }

    false
}

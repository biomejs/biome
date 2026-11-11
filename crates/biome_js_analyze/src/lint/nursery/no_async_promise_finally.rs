use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsMemberExpression,
    JsCallExpression, JsIdentifierExpression, JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_async_promise_finally::NoAsyncPromiseFinallyOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow async functions as `.finally()` callbacks.
    ///
    /// The function passed to a promise's `.finally()` method runs after the promise settles,
    /// whether it succeeded or failed. When this function is async, it returns a promise of its own.
    /// If that promise is rejected, the promise returned by `.finally()` is rejected with that new error,
    /// and the original value or error is lost.
    ///
    /// This rule reports async functions written directly inside `.finally()`,
    /// and names that refer to an async function declared in the same file,
    /// either with a `function` declaration or a `const` variable.
    /// Imported functions and functions accessed through an object are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following code logs `cleanup` instead of `original`:
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     await Promise.reject(new Error("original")).finally(async () => {
    ///         throw new Error("cleanup");
    ///     });
    /// } catch (error) {
    ///     console.log(error.message); // "cleanup"
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// promise.finally(async () => {
    ///     await cleanup();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// async function cleanup() {}
    ///
    /// promise.finally(cleanup);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const cleanup = async () => {};
    ///
    /// promise.finally(cleanup);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// promise.finally(() => {
    ///     cleanupSynchronously();
    /// });
    ///
    /// function cleanup() {}
    /// promise.finally(cleanup);
    ///
    /// // If the cleanup has to be asynchronous, handle its errors so they can't replace the original result.
    /// promise.finally(() => {
    ///     return asyncCleanup().catch((error) => {
    ///         logCleanupError(error);
    ///     });
    /// });
    /// ```
    ///
    pub NoAsyncPromiseFinally {
        version: "2.6.7",
        name: "noAsyncPromiseFinally",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-async-promise-finally").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoAsyncPromiseFinally {
    type Query = Semantic<JsCallExpression>;
    /// The async function passed to `.finally()`. It is outside the call when the argument is a
    /// name that refers to it.
    type State = AnyJsFunction;
    type Signals = Option<Self::State>;
    type Options = NoAsyncPromiseFinallyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        if !is_finally_member(ctx.model(), &member) {
            return None;
        }

        let callback = finally_callback(call)?.omit_type_wrappers()?;
        let function = match callback {
            AnyJsExpression::JsArrowFunctionExpression(function) => function.into(),
            AnyJsExpression::JsFunctionExpression(function) => function.into(),
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                referenced_function(ctx.model(), &identifier)?
            }
            _ => return None,
        };
        (function.is_async() && !function.is_generator()).then_some(function)
    }

    fn diagnostic(ctx: &RuleContext<Self>, function: &Self::State) -> Option<RuleDiagnostic> {
        let callback = finally_callback(ctx.query())?;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            callback.range(),
            markup! {
                "This "<Emphasis>"async"</Emphasis>" function is passed to "<Emphasis>".finally()"</Emphasis>"."
            },
        );
        let diagnostic = if callback.range().contains_range(function.range()) {
            diagnostic
        } else {
            diagnostic.detail(
                function.async_token()?.text_trimmed_range(),
                markup! { "The function is declared "<Emphasis>"async"</Emphasis>" here." },
            )
        };
        Some(
            diagnostic
                .note(markup! {
                    "If this function fails, the promise returned by "<Emphasis>".finally()"</Emphasis>" is rejected with its error, and the original result or error of the promise is lost."
                })
                .note(markup! {
                    "Use a function that isn't "<Emphasis>"async"</Emphasis>". If the cleanup has to be asynchronous, handle its errors inside the function, for example with "<Emphasis>".catch()"</Emphasis>"."
                }),
        )
    }
}

/// Returns the first argument of the call, unless it is a spread argument.
fn finally_callback(call: &JsCallExpression) -> Option<AnyJsExpression> {
    call.arguments()
        .ok()?
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()
        .cloned()
}

/// Returns `true` if `member` reads a property named `finally`, either by name or through a
/// `const` variable that holds the string `"finally"`, as in `promise[method]`.
fn is_finally_member(model: &SemanticModel, member: &AnyJsMemberExpression) -> bool {
    if let Some(name) = member.member_name() {
        return name.text() == "finally";
    }
    let AnyJsMemberExpression::JsComputedMemberExpression(member) = member else {
        return false;
    };
    member
        .member()
        .ok()
        .and_then(AnyJsExpression::omit_type_wrappers)
        .and_then(|member| declaration(model, member.as_js_identifier_expression()?))
        .and_then(|declaration| match declaration {
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                const_initializer(&declarator)
            }
            _ => None,
        })
        .and_then(|initializer| initializer.as_static_value())
        .is_some_and(|value| value.text() == "finally")
}

/// Returns the function that `identifier` refers to, if it is declared in this file as a function
/// or as a `const` variable initialized with a function expression.
fn referenced_function(
    model: &SemanticModel,
    identifier: &JsIdentifierExpression,
) -> Option<AnyJsFunction> {
    match declaration(model, identifier)? {
        AnyJsBindingDeclaration::JsFunctionDeclaration(function) => Some(function.into()),
        AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function) => {
            Some(function.into())
        }
        AnyJsBindingDeclaration::JsFunctionExpression(function) => Some(function.into()),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            match const_initializer(&declarator)? {
                AnyJsExpression::JsArrowFunctionExpression(function) => Some(function.into()),
                AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns the declaration of the variable or function that `identifier` refers to.
fn declaration(
    model: &SemanticModel,
    identifier: &JsIdentifierExpression,
) -> Option<AnyJsBindingDeclaration> {
    model
        .binding(&identifier.name().ok()?)?
        .tree()
        .declaration()
}

/// Returns the initial value of `declarator` if it declares a `const` variable, without
/// parentheses or TypeScript type syntax around it.
fn const_initializer(declarator: &JsVariableDeclarator) -> Option<AnyJsExpression> {
    if !declarator.declaration()?.is_const() {
        return None;
    }
    declarator
        .initializer()?
        .expression()
        .ok()?
        .omit_type_wrappers()
}

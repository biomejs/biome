use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, global_identifier, AnyJsClassMember, AnyJsExpression,
    AnyTsType, JsArrowFunctionExpression, JsCallExpression, JsClassDeclaration, JsClassExpression,
    JsClassMemberList, JsExpressionStatement, JsExtendsClause, JsFunctionDeclaration,
    JsIdentifierExpression, JsInitializerClause, JsMethodClassMember, JsMethodObjectMember,
    JsStaticMemberExpression, JsSyntaxKind, JsThisExpression, JsVariableDeclarator,
    TsReturnTypeAnnotation,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind,
};

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
    /// ```ts,expect_diagnostic
    /// const promise = new Promise((resolve) => resolve('value'));
    /// promise.then(() => { }).finally(() => { });
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// Promise.all([p1, p2, p3])
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    ///   async someMethod() {
    ///     this.returnsPromise();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Parent {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    /// }
    ///
    /// class Child extends Parent {
    ///   async someMethod() {
    ///     this.returnsPromise();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    /// }
    /// const api = new Api();
    /// api.returnsPromise().then(() => {}).finally(() => {});
    /// ```
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
    ///
    /// await Promise.all([p1, p2, p3])
    ///
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    ///   async someMethod() {
    ///     await this.returnsPromise();
    ///   }
    /// }
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
        match expression.omit_parentheses() {
            AnyJsExpression::JsCallExpression(js_call_expression) => {
                let any_js_expression = js_call_expression.callee().ok()?;

                if !is_callee_a_promise(&any_js_expression, model)? {
                    return None;
                }

                if is_handled_promise(&js_call_expression).unwrap_or_default() {
                    return None;
                }

                Some(())
            }
            AnyJsExpression::JsIdentifierExpression(js_identifier_expression) => {
                if !is_binding_a_promise(&js_identifier_expression, model, None)? {
                    return None;
                }

                Some(())
            }
            AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
                if !is_member_expression_callee_a_promise(&static_member_expr, model)? {
                    return None;
                }
                Some(())
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
        AnyJsExpression::JsIdentifierExpression(js_ident_expr) => {
            is_binding_a_promise(js_ident_expr, model, None)
        }
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            is_member_expression_callee_a_promise(static_member_expr, model)
        }
        _ => Some(false),
    }
}

/// Checks if a binding is a promise.
///
/// This function inspects the binding of a given `JsIdentifierExpression` to determine
/// if it is a promise. It returns `Some(true)` if the binding is a promise, `Some(false)` if it is not,
/// and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `js_ident_expr` - A reference to a `JsIdentifierExpression` representing the identifier to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
/// * `target_method_name` - An optional name of the method to check if it is a promise.
///
/// # Returns
///
/// * `Some(true)` if the binding is a promise.
/// * `Some(false)` if the binding is not a promise.
/// * `None` if there is an error in the process.
///
fn is_binding_a_promise(
    js_ident_expr: &JsIdentifierExpression,
    model: &SemanticModel,
    target_method_name: Option<&str>,
) -> Option<bool> {
    let reference = js_ident_expr.name().ok()?;
    let binding = model.binding(&reference)?;
    let any_js_binding_decl = binding.tree().declaration()?;

    match any_js_binding_decl {
        AnyJsBindingDeclaration::JsFunctionDeclaration(func_decl) => {
            Some(is_function_a_promise(&func_decl))
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(js_var_decl) => Some(
            is_initializer_a_promise(&js_var_decl.initializer()?, model, target_method_name)
                .unwrap_or_default()
                || is_variable_annotation_a_promise(&js_var_decl).unwrap_or_default(),
        ),
        _ => Some(false),
    }
}

fn is_function_a_promise(func_decl: &JsFunctionDeclaration) -> bool {
    func_decl.async_token().is_some()
        || is_return_type_a_promise(func_decl.return_type_annotation()).unwrap_or_default()
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
    let value_token = js_name.value_token().ok()?;
    let name = value_token.text_trimmed();

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
///
/// globalThis.Promise.reject('value').finally();
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

    if is_expression_a_promise(&expr, model) {
        return Some(true);
    }

    match expr {
        AnyJsExpression::JsCallExpression(js_call_expr) => {
            let callee = js_call_expr.callee().ok()?;
            is_callee_a_promise(&callee, model)
        }
        AnyJsExpression::JsIdentifierExpression(js_ident_expr) => {
            let value_token = static_member_expr
                .member()
                .ok()
                .and_then(|js_name| js_name.value_token().ok());

            if let Some(token) = value_token {
                return is_binding_a_promise(&js_ident_expr, model, Some(token.text_trimmed()));
            }
            is_binding_a_promise(&js_ident_expr, model, None)
        }
        AnyJsExpression::JsThisExpression(js_this_expr) => {
            let js_name = static_member_expr.member().ok()?;
            let value_token = js_name.value_token().ok()?;
            check_this_expression(&js_this_expr, value_token.text_trimmed(), model)
        }
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            is_member_expression_callee_a_promise(&static_member_expr, model)
        }
        _ => Some(false),
    }
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

/// Checks if the initializer is an async function or returns a promise.
///
/// This function inspects the initializer of a given `JsVariableDeclarator` to determine
/// if it is an async function or returns a promise. It returns `Some(true)` if the initializer
/// is an async function or returns a promise, `Some(false)` if it is not, and `None` if there is an error in the process.
///
/// # Arguments
///
/// * `js_variable_declarator` - A reference to a `JsVariableDeclarator` to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
/// * `target_method_name` - An optional name of the method to check if it is a promise.
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
///
/// const promise = new Promise((resolve) => resolve('value'));
///
/// const promiseWithGlobalIdentifier = new window.Promise((resolve, reject) => resolve('value'));
/// ```
fn is_initializer_a_promise(
    initializer_clause: &JsInitializerClause,
    model: &SemanticModel,
    target_method_name: Option<&str>,
) -> Option<bool> {
    let expr = initializer_clause.expression().ok()?;
    match expr.omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(arrow_func) => Some(
            arrow_func.async_token().is_some()
                || is_return_type_a_promise(arrow_func.return_type_annotation())
                    .unwrap_or_default(),
        ),
        AnyJsExpression::JsFunctionExpression(func_expr) => Some(
            func_expr.async_token().is_some()
                || is_return_type_a_promise(func_expr.return_type_annotation()).unwrap_or_default(),
        ),
        AnyJsExpression::JsNewExpression(js_new_epr) => {
            let any_js_expr = js_new_epr.callee().ok()?;
            if is_expression_a_promise(&any_js_expr, model) {
                return Some(true);
            }
            let ident_expr = any_js_expr.as_js_identifier_expression()?;
            let reference = ident_expr.name().ok()?;
            let binding = model.binding(&reference)?;
            let any_js_binding_decl = binding.tree().declaration()?;
            match any_js_binding_decl {
                AnyJsBindingDeclaration::JsClassDeclaration(class_decl) => {
                    find_and_check_class_member(&class_decl.members(), target_method_name?, model)
                }
                AnyJsBindingDeclaration::JsVariableDeclarator(js_var_decl) => {
                    let initializer = js_var_decl.initializer()?;
                    is_initializer_a_promise(&initializer, model, target_method_name)
                }
                _ => None,
            }
        }
        AnyJsExpression::JsClassExpression(class_expr) => {
            find_and_check_class_member(&class_expr.members(), target_method_name?, model)
        }
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
///
/// const promise: Promise<string> = new Promise((resolve) => resolve('value'));
/// ```
fn is_variable_annotation_a_promise(js_variable_declarator: &JsVariableDeclarator) -> Option<bool> {
    let any_ts_var_anno = js_variable_declarator.variable_annotation()?;
    let ts_type_anno = any_ts_var_anno.as_ts_type_annotation()?;
    let any_ts_type = ts_type_anno.ty().ok()?;
    is_ts_type_a_promise(&any_ts_type)
}

/// Checks if an expression is a `Promise`.
///
/// This function inspects a given `AnyJsExpression` to determine if it represents a `Promise`,
/// either as a global identifier (e.g., `window.Promise`) or directly (e.g., `Promise.resolve`).
/// It also checks that the found `Promise` is not a binding.
/// It returns `true` if the expression is a `Promise` and is not a binding, otherwise `false`.
///
/// # Arguments
///
/// * `expr` - A reference to an `AnyJsExpression` to check.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `true` if the expression is a `Promise` and is not a binding.
/// * `false` otherwise.
///
/// # Examples
///
/// Example TypeScript code that would return `true`:
/// ```typescript
/// window.Promise.resolve();
/// globalThis.Promise.resolve();
/// Promise.resolve('value').then(() => { });
/// Promise.all([p1, p2, p3]);
/// ```
///
/// Example TypeScript code that would return `false`:
/// ```typescript
/// const Promise = { resolve(): {} };
/// Promise.resolve()
/// ```
fn is_expression_a_promise(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    let (reference, value) = match global_identifier(expr) {
        Some(result) => result,
        None => return false,
    };

    if value.text() != "Promise" {
        return false;
    }

    if model.binding(&reference).is_some() {
        return false;
    }

    true
}

/// Traverses up the syntax tree to find the class declaration and checks if a method is a promise.
///
/// This function traverses up the syntax tree from the given `JsThisExpression` to find the nearest
/// class declaration. It then searches for a method or property in the class that matches the provided
/// `target_name`. If a matching member is found, it checks if the member is a promise.
///
/// # Arguments
///
/// * `js_this_expression` - A `JsThisExpression` representing the `this` keyword in the syntax tree.
/// * `target_name` - The name of the method or property to search for.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `Some(true)` if the class member is a promise.
/// * `Some(false)` if the class member is not a promise.
/// * `None` if there is an error in the process or if the class member is not found.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
/// ```typescript
/// class Api {
///   async returnsPromise(): Promise<string> {
///     return 'value';
///   }
///   async someMethod() {
///     this.returnsPromise();
///   }
/// }
/// ```
///
/// Example TypeScript code that would return `Some(false)`:
/// ```typescript
/// class Api {
///   returnsString(){
///     return 'value';
///   }
///   async someMethod() {
///     this.returnsString();
///   }
/// }
/// ```
fn check_this_expression(
    js_this_expression: &JsThisExpression,
    target_name: &str,
    model: &SemanticModel,
) -> Option<bool> {
    js_this_expression
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(|ancestor| {
            if ancestor.kind() == JsSyntaxKind::JS_CLASS_MEMBER_LIST {
                let class_member_list = JsClassMemberList::cast(ancestor)?;
                return find_and_check_class_member(&class_member_list, target_name, model);
            }
            None
        })
}

/// Finds a class method or property by matching the given name and checks if it is a promise.
///
/// This function searches for a class method or property in the given `JsClassMemberList`
/// by matching the provided `target_name`. If a matching member is found, it checks if the member
/// is a promise. If no matching member is found, it checks the parent class (if any) and recursively
/// checks the method or property in the parent class.
///
/// # Arguments
///
/// * `class_member_list` - A reference to a `JsClassMemberList` representing the class members to search in.
/// * `target_name` - The name of the method or property to search for.
/// * `model` - A reference to the `SemanticModel` used for resolving bindings.
///
/// # Returns
///
/// * `Some(true)` if the class member is a promise.
/// * `Some(false)` if the class member is not a promise.
/// * `None` if there is an error in the process or if the class member is not found.
///
fn find_and_check_class_member(
    class_member_list: &JsClassMemberList,
    target_name: &str,
    model: &SemanticModel,
) -> Option<bool> {
    // Check current class first
    if let Some(member) = find_class_method_or_property(class_member_list, target_name) {
        return is_class_member_a_promise(&member, model);
    }

    // Check parent class if exists
    check_parent_class(class_member_list, target_name, model)
}

fn check_parent_class(
    class_member_list: &JsClassMemberList,
    target_name: &str,
    model: &SemanticModel,
) -> Option<bool> {
    let parent_class_decl =
        if let Some(class_decl) = class_member_list.parent::<JsClassDeclaration>() {
            get_parent_class_declaration(&class_decl.extends_clause()?, model)?
        } else if let Some(class_expr) = class_member_list.parent::<JsClassExpression>() {
            get_parent_class_declaration(&class_expr.extends_clause()?, model)?
        } else {
            return None;
        };

    find_and_check_class_member(&parent_class_decl.members(), target_name, model)
}

/// Extracts the parent class declaration from an extends clause
fn get_parent_class_declaration(
    extends_clause: &JsExtendsClause,
    model: &SemanticModel,
) -> Option<JsClassDeclaration> {
    let super_class = extends_clause.super_class().ok()?;
    let identifier_expression = super_class.as_js_identifier_expression()?;
    let reference = identifier_expression.name().ok()?;
    let binding = model.binding(&reference)?;
    let any_js_binding_decl = binding.tree().declaration()?;

    match any_js_binding_decl {
        AnyJsBindingDeclaration::JsClassDeclaration(parent_class_decl) => Some(parent_class_decl),
        _ => None,
    }
}

fn find_class_method_or_property(
    class_member_list: &JsClassMemberList,
    target_name: &str,
) -> Option<AnyJsClassMember> {
    class_member_list.iter().find(|member| match member {
        AnyJsClassMember::JsMethodClassMember(method) => method
            .name()
            .ok()
            .and_then(|name| name.name())
            .is_some_and(|class_member_name| class_member_name.text() == target_name),
        AnyJsClassMember::JsPropertyClassMember(property) => property
            .name()
            .ok()
            .and_then(|name| name.name())
            .is_some_and(|class_member_name| class_member_name.text() == target_name),
        _ => false,
    })
}

fn is_class_member_a_promise(
    class_member: &AnyJsClassMember,
    model: &SemanticModel,
) -> Option<bool> {
    match class_member {
        AnyJsClassMember::JsMethodClassMember(method) => Some(
            method.async_token().is_some()
                || is_return_type_a_promise(method.return_type_annotation()).unwrap_or_default(),
        ),
        AnyJsClassMember::JsPropertyClassMember(property) => {
            if let Some(property_annotation) = property.property_annotation() {
                let ts_type_annotation = property_annotation.as_ts_type_annotation()?;
                let any_ts_type = ts_type_annotation.ty().ok()?;

                return is_ts_type_a_promise(&any_ts_type);
            }

            if let Some(initializer_clause) = property.value() {
                return is_initializer_a_promise(&initializer_clause, model, None);
            }

            None
        }
        _ => None,
    }
}

fn is_ts_type_a_promise(any_ts_type: &AnyTsType) -> Option<bool> {
    match any_ts_type {
        AnyTsType::TsFunctionType(func_type) => {
            let return_type = func_type.return_type().ok()?;
            let ref_type = return_type.as_any_ts_type()?.as_ts_reference_type()?;
            let name = ref_type.name().ok()?;
            let identifier = name.as_js_reference_identifier()?;

            Some(identifier.has_name("Promise"))
        }
        AnyTsType::TsReferenceType(ts_ref_type) => {
            let name = ts_ref_type.name().ok()?;
            let identifier = name.as_js_reference_identifier()?;

            Some(identifier.has_name("Promise"))
        }
        _ => None,
    }
}

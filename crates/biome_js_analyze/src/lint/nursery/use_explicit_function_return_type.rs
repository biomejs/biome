use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_semantic::HasClosureAstNode;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, AnyTsType, JsFileSource,
    JsStatementList, JsSyntaxKind,
};
use biome_js_syntax::{
    AnyJsFunction, JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember,
    JsMethodObjectMember,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxNodeOptionExt, TextRange};

declare_lint_rule! {
    /// Require explicit return types on functions and class methods.
    ///
    /// Functions in TypeScript often don't need to be given an explicit return type annotation.
    /// Leaving off the return type is less code to read or write and allows the compiler to infer it from the contents of the function.
    ///
    /// However, explicit return types do make it visually more clear what type is returned by a function.
    /// They can also speed up TypeScript type checking performance in large codebases with many large functions.
    /// Explicit return types also reduce the chance of bugs by asserting the return type, and it avoids surprising "action at a distance," where changing the body of one function may cause failures inside another function.
    ///
    /// This rule enforces that functions do have an explicit return type annotation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that no value is returned (void)
    /// function test() {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a number is returned
    /// var fn = function () {
    ///    return 1;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a string is returned
    /// var arrowFn = () => 'test';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Test {
    ///   // Should indicate that no value is returned (void)
    ///   method() {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that no value is returned (void)
    /// function test(a: number) {
    ///   a += 1;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should use const assertions
    /// const func = (value: number) => ({ type: 'X', value }) as any;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const arrowFn = () => () => {};
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const arrowFn = () => {
    ///   return () => { };
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// // No return value should be expected (void)
    /// function test(): void {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type number
    /// var fn = function (): number {
    ///   return 1;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type string
    /// var arrowFn = (): string => 'test';
    /// ```
    ///
    /// ```ts
    /// class Test {
    ///   // No return value should be expected (void)
    ///   method(): void {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// const func = (value: number) => ({ foo: 'bar', value }) as const;
    /// ```
    ///
    /// ```ts
    /// // Callbacks without return types
    /// setTimeout(function() { console.log("Hello!"); }, 1000);
    /// ```
    /// ```ts
    /// // IIFE
    /// (() => {})();
    /// ```
    ///
    /// ```ts
    /// const arrowFn = () => (): void => {};
    /// ```
    ///
    /// ```ts
    /// const arrowFn = () => {
    ///   return (): void => { };
    /// }
    ///
    pub UseExplicitFunctionReturnType {
        version: "next",
        name: "useExplicitFunctionReturnType",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("explicit-function-return-type")],
    }
}

declare_node_union! {
    pub AnyJsFunctionWithReturnType = AnyJsFunction | JsMethodClassMember | JsMethodObjectMember | JsGetterClassMember | JsGetterObjectMember
}

impl Rule for UseExplicitFunctionReturnType {
    type Query = Ast<AnyJsFunctionWithReturnType>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        if !source_type.is_typescript() || source_type.is_definition_file() {
            return None;
        }

        let node = ctx.query();
        match node {
            AnyJsFunctionWithReturnType::AnyJsFunction(func) => {
                if func.return_type_annotation().is_some() {
                    return None;
                }

                if is_direct_const_assertion_in_arrow_functions(func) {
                    return None;
                }

                if is_function_used_in_argument_or_expression_list(func) {
                    return None;
                }

                if is_higher_order_function(func) {
                    return None;
                }

                let func_range = func.syntax().text_range();
                if let Ok(Some(AnyJsBinding::JsIdentifierBinding(id))) = func.id() {
                    return Some(TextRange::new(
                        func_range.start(),
                        id.syntax().text_range().end(),
                    ));
                }

                Some(func_range)
            }
            AnyJsFunctionWithReturnType::JsMethodClassMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }

                Some(method.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsGetterClassMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some(getter.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsMethodObjectMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }

                Some(method.node_text_range())
            }
            AnyJsFunctionWithReturnType::JsGetterObjectMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some(getter.node_text_range())
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Missing return type on function."
                },
            )
            .note(markup! {
                "Declaring the return type makes the code self-documenting and can speed up TypeScript type checking."
            })
            .note(markup! {
                "Add a return type annotation."
            }),
        )
    }
}

/// Checks if an arrow function immediately returns an `as const` value.
///
/// # Examples
///
/// ```typescript
/// const func = (value: number) => ({ foo: 'bar', value }) as const;
/// const func = () => x as const;
/// ```
fn is_direct_const_assertion_in_arrow_functions(func: &AnyJsFunction) -> bool {
    let AnyJsFunction::JsArrowFunctionExpression(arrow_func) = func else {
        return false;
    };

    let Ok(AnyJsFunctionBody::AnyJsExpression(expr)) = arrow_func.body() else {
        return false;
    };

    let AnyJsExpression::TsAsExpression(ts_expr) = expr else {
        return false;
    };

    let Ok(AnyTsType::TsReferenceType(ts_ref)) = ts_expr.ty() else {
        return false;
    };

    ts_ref.text() == "const"
}

/// Checks if a function is allowed within specific expression contexts.
/// These include function calls, array elements, and parenthesized expressions.
///
/// # Examples
///
/// JS_CALL_ARGUMENT_LIST:
/// - `window.addEventListener('click', () => {});`
/// - `const foo = arr.map(i => i * i);`
/// - `setTimeout(function() { console.log("Hello!"); }, 1000);`
///
/// JS_ARRAY_ELEMENT_LIST:
/// - `[function () {}, () => {}];`
///
/// JS_PARENTHESIZED_EXPRESSION:
/// - `(function () {});`
/// - `(() => {})();`
fn is_function_used_in_argument_or_expression_list(func: &AnyJsFunction) -> bool {
    matches!(
        func.syntax().parent().kind(),
        Some(
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
                // We include JS_PARENTHESIZED_EXPRESSION for IIFE (Immediately Invoked Function Expressions).
                // We also assume that the parent of the parent is a call expression.
                | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
        )
    )
}

/// Checks whether the given function is a higher-order function, i.e., a function
/// that returns another function either directly in its body or as an expression.
///
/// A higher-order function is one that returns either a regular function or an arrow
/// function from within its body.
///
/// # Arguments
///
/// * `func` - A reference to an `AnyJsFunction` that represents the JavaScript function to inspect.
///
/// # Returns
///
/// * `true` if the function returns another function (either a regular function or an arrow function).
/// * `false` if it does not return a function or if the body is not a valid returnable function expression.
///
/// # Note
///
/// This function currently **does not support** detecting a return of a function
/// inside other statements like `if` statements or `switch` statements. It only detects
/// direct returns of functions or function returns in a straightforward function body.
fn is_higher_order_function(func: &AnyJsFunction) -> bool {
    match func.body().ok() {
        Some(AnyJsFunctionBody::AnyJsExpression(expr)) => {
            matches!(
                expr,
                AnyJsExpression::JsArrowFunctionExpression(_)
                    | AnyJsExpression::JsFunctionExpression(_)
            )
        }
        Some(AnyJsFunctionBody::JsFunctionBody(func_body)) => {
            check_statements_for_function_return(func_body.statements())
        }
        _ => false,
    }
}

/// Checks whether the given list of JavaScript statements contains a return statement
/// that returns a function expression (either a regular function or an arrow function).
///
/// # Arguments
///
/// * `statements` - A list of JavaScript statements (`JsStatementList`) to inspect.
///
/// # Returns
///
/// * `true` if the list contains a return statement with a function expression as its argument.
/// * `false` if no such return statement is found or if the list is empty.
fn check_statements_for_function_return(statements: JsStatementList) -> bool {
    statements.into_iter().any(|statement| {
        if let AnyJsStatement::JsReturnStatement(return_stmt) = statement {
            if let Some(args) = return_stmt.argument() {
                return matches!(
                    args,
                    AnyJsExpression::JsFunctionExpression(_)
                        | AnyJsExpression::JsArrowFunctionExpression(_)
                );
            }
        }
        false
    })
}

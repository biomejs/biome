use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{Markup, markup};
use biome_diagnostics::Severity;
use biome_js_semantic::HasClosureAstNode;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyJsObjectMember, AnyJsStatement, AnyTsType,
    JsArrowFunctionExpression, JsCallExpression, JsConstructorClassMember, JsFileSource,
    JsFormalParameter, JsFunctionDeclaration, JsGetterClassMember, JsGetterObjectMember,
    JsInitializerClause, JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsModuleItemList,
    JsObjectExpression, JsParameters, JsParenthesizedExpression, JsPropertyClassMember,
    JsPropertyObjectMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsStatementList, JsSyntaxKind, JsVariableDeclaration, JsVariableDeclarationClause,
    JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement, TsCallSignatureTypeMember,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration,
    TsGetterSignatureClassMember, TsMethodSignatureClassMember, TsMethodSignatureTypeMember,
    static_value::StaticValue,
};
use biome_rowan::{
    AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeOptionExt, TextRange, declare_node_union,
};
use biome_rule_options::use_explicit_type::UseExplicitTypeOptions;

declare_lint_rule! {
    /// Enforce types in functions, methods, variables, and parameters.
    ///
    /// Functions in TypeScript often don't need to be given an explicit return type annotation.
    /// Leaving off the return type is less code to read or write and allows the compiler to infer it from the contents of the function.
    ///
    /// However, explicit return types do make it visually clearer what type is returned by a function.
    /// They can also speed up TypeScript type-checking performance in large codebases with many large functions.
    /// Explicit return types also reduce the chance of bugs by asserting the return type, and it avoids surprising "action at a distance," where changing the body of one function may cause failures inside another function.
    ///
    /// Annotating module-level variables serves a similar purpose. This rule only allows assignment of literals and some objects to untyped variables.
    /// Objects that are allowed must not contain spread syntax and values that aren't literals.
    /// Additionally, `let` and `var` variables with `null` or `undefined` as value require explicit annotation.
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
    /// var func = (value: number) => ({ type: 'X', value }) as any;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Unspecified variable type
    /// function fn(): string {
    ///     return "Not inline";
    /// }
    /// const direct = fn();
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Unspecified object member type
    /// function fn(): string {
    ///     return "Not inline";
    /// }
    /// const nested = { result: fn() };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // let bindings of null and undefined are usually overwritten by other code
    /// let foo = null;
    /// ```
    ///
    /// The following example is considered incorrect for a higher-order function, as the returned function does not specify a return type:
    ///
    /// ```ts,expect_diagnostic
    /// var arrowFn = () => () => {};
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// var arrowFn = () => {
    ///   return () => { };
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for a higher-order function because the function body contains multiple statements. We only check whether the first statement is a function return.
    ///
    /// ```ts,expect_diagnostic
    /// // A function has multiple statements in the body
    /// function f() {
    ///   if (x) {
    ///     return 0;
    ///   }
    ///   return (): void => {}
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // A function has multiple statements in the body
    /// function f() {
    ///   let str = "test";
    ///   return (): string => {
    ///     str;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // A function has multiple statements in the body
    /// function f() {
    ///   let str = "test";
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for an interface method without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// interface Array<Type> {
    ///   method();
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for a type declaration of a function without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// type MyObject = {
    ///   (input: string);
    ///   propertyName: string;
    /// };
    /// ```
    ///
    /// The following example is considered incorrect for an abstract class method without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// abstract class MyClass {
    ///   public abstract method();
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for an abstract class getter without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// abstract class P<T> {
    ///   abstract get poke();
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for a function declaration in a namespace without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// declare namespace myLib {
    ///   function makeGreeting(s: string);
    /// }
    /// ```
    ///
    /// The following example is considered incorrect for a module function export without a return type:
    ///
    /// ```ts,expect_diagnostic
    /// declare module "foo" {
    ///   export default function bar();
    /// }
    /// ```
    ///
    /// The following example is considered incorrect because `arg` has `any` type.
    ///
    /// ```ts,expect_diagnostic
    /// var arrowFn = (arg: any): string => `test ${arg}`;
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
    /// // A literal value
    /// const PREFIX = "/prefix";
    /// ```
    ///
    /// ```ts
    /// // Explicit variable annotation
    /// function func(): string {
    ///     return "";
    /// }
    /// let something: string = func();
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
    /// The following example is considered correct code for a function immediately returning a value with `as const`:
    ///
    /// ```ts
    /// var func = (value: number) => ({ foo: 'bar', value }) as const;
    /// ```
    ///
    /// The following example is considered correct code for a value assigned using type assertion:
    ///
    /// ```ts
    /// function fn(): string {
    ///     return "Not inline";
    /// }
    /// const direct = fn() as string;
    /// const nested = { result: fn() as string };
    /// ```
    ///
    /// The following examples are considered correct code for a function allowed within specific expression contexts, such as an IIFE, a function passed as an argument, or a function inside an array:
    ///
    /// ```ts
    /// // Callbacks without return types
    /// setTimeout(function() { console.log("Hello!"); }, 1000);
    /// ```
    ///
    /// ```ts
    /// // Callbacks without argument types (immediately nested in a function call)
    /// new Promise((resolve) => resolve(1));
    /// ```
    ///
    /// ```ts
    /// // IIFE
    /// (() => {})();
    /// ```
    ///
    /// The following example is considered correct code for a higher-order function, where the returned function explicitly specifies a return type and the function body contains only one statement:
    ///
    /// ```ts
    /// // the outer function returns an inner function that has a `void` return type
    /// var arrowFn = () => (): void => {};
    /// ```
    ///
    /// ```ts
    /// // the outer function returns an inner function that has a `void` return type
    /// var arrowFn = () => {
    ///   return (): void => { };
    /// }
    /// ```
    ///
    /// The following examples are considered correct for type annotations on variables in function expressions:
    ///
    /// ```ts
    /// // A function with a type assertion using `as`
    /// var asTyped = (() => '') as () => string;
    /// ```
    ///
    /// ```ts
    /// // A function with a type assertion using `<>`
    /// var castTyped = <() => string>(() => '');
    /// ```
    ///
    /// ```ts
    /// // A variable declarator with a type annotation.
    /// type FuncType = () => string;
    /// var arrowFn: FuncType = () => 'test';
    /// ```
    ///
    /// ```ts
    /// // A function is a default parameter with a type annotation
    /// type CallBack = () => void;
    /// var f = (gotcha: CallBack = () => { }): void => { };
    /// ```
    ///
    /// ```ts
    /// // A class property with a type annotation
    /// type MethodType = () => void;
    /// class App {
    ///     private method: MethodType = () => { };
    /// }
    /// ```
    ///
    pub UseExplicitType {
        version: "1.9.3",
        name: "useExplicitType",
        language: "ts",
        recommended: false,
        severity: Severity::Error,
        sources: &[
            RuleSource::EslintTypeScript("explicit-function-return-type").inspired(),
            RuleSource::EslintTypeScript("explicit-module-boundary-types").inspired(),
        ],
    }
}

declare_node_union! {
    pub AnyEntityWithTypes =
        AnyJsFunction
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsGetterClassMember
        | JsGetterObjectMember
        | TsMethodSignatureTypeMember
        | TsCallSignatureTypeMember
        | TsMethodSignatureClassMember
        | TsGetterSignatureClassMember
        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration
        | JsConstructorClassMember
        | JsSetterObjectMember
        | JsSetterClassMember
        | JsVariableDeclarator
}

pub enum ViolationKind {
    UntypedParameter,
    AnyParameter,
    UntypedFunction,
    UntypedMember,
    UntypedDeclaration,
    UntypedVariable,
}

impl ViolationKind {
    fn as_message(&self) -> Markup<'_> {
        match self {
            Self::UntypedParameter => markup! {
                "The parameter doesn't have a type defined."
            },
            Self::AnyParameter => markup! {
                "The parameter has an "<Emphasis>"any"</Emphasis>" type."
            },
            Self::UntypedVariable => markup! {
                "The variable doesn't have a type defined."
            },
            Self::UntypedFunction => markup! {
                "Missing return type on function."
            },
            Self::UntypedMember => markup! {
                "Missing return type on member."
            },
            Self::UntypedDeclaration => markup! {
                "Missing return type on function declaration."
            },
        }
    }

    fn as_advice(&self) -> Markup<'_> {
        match self {
            Self::UntypedParameter => markup! {
                "Add a type to the parameter."
            },
            Self::AnyParameter => markup! {
                "Replace "<Emphasis>"any"</Emphasis>" with "<Emphasis>"unknown"</Emphasis>" or a more specific type."
            },
            Self::UntypedVariable => markup! {
                "Add a type to the variable."
            },
            Self::UntypedFunction => markup! {
                "Add a return type to the function."
            },
            Self::UntypedMember => markup! {
                "Add a return type to the member."
            },
            Self::UntypedDeclaration => markup! {
                "Add a return type to the function declaration."
            },
        }
    }
}

type State = (TextRange, ViolationKind);

impl Rule for UseExplicitType {
    type Query = Ast<AnyEntityWithTypes>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = UseExplicitTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        if !source_type.is_typescript() || source_type.is_definition_file() {
            return None;
        }

        let node = ctx.query();
        match node {
            AnyEntityWithTypes::AnyJsFunction(func) => {
                if is_function_used_in_argument(func) {
                    // Inline callbacks are usually inferred
                    return None;
                }
                if let Some(state) = handle_any_function(func) {
                    Some(state)
                } else {
                    let parameters = func.parameters().ok()?;

                    match parameters {
                        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                            // a binding as an argument can't have a type, so we need to raise a diagnostic
                            return Some((binding.range(), ViolationKind::UntypedParameter));
                        }
                        AnyJsArrowFunctionParameters::JsParameters(parameters) => {
                            if let Some(state) = has_untyped_parameter(&parameters) {
                                return Some(state);
                            }
                        }
                    }

                    None
                }
            }
            AnyEntityWithTypes::JsMethodClassMember(method) => {
                if method.return_type_annotation().is_some() {
                    let parameters = method.parameters().ok()?;
                    if let Some(state) = has_untyped_parameter(&parameters) {
                        return Some(state);
                    }
                    return None;
                }
                Some((method.node_text_range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::JsGetterClassMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some((getter.node_text_range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::JsMethodObjectMember(method) => {
                if method.return_type_annotation().is_some() {
                    let parameters = method.parameters().ok()?;

                    if let Some(state) = has_untyped_parameter(&parameters) {
                        return Some(state);
                    }
                    return None;
                }

                Some((method.node_text_range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::JsGetterObjectMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }

                Some((getter.node_text_range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::TsMethodSignatureTypeMember(member) => {
                if member.return_type_annotation().is_some() {
                    let parameters = member.parameters().ok()?;
                    if let Some(state) = has_untyped_parameter(&parameters) {
                        return Some(state);
                    }
                    return None;
                }

                Some((member.range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::TsCallSignatureTypeMember(member) => {
                if member.return_type_annotation().is_some() {
                    let parameters = member.parameters().ok()?;
                    if let Some(state) = has_untyped_parameter(&parameters) {
                        return Some(state);
                    }

                    return None;
                }
                Some((member.range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::TsMethodSignatureClassMember(member) => {
                if member.return_type_annotation().is_some() {
                    return None;
                }
                Some((member.range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::TsGetterSignatureClassMember(member) => {
                if member.return_type().is_some() {
                    return None;
                }
                Some((member.range(), ViolationKind::UntypedMember))
            }
            AnyEntityWithTypes::TsDeclareFunctionDeclaration(decl) => {
                if decl.return_type_annotation().is_some() {
                    return None;
                }
                Some((decl.range(), ViolationKind::UntypedDeclaration))
            }
            AnyEntityWithTypes::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                if decl.return_type_annotation().is_some() {
                    let parameters = decl.parameters().ok()?;
                    if let Some(state) = has_untyped_parameter(&parameters) {
                        return Some(state);
                    }
                    return None;
                }
                Some((decl.range(), ViolationKind::UntypedDeclaration))
            }
            AnyEntityWithTypes::JsVariableDeclarator(declarator) => {
                handle_variable_declarator(declarator)
            }
            AnyEntityWithTypes::JsConstructorClassMember(constructor) => {
                let parameters = constructor.parameters().ok()?;

                let parameters = parameters
                    .parameters()
                    .iter()
                    .flatten()
                    .filter_map(|parameter| parameter.as_any_js_formal_parameter().cloned())
                    .filter_map(|parameter| parameter.as_js_formal_parameter().cloned());
                for parameter in parameters {
                    if let Some(state) = parameter_has_not_type(&parameter) {
                        return Some(state);
                    }
                }

                None
            }
            AnyEntityWithTypes::JsSetterObjectMember(setter) => {
                let parameter = setter.parameter().ok()?;
                let parameter = parameter.as_js_formal_parameter()?;
                if let Some(state) = parameter_has_not_type(parameter) {
                    return Some(state);
                }
                None
            }
            AnyEntityWithTypes::JsSetterClassMember(setter) => {
                let parameter = setter.parameter().ok()?;
                let parameter = parameter.as_js_formal_parameter()?;
                if let Some(state) = parameter_has_not_type(parameter) {
                    return Some(state);
                }
                None
            }
        }
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().syntax().first_token()?.text_trimmed_range())
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (range, violation): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                violation.as_message(),
            )
            .note(markup! {
                "Declaring the type makes the code self-documented and can speed up TypeScript type checking."
            })
            .note(violation.as_advice()),
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

    ts_ref.to_trimmed_text().text() == "const"
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
fn is_function_used_in_argument(func: &AnyJsFunction) -> bool {
    matches!(
        func.syntax().parent().kind(),
        Some(JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
    )
}

/// Checks whether a function can be inlined without being tagged with a type.
///
/// # Examples
///
/// ```ts
/// const x: Type = { prop: () => {} }
/// f({ prop: () => {} })
/// ```
///
fn can_inline_function(func: &AnyJsFunction) -> bool {
    let object_expression = func.syntax().ancestors().find_map(JsObjectExpression::cast);

    let Some(object_expression) = object_expression else {
        return false;
    };

    for ancestor in object_expression.syntax().ancestors() {
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor)
            && declarator.variable_annotation().is_some()
        {
            return true;
        }

        if JsCallExpression::can_cast(ancestor.kind()) {
            return true;
        }
    }

    false
}

/// Checks where the arrow function is inlined inside a typed return statement or arrow function
///
/// # Examples
///
/// ```ts
/// function getObjectWithFunction(): Behavior {
///   return {
///     attribute: 'value',
///     func: function myFunc(): string { return "value" },
///     arrowFunc: () => {},
///   }
/// }
/// ```
///
/// ```ts
/// const getObjectWithFunction1 = (): Behavior => {
///   return {
///     namedFunc: function myFunc(): string { return "value" },
///     arrowFunc: () => {},
///   }
/// }
/// ```
fn is_function_inside_typed_return(func: &AnyJsFunction) -> bool {
    let return_statement = func.syntax().ancestors().find_map(JsReturnStatement::cast);

    let Some(return_statement) = return_statement else {
        return false;
    };

    for ancestor in return_statement.syntax().ancestors() {
        if let Some(function_declaration) = JsFunctionDeclaration::cast_ref(&ancestor)
            && function_declaration.return_type_annotation().is_some()
        {
            return true;
        }

        if let Some(function_expression) = JsArrowFunctionExpression::cast_ref(&ancestor)
            && function_expression.return_type_annotation().is_some()
        {
            return true;
        }
    }

    false
}

/// Check if a function is an arrow function
fn is_arrow_func(func: &AnyJsFunction) -> bool {
    matches!(func, AnyJsFunction::JsArrowFunctionExpression(_))
}

/// Checks if a function is an IIFE (Immediately Invoked Function Expressions)
///
/// # Examples
///
/// ```typescript
/// (function () {});
/// (() => {})();
/// ```
fn is_iife(func: &AnyJsFunction) -> bool {
    func.parent::<JsParenthesizedExpression>()
        .and_then(|expr| expr.parent::<JsCallExpression>())
        .is_some()
}

/// Checks whether the given function is a higher-order function, i.e., a function
/// that returns another function either directly in its body or as an expression.
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
/// inside other statements, such as `if` statements or `switch` statements. It only checks
/// whether the first statement is a return of a function in a straightforward function body.
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
            is_first_statement_function_return(func_body.statements())
        }
        _ => false,
    }
}

/// Checks whether the first statement in the given list of JavaScript statements is a return statement
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
fn is_first_statement_function_return(statements: JsStatementList) -> bool {
    statements
        .into_iter()
        .next()
        .and_then(|stmt| {
            if let AnyJsStatement::JsReturnStatement(return_stmt) = stmt {
                return_stmt.argument()
            } else {
                None
            }
        })
        .is_some_and(|args| {
            matches!(
                args,
                AnyJsExpression::JsFunctionExpression(_)
                    | AnyJsExpression::JsArrowFunctionExpression(_)
            )
        })
}

/// Checks if a given function expression has a type annotation.
fn is_typed_function_expressions(func: &AnyJsFunction) -> bool {
    let syntax = func.syntax();
    is_type_assertion(syntax)
        || is_variable_declarator_with_type_annotation(syntax)
        || is_default_function_parameter_with_type_annotation(syntax)
        || is_class_property_with_type_annotation(syntax)
        || is_property_of_object_with_type(syntax)
}

/// Checks if a function is a variable declarator with a type annotation.
///
/// # Examples
///
/// ```typescript
/// type FuncType = () => string;
/// const arrowFn: FuncType = () => 'test';
/// ```
fn is_variable_declarator_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsVariableDeclarator>())
        .is_some_and(|decl| decl.variable_annotation().is_some())
}

/// Checks if a function is a default parameter with a type annotation.
///
/// ## Example
///
/// ```typescript
/// type CallBack = () => void;
/// const f = (gotcha: CallBack = () => { }): void => { };
/// ```
fn is_default_function_parameter_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsFormalParameter>())
        .is_some_and(|param| param.type_annotation().is_some())
}

/// Checks if a function is a class property with a type annotation.
///
/// # Examples
///
/// ```typescript
/// type MethodType = () => void;
/// class App {
///     private method: MethodType = () => { };
/// }
/// ```
fn is_class_property_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsPropertyClassMember>())
        .is_some_and(|prop| prop.property_annotation().is_some())
}

/// Checks if a function is a property or a nested property of a typed object.
///
/// # Examples
///
/// ```typescript
/// const x: Foo = { prop: () => {} }
/// const x = { prop: () => {} } as Foo
/// const x = <Foo>{ prop: () => {} }
/// const x: Foo = { bar: { prop: () => {} } }
/// ```
fn is_property_of_object_with_type(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsPropertyObjectMember::cast)
        .and_then(|prop| prop.syntax().grand_parent())
        .and_then(JsObjectExpression::cast)
        .is_some_and(|obj_expression| {
            let obj_syntax = obj_expression.syntax();
            is_type_assertion(obj_syntax)
                || is_variable_declarator_with_type_annotation(obj_syntax)
                || is_property_of_object_with_type(obj_syntax)
        })
}

/// Checks if a function has a type assertion.
///
/// # Examples
///
/// ```typescript
/// const asTyped = (() => '') as () => string;
/// const castTyped = <() => string>(() => '');
/// ```
fn is_type_assertion(syntax: &SyntaxNode<JsLanguage>) -> bool {
    fn is_attribute_kind(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::TS_AS_EXPRESSION | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        )
    }

    syntax.parent().is_some_and(|parent| {
        if parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            parent
                .parent()
                .is_some_and(|grandparent| is_attribute_kind(grandparent.kind()))
        } else {
            is_attribute_kind(parent.kind())
        }
    })
}

fn handle_any_function(func: &AnyJsFunction) -> Option<State> {
    if func.return_type_annotation().is_some() {
        return None;
    }

    if is_direct_const_assertion_in_arrow_functions(func) {
        return None;
    }

    if is_iife(func) {
        return None;
    }

    // TODO: why only arrow functions are ignored inside typed return?
    // see getObjectWithFunction in valid.ts test
    if is_arrow_func(func) && (can_inline_function(func) || is_function_inside_typed_return(func)) {
        return None;
    }

    if is_higher_order_function(func) {
        return None;
    }

    if is_typed_function_expressions(func) {
        return None;
    }

    let func_range = func.syntax().text_trimmed_range();
    if let Ok(Some(AnyJsBinding::JsIdentifierBinding(id))) = func.id() {
        return Some((
            TextRange::new(func_range.start(), id.syntax().text_trimmed_range().end()),
            ViolationKind::UntypedFunction,
        ));
    }

    Some((func_range, ViolationKind::UntypedFunction))
}

/// Checks if a variable declarator needs to have an explicit type.
fn handle_variable_declarator(declarator: &JsVariableDeclarator) -> Option<State> {
    // Explicit annotation is always sufficient
    let has_explicit_type = declarator
        .variable_annotation()
        .is_some_and(|ty| ty.as_ts_type_annotation().is_some_and(|ty| ty.ty().is_ok()));
    if has_explicit_type {
        return None;
    }

    let variable_declaration = declarator
        .parent::<JsVariableDeclaratorList>()?
        .parent::<JsVariableDeclaration>()?;
    let is_top_level = declarator
        .syntax()
        .ancestors()
        .find_map(JsVariableStatement::cast)
        .is_some_and(|statement| {
            statement
                .syntax()
                .parent()
                .is_some_and(|parent| JsModuleItemList::can_cast(parent.kind()))
        })
        || variable_declaration
            .parent::<JsVariableDeclarationClause>()
            .is_some();

    if !is_top_level {
        return None;
    }

    let initializer_expression = declarator
        .initializer()
        .and_then(|init| init.expression().ok())
        .map(|expr| expr.omit_parentheses());

    let is_const = variable_declaration.is_const();

    if let Some(initializer_expression) = initializer_expression {
        if is_allowed_in_untyped_expression(&initializer_expression, is_const) {
            return None;
        }
    } else if is_const {
        // `const` without RHS is invalid anyway and should be reported elsewhere.
        return None;
    }

    Some((
        declarator.id().ok()?.syntax().text_trimmed_range(),
        ViolationKind::UntypedVariable,
    ))
}

/// Checks if an expression can be part of an untyped expression or will be checked separately.
///
/// This returns true for constructs that are trivially understood by the reader and the compiler
/// without following any other definitions, such as literals and objects built of literals,
/// as well as type assertions.
///
/// If `allow_placeholders` is false, excludes `null` and `undefined`.
fn is_allowed_in_untyped_expression(expr: &AnyJsExpression, allow_placeholders: bool) -> bool {
    // `undefined` is not a trivially inferrable type for some reason
    let is_undefined_literal = matches!(expr.as_static_value(), Some(StaticValue::Undefined(_)));
    let rhs_is_null = matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNullLiteralExpression(_))
    );
    let is_trivial_rhs = expr.has_trivially_inferrable_type()
        || matches!(
            expr,
            // Casts are already fine, no less clear than annotations.
            AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsTypeAssertionExpression(_)
        );

    if matches!(
        expr,
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    ) {
        // We'll check functions separately
        return true;
    }

    // Allow assignment of some trivial object literals.
    if let AnyJsExpression::JsObjectExpression(object_expr) = expr {
        let has_only_allowed_members = object_expr.members().iter().all(|member| {
            let Ok(member) = member else { return true };
            match member {
                // Functions are checked separately, do not produce a bogus
                // diagnostic that will be resolved by adding types to that function
                AnyJsObjectMember::JsGetterObjectMember(_)
                | AnyJsObjectMember::JsSetterObjectMember(_)
                | AnyJsObjectMember::JsMethodObjectMember(_) => true,
                AnyJsObjectMember::JsBogusMember(_) => true,
                AnyJsObjectMember::JsPropertyObjectMember(prop) => match prop.value() {
                    // Recurse into regular properties
                    Ok(value) => is_allowed_in_untyped_expression(&value, allow_placeholders),
                    Err(_) => true,
                },
                // Anything else is too complicated to mentally parse without types
                _ => false,
            }
        });
        if has_only_allowed_members {
            return true;
        }
    }

    // Const assignments of trivial expressions such as literals
    // can remain unannotated.
    // Let assignments are slightly different: init-less, null and
    // undefined usually indicate "we'll assign this value elsewhere".
    // Require types in those cases, but still allow other literals
    // as assignments of other types to those won't compile.
    if allow_placeholders {
        is_trivial_rhs || is_undefined_literal
    } else {
        is_trivial_rhs && !rhs_is_null
    }
}

fn has_untyped_parameter(parameters: &JsParameters) -> Option<State> {
    let parameters = parameters
        .items()
        .iter()
        .flatten()
        .filter_map(|parameter| parameter.as_any_js_formal_parameter().cloned())
        .filter_map(|parameter| parameter.as_js_formal_parameter().cloned());
    for parameter in parameters {
        if let Some(state) = parameter_has_not_type(&parameter) {
            return Some(state);
        }
    }

    None
}

/// The formal parameter is triggered if:
/// - it doesn't have any type
/// - it its type is `any`
fn parameter_has_not_type(parameter: &JsFormalParameter) -> Option<State> {
    let ty = parameter.type_annotation();

    if let Some(ty) = ty {
        let ty = ty.ty().ok()?;
        if matches!(ty, AnyTsType::TsAnyType(_)) {
            Some((ty.range(), ViolationKind::AnyParameter))
        } else {
            None
        }
    } else {
        Some((parameter.range(), ViolationKind::UntypedParameter))
    }
}

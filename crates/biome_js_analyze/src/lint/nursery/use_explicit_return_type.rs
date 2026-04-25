use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::HasClosureAstNode;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement, AnyTsType,
    JsArrowFunctionExpression, JsCallExpression, JsFileSource, JsFormalParameter,
    JsFunctionDeclaration, JsGetterClassMember, JsGetterObjectMember, JsInitializerClause,
    JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsObjectExpression,
    JsPropertyClassMember, JsPropertyObjectMember, JsReturnStatement, JsStatementList,
    JsSyntaxKind, JsVariableDeclarator,
};
use biome_rowan::AstNodeList;
use biome_rowan::{
    AstNode, SyntaxNode, SyntaxNodeOptionExt, TextRange, TokenText, declare_node_union,
};
use biome_rule_options::use_explicit_return_type::UseExplicitReturnTypeOptions;

declare_lint_rule! {
    /// Require explicit return types on functions and class methods.
    ///
    /// Functions in TypeScript often don't need to be given an explicit return type annotation.
    /// Leaving off the return type is less code to read or write and allows the compiler to infer it from the contents of the function.
    ///
    /// However, explicit return types are required when using TypeScript's
    /// [isolatedModules](https://www.typescriptlang.org/tsconfig/#isolatedModules) and similar
    /// single-file transpilation tools, because the compiler cannot perform
    /// cross-file type inference. Without explicit return types, exported functions may produce
    /// incomplete or incorrect `.d.ts` declarations.
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
    /// The following example is considered incorrect because not all return statements return a function. A higher-order function is only valid when every return statement returns a function expression:
    ///
    /// ```ts,expect_diagnostic
    /// function f() {
    ///   if (x) {
    ///     return 0;
    ///   }
    ///   return (): void => {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
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
    /// The following example is considered correct code for a function immediately returning a value with `as const`:
    ///
    /// ```ts
    /// var func = (value: number) => ({ foo: 'bar', value }) as const;
    /// ```
    ///
    /// The following examples are considered correct code for a function passed as an argument:
    ///
    /// ```ts
    /// // Callbacks without return types
    /// setTimeout(function() { console.log("Hello!"); }, 1000);
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
    /// ## Options
    ///
    /// ### `allowExpressions`
    ///
    /// When set to `true`, only function declarations and class methods are checked.
    /// Function expressions (assigned to variables, passed as arguments, etc.) are allowed
    /// without return types.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowExpressions": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// // Callbacks and standalone expressions are allowed
    /// setTimeout(function() { console.log("Hello!"); }, 1000);
    /// foo(() => 1);
    /// ```
    ///
    /// ### `allowIifes`
    ///
    /// When set to `true`, IIFEs (Immediately Invoked Function Expressions) are allowed
    /// without explicit return types.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowIifes": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// // IIFEs are allowed
    /// (function () {
    ///   return 1;
    /// })();
    /// (() => {
    ///   return 1;
    /// })();
    /// ```
    ///
    /// ### `allowedNames`
    ///
    /// An array of function/method names that are allowed to not have explicit return types.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedNames": ["myFunction"]
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// // Functions with allowed names don't need return types
    /// function myFunction() {
    ///   return 42;
    /// }
    /// ```
    ///
    pub UseExplicitReturnType {
        version: "2.4.11",
        name: "useExplicitReturnType",
        language: "ts",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintTypeScript("explicit-function-return-type").inspired(),
        ],
        issue_number: Some("2017"),
    }
}

declare_node_union! {
    pub AnyFunctionLikeWithReturnType =
        AnyJsFunction
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsGetterClassMember
        | JsGetterObjectMember
}

impl Rule for UseExplicitReturnType {
    type Query = Ast<AnyFunctionLikeWithReturnType>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseExplicitReturnTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        if !source_type.is_typescript() || source_type.is_definition_file() {
            return None;
        }

        let options = ctx.options();
        let allow_expressions = options.allow_expressions.unwrap_or(false);
        let allow_iifes = options.allow_iifes.unwrap_or(false);

        let node = ctx.query();
        match node {
            AnyFunctionLikeWithReturnType::AnyJsFunction(func) => {
                handle_any_function(func, options, allow_expressions, allow_iifes)
            }
            AnyFunctionLikeWithReturnType::JsMethodClassMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }
                if is_name_allowed(options, get_method_class_member_name(method).as_ref()) {
                    return None;
                }
                Some(method.node_text_range())
            }
            AnyFunctionLikeWithReturnType::JsGetterClassMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }
                if is_name_allowed(options, get_getter_class_member_name(getter).as_ref()) {
                    return None;
                }
                Some(getter.node_text_range())
            }
            AnyFunctionLikeWithReturnType::JsMethodObjectMember(method) => {
                if method.return_type_annotation().is_some() {
                    return None;
                }
                if is_name_allowed(options, get_method_object_member_name(method).as_ref()) {
                    return None;
                }
                // Object methods inside call arguments are usually inferred
                if is_member_inside_call_argument(method.syntax()) {
                    return None;
                }
                Some(method.node_text_range())
            }
            AnyFunctionLikeWithReturnType::JsGetterObjectMember(getter) => {
                if getter.return_type().is_some() {
                    return None;
                }
                if is_name_allowed(options, get_getter_object_member_name(getter).as_ref()) {
                    return None;
                }
                // Object getters inside call arguments are usually inferred
                if is_member_inside_call_argument(getter.syntax()) {
                    return None;
                }
                Some(getter.node_text_range())
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Missing return type on function."
                },
            )
            .note(markup! {
                "Declaring the return type makes the code self-documented and can speed up TypeScript type checking."
            })
            .note(markup! {
                "Add a return type to the function."
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

    is_const_assertion(&expr)
}

/// Recursively checks if an expression is a `const` assertion, possibly wrapped in `satisfies`.
fn is_const_assertion(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::TsAsExpression(ts_expr) => {
            let Ok(AnyTsType::TsReferenceType(ts_ref)) = ts_expr.ty() else {
                return false;
            };
            ts_ref.to_trimmed_text().text() == "const"
        }
        AnyJsExpression::TsSatisfiesExpression(ts_expr) => {
            if let Ok(inner) = ts_expr.expression() {
                is_const_assertion(&inner)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a function is used as a callback argument.
fn is_function_used_in_argument(func: &AnyJsFunction) -> bool {
    matches!(
        func.syntax().parent().kind(),
        Some(JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
    )
}

/// Checks whether a function can be inlined without being tagged with a type.
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

        // Also check class properties with type annotations
        if let Some(prop) = JsPropertyClassMember::cast_ref(&ancestor)
            && prop.property_annotation().is_some()
        {
            return true;
        }

        if JsCallExpression::can_cast(ancestor.kind()) {
            return true;
        }
    }

    false
}

/// Checks if any ancestor of the function has a valid return type.
///
/// The function must be either:
/// - A return statement argument (possibly via an object property)
/// - The body of a bodyless arrow function
///
/// Then it walks up ancestors looking for:
/// - A function (arrow/expression/declaration) with a return type annotation
/// - A typed variable declarator (`const x: Foo = ...`)
/// - A typed class property (`public x: Foo = ...`)
fn ancestor_has_return_type(func: &AnyJsFunction) -> bool {
    let syntax = func.syntax();
    let parent = match syntax.parent() {
        Some(p) => p,
        None => return false,
    };

    // If the parent is a property (e.g. `{ arrowFn: () => 'test' }`), check if the func
    // itself is a bodyless arrow, then walk upward from the property.
    let walk_start = if JsPropertyObjectMember::can_cast(parent.kind()) {
        let is_bodyless_arrow = JsArrowFunctionExpression::cast(syntax.clone())
            .and_then(|arrow| arrow.body().ok())
            .is_some_and(|body| matches!(body, AnyJsFunctionBody::AnyJsExpression(_)));
        if !is_bodyless_arrow {
            return false;
        }
        parent
    } else {
        // The parent must be a return statement or a bodyless arrow expression
        let is_return_stmt = JsReturnStatement::can_cast(parent.kind());
        let is_bodyless_arrow = JsArrowFunctionExpression::cast_ref(&parent)
            .and_then(|arrow| arrow.body().ok())
            .is_some_and(|body| matches!(body, AnyJsFunctionBody::AnyJsExpression(_)));
        if !is_return_stmt && !is_bodyless_arrow {
            return false;
        }
        parent
    };

    // Walk up ancestors looking for typed context
    for ancestor in walk_start.ancestors().skip(1) {
        if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&ancestor)
            && arrow.return_type_annotation().is_some()
        {
            return true;
        } else if let Some(func_expr) = biome_js_syntax::JsFunctionExpression::cast_ref(&ancestor)
            && func_expr.return_type_annotation().is_some()
        {
            return true;
        } else if let Some(func_decl) = JsFunctionDeclaration::cast_ref(&ancestor)
            && func_decl.return_type_annotation().is_some()
        {
            return true;
        } else if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor) {
            return declarator.variable_annotation().is_some();
        } else if let Some(prop) = JsPropertyClassMember::cast_ref(&ancestor) {
            return prop.property_annotation().is_some();
        } else if biome_js_syntax::JsExpressionStatement::can_cast(ancestor.kind()) {
            return false;
        }
    }

    false
}

/// Checks whether the given function is a higher-order function, i.e., a function
/// that returns another function either directly in its body or as an expression.
///
/// - For bodyless arrows: checks if the body IS a function expression.
/// - For block bodies: collects ALL return statements and checks that every one
///   returns a function expression. Returns `false` if there are no returns.
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
            all_returns_are_functions(&func_body.statements())
        }
        _ => false,
    }
}

/// Checks that a function body's return statements ALL return function expressions.
/// Returns `false` if there are no return statements at all.
fn all_returns_are_functions(statements: &JsStatementList) -> bool {
    let returns = collect_return_statements(statements);
    if returns.is_empty() {
        return false;
    }
    returns.iter().all(|ret| {
        ret.argument().is_some_and(|arg| {
            matches!(
                arg,
                AnyJsExpression::JsFunctionExpression(_)
                    | AnyJsExpression::JsArrowFunctionExpression(_)
            )
        })
    })
}

/// Collects all return statements from a statement list, descending into
/// control-flow blocks (if/else, switch, block) but NOT into nested functions.
/// Uses an iterative worklist to avoid recursion.
fn collect_return_statements(statements: &JsStatementList) -> Vec<JsReturnStatement> {
    let mut returns = Vec::new();
    let mut queue: Vec<_> = statements.iter().collect();

    while let Some(stmt) = queue.pop() {
        match stmt {
            AnyJsStatement::JsReturnStatement(ret) => {
                returns.push(ret);
            }
            AnyJsStatement::JsIfStatement(if_stmt) => {
                if let Ok(consequence) = if_stmt.consequent() {
                    queue.push(consequence);
                }
                if let Some(else_clause) = if_stmt.else_clause()
                    && let Ok(alternate) = else_clause.alternate()
                {
                    queue.push(alternate);
                }
            }
            AnyJsStatement::JsBlockStatement(block) => {
                queue.extend(block.statements());
            }
            AnyJsStatement::JsSwitchStatement(switch_stmt) => {
                for case in switch_stmt.cases() {
                    match case {
                        biome_js_syntax::AnyJsSwitchClause::JsCaseClause(clause) => {
                            queue.extend(clause.consequent());
                        }
                        biome_js_syntax::AnyJsSwitchClause::JsDefaultClause(clause) => {
                            queue.extend(clause.consequent());
                        }
                    }
                }
            }
            AnyJsStatement::JsTryStatement(try_stmt) => {
                if let Ok(body) = try_stmt.body() {
                    queue.extend(body.statements());
                }
                if let Ok(catch) = try_stmt.catch_clause()
                    && let Ok(catch_body) = catch.body()
                {
                    queue.extend(catch_body.statements());
                }
            }
            AnyJsStatement::JsTryFinallyStatement(try_stmt) => {
                if let Ok(body) = try_stmt.body() {
                    queue.extend(body.statements());
                }
                if let Some(catch) = try_stmt.catch_clause()
                    && let Ok(catch_body) = catch.body()
                {
                    queue.extend(catch_body.statements());
                }
                if let Ok(finally) = try_stmt.finally_clause()
                    && let Ok(finally_body) = finally.body()
                {
                    queue.extend(finally_body.statements());
                }
            }
            AnyJsStatement::JsForStatement(stmt) => {
                if let Ok(body) = stmt.body() {
                    queue.push(body);
                }
            }
            AnyJsStatement::JsForInStatement(stmt) => {
                if let Ok(body) = stmt.body() {
                    queue.push(body);
                }
            }
            AnyJsStatement::JsForOfStatement(stmt) => {
                if let Ok(body) = stmt.body() {
                    queue.push(body);
                }
            }
            AnyJsStatement::JsWhileStatement(stmt) => {
                if let Ok(body) = stmt.body() {
                    queue.push(body);
                }
            }
            AnyJsStatement::JsDoWhileStatement(stmt) => {
                if let Ok(body) = stmt.body() {
                    queue.push(body);
                }
            }
            _ => {}
        }
    }

    returns
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

fn is_variable_declarator_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsVariableDeclarator>())
        .is_some_and(|decl| decl.variable_annotation().is_some())
}

fn is_default_function_parameter_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsFormalParameter>())
        .is_some_and(|param| param.type_annotation().is_some())
}

fn is_class_property_with_type_annotation(syntax: &SyntaxNode<JsLanguage>) -> bool {
    syntax
        .parent()
        .and_then(JsInitializerClause::cast)
        .and_then(|init| init.parent::<JsPropertyClassMember>())
        .is_some_and(|prop| prop.property_annotation().is_some())
}

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
                || is_default_function_parameter_with_type_annotation(obj_syntax)
                || is_class_property_with_type_annotation(obj_syntax)
                || is_property_of_object_with_type(obj_syntax)
        })
}

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

/// Checks if a node (like a method or getter) is inside an object expression that
/// is used as a call argument. In that case, the types are usually inferred.
fn is_member_inside_call_argument(syntax: &SyntaxNode<JsLanguage>) -> bool {
    for ancestor in syntax.ancestors() {
        if JsObjectExpression::can_cast(ancestor.kind()) {
            // Check if the object expression is inside a call argument list
            if ancestor
                .parent()
                .is_some_and(|p| p.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
            {
                return true;
            }
        }
    }
    false
}

/// Checks if a function is an expression (not a declaration).
/// A function is an expression if it's:
/// - Assigned to a variable (const foo = () => {} or const foo = function() {})
/// - A default export arrow/function expression (export default () => {})
/// - A function expression used directly
///
/// But NOT a function declaration (function foo() {}) or class method.
fn is_function_expression(func: &AnyJsFunction) -> bool {
    match func {
        AnyJsFunction::JsFunctionDeclaration(_) => false,
        AnyJsFunction::JsFunctionExportDefaultDeclaration(_) => {
            // export default function() {} is an expression context
            true
        }
        AnyJsFunction::JsArrowFunctionExpression(_) | AnyJsFunction::JsFunctionExpression(_) => {
            true
        }
    }
}

/// Gets the name of a function for allowedNames checking
fn get_function_name(func: &AnyJsFunction) -> Option<TokenText> {
    match func {
        AnyJsFunction::JsFunctionDeclaration(decl) => {
            let id = decl.id().ok()?;
            let binding = id.as_js_identifier_binding()?;
            Some(binding.name_token().ok()?.token_text_trimmed())
        }
        AnyJsFunction::JsFunctionExportDefaultDeclaration(decl) => {
            let id = decl.id()?;
            let binding = id.as_js_identifier_binding()?;
            Some(binding.name_token().ok()?.token_text_trimmed())
        }
        AnyJsFunction::JsFunctionExpression(expr) => {
            // First check the function's own name: function myFunc() {}
            if let Some(id) = expr.id()
                && let Some(binding) = id.as_js_identifier_binding()
            {
                return Some(binding.name_token().ok()?.token_text_trimmed());
            }
            // Then check if assigned to a variable: const myFunc = function() {}
            get_variable_name_from_function(func)
        }
        AnyJsFunction::JsArrowFunctionExpression(_) => {
            // Arrow functions don't have names, check the variable: const myFunc = () => {}
            get_variable_name_from_function(func)
        }
    }
}

/// Gets the variable name when a function is assigned to a variable or object property.
fn get_variable_name_from_function(func: &AnyJsFunction) -> Option<TokenText> {
    let syntax = func.syntax();

    // Check for variable declarator: const foo = () => {}
    if let Some(init_clause) = syntax.parent().and_then(JsInitializerClause::cast) {
        if let Some(declarator) = init_clause.parent::<JsVariableDeclarator>()
            && let Ok(id) = declarator.id()
            && let Some(binding) = id.as_any_js_binding()
            && let Some(ident) = binding.as_js_identifier_binding()
        {
            return Some(ident.name_token().ok()?.token_text_trimmed());
        }

        // Check for class property: arrow = () => 'arrow'
        if let Some(prop) = init_clause.parent::<JsPropertyClassMember>()
            && let Ok(name) = prop.name()
            && let Some(lit) = name.as_js_literal_member_name()
        {
            return Some(lit.value().ok()?.token_text_trimmed());
        }
    }

    // Check for object property: { fn: function() {} } or { arrowFn: () => {} }
    if let Some(prop) = syntax.parent().and_then(JsPropertyObjectMember::cast)
        && let Ok(name) = prop.name()
        && let Some(lit) = name.as_js_literal_member_name()
    {
        return Some(lit.value().ok()?.token_text_trimmed());
    }

    None
}

/// Gets the name of a class method member
fn get_method_class_member_name(method: &JsMethodClassMember) -> Option<TokenText> {
    let name = method.name().ok()?;
    let lit = name.as_js_literal_member_name()?;
    Some(lit.value().ok()?.token_text_trimmed())
}

/// Gets the name of a getter class member
fn get_getter_class_member_name(getter: &JsGetterClassMember) -> Option<TokenText> {
    let name = getter.name().ok()?;
    let lit = name.as_js_literal_member_name()?;
    Some(lit.value().ok()?.token_text_trimmed())
}

/// Gets the name of a method object member
fn get_method_object_member_name(method: &JsMethodObjectMember) -> Option<TokenText> {
    let name = method.name().ok()?;
    let lit = name.as_js_literal_member_name()?;
    Some(lit.value().ok()?.token_text_trimmed())
}

/// Gets the name of a getter object member
fn get_getter_object_member_name(getter: &JsGetterObjectMember) -> Option<TokenText> {
    let name = getter.name().ok()?;
    let lit = name.as_js_literal_member_name()?;
    Some(lit.value().ok()?.token_text_trimmed())
}

/// Checks if a name is in the allowedNames list
fn is_name_allowed(options: &UseExplicitReturnTypeOptions, name: Option<&TokenText>) -> bool {
    let Some(name) = name else {
        return false;
    };
    options
        .allowed_names
        .iter()
        .flatten()
        .any(|allowed| allowed.as_str() == name.text())
}

fn handle_any_function(
    func: &AnyJsFunction,
    options: &UseExplicitReturnTypeOptions,
    allow_expressions: bool,
    allow_iifes: bool,
) -> Option<TextRange> {
    if func.return_type_annotation().is_some() {
        return None;
    }

    // Check allowedNames
    if is_name_allowed(options, get_function_name(func).as_ref()) {
        return None;
    }

    // Check allowExpressions: skip function expressions (but not declarations)
    if allow_expressions && is_function_expression(func) {
        // allowExpressions still requires return types on:
        // - function declarations: function foo() {}
        // But not on:
        // - const foo = () => {}
        // - const foo = function() {}
        // - fn(() => {})
        // - export default () => {}
        //
        // However, variable-assigned functions and class property functions
        // are still checked when allowExpressions is true:
        // - const foo = () => {} -> still requires return type
        // - const foo = function () {} -> still requires return type
        // - class { public a = () => {} } -> still requires return type
        //
        // Only truly "expression" contexts are allowed:
        // - fn(() => {})  (callback)
        // - [() => {}]  (array element)
        // - (() => {})  (parenthesized)
        // - (function () {}) (parenthesized function)
        // - (() => {})() (IIFE)

        // Variable-assigned functions and class properties still need return types
        let syntax = func.syntax();
        let is_variable_assigned = syntax
            .parent()
            .and_then(JsInitializerClause::cast)
            .is_some_and(|init| {
                init.parent::<JsVariableDeclarator>().is_some()
                    || init.parent::<JsPropertyClassMember>().is_some()
            });

        let is_export_default =
            matches!(func, AnyJsFunction::JsFunctionExportDefaultDeclaration(_));

        if !is_variable_assigned && !is_export_default {
            return None;
        }
    }

    if is_direct_const_assertion_in_arrow_functions(func) {
        return None;
    }

    if allow_iifes && func.is_iife() {
        return None;
    }

    if is_function_used_in_argument(func) {
        return None;
    }

    if can_inline_function(func) {
        return None;
    }

    if ancestor_has_return_type(func) {
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
        return Some(TextRange::new(
            func_range.start(),
            id.syntax().text_trimmed_range().end(),
        ));
    }

    Some(func_range)
}

use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{BindingExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression, AnyJsStatement,
    JsAssignmentExpression, JsComputedMemberAssignment, JsConditionalExpression,
    JsConstructorClassMember, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsGetterClassMember,
    JsGetterObjectMember, JsInitializerClause, JsInstanceofExpression, JsMethodClassMember,
    JsMethodObjectMember, JsModule, JsNewExpression, JsNewTargetExpression, JsReferenceIdentifier,
    JsScript, JsSetterClassMember, JsSetterObjectMember, JsStaticInitializationBlockClassMember,
    JsStaticMemberAssignment, JsSyntaxKind, JsSyntaxNode, JsThisExpression, JsVariableDeclarator,
    T,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxResult, TextRange, WalkEvent,
    declare_node_union,
};
use biome_rule_options::use_arrow_function::UseArrowFunctionOptions;

declare_lint_rule! {
    /// Use arrow functions over function expressions.
    ///
    /// An arrow function expression is a compact alternative to a regular function expression,
    /// with an important distinction:
    /// `this` is not bound to the arrow function. It inherits `this` from its parent scope.
    ///
    /// This rule proposes turning all function expressions that are not generators (`function*`) and don't use `this` into arrow functions.
    ///
    /// This rule does not modify top-level function declarations ([discuss here](https://github.com/biomejs/biome/discussions/7108)).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const z = function() {
    ///     return 0;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const delegatedFetch = async function(url) {
    ///     return await fetch(url);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const f = function() {
    ///     return this.prop;
    /// }
    /// ```
    ///
    /// Named function expressions are ignored:
    ///
    /// ```js
    /// const z = function z() {
    ///     return 0;
    /// }
    /// ```
    ///
    /// Functions that reference the [arguments
    /// object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/arguments)
    /// are ignored because the arguments object is not available to arrow
    /// functions.
    ///
    /// ```js
    /// const q = function () {
    ///     return arguments[0];
    /// }
    /// ```
    ///
    /// Function expressions whose `prototype` is used are ignored,
    /// because arrow functions don't have a `prototype`:
    ///
    /// ```js
    /// const f = function () {};
    /// console.log(f.prototype.toString());
    /// ```
    ///
    /// Function expressions that declare the type of `this` are  also ignored:
    ///
    /// ```ts
    /// const z = function(this: A): number {
    ///     return 0;
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - [useConsistentFunctionStyle](https://biomejs.dev/linter/rules/use-consistent-function-style)
    ///
    pub UseArrowFunction {
        version: "1.0.0",
        name: "useArrowFunction",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-arrow-callback").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseArrowFunction {
    type Query = Semantic<AnyThisScope>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseArrowFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let scope = ctx.query();
        let model = ctx.model();
        let root = scope.syntax();
        let mut preorder = root.preorder_of_kinds(
            AnyThisScope::KIND_SET
                .union(JsThisExpression::KIND_SET)
                .union(JsNewTargetExpression::KIND_SET)
                .union(JsReferenceIdentifier::KIND_SET),
        );

        let AnyThisScope::JsFunctionExpression(function_expression) = scope else {
            return None;
        };
        if function_expression.star_token().is_some() || function_expression.id().is_some() {
            // Ignore generators and function with a name.
            return None;
        }
        let has_this_parameter = function_expression
            .parameters()
            .ok()?
            .items()
            .iter()
            .nth(0)
            .and_then(|param| param.ok())
            .is_some_and(|param| param.as_ts_this_parameter().is_some());
        if has_this_parameter {
            // Ignore functions that explicitly declare a `this` type.
            return None;
        }
        let requires_prototype =
            value_parent(function_expression.syntax()).is_some_and(|ancestor| {
                matches!(
                    ancestor.kind(),
                    JsSyntaxKind::JS_NEW_EXPRESSION | JsSyntaxKind::JS_EXTENDS_CLAUSE
                ) || (function_expression.async_token().is_none()
                    && is_prototype_of_binding_used(&ancestor, model))
            });
        if requires_prototype {
            // Ignore cases where a prototype is required
            return None;
        }

        while let Some(event) = preorder.next() {
            match event {
                WalkEvent::Enter(node) => {
                    if node != *root
                        && let Some(_) = AnyThisScope::cast_ref(&node)
                    {
                        // Stop crawling when we hit another function scope.
                        preorder.skip_subtree();
                    } else if matches!(
                        node.kind(),
                        JsSyntaxKind::JS_THIS_EXPRESSION | JsSyntaxKind::JS_NEW_TARGET_EXPRESSION
                    ) {
                        // If this function contains a this expression, it is not convertible
                        // into an arrow function
                        return None;
                    }

                    if let Some(reference) = JsReferenceIdentifier::cast_ref(&node)
                        && reference.name().is_ok_and(|name| name == "arguments")
                        && reference.binding(model).is_none()
                    {
                        // This method refers to arguments, and arguments has not
                        // been reassigned as a variable (which is only permitted in
                        // script mode, not module mode)
                        return None;
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        Some(())
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        let node = ctx.query();
        let scope = node;
        let AnyThisScope::JsFunctionExpression(function_expression) = scope else {
            return None;
        };
        Some(function_expression.function_token().ok()?.text_range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"function expression"</Emphasis>" can be turned into an "<Emphasis>"arrow function"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>"Function expressions"</Emphasis>" that don't use "<Emphasis>"this"</Emphasis>" can be turned into "<Emphasis>"arrow functions"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let scope = ctx.query();
        let AnyThisScope::JsFunctionExpression(function_expression) = scope else {
            return None;
        };
        let mut arrow_function_builder = make::js_arrow_function_expression(
            function_expression.parameters().ok()?.into(),
            make::token(T![=>]).with_trailing_space(),
            to_arrow_body(function_expression.body().ok()?),
        );
        if let Some(async_token) = function_expression.async_token() {
            arrow_function_builder = arrow_function_builder.with_async_token(async_token);
        }
        if let Some(type_parameters) = function_expression.type_parameters() {
            let mut type_parameters_iter =
                type_parameters.items().iter().filter_map(|item| item.ok());
            let type_parameter = type_parameters_iter.next();
            // Keep a trailing comma when there is a single type parameter in arrow functions and JSX is enabled
            // Or the parser will treat it as a JSX tag and fail to parse it.
            let type_parameters = if type_parameter.is_some()
                && type_parameters_iter.next().is_none()
                && ctx.source_type::<JsFileSource>().is_jsx()
            {
                make::ts_type_parameters(
                    make::token(T![<]),
                    make::ts_type_parameter_list(type_parameter, Some(make::token(T![,]))),
                    make::token(T![>]),
                )
            } else {
                type_parameters
            };
            arrow_function_builder = arrow_function_builder.with_type_parameters(type_parameters);
        }
        if let Some(return_type_annotation) = function_expression.return_type_annotation() {
            arrow_function_builder =
                arrow_function_builder.with_return_type_annotation(return_type_annotation);
        }
        let arrow_function = arrow_function_builder.build();
        let arrow_function = if needs_parentheses(function_expression) {
            AnyJsExpression::from(make::parenthesized(arrow_function.trim_trailing_trivia()?))
        } else {
            AnyJsExpression::from(arrow_function)
        };
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::from(function_expression.clone()),
            arrow_function,
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an "<Emphasis>"arrow function"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the first ancestor of `node` that uses its value, skipping the nodes that pass it on
/// unchanged: parentheses, TypeScript type-only wrappers, the branches of a conditional
/// expression, and the operands of a logical expression.
///
/// e.g. the initializer clause in `const f = cond ? function() {} : g;`
fn value_parent(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    let mut node = node.clone();
    loop {
        let parent = node.parent()?;
        let passes_value = match parent.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_INSTANTIATION_EXPRESSION
            | JsSyntaxKind::JS_LOGICAL_EXPRESSION => true,
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => JsConditionalExpression::cast_ref(&parent)
                .is_some_and(|conditional| {
                    conditional.test().is_ok_and(|test| test.syntax() != &node)
                }),
            _ => false,
        };
        if !passes_value {
            return Some(parent);
        }
        node = parent;
    }
}

/// Returns `true` if `parent` binds a function expression to a variable that is used
/// in a way that requires a `prototype`.
///
/// e.g. `const f = function() {}; f.prototype.x = 0;`
fn is_prototype_of_binding_used(parent: &JsSyntaxNode, model: &SemanticModel) -> bool {
    let binding = if let Some(initializer) = JsInitializerClause::cast_ref(parent) {
        let Some(declarator) = initializer.parent::<JsVariableDeclarator>() else {
            return false;
        };
        let Some(id) = declarator
            .id()
            .ok()
            .and_then(|id| id.as_any_js_binding()?.as_js_identifier_binding().cloned())
        else {
            return false;
        };
        model.as_binding(&id)
    } else if let Some(assignment) = JsAssignmentExpression::cast_ref(parent) {
        let Some(binding) = assignment.left().ok().and_then(|left| {
            left.as_any_js_assignment()?
                .as_js_identifier_assignment()?
                .binding(model)
        }) else {
            return false;
        };
        binding
    } else {
        return false;
    };
    binding
        .all_reads()
        .any(|reference| is_prototype_required_by(&reference.syntax()))
}

/// Returns `true` if the reference `reference` is used in a way that requires a `prototype`:
/// `f.prototype`, `f["prototype"]`, `new f()`, `class extends f {}`, or `x instanceof f`.
///
/// Parentheses and TypeScript type-only wrappers around the reference are ignored,
/// e.g. `new (f as any)()`.
fn is_prototype_required_by(reference: &JsSyntaxNode) -> bool {
    let Some(expression) = reference.parent() else {
        return false;
    };
    let expression = expression
        .ancestors()
        .take_while(|node| {
            node == &expression
                || matches!(
                    node.kind(),
                    JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                        | JsSyntaxKind::TS_AS_EXPRESSION
                        | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                        | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                        | JsSyntaxKind::TS_INSTANTIATION_EXPRESSION
                )
        })
        .last()
        .unwrap_or(expression);
    let Some(parent) = expression.parent() else {
        return false;
    };
    let is_expression =
        |node: SyntaxResult<AnyJsExpression>| node.is_ok_and(|node| node.syntax() == &expression);
    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE => true,
        JsSyntaxKind::JS_NEW_EXPRESSION => {
            JsNewExpression::cast(parent).is_some_and(|new| is_expression(new.callee()))
        }
        JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => JsInstanceofExpression::cast(parent)
            .is_some_and(|instanceof| is_expression(instanceof.right())),
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            AnyJsMemberExpression::cast(parent).is_some_and(|member| {
                is_expression(member.object())
                    && member
                        .member_name()
                        .is_some_and(|name| name.text() == "prototype")
            })
        }
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => JsStaticMemberAssignment::cast(parent)
            .is_some_and(|member| {
                is_expression(member.object())
                    && member.member().is_ok_and(|name| {
                        name.as_js_name()
                            .and_then(|name| name.value_token().ok())
                            .is_some_and(|token| token.text_trimmed() == "prototype")
                    })
            }),
        JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => JsComputedMemberAssignment::cast(parent)
            .is_some_and(|member| {
                is_expression(member.object())
                    && member.member().is_ok_and(|name| {
                        name.omit_parentheses()
                            .as_static_value()
                            .is_some_and(|name| name.text() == "prototype")
                    })
            }),
        _ => false,
    }
}

/// Returns `true` if `function_expression` needs parenthesis when turned into an arrow function.
fn needs_parentheses(function_expression: &JsFunctionExpression) -> bool {
    function_expression.syntax().parent().is_some_and(|parent| {
        // Copied from the implementation of `NeedsParentheses` for `JsArrowFunctionExpression`
        // in the `biome_js_formatter` crate.
        // TODO: Should `NeedsParentheses` be moved in `biome_js_syntax`?
        matches!(
            parent.kind(),
            JsSyntaxKind::TS_AS_EXPRESSION
                    | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                    | JsSyntaxKind::JS_UNARY_EXPRESSION
                    | JsSyntaxKind::JS_AWAIT_EXPRESSION
                    | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                    // Conditional expression
                    // NOTE: parens are only needed when the arrow function appears in the test.
                    // To simplify we always add parens.
                    | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                    // Lower expression
                    | JsSyntaxKind::JS_EXTENDS_CLAUSE
                    | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                    // Callee
                    | JsSyntaxKind::JS_CALL_EXPRESSION
                    | JsSyntaxKind::JS_NEW_EXPRESSION
                    // Member-like
                    | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                    | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
                    // Template tag
                    | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
                    // Binary-like
                    | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                    | JsSyntaxKind::JS_BINARY_EXPRESSION
                    | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
                    | JsSyntaxKind::JS_IN_EXPRESSION
        )
    })
}

declare_node_union! {
    pub AnyThisScope =
        JsConstructorClassMember
        | JsFunctionExpression
        | JsFunctionDeclaration
        | JsFunctionExportDefaultDeclaration
        | JsGetterClassMember
        | JsGetterObjectMember
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsModule
        | JsScript
        | JsSetterClassMember
        | JsSetterObjectMember
        | JsStaticInitializationBlockClassMember
}

/// Get a minimal arrow function body from a regular function body.
fn to_arrow_body(body: JsFunctionBody) -> AnyJsFunctionBody {
    let directives = body.directives();
    let body_statements = body.statements();
    let early_result = AnyJsFunctionBody::from(body);
    if !directives.is_empty() {
        // The function body has at least one directive.
        // e.g. `function() { "directive"; return 0; }`
        return early_result;
    }
    let Some(AnyJsStatement::JsReturnStatement(return_statement)) = body_statements.iter().next()
    else {
        return early_result;
    };
    let Some(return_arg) = return_statement.argument() else {
        return early_result;
    };
    if body_statements.syntax().has_comments_direct()
        || return_statement.syntax().has_comments_direct()
        || return_arg.syntax().has_comments_direct()
    {
        // To keep comments, we keep the regular function body
        return early_result;
    }
    if matches!(return_arg, AnyJsExpression::JsSequenceExpression(_))
        || return_arg
            .syntax()
            .first_token()
            .is_some_and(|token| token.kind() == T!['{'])
    {
        // () => (first, second)
        // () => ({ ... })
        return AnyJsFunctionBody::AnyJsExpression(make::parenthesized(return_arg).into());
    }
    // () => expression
    AnyJsFunctionBody::AnyJsExpression(return_arg)
}

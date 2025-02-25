use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, FixKind, Phases, QueryMatch, Queryable,
    Rule, RuleDiagnostic, RuleSource, RuleSourceKind, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, JsConstructorClassMember, JsFileSource,
    JsFunctionBody, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember,
    JsSetterObjectMember, JsStaticInitializationBlockClassMember, JsSyntaxKind, T,
};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Language,
    SyntaxNode, TextRange, TriviaPieceKind, WalkEvent,
};

declare_lint_rule! {
    /// Use arrow functions over function expressions.
    ///
    /// An arrow function expression is a compact alternative to a regular function expression,
    /// with an important distinction:
    /// `this` is not bound to the arrow function. It inherits `this` from its parent scope.
    ///
    /// This rule proposes turning all function expressions that are not generators (`function*`) and don't use `this` into arrow functions.
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
    /// Function expressions that declare the type of `this` are  also ignored:
    ///
    /// ```ts
    /// const z = function(this: A): number {
    ///     return 0;
    /// }
    /// ```
    pub UseArrowFunction {
        version: "1.0.0",
        name: "useArrowFunction",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-arrow-callback")],
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseArrowFunction {
    type Query = ActualThisScope;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let AnyThisScopeMetadata { scope, has_this } = ctx.query();
        if *has_this {
            return None;
        }
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
        let requires_prototype = function_expression
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
            .is_some_and(|ancestor| {
                matches!(
                    ancestor.kind(),
                    JsSyntaxKind::JS_NEW_EXPRESSION | JsSyntaxKind::JS_EXTENDS_CLAUSE
                )
            });
        if requires_prototype {
            // Ignore cases where a prototype is required
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().scope.range(),
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
        let AnyThisScopeMetadata { scope, .. } = ctx.query();
        let AnyThisScope::JsFunctionExpression(function_expression) = scope else {
            return None;
        };
        let mut arrow_function_builder = make::js_arrow_function_expression(
            function_expression.parameters().ok()?.into(),
            make::token(T![=>]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
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

#[derive(Debug, Clone)]
pub struct AnyThisScopeMetadata {
    scope: AnyThisScope,
    has_this: bool,
}

pub struct ActualThisScope(AnyThisScopeMetadata);

impl QueryMatch for ActualThisScope {
    fn text_range(&self) -> TextRange {
        self.0.scope.range()
    }
}

impl Queryable for ActualThisScope {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyThisScopeMetadata;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, AnyThisScopeVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

#[derive(Default)]
struct AnyThisScopeVisitor {
    /// Vector to hold a function or block where `this` is scoped.
    /// The function or block is associated to a boolean indicating whether it contains `this`.
    stack: Vec<AnyThisScopeMetadata>,
}

impl Visitor for AnyThisScopeVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(scope) = AnyThisScope::cast_ref(node) {
                    self.stack.push(AnyThisScopeMetadata {
                        scope,
                        has_this: false,
                    });
                } else if matches!(
                    node.kind(),
                    JsSyntaxKind::JS_THIS_EXPRESSION | JsSyntaxKind::JS_NEW_TARGET_EXPRESSION
                ) {
                    // When the visitor enters a `this` expression or `new.target`,
                    // set the `has_this` flag for the top entry on the stack to `true`.
                    if let Some(AnyThisScopeMetadata { has_this, .. }) = self.stack.last_mut() {
                        *has_this = true;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if AnyThisScope::can_cast(node.kind()) {
                    if let Some(scope_metadata) = self.stack.pop() {
                        ctx.match_query(ActualThisScope(scope_metadata));
                    }
                }
            }
        }
    }
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
    if matches!(
        return_arg,
        AnyJsExpression::JsSequenceExpression(_) | AnyJsExpression::JsObjectExpression(_)
    ) {
        // () => (first, second)
        // () => ({ ... })
        return AnyJsFunctionBody::AnyJsExpression(make::parenthesized(return_arg).into());
    }
    // () => expression
    AnyJsFunctionBody::AnyJsExpression(return_arg)
}

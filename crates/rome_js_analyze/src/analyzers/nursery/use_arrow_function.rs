use crate::JsRuleAction;
use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, AddVisitor, Phases, QueryMatch, Queryable,
    Rule, RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, JsConstructorClassMember, JsFunctionBody,
    JsFunctionDeclaration, JsFunctionExportDefaultDeclaration, JsFunctionExpression,
    JsGetterClassMember, JsGetterObjectMember, JsLanguage, JsMethodClassMember,
    JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember, JsSetterObjectMember,
    JsStaticInitializationBlockClassMember, JsThisExpression, T,
};
use rome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Language,
    SyntaxNode, TextRange, TriviaPieceKind, WalkEvent,
};

declare_rule! {
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
    /// ## Valid
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
    pub(crate) UseArrowFunction {
        version: "1.0.0",
        name: "useArrowFunction",
        recommended: true,
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
        let has_this_parameter =
            function_expression
                .parameters()
                .ok()?
                .items()
                .iter()
                .any(|param| {
                    param
                        .map(|param| param.as_ts_this_parameter().is_some())
                        .unwrap_or_default()
                });
        if has_this_parameter {
            // Ignore functions that explicitly declare a `this` type.
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
            arrow_function_builder = arrow_function_builder.with_type_parameters(type_parameters);
        }
        if let Some(return_type_annotation) = function_expression.return_type_annotation() {
            arrow_function_builder =
                arrow_function_builder.with_return_type_annotation(return_type_annotation);
        }
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsFunctionExpression(function_expression.clone()),
            AnyJsExpression::JsArrowFunctionExpression(arrow_function_builder.build()),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Use an "<Emphasis>"arrow function"</Emphasis>" instead." }
                .to_owned(),
            mutation,
        })
    }
}

declare_node_union! {
    pub(crate) AnyThisScope =
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
pub(crate) struct AnyThisScopeMetadata {
    scope: AnyThisScope,
    has_this: bool,
}

pub(crate) struct ActualThisScope(AnyThisScopeMetadata);

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
                }
                if JsThisExpression::can_cast(node.kind()) {
                    // When the visitor enters a `this` expression, set the
                    // `has_this` flag for the top entry on the stack to `true`
                    if let Some(AnyThisScopeMetadata { has_this, .. }) = self.stack.last_mut() {
                        *has_this = true;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(exit_scope) = AnyThisScope::cast_ref(node) {
                    if let Some(scope_metadata) = self.stack.pop() {
                        if scope_metadata.scope == exit_scope {
                            ctx.match_query(ActualThisScope(scope_metadata));
                        }
                    }
                }
            }
        }
    }
}

/// Get a minimal arrow function body from a regular function body.
fn to_arrow_body(body: JsFunctionBody) -> AnyJsFunctionBody {
    let body_statements = body.statements();
    // () => { ... }
    let mut result = AnyJsFunctionBody::from(body);
    let Some(AnyJsStatement::JsReturnStatement(return_statement)) = body_statements.iter().next()
    else {
        return result;
    };
    let Some(return_arg) = return_statement.argument() else {
        return result;
    };
    if body_statements.syntax().has_comments_direct()
        || return_statement.syntax().has_comments_direct()
        || return_arg.syntax().has_comments_direct()
    {
        // To keep comments, we keep the regular function body
        return result;
    }
    // () => expression
    result = AnyJsFunctionBody::AnyJsExpression(return_arg.clone());
    let Some(first_token) = return_arg.syntax().first_token() else {
        return result;
    };
    if first_token.kind() == T!['{'] {
        // () => ({ ... })
        result = AnyJsFunctionBody::AnyJsExpression(
            make::js_parenthesized_expression(
                make::token(T!['(']),
                return_arg,
                make::token(T![')']),
            )
            .into(),
        );
    }
    result
}

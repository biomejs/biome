use crate::JsRuleAction;
use biome_analyze::RuleSource;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsLiteralExpression;
use biome_js_syntax::{
    AnyJsExpression, AnyTsPropertyAnnotation, AnyTsVariableAnnotation, JsFormalParameter,
    JsInitializerClause, JsPropertyClassMember, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, TsPropertyParameter, TsReadonlyModifier, TsTypeAnnotation,
};
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use biome_rule_options::no_inferrable_types::NoInferrableTypesOptions;

declare_lint_rule! {
    /// Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
    ///
    /// TypeScript is able to infer the types of parameters, properties, and variables from their default or initial values.
    /// There is no need to use an explicit `:` type annotation for trivially inferred types (boolean, bigint, number, regex, string).
    /// Doing so adds unnecessary verbosity to code making it harder to read.
    ///
    /// In contrast to ESLint's rule, this rule allows to use a wide type for `const` declarations.
    /// Moreover, the rule does not recognize `undefined` values, primitive type constructors (String, Number, ...), and `RegExp` type.
    /// These global variables could be shadowed by local ones.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// const variable: 1 = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let variable: number = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class SomeClass {
    ///   readonly field: 1 = 1;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class SomeClass {
    ///   field: number = 1;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(param: number = 1): void {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const variable: number = 1;
    /// ```
    ///
    /// ```ts
    /// let variable: 1 | 2 = 1;
    /// ```
    ///
    /// ```ts
    /// class SomeClass {
    ///   readonly field: number = 1;
    /// }
    /// ```
    ///
    /// ```ts
    /// // `undefined` could be shadowed
    /// const variable: undefined = undefined;
    /// ```
    ///
    /// ```ts
    /// // `RegExp` could be shadowed
    /// const variable: RegExp = /a/;
    /// ```
    ///
    /// ```ts
    /// // `String` could be shadowed
    /// let variable: string = String(5);
    /// ```
    ///
    /// ```ts
    /// class SomeClass {
    ///   field: 1 | 2 = 1;
    /// }
    /// ```
    ///
    /// ```ts
    /// function f(param: 1 | 2 = 1): void {}
    /// ```
    ///
    pub NoInferrableTypes {
        version: "1.0.0",
        name: "noInferrableTypes",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-inferrable-types").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoInferrableTypes {
    type Query = Ast<JsInitializerClause>;
    type State = TsTypeAnnotation;
    type Signals = Option<Self::State>;
    type Options = NoInferrableTypesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let init = ctx.query();
        let init_expr = init.expression().ok()?.omit_parentheses();
        if init_expr.has_trivially_inferrable_type() {
            // `is_const` signals a const context (const declarations, readonly properties)
            // non const contexts are other situations (let/var declarations, mutable properties, formal parameters)
            let mut is_const = false;
            let mut type_annotation = None;
            if let Some(param) = init.parent::<JsFormalParameter>() {
                if let Some(prop_param) = param.parent::<TsPropertyParameter>() {
                    is_const = prop_param
                        .modifiers()
                        .into_iter()
                        .any(|x| TsReadonlyModifier::can_cast(x.syntax().kind()));
                }
                type_annotation = param.type_annotation();
            } else if let Some(prop) = init.parent::<JsPropertyClassMember>() {
                is_const = prop
                    .modifiers()
                    .into_iter()
                    .any(|x| TsReadonlyModifier::can_cast(x.syntax().kind()));
                type_annotation = match prop.property_annotation()? {
                    AnyTsPropertyAnnotation::TsTypeAnnotation(annotation) => Some(annotation),
                    _ => None,
                };
            } else if let Some(declarator) = init.parent::<JsVariableDeclarator>() {
                is_const = declarator
                    .parent::<JsVariableDeclaratorList>()?
                    .parent::<JsVariableDeclaration>()?
                    .is_const();
                type_annotation = match declarator.variable_annotation()? {
                    AnyTsVariableAnnotation::TsTypeAnnotation(annotation) => Some(annotation),
                    _ => None,
                };
            }
            if let Some(type_annotation) = type_annotation {
                let ty = type_annotation.ty().ok()?.omit_parentheses();
                // In const contexts, literal type annotations are rejected.
                // e.g. `const x: 1 = <literal>`
                //
                // However, we ignore `null` and `undefined` literal types,
                // because in unsafe null mode, TypeScript widen an unannotated variable to `any`.
                //
                // In non-const contexts, wide type annotation are rejected.
                // e.g. `let x: number = <literal>`
                //
                // However, we ignore the case where <literal> is `null`,
                // because in unsafe null mode, it is possible to assign `null` and `undefined` to any type.
                if (is_const && ty.is_non_null_literal_type())
                    || (!is_const
                        && ty.is_primitive_type()
                        && !matches!(
                            init_expr,
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsNullLiteralExpression(_)
                            )
                        ))
                {
                    return Some(type_annotation);
                }
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, annotation: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            annotation.range(),
            markup! {
                "This type annotation is trivially inferred from its initialization."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, annotation: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let first_token = annotation.syntax().first_token()?;
        let prev_token = first_token.prev_token()?;
        let new_prev_token = prev_token.append_trivia_pieces(first_token.leading_trivia().pieces());
        let last_token = annotation.syntax().last_token()?;
        let next_token = last_token.next_token()?;
        let new_next_token =
            next_token.prepend_trivia_pieces(last_token.trailing_trivia().pieces());
        mutation.replace_token_discard_trivia(prev_token, new_prev_token);
        mutation.replace_token_discard_trivia(next_token, new_next_token);
        mutation.remove_node(annotation.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the type annotation." }.to_owned(),
            mutation,
        ))
    }
}

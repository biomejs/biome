use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, TsNonNullAssertionAssignment, TsNonNullAssertionExpression, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow non-null assertions using the `!` postfix operator.
    ///
    /// TypeScript's `!` non-null assertion operator asserts to the type system that an expression is non-nullable, as
    /// in not `null` or `undefined`. Using assertions to tell the type system new information is often a sign that
    /// code is not fully type-safe. It's generally better to structure program logic so that TypeScript understands
    /// when values may be nullable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Example {
    ///   property?: string;
    /// }
    /// declare const foo: Example;
    /// const includesBaz = foo.property!.includes('baz');
    /// ```
    /// ```ts,expect_diagnostic
    /// (b!! as number) = "test";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Example {
    ///   property?: string;
    /// }
    ///
    /// declare const foo: Example;
    /// const includesBaz = foo.property?.includes('baz') ?? false;
    /// ```
    ///
    pub NoNonNullAssertion {
        version: "1.0.0",
        name: "noNonNullAssertion",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-non-null-assertion")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyTsNonNullAssertion = TsNonNullAssertionExpression | TsNonNullAssertionAssignment
}

impl Rule for NoNonNullAssertion {
    type Query = Ast<AnyTsNonNullAssertion>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(node) => node
                .parent::<TsNonNullAssertionExpression>()
                .map_or(Some(()), |_| None),
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(node) => node
                .parent::<TsNonNullAssertionAssignment>()
                .map_or(Some(()), |_| None),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Forbidden non-null assertion."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        match node {
            AnyTsNonNullAssertion::TsNonNullAssertionAssignment(_) => None,
            AnyTsNonNullAssertion::TsNonNullAssertionExpression(node) => {
                let mut mutation = ctx.root().begin();

                // get the expression without assertion marker
                // the loop handles repetitive (useless) assertion marker such as `expr!!!`.
                let mut expr = node.expression();
                while let Ok(AnyJsExpression::TsNonNullAssertionExpression(assertion)) = expr {
                    expr = assertion.expression()
                }
                let assertion_less_expr = expr.ok()?;
                let old_node = AnyJsExpression::TsNonNullAssertionExpression(node.clone());

                match node.parent::<AnyJsExpression>()? {
                    AnyJsExpression::JsComputedMemberExpression(parent)
                        if parent.object().is_ok_and(|object| {
                            object
                                .as_ts_non_null_assertion_expression()
                                .is_some_and(|object| object == node)
                        }) =>
                    {
                        if parent.is_optional() {
                            // object!?["prop"] --> object?.["prop"]
                            mutation.replace_node(old_node, assertion_less_expr);
                        } else {
                            // object!["prop"] --> object?["prop"]
                            let new_parent = parent
                                .clone()
                                .with_optional_chain_token(Some(make::token(T![?.])))
                                .with_object(assertion_less_expr);
                            mutation.replace_node(parent, new_parent);
                        }
                    }
                    AnyJsExpression::JsCallExpression(parent) => {
                        if parent.is_optional() {
                            // f!?() --> f?()
                            mutation.replace_node(old_node, assertion_less_expr);
                        } else {
                            // f!() --> f?.()
                            let new_parent = parent
                                .clone()
                                .with_optional_chain_token(Some(make::token(T![?.])))
                                .with_callee(assertion_less_expr);
                            mutation.replace_node(parent, new_parent);
                        }
                    }
                    AnyJsExpression::JsStaticMemberExpression(parent) => {
                        if parent.is_optional() {
                            // object!?.prop --> object?.prop
                            mutation.replace_node(old_node, assertion_less_expr);
                        } else {
                            // object!.prop --> object?.prop
                            let new_parent = parent
                                .clone()
                                .with_operator_token_token(make::token(T![?.]))
                                .with_object(assertion_less_expr);
                            mutation.replace_node(parent, new_parent);
                        }
                    }
                    _ => {
                        // unsupported
                        return None;
                    }
                };

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                     markup! { "Replace with optional chain operator "<Emphasis>"?."</Emphasis>" This operator includes runtime checks, so it is safer than the compile-only non-null assertion operator" }
                        .to_owned(),
                    mutation,
                ))
            }
        }
    }
}

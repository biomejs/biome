use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;

use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsType, JsFileSource, JsSyntaxKind, TsTypeConstraintClause, TsTypeParameter,
    TsTypeParameterList, T,
};
use biome_rowan::{
    trim_leading_trivia_pieces, AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeOptionExt,
};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow using `any` or `unknown` as type constraint.
    ///
    /// Generic type parameters (`<T>`) in TypeScript may be **constrained** with [`extends`](https://www.typescriptlang.org/docs/handbook/generics.html#generic-constraints).
    /// A supplied type must then be a subtype of the supplied constraint.
    /// All types are subtypes of `any` and `unknown`.
    /// It is thus useless to extend from `any` or `unknown`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface FooAny<T extends any> {}
    /// ```
    /// ```ts,expect_diagnostic
    /// type BarAny<T extends any> = {};
    /// ```
    /// ```ts,expect_diagnostic
    /// class BazAny<T extends any> {
    /// }
    /// ```
    /// ```ts,expect_diagnostic
    /// class BazAny {
    ///   quxAny<U extends any>() {}
    /// }
    /// ```
    /// ```ts,expect_diagnostic
    /// const QuuxAny = <T extends any>() => {};
    /// ```
    /// ```ts,expect_diagnostic
    /// function QuuzAny<T extends any>() {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface FooUnknown<T extends unknown> {}
    /// ```
    /// ```ts,expect_diagnostic
    /// type BarUnknown<T extends unknown> = {};
    /// ```
    /// ```ts,expect_diagnostic
    /// class BazUnknown<T extends unknown> {
    /// }
    /// ```ts,expect_diagnostic
    /// class BazUnknown {
    ///   quxUnknown<U extends unknown>() {}
    /// }
    /// ```
    /// ```ts,expect_diagnostic
    /// const QuuxUnknown = <T extends unknown>() => {};
    /// ```
    /// ```ts,expect_diagnostic
    /// function QuuzUnknown<T extends unknown>() {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Foo<T> {}
    ///
    /// type Bar<T> = {};
    ///```
    pub NoUselessTypeConstraint {
        version: "1.0.0",
        name: "noUselessTypeConstraint",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-type-constraint")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessTypeConstraint {
    type Query = Ast<TsTypeConstraintClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let ty = node.ty().ok()?;
        matches!(ty, AnyTsType::TsAnyType(_) | AnyTsType::TsUnknownType(_)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Constraining a type parameter to "<Emphasis>"any"</Emphasis>" or "<Emphasis>"unknown"</Emphasis>" is useless."
                },
            )
            .note(markup! {
                "All types are subtypes of "<Emphasis>"any"</Emphasis>" and "<Emphasis>"unknown"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        // If the parent is a type param without default type
        if let Some(type_param) = node
            .parent::<TsTypeParameter>()
            .filter(|type_param| type_param.default().is_none())
        {
            let type_params = type_param.parent::<TsTypeParameterList>()?;
            if type_params.len() == 1
                && type_params.trailing_separator().is_none()
                && !ctx.source_type::<JsFileSource>().variant().is_standard()
                && type_params.syntax().grand_parent().kind()
                    == Some(JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
            {
                let name = type_param.name().ok()?;
                let trailing_pieces = name.syntax().last_trailing_trivia()?.pieces();
                let new_name =
                    name.with_trailing_trivia_pieces(trim_leading_trivia_pieces(trailing_pieces))?;
                let new_type_param = type_param.with_name(new_name).with_constraint(None);
                // Add a trailing comma to disambiguate JSX and arrow functions
                let new_type_params =
                    make::ts_type_parameter_list([new_type_param], [make::token(T![,])]);
                mutation.replace_node(type_params, new_type_params);
            } else {
                let prev = node.syntax().prev_sibling()?;
                // Remove the extra space
                mutation.replace_element_discard_trivia(
                    prev.clone().into(),
                    prev.trim_trailing_trivia()?.into(),
                );
                mutation.remove_node(node.clone());
            }
        } else {
            mutation.remove_node(node.clone());
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the constraint." }.to_owned(),
            mutation,
        ))
    }
}

use biome_analyze::Ast;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsSyntaxKind, TsTypeParameters};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeOptionExt};

declare_lint_rule! {
    /// Disallow empty type parameters in type aliases and interfaces.
    ///
    /// TypeScript permits the use of empty type parameter lists in type alias and interface declarations; however, this practice is generally discouraged.
    /// Allowing empty type parameter lists can lead to unclear or ambiguous code, where the intention of the generic type is not self-evident.
    /// This rule disallows empty type parameter lists in type alias and interface declarations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Foo<> {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Bar<> = {};
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Foo {}
    /// ```
    ///
    /// ```ts
    /// type Foo<T> = {
    ///  bar: T;
    /// }
    /// ```
    pub NoEmptyTypeParameters {
        version: "1.5.0",
        name: "noEmptyTypeParameters",
        language: "ts",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoEmptyTypeParameters {
    type Query = Ast<TsTypeParameters>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        (node.items().is_empty()
            // We only handle interfaces and type aliases where an empty type parameter list is allowed.
            // other empty type parameter lists are parse errors.
            && matches!(
                node.syntax().parent().kind(),
                Some(
                    JsSyntaxKind::TS_INTERFACE_DECLARATION
                        | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
                )
            ))
        .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {"Using an "<Emphasis>"empty type parameter list"</Emphasis>" is confusing."},
        ).note("Remove the empty type parameter list or add a type parameter."))
    }
}

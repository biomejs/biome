use biome_analyze::Ast;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsSyntaxKind, TextRange, TsTypeParameters};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeOptionExt};

declare_rule! {
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
    /// ## Valid
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
    ///
    pub(crate) NoEmptyTypeParameters {
        version: "1.5.0",
        name: "noEmptyTypeParameters",
        recommended: true,
    }
}

impl Rule for NoEmptyTypeParameters {
    type Query = Ast<TsTypeParameters>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.items().is_empty()
            && matches!(
                node.syntax().parent().kind(),
                Some(
                    JsSyntaxKind::TS_INTERFACE_DECLARATION
                        | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
                )
            )
        {
            return Some(node.items().range());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            reference,
            markup! {"Empty type parameter list "<Emphasis>"<>"</Emphasis>" is not recommended"},
        ))
    }
}

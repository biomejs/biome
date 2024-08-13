use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::TsEnumDeclaration;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow TypeScript enum.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// enum Foo {
    ///     BAR = 'bar'
    /// }
    /// ```
    ///
    pub NoEnum {
        version: "next",
        name: "noEnum",
        language: "ts",
        recommended: false,
    }
}

impl Rule for NoEnum {
    type Query = Ast<TsEnumDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"enum"</Emphasis>
                },
            )
            .note(markup! {
                "Usage of "<Emphasis>"enum"</Emphasis>" is disallowed."
            }),
        )
    }
}

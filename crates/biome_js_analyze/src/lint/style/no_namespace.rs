use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::TsModuleDeclaration;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of TypeScript's `namespace`s.
    ///
    /// Namespaces are an old way to organize your code in TypeScript.
    /// They are not recommended anymore and should be replaced by ES6 modules
    /// (the `import`/`export` syntax).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// module foo {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare module foo {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// namespace foo {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare namespace foo {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// import foo from 'foo';
    /// export { bar };
    /// ```
    ///
    /// ```ts
    /// declare global {}
    /// ```
    ///
    /// ```ts
    /// declare module 'foo' {}
    /// ```
    ///
    pub NoNamespace {
        version: "1.0.0",
        name: "noNamespace",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-namespace")],
        recommended: false,
    }
}

impl Rule for NoNamespace {
    type Query = Ast<TsModuleDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "TypeScript's namespaces are an outdated way to organize code."
                },
            )
            .note(markup! {
                "Prefer the ES6 modules (import/export) over namespaces."
            }),
        )
    }
}

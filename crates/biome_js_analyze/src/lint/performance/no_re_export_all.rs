use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::JsExportFromClause;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Avoid re-export all.
    ///
    /// Deeply nested import chains in modular projects, where a barrel file imports another barrel file, can lead to increased load times and complexity.
    /// This structure results in the unnecessary loading of many modules, significantly impacting performance in large-scale applications.
    /// Additionally, it complicates the codebase, making it difficult to navigate and understand the project's dependency graph.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export * from "foo";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export * as foo from "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export { foo } from "foo";
    /// ```
    ///
    /// ```ts
    /// export type * from "foo";
    /// export type * as bar from "bar";
    /// ```
    ///
    pub NoReExportAll {
        version: "1.6.0",
        name: "noReExportAll",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintBarrelFiles("avoid-re-export-all")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for NoReExportAll {
    type Query = Ast<JsExportFromClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().type_token().is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Do not use export all ( "<Emphasis>"export * from ..."</Emphasis>" )."
                },
            )
            .note(markup! {
                "Use named export instead."
            }),
        )
    }
}

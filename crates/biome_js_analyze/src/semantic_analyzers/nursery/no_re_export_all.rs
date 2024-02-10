use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsExport;
use biome_rowan::AstNode;

declare_rule! {
    /// Avoid re-export all
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
    /// export * from 'foo';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export * as foo from 'foo';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export { foo } from "foo";
    /// ```
    ///
    pub(crate) NoReExportAll {
        version: "next",
        name: "noReExportAll",
        recommended: false,
    }
}

impl Rule for NoReExportAll {
    type Query = Ast<JsExport>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        node.export_clause()
            .ok()?
            .as_js_export_from_clause()
            .and_then(|clause| {
                if clause.from_token().is_ok() && clause.star_token().is_ok() {
                    Some(())
                } else {
                    None
                }
            })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Do not use export all (`export * from ...`)"
            },
        ))
    }
}

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_analyze::{Ast, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::{JsExport, JsExportFromClause, JsExportNamedFromClause, JsModule};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of barrel file.
    ///
    /// A barrel file is a file that re-exports all of the exports from other files in a directory.
    /// This structure results in the unnecessary loading of many modules, significantly impacting performance in large-scale applications.
    /// Additionally, it complicates the codebase, making it difficult to navigate and understand the project's dependency graph.
    /// This rule ignores .d.ts files and type-only exports.
    ///
    /// For a more detailed explanation, check out https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-7/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export * from "foo";
    /// export * from "bar";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export { foo } from "foo";
    /// export { bar } from "bar";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export { default as module1 } from "./module1";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// export type * from "foo";
    /// export type { foo } from "foo";
    /// ```
    ///
    pub NoBarrelFile {
        version: "1.6.0",
        name: "noBarrelFile",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintBarrelFiles("avoid-barrel-files")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoBarrelFile {
    type Query = Ast<JsModule>;
    type State = JsExport;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let items = ctx.query().items();
        for item in items {
            if let Some(export) = JsExport::cast(item.into()) {
                if let Ok(export_from_clause) = export.export_clause() {
                    if let Some(export_from_clause) =
                        JsExportFromClause::cast_ref(export_from_clause.syntax())
                    {
                        if export_from_clause.type_token().is_none() {
                            return Some(export);
                        }
                    }

                    if let Some(export_from_clause) =
                        JsExportNamedFromClause::cast(export_from_clause.into_syntax())
                    {
                        if export_from_clause.type_token().is_some() {
                            continue;
                        }
                        if !export_from_clause
                            .specifiers()
                            .into_iter()
                            .flatten()
                            .all(|s| s.type_token().is_some())
                        {
                            return Some(export);
                        }
                    }
                    continue;
                }
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, js_export: &Self::State) -> Option<RuleDiagnostic> {
        let span = js_export.range();
        Some(RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Avoid barrel files, they slow down performance, and cause large module graphs with modules that go unused."
            },
        ).note(
            markup! {
                "Check "<Hyperlink href="https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-7/">"this thorough explanation"</Hyperlink>" to better understand the context."
            }))
    }
}

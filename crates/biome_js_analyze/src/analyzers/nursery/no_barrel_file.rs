use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_analyze::{Ast, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::{JsExport, JsExportFromClause, JsExportNamedFromClause};
use biome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Disallow the use of barrel file.
    ///
    /// A barrel file is a file that re-exports all of the exports from other files in a directory.
    /// This structure results in the unnecessary loading of many modules, significantly impacting performance in large-scale applications.
    /// Additionally, it complicates the codebase, making it difficult to navigate and understand the project's dependency graph.
    ///
    /// For a more detailed explanation, check out https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-7/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// export * from "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// export { foo } from "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// export { default as module2 } from "./module2";
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
        version: "next",
        name: "noBarrelFile",
        recommended: false,
        source: RuleSource::EslintBarrelFiles("avoid-namespace-import"),
        source_kind: RuleSourceKind::Inspired,
    }
}

declare_node_union! {
    pub AnyJsExportFromClause = JsExportFromClause | JsExportNamedFromClause
}

impl Rule for NoBarrelFile {
    type Query = Ast<AnyJsExportFromClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let export_from_clause = ctx.query();
        match export_from_clause {
            AnyJsExportFromClause::JsExportFromClause(clause) => {
                if clause.type_token().is_some() {
                    return None;
                }
            }
            AnyJsExportFromClause::JsExportNamedFromClause(clause) => {
                if clause.type_token().is_some() {
                    return None;
                }
                if clause
                    .specifiers()
                    .into_iter()
                    .flatten()
                    .all(|s| s.type_token().is_some())
                {
                    return None;
                }
            }
        };
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let span = match JsExport::cast(node.syntax().parent()?) {
            Some(node) => node.range(),
            // `JsExportFromClause` and `JsExportNamedFromClause` are always inside JsExport`
            None => unreachable!(),
        };
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

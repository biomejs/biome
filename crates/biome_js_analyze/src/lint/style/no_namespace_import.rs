use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::JsImportNamespaceClause;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of namespace imports.
    ///
    /// Namespace imports might impact the efficiency of tree shaking, a process that removes unused code from bundles.
    /// The effectiveness of tree shaking largely depends on the bundler (e.g., Webpack, Rollup) and its configuration.
    /// Modern bundlers are generally capable of handling namespace imports effectively, but using named imports is recommended for optimal tree shaking and minimizing bundle size.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import * as foo from "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// import { foo } from "foo"
    /// import type { bar } from "bar"
    /// import type * as baz from "baz"
    /// ```
    ///
    pub NoNamespaceImport {
        version: "1.6.0",
        name: "noNamespaceImport",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintBarrelFiles("avoid-namespace-import")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for NoNamespaceImport {
    type Query = Ast<JsImportNamespaceClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import_namespace_clause = ctx.query();
        // Allow type import e.g. `import type * as foo from "foo"`
        if import_namespace_clause.type_token().is_some() {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid namespace imports, it can prevent efficient tree shaking and increase bundle size."
                },
            )
            .note(markup! {
                "Use named imports instead."
            }),
        )
    }
}

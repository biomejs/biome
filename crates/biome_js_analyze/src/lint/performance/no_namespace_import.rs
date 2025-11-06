use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsModuleSource, JsImportNamespaceClause};
use biome_rowan::AstNode;
use biome_rule_options::no_namespace_import::NoNamespaceImportOptions;

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
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `allowlist`
    ///
    /// Allows to specify module names that are permitted to use namespace imports.
    /// This can be useful for libraries that are designed to work with namespace imports,
    /// or when you need to import many exports from a module.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowlist": ["zod", "valibot"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// import * as z from "zod";
    /// import * as v from "valibot";
    /// ```
    ///
    pub NoNamespaceImport {
        version: "1.6.0",
        name: "noNamespaceImport",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintBarrelFiles("avoid-namespace-import").same()],
    }
}

impl Rule for NoNamespaceImport {
    type Query = Ast<JsImportNamespaceClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNamespaceImportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import_namespace_clause = ctx.query();
        // Allow type import e.g. `import type * as foo from "foo"`
        if import_namespace_clause.type_token().is_some() {
            return None;
        }

        // Check if the module is in the allowlist
        if let Ok(source) = import_namespace_clause.source() {
            if let AnyJsModuleSource::JsModuleSource(js_module_source) = source {
                if let Ok(inner_text) = js_module_source.inner_string_text() {
                    let module_name = inner_text.text();
                    if ctx
                        .options()
                        .allowlist
                        .iter()
                        .flatten()
                        .any(|allowed| allowed.as_ref() == module_name)
                    {
                        return None;
                    }
                }
            }
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

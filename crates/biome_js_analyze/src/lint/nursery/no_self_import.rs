use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsImportLike;
use biome_module_graph::JsImportPath;
use biome_rowan::AstNode;
use biome_rule_options::no_self_import::NoSelfImportOptions;

declare_lint_rule! {
    /// Forbid a module from importing itself.
    ///
    /// A module that imports itself is almost always the result of a mistake
    /// made while refactoring. It creates a circular dependency on the module
    /// itself, which can leave the imported value `undefined` during evaluation
    /// and cause some bundlers to emit a warning or fail.
    ///
    /// This rule reports both static `import` statements and dynamic imports
    /// such as `import()` and `require()` calls that resolve to the file they
    /// appear in.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=foo.js
    /// import foo from "./foo.js";
    /// ```
    ///
    /// ```js,expect_diagnostic,file=bar.js
    /// const bar = require("./bar.js");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// import bar from "./bar.js";
    /// ```
    pub NoSelfImport {
        version: "next",
        name: "noSelfImport",
        language: "js",
        sources: &[RuleSource::EslintImport("no-self-import").same()],
        domains: &[RuleDomain::Project],
        severity: Severity::Error,
        recommended: false,
    }
}

impl Rule for NoSelfImport {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSelfImportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_info = ctx.js_module_info_for_path(ctx.file_path())?;
        let node = ctx.query();

        let JsImportPath { resolved_path, .. } = module_info.get_import_path_by_js_node(node)?;

        (resolved_path.as_path()? == ctx.file_path()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This module imports itself."
                },
            )
            .note(markup! {
                "A module that imports itself is a circular dependency, which can leave the imported value "<Emphasis>"undefined"</Emphasis>" during evaluation and cause some bundlers to warn or fail."
            })
            .note(markup! {
                "Remove this import, or update it to point to the intended module."
            }),
        )
    }
}

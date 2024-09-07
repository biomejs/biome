use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::CanBeImportedExported;
use biome_js_syntax::AnyJsImportSpecifier;
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow exporting an imported variable.
    ///
    /// In JavaScript, you can re-export a variable either by using `export from` or
    /// by first importing the variable and then exporting it with a regular `export`.
    ///
    /// You may prefer to use the first approach, as it clearly communicates the intention
    /// to re-export an import, and can make static analysis easier.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { A } from "mod";
    /// export { A };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import * as ns from "mod";
    /// export { ns };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import D from "mod";
    /// export { D };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export { A } from "mod";
    /// export * as ns from "mod";
    /// export { default as D } from "mod";
    /// ```
    ///
    pub NoExportedImports {
        version: "1.9.0",
        name: "noExportedImports",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoExportedImports {
    type Query = Semantic<AnyJsImportSpecifier>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let specifier = ctx.query();
        let local_name = specifier.local_name().ok()?;
        let local_name = local_name.as_js_identifier_binding()?;
        if local_name.is_exported(ctx.model()) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let specifier = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                specifier.range(),
                markup! {
                    "An import should not be exported. Use "<Emphasis>"export from"</Emphasis>" instead."
                },
            )
            .note(markup! {
                <Emphasis>"export from"</Emphasis>" makes it clearer that the intention is to re-export a variable."
            }),
        )
    }
}

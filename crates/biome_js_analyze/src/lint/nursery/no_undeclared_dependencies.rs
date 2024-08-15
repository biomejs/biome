use crate::services::manifest::Manifest;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::AnyJsImportLike;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of dependencies that aren't specified in the `package.json`.
    ///
    /// Indirect dependencies will trigger the rule because they aren't declared in the `package.json`. This means that if package `@org/foo` has a dependency on `lodash`, and then you use
    /// `import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.
    ///
    /// The rule ignores imports using a protocol such as `node:`, `bun:`, `jsr:`, `https:`.
    ///
    /// To ensure that Visual Studio Code uses relative imports when it automatically imports a variable,
    /// you may set [`javascript.preferences.importModuleSpecifier` and `typescript.preferences.importModuleSpecifier`](https://code.visualstudio.com/docs/getstarted/settings) to `relative`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// import "vite";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// import { A } from "./local.js";
    /// ```
    ///
    /// ```js,ignore
    /// import assert from "node:assert";
    /// ```
    pub NoUndeclaredDependencies {
        version: "1.6.0",
        name: "noUndeclaredDependencies",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoUndeclaredDependencies {
    type Query = Manifest<AnyJsImportLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }

        let token_text = node.inner_string_text()?;
        let text = token_text.text();

        // Ignore relative path imports
        // Ignore imports using a protocol such as `node:`, `bun:`, `jsr:`, `https:`, and so on.
        if text.starts_with('.') || text.contains(':') {
            return None;
        }

        let mut parts = text.split('/');
        let mut pointer = 0;
        if let Some(maybe_scope) = parts.next() {
            pointer += maybe_scope.len();
            if maybe_scope.starts_with('@') {
                // scoped package: @mui/material/Button
                // the package name is @mui/material, not @mui
                pointer += parts.next().map_or(0, |s| s.len() + 1)
            }
        }
        let package_name = &text[..pointer];

        if ctx.is_dependency(package_name)
            || ctx.is_dev_dependency(package_name)
            || ctx.is_peer_dependency(package_name)
            || ctx.is_optional_dependency(package_name)
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The current dependency isn't specified in your package.json."
                },
            )
            .note(markup! {
                "This could lead to errors."
            })
            .note(markup! {
                "Add the dependency in your manifest."
            }),
        )
    }
}

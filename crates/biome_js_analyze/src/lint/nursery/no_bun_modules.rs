use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportLike, inner_string_text};
use biome_resolver::is_builtin_bun_module;
use biome_rowan::TextRange;
use biome_rule_options::no_bun_modules::NoBunModulesOptions;

use crate::services::manifest::Manifest;

declare_lint_rule! {
    /// Forbid the use of Bun builtin modules.
    ///
    /// This can be useful for client-side web projects that don't have access to those modules.
    ///
    /// The rule doesn't trigger if there are dependencies declared in the `package.json` that match
    /// the name of a built-in Bun module.
    ///
    /// Type-only imports are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { Database } from "bun:sqlite";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { Database } from "custom-sqlite";
    /// ```
    ///
    /// ```ts
    /// import type { DatabaseOptions } from "bun:sqlite";
    /// ```
    pub NoBunModules {
        version: "2.5.12",
        name: "noBunModules",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoBunModules {
    type Query = Manifest<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoBunModulesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }
        if let AnyJsImportLike::JsModuleSource(module_source) = &node
            && module_source.imports_only_types()
        {
            return None;
        }
        let module_name = node.module_name_token()?;
        let module_name_text = inner_string_text(&module_name);
        let module_name_text = module_name_text.text();
        // Ignore dependencies
        if ctx.is_dependency(module_name_text)
            || ctx.is_dev_dependency(module_name_text)
            || ctx.is_peer_dependency(module_name_text)
            || ctx.is_optional_dependency(module_name_text)
        {
            return None;
        }
        is_builtin_bun_module(module_name_text).then_some(module_name.text_trimmed_range())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This import references a Bun builtin module."
                },
            )
            .note(markup! {
                "Bun builtin modules are unavailable outside the Bun runtime, so this import breaks client-side and other non-Bun environments."
            }).note(markup!{
                "Remove this import or replace it with a runtime-agnostic alternative."
            }),
        )
    }
}

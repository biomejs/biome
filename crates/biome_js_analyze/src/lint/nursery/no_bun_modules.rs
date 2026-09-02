use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike, inner_string_text};
use biome_resolver::is_builtin_bun_module;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_bun_modules::NoBunModulesOptions;

use crate::services::manifest::Manifest;

declare_lint_rule! {
    /// Forbid the use of Bun builtin modules.
    ///
    /// This can be useful for client-side web projects that don't have access to those modules.
    ///
    /// The rule also isn't triggered if there are dependencies declared in the `package.json` that match
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
    pub NoBunModules {
        version: "next",
        name: "noBunModules",
        language: "js",
        sources: &[RuleSource::EslintImport("no-nodejs-modules").inspired()],
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
            && let Some(import_clause) = module_source.parent::<AnyJsImportClause>()
            && import_clause.type_token().is_some()
        {
            // Ignore type-only imports
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
                    "Using Bun modules is forbidden."
                },
            )
            .note(markup! {
                "Can be useful for client-side web projects that do not have access to those modules."
            }).note(markup!{
                "Remove the import module."
            }),
        )
    }
}

use crate::globals::is_node_builtin_module;
use crate::services::manifest::Manifest;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{inner_string_text, AnyJsImportClause, AnyJsImportLike};
use biome_rowan::AstNode;
use biome_rowan::TextRange;

declare_lint_rule! {
    /// Forbid the use of Node.js builtin modules.
    ///
    /// This can be useful for client-side web projects that don't have access to those modules.
    ///
    /// The rule also isn't triggered if there are dependencies declared in the `package.json` that match
    /// the name of a built-in Node.js module.
    ///
    /// Type-only imports are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import fs from "fs";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import path from "node:path";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import fs from "fs-custom";
    /// ```
    ///
    /// ```ts
    /// import type path from "node:path";
    /// ```
    pub NoNodejsModules {
        version: "1.5.0",
        name: "noNodejsModules",
        language: "js",
        sources: &[RuleSource::EslintImport("no-nodejs-modules")],
        recommended: false,
    }
}

impl Rule for NoNodejsModules {
    type Query = Manifest<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }
        if let AnyJsImportLike::JsModuleSource(module_source) = &node {
            if let Some(import_clause) = module_source.parent::<AnyJsImportClause>() {
                if import_clause.type_token().is_some() {
                    // Ignore type-only imports
                    return None;
                }
            }
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
        is_node_builtin_module(module_name_text).then_some(module_name.text_trimmed_range())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Using Node.js modules is forbidden."
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

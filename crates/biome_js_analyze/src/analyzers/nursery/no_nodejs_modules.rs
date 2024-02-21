use crate::globals::node::is_node_builtin_module;
use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{inner_string_text, AnyJsImportSpecifierLike};
use biome_rowan::TextRange;

declare_rule! {
    /// Forbid the use of Node.js builtin modules.
    ///
    /// This can be useful for client-side web projects that don't have access to those modules.
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
    pub NoNodejsModules {
        version: "1.5.0",
        name: "noNodejsModules",
        source: RuleSource::EslintImport("no-nodejs-modules"),
        recommended: false,
    }
}

impl Rule for NoNodejsModules {
    type Query = Ast<AnyJsImportSpecifierLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_name = ctx.query().module_name_token()?;
        is_node_builtin_module(&inner_string_text(&module_name))
            .then_some(module_name.text_trimmed_range())
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

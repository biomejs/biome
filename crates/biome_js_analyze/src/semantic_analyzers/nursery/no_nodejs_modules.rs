use crate::globals::node::NODE_BUILTINS;
use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsImportCallExpression, JsModuleSource};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, TextRange};

declare_node_union! {
    pub(crate) AnyJsImportLike = JsModuleSource | JsCallExpression | JsImportCallExpression
}

declare_rule! {
    /// Forbid the use of Node.js builtin modules. Can be useful for client-side web projects that
    /// do not have access to those modules.
    ///
    /// Source: https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-nodejs-modules.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import fs from "fs";
    /// import path from "node:path";
    /// ```
    pub(crate) NoNodejsModules {
        version: "next",
        name: "noNodejsModules",
        recommended: false,
    }
}

impl Rule for NoNodejsModules {
    type Query = Semantic<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyJsImportLike::JsModuleSource(source) => {
                let source_text = source.inner_string_text().ok()?;
                if is_node_module(source_text.text()) {
                    return Some(source.range());
                }
            }
            AnyJsImportLike::JsCallExpression(expression) => {
                let callee = expression.callee().ok()?;
                let callee = callee.as_js_identifier_expression()?;
                let name = callee.name().ok()?.name().ok()?;
                if name.text() == "require" || name.text() == "import" {
                    let arguments = expression.arguments().ok()?.args();
                    if arguments.len() == 1 {
                        // SAFETY: the list has one argument, checked by the if before
                        let argument = arguments.iter().next().unwrap().ok()?;
                        let argument = argument
                            .as_any_js_expression()?
                            .as_any_js_literal_expression()?
                            .as_js_string_literal_expression()?;
                        let argument_text = argument.inner_string_text().ok()?;
                        if is_node_module(argument_text.text()) {
                            return Some(argument.range());
                        }
                    }
                }
            }
            AnyJsImportLike::JsImportCallExpression(import_call) => {
                let arguments = import_call.arguments().ok()?.args();
                if arguments.len() == 1 {
                    // SAFETY: the list has one argument, checked by the if before
                    let argument = arguments.iter().next().unwrap().ok()?;
                    let argument = argument
                        .as_any_js_expression()?
                        .as_any_js_literal_expression()?
                        .as_js_string_literal_expression()?;
                    let argument_text = argument.inner_string_text().ok()?;
                    if is_node_module(argument_text.text()) {
                        return Some(argument.range());
                    }
                }
            }
        };

        None
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference,
                markup! {
                    "Using Node.js modules are forbidden."
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

/// TODO(@anonrig): Use https://github.com/inspect-js/is-core-module/blob/main/core.json
fn is_node_module(name: &str) -> bool {
    NODE_BUILTINS.contains(&name)
}

use crate::globals::node::NODE_BUILTIN_MODULES;
use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsImportDefaultClause, JsImportNamespaceClause};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, TextRange};

declare_rule! {
    /// Forbid the use of Node.js builtin modules. Can be useful for client-side web projects that
    /// do not have access to those modules.
    ///
    /// TODO(@anonrig): Add "allow" array option to allow specific modules.
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
        let label_statement = ctx.query();

        // Handle require() calls.
        if let AnyJsImportLike::JsCallExpression(call) = label_statement {
            let args = call.arguments().ok()?.args();
            let first_arg = args.first()?.ok()?;
            let literal_expression = first_arg
                .as_any_js_expression()?
                .as_any_js_literal_expression()?
                .as_js_string_literal_expression()?;
            return if is_node_module(literal_expression.inner_string_text().ok()?.text()) {
                Some(literal_expression.range())
            } else {
                None
            };
        }

        // Handle import statements.
        let statement = match label_statement {
            AnyJsImportLike::JsImportDefaultClause(clause) => clause.source().ok()?,
            AnyJsImportLike::JsImportNamespaceClause(clause) => clause.source().ok()?,
            _ => unreachable!(),
        };

        if is_node_module(statement.inner_string_text().ok()?.text()) {
            Some(statement.range())
        } else {
            None
        }
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
            }),
        )
    }
}

/// TODO(@anonrig): Use https://github.com/inspect-js/is-core-module/blob/main/core.json
fn is_node_module(name: &str) -> bool {
    NODE_BUILTIN_MODULES.contains(&name)
}

declare_node_union! {
    pub(crate) AnyJsImportLike = JsImportDefaultClause | JsImportNamespaceClause | JsCallExpression
}

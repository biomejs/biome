use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    inner_string_text, JsCallExpression, JsImportCallExpression, JsModuleSource, JsSyntaxKind,
    JsSyntaxToken,
};
use biome_rowan::{declare_node_union, BatchMutationExt};

use crate::{globals::node::is_node_builtin_module, JsRuleAction};

declare_rule! {
    /// Enforces using the `node:` protocol for Node.js builtin modules.
    ///
    /// The rule marks traditional imports like `import fs from "fs";` as invalid,
    /// suggesting the format `import fs from "node:fs";` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import fs from 'fs';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import os from 'os';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import path from 'path';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import fs from 'node:fs';
    ///
    /// import os from 'node:os';
    ///
    /// import path from 'node:path';
    /// ```
    ///
    pub(crate) UseNodejsImportProtocol {
        version: "1.5.0",
        name: "useNodejsImportProtocol",
        source: RuleSource::EslintUnicorn("prefer-node-protocol"),
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNodejsImportProtocol {
    type Query = Ast<AnyJsImportLike>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_name = ctx.query().module_name_token()?;
        is_node_module_without_protocol(&inner_string_text(&module_name)).then_some(module_name)
    }

    fn diagnostic(_: &RuleContext<Self>, module_name: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            module_name.text_trimmed_range(),
            markup! {
                "A Node.js builtin module should be imported with the "<Emphasis>"node:"</Emphasis>" protocol."
            },
        )
        .note(markup!{
            "Using the "<Emphasis>"node:"</Emphasis>" protocol is more explicit and signals that the imported module belongs to Node.js."
        }))
    }

    fn action(ctx: &RuleContext<Self>, module_name: &Self::State) -> Option<JsRuleAction> {
        debug_assert!(
            module_name.kind() == JsSyntaxKind::JS_STRING_LITERAL,
            "The module name token should be a string literal."
        );
        let delimiter = module_name.text_trimmed().chars().nth(0)?;
        let module_inner_name = inner_string_text(module_name);
        let new_module_name = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_STRING_LITERAL,
            &format!("{delimiter}node:{module_inner_name}{delimiter}"),
            [],
            [],
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_token(module_name.clone(), new_module_name);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Add the "<Emphasis>"node:"</Emphasis>" protocol." }.to_owned(),
            mutation,
        })
    }
}

fn is_node_module_without_protocol(module_name: &str) -> bool {
    !module_name.starts_with("node:") && is_node_builtin_module(module_name)
}

declare_node_union! {
    pub(crate) AnyJsImportLike = JsModuleSource | JsCallExpression |  JsImportCallExpression
}

impl AnyJsImportLike {
    pub fn module_name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsImportLike::JsModuleSource(source) => source.value_token().ok(),
            AnyJsImportLike::JsCallExpression(expression) => {
                let callee = expression.callee().ok()?;
                let name = callee.as_js_reference_identifier()?.value_token().ok()?;
                if name.text_trimmed() == "require" {
                    let [Some(argument)] = expression.arguments().ok()?.get_arguments_by_index([0])
                    else {
                        return None;
                    };
                    argument
                        .as_any_js_expression()?
                        .as_any_js_literal_expression()?
                        .as_js_string_literal_expression()?
                        .value_token()
                        .ok()
                } else {
                    None
                }
            }
            AnyJsImportLike::JsImportCallExpression(import_call) => {
                let [Some(argument)] = import_call.arguments().ok()?.get_arguments_by_index([0])
                else {
                    return None;
                };
                argument
                    .as_any_js_expression()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?
                    .value_token()
                    .ok()
            }
        }
    }
}

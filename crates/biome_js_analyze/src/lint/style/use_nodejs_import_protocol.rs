use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{inner_string_text, AnyJsImportLike, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::BatchMutationExt;

use crate::{globals::is_node_builtin_module, JsRuleAction};

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
    pub UseNodejsImportProtocol {
        version: "1.5.0",
        name: "useNodejsImportProtocol",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-node-protocol")],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNodejsImportProtocol {
    type Query = Ast<AnyJsImportLike>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }
        let module_name = node.module_name_token()?;
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
        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"node:"</Emphasis>" protocol." }.to_owned(),
            mutation,
        ))
    }
}

fn is_node_module_without_protocol(module_name: &str) -> bool {
    !module_name.starts_with("node:") && is_node_builtin_module(module_name)
}

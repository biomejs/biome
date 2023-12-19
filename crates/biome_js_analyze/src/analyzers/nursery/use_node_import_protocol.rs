use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsModuleSource;
use biome_rowan::AstNode;

use crate::globals::node::NODE_BUILTINS;

declare_rule! {
    /// Enforces using the `node:` protocol for Node.js builtin modules.
    ///
    /// The rule marks traditional imports like `import fs from "fs";` as invalid,
    /// suggesting the format `import fs from "node:fs";` instead.
    ///
    /// Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-node-protocol.md
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
    pub(crate) UseNodeImportProtocol {
        version: "next",
        name: "useNodeImportProtocol",
        recommended: false,
    }
}

impl Rule for UseNodeImportProtocol {
    type Query = Ast<JsModuleSource>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let module_name = binding.inner_string_text().ok()?;

        (!is_builtin_module(&module_name)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding: &JsModuleSource = ctx.query();
        let module_name = binding.inner_string_text().ok()?.to_string();

        Some(RuleDiagnostic::new(
            rule_category!(),
            binding.range(),
            markup! {
                "Import from Node.js builtin module \""<Emphasis>{module_name}</Emphasis>"\" should use the \""<Emphasis>"node:"</Emphasis>"\" protocol."
            },
        )
        .note(markup! {
            "Please change to \"node:"{module_name}"\"."
        }))
    }
}

fn is_builtin_module(module_name: &str) -> bool {
    NODE_BUILTINS.binary_search(&module_name).is_ok()
}

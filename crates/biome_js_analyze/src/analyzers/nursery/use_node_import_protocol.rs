use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsModuleSource;
use biome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Enforces using the `node:` protocol for Node.js builtin modules.
    ///
    /// The prefer-node-protocol rule in ESLint enforces the use of the node: protocol
    /// when importing Node.js builtin modules in JavaScript code.
    /// This helps differentiate between built-in modules and third-party ones, improving code clarity.
    /// The rule marks traditional imports like import fs from 'fs'; as invalid,
    /// suggesting the format import fs from 'node:fs'; instead.
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

pub(crate) struct NodeImportProtocolState {
    /// The range of the import statement in the source code.
    range: TextRange,

    /// The builtin module name that was imported.
    module_name: String,
}

impl Rule for UseNodeImportProtocol {
    type Query = Ast<JsModuleSource>;
    type State = NodeImportProtocolState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding: &JsModuleSource = ctx.query();
        let Ok(module_name) = binding.inner_string_text() else {
            return None;
        };

        if !is_builtin_module(&module_name) {
            return None;
        }

        Some(NodeImportProtocolState {
            range: binding.range(),
            module_name: module_name.to_string(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let NodeImportProtocolState { range, module_name } = state;

        Some(RuleDiagnostic::new(
            rule_category!(),
            *range,
            markup! {
                "Import from Node.js builtin module \""{module_name}"\" should use the \"node:\" protocol."
            },
        )
        .note(markup! {
            "Please change to \"node:"{module_name}"\"."
        }))
    }
}

const NODE_BUILTINS: &[&str] = &[
    "assert",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "https",
    "module",
    "net",
    "os",
    "path",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "tty",
    "url",
    "util",
    "vm",
    "zlib",
];

fn is_builtin_module(module_name: &str) -> bool {
    NODE_BUILTINS.contains(&module_name)
}

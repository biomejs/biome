use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression, global_identifier};
use biome_rowan::AstNode;
use biome_rule_options::no_process_exit::NoProcessExitOptions;

use crate::services::semantic::Semantic;
use crate::shared::node_process::is_process_module_import;

declare_lint_rule! {
    /// Disallow the use of `process.exit()`.
    ///
    /// In Node.js, `process.exit()` stops the program right away.
    /// Work that is still in progress, such as writing a file, sending a network request, or waiting on a timer, never finishes.
    /// The program may also stop without printing why, which makes the problem hard to find.
    ///
    /// Throw an error instead.
    /// An error that nothing catches also stops the program, and Node.js prints its message and where it happened.
    /// To choose the exit code (the number the program reports when it ends, where `0` means success),
    /// set `process.exitCode` and let the program finish on its own.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (somethingBadHappened) {
    ///     console.error("Something bad happened!");
    ///     process.exit(1);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (somethingBadHappened) {
    ///     throw new Error("Something bad happened!");
    /// }
    ///
    /// if (somethingElseHappened) {
    ///     console.error("Something else happened!");
    ///     process.exitCode = 1;
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to require importing `process` from `node:process` instead of using the global, see [`noProcessGlobal`](https://biomejs.dev/linter/rules/no-process-global/).
    /// - If you want to disallow reading `process.env`, see [`noProcessEnv`](https://biomejs.dev/linter/rules/no-process-env/).
    ///
    pub NoProcessExit {
        version: "2.6.0",
        name: "noProcessExit",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-process-exit").same(),
            RuleSource::EslintN("no-process-exit").same(),
            RuleSource::EslintUnicorn("no-process-exit").inspired(),
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoProcessExit {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProcessExitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        if callee.member_name()?.text() != "exit" {
            return None;
        }

        let object = callee.object().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
        if name.text() != "process" {
            return None;
        }

        match ctx.model().binding(&reference) {
            None => Some(()),
            Some(binding) => is_process_module_import(&binding).then_some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use "<Emphasis>"process.exit()"</Emphasis>"."
                },
            )
            .note(markup! {
                "It stops the program right away, before work in progress finishes and without saying why it stopped."
            })
            .note(markup! {
                "Throw an error instead, or set "<Emphasis>"process.exitCode"</Emphasis>" to choose the exit code."
            }),
        )
    }
}

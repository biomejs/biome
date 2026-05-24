use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsImport, JsStaticMemberExpression, global_identifier};
use biome_rowan::AstNode;
use biome_rule_options::no_process_env::NoProcessEnvOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow the use of `process.env`, `Bun.env`, and imported `env`.
    ///
    /// The `process.env` object in Node.js and `Bun.env` in Bun store configuration settings. Using them directly throughout a project can cause problems:
    ///
    /// 1. It's harder to maintain
    /// 2. It can lead to conflicts in team development
    /// 3. It complicates deployment across multiple servers
    ///
    /// A better practice is to keep all settings in one configuration file and reference it throughout the project.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (process.env.NODE_ENV === 'development') {
    ///   // ...
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { env } from 'node:process';
    /// console.log(env.HOME);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (Bun.env.DEBUG === 'true') {
    ///   // ...
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const config = require('./config');
    /// if (config.NODE_ENV === 'development') {
    ///   // ...
    /// }
    /// ```
    ///
    pub NoProcessEnv {
        version: "1.9.1",
        name: "noProcessEnv",
        language: "js",
        sources: &[RuleSource::EslintN("no-process-env").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

impl Rule for NoProcessEnv {
    type Query = Semantic<JsStaticMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProcessEnvOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let static_member_expr = ctx.query();
        let model = ctx.model();
        let object = static_member_expr.object().ok()?;
        let member_expr = static_member_expr.member().ok()?;
        let member_name = member_expr.as_js_name()?.to_trimmed_text().text();

        let (reference, name) =
            global_identifier(&object.as_any_global_identifier_expression()?)?;
        let object_name = name.text();

        // Case 1: process.env (existing behavior)
        if object_name == "process" && member_name == "env" {
            return match model.binding(&reference) {
                None => Some(()),
                Some(binding) => is_process_module_import(&binding).then_some(()),
            };
        }

        // Case 2: env.FOO where env is imported from 'node:process' or 'process'
        if object_name == "env" {
            if let Some(binding) = model.binding(&reference) {
                return is_process_module_import(&binding).then_some(());
            }
        }

        // Case 3: Bun.env
        if object_name == "Bun" && member_name == "env" {
            // Check if Bun is a global (not bound to any import)
            return model.binding(&reference).is_none().then_some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use environment variables directly."
                },
            )
            .note(markup! {
                "Use a centralized configuration file instead for better maintainability and deployment consistency."
            }),
        )
    }
}

fn is_process_module_import(binding: &biome_js_semantic::Binding) -> bool {
    const PROCESS_MODULE_NAMES: [&str; 2] = ["process", "node:process"];
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| PROCESS_MODULE_NAMES.contains(&source.text()))
}

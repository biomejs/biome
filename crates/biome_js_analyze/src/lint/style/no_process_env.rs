use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsNamedImportSpecifier, JsImport, JsStaticMemberExpression, global_identifier,
};
use biome_rowan::AstNode;
use biome_rule_options::no_process_env::NoProcessEnvOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow the use of `process.env`.
    ///
    /// The `process.env` object in Node.js stores configuration settings. Using it directly throughout a project can cause problems:
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
        let node = ctx.query();
        let model = ctx.model();
        let object = node.object().ok()?;
        let member = node.member().ok()?.as_js_name()?.to_trimmed_text();

        let (reference, name) =
            global_identifier(&object.as_any_global_identifier_expression()?)?;

        if member.text() == "env" && name.text() == "process" {
            return match model.binding(&reference) {
                None => Some(()),
                Some(binding) => is_process_module_import(&binding).then_some(()),
            };
        }

        model.binding(&reference).filter(is_env_from_process).map(|_| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use "<Emphasis>"process.env"</Emphasis>"."
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

fn is_env_from_process(binding: &biome_js_semantic::Binding) -> bool {
    const PROCESS_MODULE_NAMES: [&str; 2] = ["process", "node:process"];
    let ancestors: Vec<_> = binding.syntax().ancestors().collect();
    ancestors
        .iter()
        .find_map(|n| AnyJsNamedImportSpecifier::cast(n.clone())?.imported_name())
        .is_some_and(|name| name.text_trimmed() == "env")
        && ancestors
            .iter()
            .find_map(|n| JsImport::cast(n.clone())?.source_text().ok())
            .is_some_and(|src| PROCESS_MODULE_NAMES.contains(&src.text()))
}

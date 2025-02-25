use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{global_identifier, JsStaticMemberExpression};
use biome_rowan::AstNode;

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
        sources: &[RuleSource::EslintN("no-process-env")],
        recommended: false,
    }
}

impl Rule for NoProcessEnv {
    type Query = Semantic<JsStaticMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let static_member_expr = ctx.query();
        let model = ctx.model();
        let object = static_member_expr.object().ok()?;
        let member_expr = static_member_expr.member().ok()?;
        if member_expr.as_js_name()?.to_trimmed_string() != "env" {
            return None;
        }

        let (reference, name) = global_identifier(&object)?;
        if name.text() != "process" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
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

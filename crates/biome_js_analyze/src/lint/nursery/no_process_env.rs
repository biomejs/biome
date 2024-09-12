use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsStaticMemberExpression, JsSyntaxKind};
use biome_rowan::AstNode;

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
        version: "next",
        name: "noProcessEnv",
        language: "js",
        sources: &[RuleSource::EslintN("no-process-env")],
        recommended: false,
    }
}

impl Rule for NoProcessEnv {
    type Query = Ast<JsStaticMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let static_member_expr = ctx.query();
        if is_process_env(static_member_expr)? {
            Some(())
        } else {
            None
        }
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

fn is_process_env(expr: &JsStaticMemberExpression) -> Option<bool> {
    let object = expr.object().ok()?;
    match object {
        AnyJsExpression::JsIdentifierExpression(ident) => {
            let is_process = ident.name().ok()?.text() == "process";
            let is_dot = expr.operator_token().ok()?.kind() == JsSyntaxKind::DOT;
            let is_env = expr.member().ok()?.as_js_name()?.text() == "env";
            Some(is_process && is_dot && is_env)
        }
        _ => None,
    }
}

use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsStaticMemberExpression, JsSyntaxKind};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoProcessEnv {
        version: "next",
        name: "noProcessEnv",
        language: "js",
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
                "The use of "<Emphasis>"process.env"</Emphasis>" is discouraged; use a centralized configuration file instead for better maintainability and deployment consistency."
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

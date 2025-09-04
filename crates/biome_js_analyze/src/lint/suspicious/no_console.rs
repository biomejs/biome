use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{js_directive_list, js_function_body, js_statement_list, token};
use biome_js_syntax::{
    AnyJsMemberExpression, JsArrowFunctionExpression, JsCallExpression, JsExpressionStatement, T,
    global_identifier,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_console::NoConsoleOptions;

declare_lint_rule! {
    /// Disallow the use of `console`.
    ///
    /// In a browser environment, it’s considered a best practice to log messages using `console`.
    /// Such messages are considered to be for debugging purposes and therefore not suitable to ship to the client.
    /// In general, calls using `console` should be stripped before being pushed to production.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.error('hello world')
    /// ```
    ///
    /// ## Options
    ///
    /// Use the options to explicitly allow a specific subset of `console` methods.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allow": ["assert", "error", "info", "warn"]
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// console.error("error message"); // Allowed
    /// console.warn("warning message"); // Allowed
    /// console.info("info message"); // Allowed
    /// console.log("log message");
    /// console.assert(true, "explanation"); // Allowed
    /// ```
    pub NoConsole {
        version: "1.6.0",
        name: "noConsole",
        language: "js",
        sources: &[RuleSource::Eslint("no-console").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConsole {
    type Query = Semantic<AnyJsMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoConsoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member_expression = ctx.query();
        let model = ctx.model();
        let object = member_expression.object().ok()?;
        let (reference, name) = global_identifier(&object)?;
        if name.text() != "console" {
            return None;
        }
        if let Some(member_name) = member_expression.member_name() {
            let member_name = member_name.text();
            if ctx
                .options()
                .allow
                .iter()
                .any(|allowed| allowed.as_ref() == member_name)
            {
                return None;
            }
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"console"</Emphasis>"."
                },
            )
            .note(markup! {
                "The use of "<Emphasis>"console"</Emphasis>" is often reserved for debugging."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let member_expression = ctx.query();
        let call_expression = JsCallExpression::cast(member_expression.syntax().parent()?)?;
        let mut mutation = ctx.root().begin();
        let parent = call_expression.syntax().parent()?;
        if let Some(stmt) = JsExpressionStatement::cast(parent.clone()) {
            if stmt.semicolon_token().is_some() {
                mutation.remove_node(stmt);
            } else {
                mutation.remove_node(call_expression.clone());
            }
        } else if JsArrowFunctionExpression::cast(parent).is_some() {
            let new_body = js_function_body(
                token(T!['{']),
                js_directive_list(vec![]),
                js_statement_list(vec![]),
                token(T!['}']),
            );
            mutation.replace_element(call_expression.clone().into(), new_body.into());
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove "<Emphasis>"console"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

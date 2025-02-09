use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make::{js_directive_list, js_function_body, js_statement_list, token};
use biome_js_syntax::{
    global_identifier, AnyJsMemberExpression, JsArrowFunctionExpression, JsCallExpression,
    JsExpressionStatement, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

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
        sources: &[RuleSource::Eslint("no-console")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConsole {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = Box<NoConsoleOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let model = ctx.model();
        let callee = call_expression.callee().ok()?;
        let member_expression = AnyJsMemberExpression::cast(callee.into_syntax())?;
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
        let node = ctx.query();
        let parent = node.clone().syntax().parent()?;
        let range = JsExpressionStatement::cast_ref(&parent)
            .map_or(node.range(), |node| node.syntax().text_trimmed_range());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
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
        let call_expression = ctx.query();
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

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct NoConsoleOptions {
    /// Allowed calls on the console object.
    pub allow: Box<[Box<str>]>,
}

use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    global_identifier, AnyJsMemberExpression, JsCallExpression, JsExpressionStatement,
};
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Disallow the use of `console`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.error('hello world')
    /// ```
    ///
    pub NoConsole {
        version: "next",
        name: "noConsole",
        source: RuleSource::Eslint("no-console"),
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConsole {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let model = ctx.model();
        let callee = call_expression.callee().ok()?;
        let member_expression = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        let object = member_expression.object().ok()?;
        let (reference, name) = global_identifier(&object)?;
        if name.text() != "console" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let node = JsExpressionStatement::cast(node.syntax().parent()?)?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Don't use "<Emphasis>"console"</Emphasis>
                },
            )
            .note(markup! {
                "Usage of "<Emphasis>"console"</Emphasis>" is disallowed."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let call_expression = ctx.query();
        let mut mutation = ctx.root().begin();

        match JsExpressionStatement::cast(call_expression.syntax().parent()?) {
            Some(stmt) if stmt.semicolon_token().is_some() => {
                mutation.remove_node(stmt);
            }
            _ => {
                mutation.remove_node(call_expression.clone());
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove console" }.to_owned(),
            mutation,
        })
    }
}

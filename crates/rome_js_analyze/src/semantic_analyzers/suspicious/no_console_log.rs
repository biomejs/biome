use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{global_identifier, AnyJsMemberExpression, JsCallExpression};
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of `console.log`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log()
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// console.info("info");
    /// console.warn("warn");
    /// console.error("error");
    /// console.assert(true);
    /// console.table(["foo", "bar"]);
    /// const console = { log() {} };
    /// console.log();
    /// ```
    ///
    pub(crate) NoConsoleLog {
        version: "1.0.0",
        name: "noConsoleLog",
        recommended: false,
    }
}

impl Rule for NoConsoleLog {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let model = ctx.model();
        let callee = call_expression.callee().ok()?;
        let member_expression = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        if member_expression.member_name()?.text() != "log" {
            return None;
        }
        let object = member_expression.object().ok()?;
        let (reference, name) = global_identifier(&object)?;
        if name.text() != "console" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Don't use "<Emphasis>"console.log"</Emphasis>
                },
            )
            .note(markup! {
                <Emphasis>"console.log"</Emphasis>" is usually a tool for debugging and you don't want to have that in production."
            }),
        )
    }
}

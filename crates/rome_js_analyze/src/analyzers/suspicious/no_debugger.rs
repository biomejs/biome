use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::JsDebuggerStatement;
use rome_rowan::{AstNode, BatchMutationExt};

use crate::{utils, JsRuleAction};

declare_rule! {
    /// Disallow the use of `debugger`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// debugger;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const test = { debugger: 1 };
    /// test.debugger;
    ///```
    pub(crate) NoDebugger {
        version: "1.0.0",
        name: "noDebugger",
        recommended: true,
    }
}

impl Rule for NoDebugger {
    type Query = Ast<JsDebuggerStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "This is an unexpected use of the "<Emphasis>"debugger"</Emphasis>" statement."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let mut mutation = ctx.root().begin();
        utils::remove_statement(&mut mutation, node)?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            mutation,
        })
    }
}

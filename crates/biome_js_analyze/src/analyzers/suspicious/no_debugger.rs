use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::JsDebuggerStatement;
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{utils::batch::JsBatchMutation, JsRuleAction};

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
        source: RuleSource::Eslint("no-debugger"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
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
        mutation.remove_statement(node.clone().into());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            mutation,
        })
    }
}

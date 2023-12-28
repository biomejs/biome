use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{global_identifier, AnyJsExpression};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of global `eval()`.
    ///
    /// JavaScript’s `eval()` function is potentially dangerous and is often misused. Using `eval()` on untrusted code can open a program up to several different injection attacks. The use of `eval()` in most contexts can be substituted for a better, alternative approach to a problem.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-eval
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// eval("var a = 0");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// f(eval);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var evalAlias = eval;
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// var a = 0;
    /// ```
    ///
    pub(crate) NoGlobalEval {
        version: "next",
        name: "noGlobalEval",
        recommended: false,
    }
}

impl Rule for NoGlobalEval {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "eval" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                <Emphasis>"eval()"</Emphasis>" can be harmful."
            },
        ))
    }
}

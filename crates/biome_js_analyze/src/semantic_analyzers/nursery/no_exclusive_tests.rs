use biome_analyze::{
    context::RuleContext, declare_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{JsCallExpression, TextRange};
use biome_rowan::{AstNode, BatchMutationExt, NodeOrToken};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow exclusive tests.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.only("foo", function () {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// it.only("foo", function () {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.only("foo", function () {});
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// test("foo", function () {});
    /// ```
    ///
    /// ```js
    /// it("foo", function () {});
    /// ```
    ///
    /// ```js
    /// test("foo", function () {});
    /// ```
    pub NoExclusiveTests {
        version: "next",
        name: "noExclusiveTests",
        recommended: true,
        source: RuleSource::EslintJest("no-exclusive-tests"),
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
    }
}

const FUNCTION_NAMES: [&str; 1] = ["only"];
const CALEE_NAMES: [&str; 3] = ["describe", "it", "test"];

impl Rule for NoExclusiveTests {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        if node.is_test_call_expression().ok()? {
            if callee.contains_a_test_pattern().ok()? {
                let callee_object = callee.get_callee_object_name()?;
                let callee_member = callee.get_callee_member_name()?;

                if FUNCTION_NAMES.contains(&callee_member.text_trimmed())
                    && CALEE_NAMES.contains(&callee_object.text_trimmed())
                {
                    return Some(callee_member.text_trimmed_range());
                }
            }
        } else if let Some(expression) = callee.as_js_computed_member_expression() {
            let value_token = expression
                .as_fields()
                .object
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?;

            if expression.l_brack_token().is_ok()
                && expression.r_brack_token().is_ok()
                && CALEE_NAMES.contains(&value_token.text_trimmed())
            {
                if let Some(literal) = expression.member().ok()?.as_any_js_literal_expression() {
                    if literal.as_js_string_literal_expression().is_some()
                        && literal.to_string() == "\"only\""
                    {
                        return Some(expression.syntax().text_trimmed_range());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Don't exclusive the test."
                },
            )
            .note("The 'only' method is often used for debugging or during implementation. It should be removed before deploying to production.")
            .note("Consider removing 'only' to ensure all tests are executed.")
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let mut mutation = ctx.root().begin();

        if let Some(expression) = callee.as_js_static_member_expression() {
            let member_name = expression.member().ok()?;
            let operator_token = expression.operator_token().ok()?;

            mutation.remove_element(member_name.into());
            mutation.remove_element(operator_token.into());
        } else if let Some(expression) = callee.as_js_computed_member_expression() {
            let l_brack = expression.l_brack_token().ok()?;
            let r_brack = expression.r_brack_token().ok()?;
            let member = expression.member().ok()?;
            let expression = member.as_any_js_literal_expression()?;

            mutation.remove_element(NodeOrToken::Token(l_brack));
            mutation.remove_element(NodeOrToken::Node(expression.syntax().clone()));
            mutation.remove_element(NodeOrToken::Token(r_brack));
        }

        Some(JsRuleAction {
            category: biome_analyze::ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the 'only' method to ensure all tests are executed." }
                .to_owned(),
            mutation,
        })
    }
}

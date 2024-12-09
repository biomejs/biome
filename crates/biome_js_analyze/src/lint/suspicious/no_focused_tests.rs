use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleDomain,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{JsCallExpression, TextRange};
use biome_rowan::{AstNode, BatchMutationExt, NodeOrToken};

declare_lint_rule! {
    /// Disallow focused tests.
    ///
    /// Disabled test are useful when developing and debugging, because it forces the test suite to run only certain tests.
    ///
    /// However, in pull/merge request, you usually want to run all the test suite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.only("foo", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.only("foo", () => {});
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// test("foo", () => {});
    /// ```
    pub NoFocusedTests {
        version: "1.6.0",
        name: "noFocusedTests",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintJest("no-focused-tests")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Test],
    }
}

const FUNCTION_NAMES: [&str; 3] = ["only", "fdescribe", "fit"];
const CALEE_NAMES: [&str; 3] = ["describe", "it", "test"];

impl Rule for NoFocusedTests {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        if node.is_test_call_expression().ok()? {
            let callee = node.callee().ok()?;
            if callee.contains_a_test_pattern().ok()? {
                let function_name = callee.get_callee_member_name()?;

                if FUNCTION_NAMES.contains(&function_name.text_trimmed()) {
                    return Some(function_name.text_trimmed_range());
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
                    "Don't focus the test."
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

        if let Some(function_name) = callee.get_callee_member_name() {
            let replaced_function;
            match function_name.text_trimmed() {
                "only" => {
                    if let Some(expression) = callee.as_js_static_member_expression() {
                        let member_name = expression.member().ok()?;
                        let operator_token = expression.operator_token().ok()?;
                        mutation.remove_element(member_name.into());
                        mutation.remove_element(operator_token.into());
                    }
                }
                "fdescribe" => {
                    replaced_function = make::js_reference_identifier(make::ident("describe"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                "fit" => {
                    replaced_function = make::js_reference_identifier(make::ident("it"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                _ => {}
            };
        } else if let Some(expression) = callee.as_js_computed_member_expression() {
            let l_brack = expression.l_brack_token().ok()?;
            let r_brack = expression.r_brack_token().ok()?;
            let member = expression.member().ok()?;
            let expression = member.as_any_js_literal_expression()?;
            mutation.remove_element(NodeOrToken::Token(l_brack));
            mutation.remove_element(NodeOrToken::Node(expression.syntax().clone()));
            mutation.remove_element(NodeOrToken::Token(r_brack));
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove focus from test." }.to_owned(),
            mutation,
        ))
    }
}

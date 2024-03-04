use biome_analyze::{
    context::RuleContext, declare_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{JsCallExpression, TextRange};
use biome_rowan::BatchMutationExt;

use crate::JsRuleAction;

declare_rule! {
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
        version: "next",
        name: "noFocusedTests",
        recommended: true,
        source: RuleSource::EslintJest("no-focused-tests"),
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
    }
}

const FUNCTION_NAMES: [&str; 3] = ["only", "fdescribe", "fit"];

impl Rule for NoFocusedTests {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_test_call_expression().ok()? {
            let callee = node.callee().ok()?;
            if callee.contains_a_test_pattern().ok()? {
                let function_name = callee.get_callee_member_name()?;

                if FUNCTION_NAMES.contains(&function_name.text_trimmed()) {
                    return Some(function_name.text_trimmed_range());
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
                .note("This is likely a change done during debugging or implementation phases, but it's unlikely what you want in production.")
                .note("Remove it.")
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let function_name = callee.get_callee_member_name()?;
        let replaced_function;

        let mut mutation = ctx.root().begin();

        match function_name.text_trimmed() {
            "only" => {
                let member = callee.as_js_static_member_expression()?;
                let member_name = member.member().ok()?;
                let operator_token = member.operator_token().ok()?;
                // let member = member.as_js_name()?;
                mutation.remove_element(member_name.into());
                mutation.remove_element(operator_token.into());
            }
            "fit" => {
                replaced_function = make::js_reference_identifier(make::ident("it"));
                mutation.replace_element(function_name.into(), replaced_function.into());
            }
            "fdescribe" => {
                replaced_function = make::js_reference_identifier(make::ident("describe"));
                mutation.replace_element(function_name.into(), replaced_function.into());
            }
            _ => {}
        };

        Some(JsRuleAction {
            category: biome_analyze::ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove focus from test." }.to_owned(),
            mutation,
        })
    }
}

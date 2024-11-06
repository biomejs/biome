use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow disabled tests.
    ///
    /// Disabled test are useful when developing and debugging, although they should not be committed in production.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.skip("test", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.skip("test", () => {});
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// test.only("test", () => {});
    /// test("test", () => {});
    /// ```
    ///
    pub NoSkippedTests {
        version: "1.6.0",
        name: "noSkippedTests",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintJest("no-disabled-tests")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
    }
}

const FUNCTION_NAMES: [&str; 4] = ["skip", "xdescribe", "xit", "xtest"];

impl Rule for NoSkippedTests {
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
                    "Don't disable tests."
                },
            )
            .note("Disabling tests is useful when debugging or creating placeholder while working.")
            .note("If this is intentional, and you want to commit a disabled test, add a suppression comment.")
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let function_name = callee.get_callee_member_name()?;
        let replaced_function;

        let mut mutation = ctx.root().begin();

        match function_name.text_trimmed() {
            "skip" => {
                let member = callee.as_js_static_member_expression()?;
                let member_name = member.member().ok()?;
                let operator_token = member.operator_token().ok()?;
                mutation.remove_element(member_name.into());
                mutation.remove_element(operator_token.into());
            }
            "xdescribe" => {
                replaced_function = make::js_reference_identifier(make::ident("describe"));
                mutation.replace_element(function_name.into(), replaced_function.into());
            }
            "xit" => {
                replaced_function = make::js_reference_identifier(make::ident("it"));
                mutation.replace_element(function_name.into(), replaced_function.into());
            }
            "xtest" => {
                replaced_function = make::js_reference_identifier(make::ident("test"));
                mutation.replace_element(function_name.into(), replaced_function.into());
            }
            _ => {}
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Enable the test." }.to_owned(),
            mutation,
        ))
    }
}

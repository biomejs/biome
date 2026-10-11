use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression, T,
};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_expect_to_contain::UseExpectToContainOptions;

declare_lint_rule! {
    /// Enforce using `toContain()` to check whether an array or string contains a value.
    ///
    /// Checking the result of `includes()` with `toBe()`, `toEqual()`, or `toStrictEqual()`
    /// works, but when the test fails, the error message only says that `true` was expected
    /// and `false` was received. `toContain()` reports the value that was searched and the
    /// item that was missing, which makes failures easier to understand.
    ///
    /// The fix is unsafe because `includes()` and `toContain()` don't compare values in exactly
    /// the same way. For example, `[NaN].includes(NaN)` is `true`, while
    /// `expect([NaN]).toContain(NaN)` fails.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(fruits.includes("apple")).toBe(true);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(fruits.includes("apple")).toEqual(false);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(fruits.includes("apple")).not.toStrictEqual(true);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(fruits).toContain("apple");
    /// expect(fruits).not.toContain("apple");
    /// ```
    ///
    pub UseExpectToContain {
        version: "next",
        name: "useExpectToContain",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-to-contain").same(),
            RuleSource::EslintVitest("prefer-to-contain").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExpectToContain {
    type Query = Ast<JsCallExpression>;
    type State = UseExpectToContainState;
    type Signals = Option<Self::State>;
    type Options = UseExpectToContainOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let matcher = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        if !EQUALITY_MATCHERS.contains(&matcher.member_name()?.text()) {
            return None;
        }

        let expected = only_argument(call)?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_any_js_literal_expression()?
            .as_js_boolean_literal_expression()?
            .value_token()
            .ok()?
            .kind()
            == T![true];

        let mut subject = matcher.object().ok()?;
        let mut has_not = false;
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            if modifier.member_name()?.text() != "not" {
                return None;
            }
            has_not = true;
            subject = modifier.object().ok()?;
        }

        let expect_call = subject.as_js_call_expression()?;
        if !expect_call.has_callee("expect") {
            return None;
        }

        let includes_call = expect_call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_js_call_expression()?
            .clone();
        if includes_call.is_optional_chain() {
            return None;
        }
        let includes = AnyJsMemberExpression::cast(includes_call.callee().ok()?.into_syntax())?;
        if includes.member_name()?.text() != "includes"
            || only_argument(&includes_call)?
                .as_any_js_expression()
                .is_none()
        {
            return None;
        }

        Some(UseExpectToContainState {
            expect_call: expect_call.clone(),
            includes_call,
            negated: expected == has_not,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let matcher = AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher.member_name()?.range(),
                markup! {
                    "The result of "<Emphasis>"includes()"</Emphasis>" is compared to a boolean."
                },
            )
            .note(markup! {
                "When this check fails, the error message only shows that a boolean didn't match, not which value was missing."
            })
            .note(markup! {
                "Use "<Emphasis>"toContain()"</Emphasis>" to check whether a value is present."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        // Rebuilding the assertion discards the tokens around `includes()` and the modifiers,
        // along with any comments attached to them.
        if call.syntax().has_inner_comments() {
            return None;
        }
        let includes =
            AnyJsMemberExpression::cast(state.includes_call.callee().ok()?.into_syntax())?;
        let search_value = only_argument(&state.includes_call)?;

        // Replaces the whole argument so that parentheses around `includes()` are removed too.
        let expect_argument = state.expect_call.arguments().ok()?.args().first()?.ok()?;
        let expect_call = state.expect_call.clone().replace_node(
            expect_argument,
            AnyJsCallArgument::AnyJsExpression(includes.object().ok()?),
        )?;
        let mut subject = AnyJsExpression::from(expect_call);
        if state.negated {
            subject = make::js_static_member_expression(
                subject,
                make::token(T![.]),
                make::js_name(make::ident("not")).into(),
            )
            .into();
        }
        let callee = make::js_static_member_expression(
            subject,
            make::token(T![.]),
            make::js_name(make::ident("toContain")).into(),
        );

        let arguments = call
            .arguments()
            .ok()?
            .replace_node(only_argument(call)?, search_value)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            call.clone(),
            make::js_call_expression(callee.into(), arguments).build(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"toContain()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseExpectToContainState {
    /// The `expect(...)` call whose first argument is `includes_call`.
    expect_call: JsCallExpression,
    /// The `value.includes(item)` call passed to `expect()`.
    includes_call: JsCallExpression,
    /// Whether the equivalent `toContain()` assertion needs a `.not` modifier.
    negated: bool,
}

const EQUALITY_MATCHERS: &[&str] = &["toBe", "toEqual", "toStrictEqual"];

/// Returns the argument of `call` if it has exactly one argument.
fn only_argument(call: &JsCallExpression) -> Option<AnyJsCallArgument> {
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    arguments.first()?.ok()
}

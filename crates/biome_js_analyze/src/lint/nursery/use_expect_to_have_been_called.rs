use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, JsCallExpression, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_expect_to_have_been_called::UseExpectToHaveBeenCalledOptions;

declare_lint_rule! {
    /// Enforce using `toHaveBeenCalled()` to check whether a mock function was called.
    ///
    /// `toHaveBeenCalledTimes(0)` and `toBeCalledTimes(0)` check that a mock function was never
    /// called by comparing its number of calls to zero. `not.toHaveBeenCalled()` states the same
    /// check directly. When it fails, Jest and Vitest also list the calls that the function
    /// received, which makes the failure easier to understand.
    ///
    /// In the same way, `not.toHaveBeenCalledTimes(0)` checks that a mock function was called
    /// at least once, which is what `toHaveBeenCalled()` does.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toHaveBeenCalledTimes(0);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).not.toHaveBeenCalledTimes(0);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toBeCalledTimes(0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(sendEmail).not.toHaveBeenCalled();
    /// expect(sendEmail).toHaveBeenCalled();
    /// expect(sendEmail).toHaveBeenCalledTimes(2);
    /// expect(sendEmail).toHaveBeenCalledTimes(expectedCalls);
    /// ```
    ///
    pub UseExpectToHaveBeenCalled {
        version: "next",
        name: "useExpectToHaveBeenCalled",
        language: "js",
        recommended: false,
        // Vitest has the same matchers, but `eslint-plugin-vitest` has no equivalent rule.
        sources: &[RuleSource::EslintJest("prefer-to-have-been-called").same()],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseExpectToHaveBeenCalled {
    type Query = Ast<JsCallExpression>;
    type State = PreferredAssertion;
    type Signals = Option<Self::State>;
    type Options = UseExpectToHaveBeenCalledOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional_chain() {
            return None;
        }
        let matcher = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        if !CALL_COUNT_MATCHERS.contains(&matcher.member_name()?.text()) {
            return None;
        }

        let count = call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone()
            .omit_type_wrappers()?;
        if count
            .as_any_js_literal_expression()?
            .as_js_number_literal_expression()?
            .as_number()?
            != 0.0
        {
            return None;
        }

        let mut subject = matcher.object().ok()?;
        let mut preferred = PreferredAssertion::NotCalled;
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax())
            && modifier.member_name()?.text() == "not"
        {
            preferred = PreferredAssertion::Called;
            subject = modifier.object().ok()?;
        }
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            if !PROMISE_MODIFIERS.contains(&modifier.member_name()?.text()) {
                return None;
            }
            subject = modifier.object().ok()?;
        }

        subject
            .as_js_call_expression()?
            .has_callee("expect")
            .then_some(preferred)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let matcher = AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher.member_name()?.range(),
                markup! {
                    "This assertion compares the number of calls to "<Emphasis>"0"</Emphasis>"."
                },
            )
            .note(markup! {
                "A comparison to zero is an indirect way to check whether a function was called, and it makes the failure message harder to understand."
            })
            .note(markup! {
                "Use "<Emphasis>{state}</Emphasis>" to check whether the function was called."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        // The matcher ignores extra arguments, but removing them could remove side effects.
        // Rebuilding the assertion discards the tokens after the `expect()` call, so there is
        // no fix when the assertion contains comments.
        if call.arguments().ok()?.args().len() != 1 || call.syntax().has_inner_comments() {
            return None;
        }

        let matcher = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        let object = matcher.object().ok()?;
        let subject = match state {
            PreferredAssertion::Called => AnyJsMemberExpression::cast(object.into_syntax())?
                .object()
                .ok()?,
            PreferredAssertion::NotCalled => AnyJsExpression::from(
                make::js_static_member_expression(
                    object,
                    make::token(T![.]),
                    make::js_name(make::ident("not")).into(),
                ),
            ),
        };
        let callee = make::js_static_member_expression(
            subject,
            make::token(T![.]),
            make::js_name(make::ident("toHaveBeenCalled")).into(),
        );
        let arguments = make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list([], []),
            make::token(T![')']),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            call.clone(),
            make::js_call_expression(callee.into(), arguments).build(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{state}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// The assertion that makes the same check as the reported one, without a call count.
#[derive(Clone, Copy)]
pub enum PreferredAssertion {
    /// `toHaveBeenCalled()`, which replaces `not.toHaveBeenCalledTimes(0)`.
    Called,
    /// `not.toHaveBeenCalled()`, which replaces `toHaveBeenCalledTimes(0)`.
    NotCalled,
}

impl biome_console::fmt::Display for PreferredAssertion {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Called => "toHaveBeenCalled()",
            Self::NotCalled => "not.toHaveBeenCalled()",
        })
    }
}

const CALL_COUNT_MATCHERS: &[&str] = &["toBeCalledTimes", "toHaveBeenCalledTimes"];

const PROMISE_MODIFIERS: &[&str] = &["resolves", "rejects"];

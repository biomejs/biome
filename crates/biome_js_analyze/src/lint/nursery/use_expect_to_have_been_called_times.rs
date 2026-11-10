use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_expect_to_have_been_called_times::UseExpectToHaveBeenCalledTimesOptions;

declare_lint_rule! {
    /// Enforce using `toHaveBeenCalledTimes()` to check how many times a mock function was called.
    ///
    /// A mock function is a replacement for a real function that a test framework creates,
    /// for example with `jest.fn()` or `vi.fn()`. It records every call it receives in its
    /// `mock.calls` array.
    ///
    /// Checking the length of that array with `toHaveLength()` works, but when the test fails,
    /// the error message describes the length of an array. `toHaveBeenCalledTimes()` reports the
    /// expected and received number of calls, which makes failures easier to understand.
    ///
    /// The fix is unsafe because `toHaveBeenCalledTimes()` only accepts mock functions.
    /// A value that isn't a mock function but has its own `mock.calls` array passes
    /// `toHaveLength()` and fails `toHaveBeenCalledTimes()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(onSave.mock.calls).toHaveLength(1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(onSave.mock.calls).not.toHaveLength(0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(onSave).toHaveBeenCalledTimes(1);
    /// expect(onSave).not.toHaveBeenCalledTimes(0);
    /// expect(onSave.mock.calls[0]).toHaveLength(2);
    /// ```
    ///
    pub UseExpectToHaveBeenCalledTimes {
        version: "next",
        name: "useExpectToHaveBeenCalledTimes",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-to-have-been-called-times").same(),
            RuleSource::EslintVitest("prefer-to-have-been-called-times").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExpectToHaveBeenCalledTimes {
    type Query = Ast<JsCallExpression>;
    type State = UseExpectToHaveBeenCalledTimesState;
    type Signals = Option<Self::State>;
    type Options = UseExpectToHaveBeenCalledTimesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let matcher = matcher(ctx.query())?;
        if matcher.member_name()?.text() != "toHaveLength" {
            return None;
        }

        let mut subject = matcher.object().ok()?.omit_parentheses();
        while let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            if !MODIFIERS.contains(&modifier.member_name()?.text()) {
                return None;
            }
            subject = modifier.object().ok()?.omit_parentheses();
        }

        let expect_call = subject.as_js_call_expression()?;
        if !expect_call.has_callee("expect") {
            return None;
        }

        let argument = expect_call.arguments().ok()?.args().first()?.ok()?;
        let calls = AnyJsMemberExpression::cast(
            argument
                .as_any_js_expression()?
                .clone()
                .omit_parentheses()
                .into_syntax(),
        )?;
        if calls.is_optional_chain() || calls.member_name()?.text() != "calls" {
            return None;
        }
        let mock =
            AnyJsMemberExpression::cast(calls.object().ok()?.omit_parentheses().into_syntax())?;
        if mock.is_optional_chain() || mock.member_name()?.text() != "mock" {
            return None;
        }

        let mock_function = mock.object().ok()?;
        // `super` is only valid in front of a property or a call, so there is no way to pass it
        // to `expect()` on its own.
        if mock_function.as_js_super_expression().is_some() {
            return None;
        }

        Some(UseExpectToHaveBeenCalledTimesState {
            argument,
            mock_function,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher(ctx.query())?.member_name()?.range(),
                markup! {
                    <Emphasis>"toHaveLength()"</Emphasis>" is used to check how many times a mock function was called."
                },
            )
            .note(markup! {
                "When this check fails, the error message describes the length of an array instead of the number of calls."
            })
            .note(markup! {
                "Use "<Emphasis>"toHaveBeenCalledTimes()"</Emphasis>" to check the number of calls."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // Replacing the argument keeps the comments before and after it, but not the ones
        // between its tokens.
        if state.argument.syntax().has_inner_comments() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            state.argument.clone(),
            AnyJsCallArgument::AnyJsExpression(state.mock_function.clone()),
        );
        match matcher(ctx.query())? {
            AnyJsMemberExpression::JsStaticMemberExpression(matcher) => {
                mutation.replace_token(
                    matcher.member().ok()?.as_js_name()?.value_token().ok()?,
                    make::ident(PREFERRED_MATCHER),
                );
            }
            AnyJsMemberExpression::JsComputedMemberExpression(matcher) => {
                let name = if ctx.preferred_quote().is_double() {
                    make::js_string_literal(PREFERRED_MATCHER)
                } else {
                    make::js_string_literal_single_quotes(PREFERRED_MATCHER)
                };
                mutation.replace_node(
                    matcher.member().ok()?.omit_parentheses(),
                    AnyJsExpression::AnyJsLiteralExpression(
                        make::js_string_literal_expression(name).into(),
                    ),
                );
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"toHaveBeenCalledTimes()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseExpectToHaveBeenCalledTimesState {
    /// The argument of `expect()` that reads `mock.calls`, such as `fn.mock.calls`.
    argument: AnyJsCallArgument,
    /// The value whose `mock.calls` is read, such as `fn` in `fn.mock.calls`.
    mock_function: AnyJsExpression,
}

/// The properties that can sit between `expect()` and a matcher, such as `not` in
/// `expect(value).not.toBe(other)`.
const MODIFIERS: &[&str] = &["not", "rejects", "resolves"];

const PREFERRED_MATCHER: &str = "toHaveBeenCalledTimes";

/// Returns the matcher that `call` invokes, such as `expect(value).toHaveLength` in
/// `expect(value).toHaveLength(1)`.
///
/// Returns `None` if `call` doesn't call a property of another value.
fn matcher(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

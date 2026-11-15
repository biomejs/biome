use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_expect_to_have_been_called_once::NoExpectToHaveBeenCalledOnceOptions;

declare_lint_rule! {
    /// Disallow `toHaveBeenCalledOnce()` in favor of `toHaveBeenCalledTimes(1)`.
    ///
    /// Vitest provides `toHaveBeenCalledOnce()` and its alias `toBeCalledOnce()` to check that
    /// a mock function (a fake function that records how it is called) was called exactly once.
    /// `toHaveBeenCalledTimes(1)` and `toBeCalledTimes(1)` make the same check.
    ///
    /// `toHaveBeenCalledTimes()` works for any number of calls, so using it for a single call
    /// keeps every check of a call count in the same form.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toHaveBeenCalledOnce();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toBeCalledOnce();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(sendEmail).toHaveBeenCalledTimes(1);
    /// expect(sendEmail).toBeCalledTimes(1);
    /// expect(sendEmail).not.toHaveBeenCalledTimes(1);
    /// ```
    ///
    pub NoExpectToHaveBeenCalledOnce {
        version: "next",
        name: "noExpectToHaveBeenCalledOnce",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintVitest("prefer-called-times").same()],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoExpectToHaveBeenCalledOnce {
    type Query = Ast<JsCallExpression>;
    type State = PreferredMatcher;
    type Signals = Option<Self::State>;
    type Options = NoExpectToHaveBeenCalledOnceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let matcher = matcher(ctx.query())?;
        let preferred = PreferredMatcher::from_matcher_name(matcher.member_name()?.text())?;

        let mut subject = matcher.object().ok()?.omit_parentheses();
        while let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            if !MODIFIERS.contains(&modifier.member_name()?.text()) {
                return None;
            }
            subject = modifier.object().ok()?.omit_parentheses();
        }

        let mut expect = subject.as_js_call_expression()?.callee().ok()?;
        if let Some(variant) = AnyJsMemberExpression::cast_ref(expect.syntax()) {
            if !EXPECT_VARIANTS.contains(&variant.member_name()?.text()) {
                return None;
            }
            expect = variant.object().ok()?;
        }

        expect
            .as_js_reference_identifier()?
            .has_name("expect")
            .then_some(preferred)
    }

    fn diagnostic(ctx: &RuleContext<Self>, preferred: &Self::State) -> Option<RuleDiagnostic> {
        let matcher_name = matcher(ctx.query())?.member_name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher_name.range(),
                markup! {
                    <Emphasis>{matcher_name.text()}"()"</Emphasis>" is used to check that a function was called exactly once."
                },
            )
            .note(markup! {
                <Emphasis>{preferred}"()"</Emphasis>" makes the same check and works for any number of calls, so all checks of a call count look the same."
            })
            .note(markup! {
                "Use "<Emphasis>{preferred}"(1)"</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, preferred: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let arguments = call.arguments().ok()?;
        // The reported matchers ignore their arguments, but removing them could remove
        // side effects.
        if !arguments.args().is_empty() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        match matcher(call)? {
            AnyJsMemberExpression::JsStaticMemberExpression(matcher) => {
                mutation.replace_token(
                    matcher.member().ok()?.as_js_name()?.value_token().ok()?,
                    make::ident(preferred.name()),
                );
            }
            AnyJsMemberExpression::JsComputedMemberExpression(matcher) => {
                let name = if ctx.preferred_quote().is_double() {
                    make::js_string_literal(preferred.name())
                } else {
                    make::js_string_literal_single_quotes(preferred.name())
                };
                mutation.replace_node(
                    matcher.member().ok()?.omit_parentheses(),
                    AnyJsExpression::AnyJsLiteralExpression(
                        make::js_string_literal_expression(name).into(),
                    ),
                );
            }
        }
        mutation.replace_node(
            arguments.clone(),
            make::js_call_arguments(
                arguments.l_paren_token().ok()?,
                make::js_call_argument_list(
                    [AnyJsCallArgument::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            make::js_number_literal_expression(make::js_number_literal(1)).into(),
                        ),
                    )],
                    [],
                ),
                arguments.r_paren_token().ok()?,
            ),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{preferred}"(1)"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// The matcher that checks the number of calls, which replaces the reported matcher when it
/// receives `1`.
#[derive(Clone, Copy)]
pub enum PreferredMatcher {
    ToBeCalledTimes,
    ToHaveBeenCalledTimes,
}

impl PreferredMatcher {
    /// Returns the replacement for the matcher named `name`, or `None` if `name` isn't
    /// a matcher that checks for exactly one call.
    fn from_matcher_name(name: &str) -> Option<Self> {
        match name {
            "toBeCalledOnce" => Some(Self::ToBeCalledTimes),
            "toHaveBeenCalledOnce" => Some(Self::ToHaveBeenCalledTimes),
            _ => None,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::ToBeCalledTimes => "toBeCalledTimes",
            Self::ToHaveBeenCalledTimes => "toHaveBeenCalledTimes",
        }
    }
}

impl biome_console::fmt::Display for PreferredMatcher {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.name())
    }
}

/// The properties that can sit between `expect()` and a matcher, such as `not` in
/// `expect(value).not.toBe(other)`.
const MODIFIERS: &[&str] = &["not", "rejects", "resolves"];

/// The properties of `expect` that create an assertion like `expect()` does, such as `soft` in
/// `expect.soft(value)`.
const EXPECT_VARIANTS: &[&str] = &["poll", "soft"];

/// Returns the callee of `call` if it accesses a member, as in `expect(value).toBe`.
fn matcher(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

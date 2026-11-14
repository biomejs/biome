use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression};
use biome_rowan::AstNode;
use biome_rule_options::use_called_with::UseCalledWithOptions;

declare_lint_rule! {
    /// Enforce using `toHaveBeenCalledWith()` to check the arguments of a function call.
    ///
    /// `toHaveBeenCalled()` and its alias `toBeCalled()` only check that a mock function
    /// (a fake function that records how it is called) was called at least once.
    /// The test still passes when the function receives the wrong arguments.
    /// `toHaveBeenCalledWith()` also checks the arguments, so the test catches more mistakes.
    ///
    /// When the value of an argument is hard to predict, pass a placeholder such as
    /// `expect.anything()` or `expect.any(Date)`. The test then still checks the number
    /// and the position of the arguments. When the function is expected to be called
    /// without arguments, use `toHaveBeenCalledWith()` without arguments.
    ///
    /// This rule also reports `toHaveBeenCalledOnce()` from Vitest, which has
    /// `toHaveBeenCalledExactlyOnceWith()` as the equivalent that checks the arguments.
    ///
    /// Assertions that use `.not` are ignored. For example,
    /// `expect(sendEmail).not.toHaveBeenCalled()` checks that the function was never called,
    /// so there are no arguments to check.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toHaveBeenCalled();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toBeCalled();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(sendEmail).toHaveBeenCalledOnce();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(sendEmail).toHaveBeenCalledWith("user@example.com", expect.anything());
    /// expect(sendEmail).toHaveBeenCalledExactlyOnceWith("user@example.com");
    /// expect(refresh).toHaveBeenCalledWith();
    /// expect(sendEmail).toHaveBeenCalledTimes(1);
    /// expect(sendEmail).not.toHaveBeenCalled();
    /// ```
    ///
    pub UseCalledWith {
        version: "next",
        name: "useCalledWith",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-called-with").same(),
            RuleSource::EslintVitest("prefer-called-with").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for UseCalledWith {
    type Query = Ast<JsCallExpression>;
    type State = PreferredMatcher;
    type Signals = Option<Self::State>;
    type Options = UseCalledWithOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let matcher = matcher(ctx.query())?;
        let preferred = PreferredMatcher::from_matcher_name(matcher.member_name()?.text())?;

        let mut subject = matcher.object().ok()?.omit_parentheses();
        while let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            // `.not` is rejected here as well. A negated assertion mostly states that the
            // function was never called, which leaves no arguments to check.
            if !matches!(modifier.member_name()?.text(), "resolves" | "rejects") {
                return None;
            }
            subject = modifier.object().ok()?.omit_parentheses();
        }

        let mut expect = subject.as_js_call_expression()?.callee().ok()?;
        if let Some(variant) = AnyJsMemberExpression::cast_ref(expect.syntax()) {
            if !matches!(variant.member_name()?.text(), "soft" | "poll") {
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
                    <Emphasis>{matcher_name.text()}"()"</Emphasis>" doesn't check the arguments that the function was called with."
                },
            )
            .note(markup! {
                "The test passes even when the function receives the wrong arguments."
            })
            .note(markup! {
                "Use "<Emphasis>{preferred}"()"</Emphasis>" and pass the expected arguments. For an argument that is hard to predict, pass "<Emphasis>"expect.anything()"</Emphasis>"."
            }),
        )
    }
}

/// The matcher that checks the arguments of the calls, in addition to what the reported
/// matcher checks.
#[derive(Clone, Copy)]
pub enum PreferredMatcher {
    ToBeCalledWith,
    ToHaveBeenCalledWith,
    ToHaveBeenCalledExactlyOnceWith,
}

impl PreferredMatcher {
    /// Returns the replacement for the matcher named `name`, or `None` if `name` isn't
    /// a matcher that ignores the arguments of the calls.
    fn from_matcher_name(name: &str) -> Option<Self> {
        match name {
            "toBeCalled" => Some(Self::ToBeCalledWith),
            "toHaveBeenCalled" => Some(Self::ToHaveBeenCalledWith),
            "toHaveBeenCalledOnce" => Some(Self::ToHaveBeenCalledExactlyOnceWith),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for PreferredMatcher {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::ToBeCalledWith => "toBeCalledWith",
            Self::ToHaveBeenCalledWith => "toHaveBeenCalledWith",
            Self::ToHaveBeenCalledExactlyOnceWith => "toHaveBeenCalledExactlyOnceWith",
        })
    }
}

/// Returns the callee of `call` if it accesses a member, as in `expect(value).toBe`.
fn matcher(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

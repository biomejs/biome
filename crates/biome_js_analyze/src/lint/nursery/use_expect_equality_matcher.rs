use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsBinaryExpression,
    JsBinaryOperator, JsCallExpression, T,
};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_expect_equality_matcher::UseExpectEqualityMatcherOptions;

declare_lint_rule! {
    /// Enforce using an equality matcher instead of comparing values with `===` or `!==` inside `expect()`.
    ///
    /// Passing `a === b` to `expect()` and checking the result with `toBe(true)` works, but when
    /// the test fails, the error message only says that `true` was expected and `false` was
    /// received. Passing `a` to `expect()` and `b` to an equality matcher, such as `toBe()`,
    /// `toEqual()`, or `toStrictEqual()`, makes the error message show both values.
    ///
    /// The fix uses `toBe()`, which is the matcher closest to `===`. It is unsafe because
    /// `toBe()` and `===` don't compare every value in the same way. For example,
    /// `NaN === NaN` is `false`, while `expect(NaN).toBe(NaN)` passes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(total === 10).toBe(true);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(name !== "admin").toEqual(true);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(user.id === 1).not.toStrictEqual(false);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(total).toBe(10);
    /// expect(name).not.toEqual("admin");
    /// expect(user.id).toStrictEqual(1);
    /// ```
    ///
    pub UseExpectEqualityMatcher {
        version: "2.6.9",
        name: "useExpectEqualityMatcher",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-equality-matcher").same(),
            RuleSource::EslintVitest("prefer-equality-matcher").same(),
            RuleSource::EslintPlaywright("prefer-equality-matcher").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExpectEqualityMatcher {
    type Query = Ast<JsCallExpression>;
    type State = UseExpectEqualityMatcherState;
    type Signals = Option<Self::State>;
    type Options = UseExpectEqualityMatcherOptions;

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
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax())
            && modifier.member_name()?.text() == "not"
        {
            has_not = true;
            subject = modifier.object().ok()?;
        }
        let mut promise_modifier = None;
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            promise_modifier = Some(match modifier.member_name()?.text() {
                "resolves" => PromiseModifier::Resolves,
                "rejects" => PromiseModifier::Rejects,
                _ => return None,
            });
            subject = modifier.object().ok()?;
        }

        let expect_call = subject.as_js_call_expression()?;
        if !expect_call.has_callee("expect") {
            return None;
        }

        let comparison = expect_call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_js_binary_expression()?
            .clone();
        let is_equal = match comparison.operator().ok()? {
            JsBinaryOperator::StrictEquality => true,
            JsBinaryOperator::StrictInequality => false,
            _ => return None,
        };

        Some(UseExpectEqualityMatcherState {
            expect_call: expect_call.clone(),
            comparison,
            promise_modifier,
            negated: (expected == is_equal) == has_not,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let matcher = AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())?;
        let operator = state.comparison.operator_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher.member_name()?.range(),
                markup! {
                    "The result of a "<Emphasis>{operator.text_trimmed()}</Emphasis>" comparison is checked against a boolean."
                },
            )
            .note(markup! {
                "When this check fails, the error message only shows that a boolean didn't match, not the values that were compared."
            })
            .note(if state.negated {
                markup! {
                    "Pass the left value to "<Emphasis>"expect()"</Emphasis>" and the right value to a negated equality matcher such as "<Emphasis>".not.toBe()"</Emphasis>", "<Emphasis>".not.toEqual()"</Emphasis>", or "<Emphasis>".not.toStrictEqual()"</Emphasis>"."
                }
            } else {
                markup! {
                    "Pass the left value to "<Emphasis>"expect()"</Emphasis>" and the right value to an equality matcher such as "<Emphasis>"toBe()"</Emphasis>", "<Emphasis>"toEqual()"</Emphasis>", or "<Emphasis>"toStrictEqual()"</Emphasis>"."
                }
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        // Rebuilding the assertion discards the tokens around the comparison and the modifiers,
        // along with any comments attached to them.
        if call.syntax().has_inner_comments() {
            return None;
        }
        let left = state.comparison.left().ok()?.trim_trailing_trivia()?;
        let right = state.comparison.right().ok()?.trim_leading_trivia()?;

        // Replaces the whole argument so that parentheses around the comparison are removed too.
        let expect_argument = state.expect_call.arguments().ok()?.args().first()?.ok()?;
        let expect_call = state
            .expect_call
            .clone()
            .replace_node(expect_argument, AnyJsCallArgument::AnyJsExpression(left))?;
        let mut subject = AnyJsExpression::from(expect_call);
        if let Some(modifier) = state.promise_modifier {
            subject = static_member(subject, modifier.name());
        }
        if state.negated {
            subject = static_member(subject, "not");
        }
        let callee = static_member(subject, "toBe");

        let arguments = call
            .arguments()
            .ok()?
            .replace_node(only_argument(call)?, AnyJsCallArgument::AnyJsExpression(right))?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            call.clone(),
            make::js_call_expression(callee, arguments).build(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"toBe()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseExpectEqualityMatcherState {
    /// The `expect(...)` call whose first argument is `comparison`.
    expect_call: JsCallExpression,
    /// The `a === b` or `a !== b` comparison passed to `expect()`.
    comparison: JsBinaryExpression,
    /// The `.resolves` or `.rejects` modifier of the assertion, if any.
    promise_modifier: Option<PromiseModifier>,
    /// Whether the equivalent equality assertion needs a `.not` modifier.
    negated: bool,
}

#[derive(Clone, Copy)]
pub enum PromiseModifier {
    Resolves,
    Rejects,
}

impl PromiseModifier {
    const fn name(self) -> &'static str {
        match self {
            Self::Resolves => "resolves",
            Self::Rejects => "rejects",
        }
    }
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

/// Builds the member access `object.name`.
fn static_member(object: AnyJsExpression, name: &str) -> AnyJsExpression {
    make::js_static_member_expression(
        object,
        make::token(T![.]),
        make::js_name(make::ident(name)).into(),
    )
    .into()
}

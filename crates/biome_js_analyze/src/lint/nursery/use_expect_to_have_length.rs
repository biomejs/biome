use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsOptionalChainExpression, JsCallExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_expect_to_have_length::UseExpectToHaveLengthOptions;

declare_lint_rule! {
    /// Enforce using `toHaveLength()` to check the length of an array or string.
    ///
    /// Checking the `length` property with `toBe()`, `toEqual()`, or `toStrictEqual()` works,
    /// but when the test fails, the error message only shows the two numbers that didn't match.
    /// `toHaveLength()` also shows the value whose length was checked, which makes failures
    /// easier to understand.
    ///
    /// The fix is unsafe because `toHaveLength()` fails when the value doesn't have a `length`
    /// property that is a number, and an equality check doesn't. For example,
    /// `expect({}.length).not.toBe(1)` passes, while `expect({}).not.toHaveLength(1)` fails.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect(files.length).toBe(1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(files.length).toEqual(1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(files.length).not.toStrictEqual(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect(files).toHaveLength(1);
    /// expect(files).not.toHaveLength(1);
    /// expect(files.name).toBe("report.txt");
    /// ```
    ///
    pub UseExpectToHaveLength {
        version: "next",
        name: "useExpectToHaveLength",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-to-have-length").same(),
            RuleSource::EslintVitest("prefer-to-have-length").same(),
            RuleSource::EslintPlaywright("prefer-to-have-length").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExpectToHaveLength {
    type Query = Ast<JsCallExpression>;
    type State = UseExpectToHaveLengthState;
    type Signals = Option<Self::State>;
    type Options = UseExpectToHaveLengthOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let matcher = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        if !EQUALITY_MATCHERS.contains(&matcher.member_name()?.text()) {
            return None;
        }

        let expected_arguments = call.arguments().ok()?.args();
        if expected_arguments.len() != 1 {
            return None;
        }
        expected_arguments.first()?.ok()?.as_any_js_expression()?;

        let mut subject = matcher.object().ok()?;
        if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax()) {
            if modifier.member_name()?.text() != "not" {
                return None;
            }
            subject = modifier.object().ok()?;
        }

        let expect_call = subject.as_js_call_expression()?;
        if !is_expect_call(expect_call) {
            return None;
        }

        let argument = expect_call
            .arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone();
        let length_access =
            AnyJsMemberExpression::cast(argument.clone().omit_type_wrappers()?.into_syntax())?;
        // `toHaveLength()` fails on a missing value, which `value?.length` allows.
        if length_access.member_name()?.text() != "length" || is_optional_chain(&length_access) {
            return None;
        }
        // `super` is only valid as the object of a member access, so it can't be passed to
        // `expect()` on its own.
        if matches!(
            length_access.object().ok()?,
            AnyJsExpression::JsSuperExpression(_)
        ) {
            return None;
        }

        Some(UseExpectToHaveLengthState {
            argument,
            length_access,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let matcher = AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())?;
        let matcher_name = matcher.member_name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher_name.range(),
                markup! {
                    "The "<Emphasis>"length"</Emphasis>" property is checked with "<Emphasis>{matcher_name.text()}"()"</Emphasis>"."
                },
            )
            .note(markup! {
                "When this check fails, the error message only shows the two numbers that didn't match, not the value whose length was checked."
            })
            .note(markup! {
                "Use "<Emphasis>"toHaveLength()"</Emphasis>" to check the length of a value."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let object = state.length_access.object().ok()?;
        // Only `object` is kept from the argument of `expect()`, so a comment attached to any
        // other token of the argument would be removed.
        let object_range = object.syntax().text_range_with_trivia();
        if state
            .argument
            .syntax()
            .borrowed_tokens()
            .filter(|token| !object_range.contains_range(token.text_trimmed_range()))
            .any(|token| {
                token
                    .leading_trivia_pieces()
                    .chain(token.trailing_trivia_pieces())
                    .any(|piece| piece.kind().is_comment())
            })
        {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(state.argument.clone(), object);

        match AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())? {
            AnyJsMemberExpression::JsStaticMemberExpression(matcher) => {
                mutation.replace_token(
                    matcher.member().ok()?.as_js_name()?.value_token().ok()?,
                    make::ident(TO_HAVE_LENGTH),
                );
            }
            AnyJsMemberExpression::JsComputedMemberExpression(matcher) => {
                let name = if ctx.preferred_quote().is_double() {
                    make::js_string_literal(TO_HAVE_LENGTH)
                } else {
                    make::js_string_literal_single_quotes(TO_HAVE_LENGTH)
                };
                mutation.replace_node(
                    matcher.member().ok()?,
                    AnyJsExpression::AnyJsLiteralExpression(
                        make::js_string_literal_expression(name).into(),
                    ),
                );
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"toHaveLength()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseExpectToHaveLengthState {
    /// The first argument of `expect()`, including the parentheses and TypeScript type syntax
    /// around `length_access`.
    argument: AnyJsExpression,
    /// The `value.length` access passed to `expect()`.
    length_access: AnyJsMemberExpression,
}

const EQUALITY_MATCHERS: &[&str] = &["toBe", "toEqual", "toStrictEqual"];

const TO_HAVE_LENGTH: &str = "toHaveLength";

/// Returns `true` if `call` is `expect(...)` or `expect.soft(...)`.
fn is_expect_call(call: &JsCallExpression) -> bool {
    if call.has_callee("expect") {
        return true;
    }
    call.callee()
        .ok()
        .and_then(|callee| AnyJsMemberExpression::cast(callee.into_syntax()))
        .is_some_and(|callee| {
            callee.member_name().is_some_and(|name| name.text() == "soft")
                && callee.object().is_ok_and(|object| {
                    object
                        .as_js_reference_identifier()
                        .is_some_and(|identifier| identifier.has_name("expect"))
                })
        })
}

/// Returns `true` if `member`, or an expression it is accessed on, uses `?.`.
///
/// Parentheses and TypeScript type syntax in the chain are looked through, so that
/// `value?.items!.length` counts as an optional chain.
fn is_optional_chain(member: &AnyJsMemberExpression) -> bool {
    let mut current = AnyJsOptionalChainExpression::cast_ref(member.syntax());
    while let Some(expression) = current {
        if expression.is_optional() {
            return true;
        }
        current = expression
            .object()
            .ok()
            .and_then(AnyJsExpression::omit_type_wrappers)
            .and_then(|object| AnyJsOptionalChainExpression::cast(object.into_syntax()));
    }
    false
}

use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsMemberExpression, JsCallExpression, JsSyntaxKind, static_value::StaticValue,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_expect_to_strict_equal::UseExpectToStrictEqualOptions;

declare_lint_rule! {
    /// Enforce using `toStrictEqual()` instead of `toEqual()` to compare values in tests.
    ///
    /// `toEqual()` ignores some differences between the two values it compares:
    ///
    /// - A property set to `undefined` is treated the same as a property that doesn't exist.
    /// - An empty slot in an array is treated the same as an item set to `undefined`.
    /// - An object created from a class is treated the same as a plain object with the same
    ///   properties.
    ///
    /// `toEqual()` can therefore treat two different values as equal. `toStrictEqual()` checks
    /// all of these differences.
    ///
    /// The fix is unsafe because it can change the result of a test.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// expect({ a: "a", b: undefined }).toEqual({ a: "a" });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// expect(user).not.toEqual(otherUser);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await expect(loadUser()).resolves.toEqual({ name: "Ada" });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// expect({ a: "a", b: undefined }).toStrictEqual({ a: "a", b: undefined });
    /// expect(user).not.toStrictEqual(otherUser);
    /// await expect(loadUser()).resolves.toStrictEqual({ name: "Ada" });
    /// ```
    ///
    pub UseExpectToStrictEqual {
        version: "next",
        name: "useExpectToStrictEqual",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-strict-equal").same(),
            RuleSource::EslintVitest("prefer-strict-equal").same(),
            RuleSource::EslintPlaywright("prefer-strict-equal").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExpectToStrictEqual {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseExpectToStrictEqualOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let matcher = matcher(ctx.query())?;
        if matcher.member_name()?.text() != "toEqual" {
            return None;
        }

        // The modifiers are read from the matcher back to `expect()`, so `not` comes before
        // `resolves` and `rejects`: `expect(value).resolves.not.toEqual(expected)`.
        let mut subject = matcher.object().ok()?.omit_parentheses();
        for modifiers in [NEGATION_MODIFIERS, PROMISE_MODIFIERS] {
            if let Some(modifier) = AnyJsMemberExpression::cast_ref(subject.syntax())
                && modifiers.contains(&modifier.member_name()?.text())
            {
                subject = modifier.object().ok()?.omit_parentheses();
            }
        }

        let mut callee = subject
            .as_js_call_expression()?
            .callee()
            .ok()?
            .omit_parentheses();
        while let Some(variant) = AnyJsMemberExpression::cast_ref(callee.syntax()) {
            if !EXPECT_VARIANTS.contains(&variant.member_name()?.text()) {
                return None;
            }
            callee = variant.object().ok()?.omit_parentheses();
        }

        callee
            .as_js_reference_identifier()?
            .has_name("expect")
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                matcher(ctx.query())?.member_name()?.range(),
                markup! {
                    <Emphasis>"toEqual()"</Emphasis>" ignores some differences between the values it compares."
                },
            )
            .note(markup! {
                "It treats a property set to "<Emphasis>"undefined"</Emphasis>" the same as a missing property, and an object created from a class the same as a plain object, so it can treat two different values as equal."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let StaticValue::String(name_token) = matcher(ctx.query())?.member_name()? else {
            return None;
        };
        let new_name_token = match name_token.kind() {
            JsSyntaxKind::JS_STRING_LITERAL if name_token.text_trimmed().starts_with('\'') => {
                make::js_string_literal_single_quotes("toStrictEqual")
            }
            JsSyntaxKind::JS_STRING_LITERAL => make::js_string_literal("toStrictEqual"),
            JsSyntaxKind::TEMPLATE_CHUNK => make::js_template_chunk("toStrictEqual"),
            _ => make::ident("toStrictEqual"),
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(name_token, new_name_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"toStrictEqual()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// The modifier that inverts a matcher, as in `expect(value).not.toEqual(expected)`.
const NEGATION_MODIFIERS: &[&str] = &["not"];

/// The modifiers that wait for a promise before the matcher runs, as in
/// `expect(promise).resolves.toEqual(expected)`.
const PROMISE_MODIFIERS: &[&str] = &["rejects", "resolves"];

/// The members of `expect` that are called like `expect()` itself and return the same
/// matchers, as in `expect.soft(value).toEqual(expected)`.
const EXPECT_VARIANTS: &[&str] = &["element", "poll", "soft"];

/// Returns the member expression that `call` calls, such as `expect(value).toEqual` in
/// `expect(value).toEqual(expected)`.
fn matcher(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

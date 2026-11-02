use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression, JsExpressionStatement};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_ignored_unsubscribe::NoSvelteIgnoredUnsubscribeOptions;

declare_lint_rule! {
    /// Disallow ignoring the function returned by a store's `subscribe()` method.
    ///
    /// Calling `subscribe()` on a Svelte store returns a function that stops the subscription.
    /// If you don't keep this function, you can never stop the subscription: the store keeps
    /// calling your callback and keeps it in memory after you no longer need it. This is a
    /// memory leak.
    ///
    /// The rule reports a `subscribe()` call when its result is not used, for example when it
    /// isn't saved in a variable, returned, or passed to another function.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// import { count } from "./stores.js";
    ///
    /// count.subscribe((value) => {
    ///     console.log(value);
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// import { onDestroy } from "svelte";
    /// import { count } from "./stores.js";
    ///
    /// const unsubscribe = count.subscribe((value) => {
    ///     console.log(value);
    /// });
    ///
    /// onDestroy(unsubscribe);
    /// </script>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte stores](https://svelte.dev/docs/svelte/stores)
    pub NoSvelteIgnoredUnsubscribe {
        version: "next",
        name: "noSvelteIgnoredUnsubscribe",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-ignored-unsubscribe").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoSvelteIgnoredUnsubscribe {
    type Query = Ast<JsExpressionStatement>;
    /// The `.subscribe` member expression that is called.
    type State = AnyJsMemberExpression;
    type Signals = Option<Self::State>;
    type Options = NoSvelteIgnoredUnsubscribeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte()
        {
            return None;
        }

        let expression = ctx.query().expression().ok()?.inner_expression()?;
        let call = JsCallExpression::cast(expression.into_syntax())?;
        let callee = call.callee().ok()?.omit_type_wrappers()?;
        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        (member.member_name()?.text() == "subscribe").then_some(member)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, member: &Self::State) -> Option<RuleDiagnostic> {
        let range = match member {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The function returned by "<Emphasis>"subscribe()"</Emphasis>" is ignored."
                },
            )
            .note(markup! {
                "Without this function, the subscription can't be stopped, so the store keeps calling the callback and keeps it in memory after it's no longer needed."
            })
            .note(markup! {
                "Save the returned function and call it when you no longer need the subscription, for example when the component is destroyed."
            }),
        )
    }
}

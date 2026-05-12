use crate::{frameworks::vue::vue_call::is_vue_api_reference, services::semantic::Semantic};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_next_tick_promise::UseVueNextTickPromiseOptions;

declare_lint_rule! {
    /// Enforces Promise syntax when using Vue `nextTick`.
    ///
    /// Vue `nextTick` returns a Promise when no callback is passed. Promise syntax composes better with `await` and keeps asynchronous control flow explicit.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { nextTick } from "vue";
    ///
    /// nextTick(() => {
    ///   // ...
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { nextTick } from "vue";
    ///
    /// await nextTick();
    /// // ...
    /// </script>
    /// ```
    ///
    pub UseVueNextTickPromise {
        version: "2.4.15",
        name: "useVueNextTickPromise",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("next-tick-style").inspired()],
    }
}

impl Rule for UseVueNextTickPromise {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseVueNextTickPromiseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let arguments = call.arguments().ok()?;
        let Some(Ok(AnyJsCallArgument::AnyJsExpression(first_argument))) =
            arguments.args().into_iter().next()
        else {
            return None;
        };

        if !is_function_expression(&first_argument) {
            return None;
        }

        let callee = call.callee().ok()?.inner_expression()?;
        if is_vue_api_reference(&callee, ctx.model(), "nextTick")
            || is_vue_instance_next_tick(&callee).unwrap_or_default()
        {
            return Some(call.syntax().text_trimmed_range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    "Vue `nextTick` was called with a callback."
                },
            )
            .note(markup! {
                "Promise syntax composes better with `await` and keeps asynchronous control flow explicit."
            })
            .note(markup! {
                "Call `nextTick` without a callback, then await it or chain `.then()` instead."
            }),
        )
    }
}

fn is_function_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    )
}

fn is_vue_instance_next_tick(expression: &AnyJsExpression) -> Option<bool> {
    let member_expression = AnyJsMemberExpression::cast_ref(expression.syntax())?;
    let member_name = member_expression.member_name()?;
    if member_name.text() != "$nextTick" {
        return Some(false);
    }
    let object = member_expression.object().ok()?;
    Some(matches!(
        object.omit_parentheses(),
        AnyJsExpression::JsThisExpression(_)
    ))
}

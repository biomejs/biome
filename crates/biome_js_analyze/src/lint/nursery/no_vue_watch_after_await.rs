use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsArrayElementList, JsAssignmentExpression, JsCallArgumentList,
    JsCallExpression, JsInitializerClause, JsLanguage, JsPropertyObjectMember,
};
use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::no_vue_watch_after_await::NoVueWatchAfterAwaitOptions;

use crate::frameworks::vue::vue_call::vue_api_name;
use crate::frameworks::vue::vue_component::{VueComponent, VueComponentQuery};
use crate::frameworks::vue::vue_setup::{
    VueSetupCallAfterAwait, find_vue_setup_calls_after_await, vue_setup_function_body,
};

declare_lint_rule! {
    /// Disallow creating Vue watchers after an `await` in `setup()`.
    ///
    /// A watcher created with `watch()` or `watchEffect()` belongs to the component whose
    /// `setup()` function is currently running, and Vue stops it when that component is removed
    /// from the page. When `setup()` pauses at an `await`, Vue stops tracking which component is
    /// being set up. A watcher created after that point doesn't belong to any component, so it
    /// keeps running after the component is gone.
    ///
    /// A watcher is allowed when the function it returns is stored or passed somewhere, for example
    /// `const stop = watch(...)`, because that function can be called later to stop the watcher.
    ///
    /// Code inside `<script setup>` isn't checked, because Vue keeps track of the component
    /// across each `await` there.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { watch } from "vue";
    ///
    /// export default {
    ///   async setup() {
    ///     await fetchUser();
    ///     watch(source, () => {});
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { watch } from "vue";
    ///
    /// export default {
    ///   async setup() {
    ///     watch(source, () => {});
    ///     await fetchUser();
    ///     const stop = watch(source, () => {});
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueWatchAfterAwait {
        version: "next",
        name: "noVueWatchAfterAwait",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-watch-after-await").same()],
    }
}

impl Rule for NoVueWatchAfterAwait {
    type Query = VueComponentQuery;
    type State = VueSetupCallAfterAwait;
    type Signals = Box<[Self::State]>;
    type Options = NoVueWatchAfterAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };

        match vue_setup_function_body(component.kind()) {
            Some(body) => find_vue_setup_calls_after_await(&body, |call| {
                is_unstoppable_watcher_call(call, ctx.model())
            }),
            None => [].into(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.call.range(),
                markup! {
                    "This watcher is created after an "<Emphasis>"await"</Emphasis>", so it keeps running after the component is removed."
                },
            )
            .detail(
                state.first_await,
                markup! {
                    "The "<Emphasis>"setup()"</Emphasis>" function pauses here. After this point, Vue no longer knows which component the watcher belongs to."
                },
            )
            .note(markup! {
                "Create the watcher before the first "<Emphasis>"await"</Emphasis>" in "<Emphasis>"setup()"</Emphasis>", or store the function it returns and call that function to stop the watcher."
            }),
        )
    }
}

/// Returns `true` if `call` creates a Vue watcher and the function that stops the watcher is
/// discarded.
fn is_unstoppable_watcher_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    let Some(name) = vue_api_name(&callee, model) else {
        return false;
    };
    if !matches!(name.text(), "watch" | "watchEffect") {
        return false;
    }
    let parent = AnyJsExpression::from(call.clone())
        .outer_expression()
        .and_then(|expression| expression.syntax().parent());
    !parent.is_some_and(|parent| STOP_HANDLE_USE_PARENT_KINDS.matches(parent.kind()))
}

/// Parents of a watcher call through which the returned stop function can still be called:
/// `const stop = watch()`, `stop = watch()`, `register(watch())`, `watch()()`,
/// `{ stop: watch() }`, and `[watch()]`.
const STOP_HANDLE_USE_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsInitializerClause::KIND_SET
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsCallArgumentList::KIND_SET)
    .union(JsCallExpression::KIND_SET)
    .union(JsPropertyObjectMember::KIND_SET)
    .union(JsArrayElementList::KIND_SET);

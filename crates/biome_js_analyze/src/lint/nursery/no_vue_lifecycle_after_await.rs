use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember, JsAwaitExpression, JsCallExpression,
    JsForOfStatement, WalkEvent,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_lifecycle_after_await::NoVueLifecycleAfterAwaitOptions;

use crate::frameworks::vue::vue_call::vue_api_name;
use crate::frameworks::vue::vue_component::{
    AnyVueComponent, VueComponent, VueComponentQuery, VueOptionsApiBasedComponent,
};
use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow registering Vue lifecycle hooks after an `await` in `setup()`.
    ///
    /// Lifecycle hooks, such as `onMounted()` and `onUnmounted()`, attach code to the component
    /// whose `setup()` function is currently running. When `setup()` pauses at an `await`, Vue
    /// stops tracking which component is being set up. A lifecycle hook called after that point
    /// isn't attached to the component, so its code never runs.
    ///
    /// This rule reports calls to these functions imported from `vue` that come after an `await`
    /// in a component's `setup()` function: `onActivated`, `onBeforeMount`, `onBeforeUnmount`,
    /// `onBeforeUpdate`, `onDeactivated`, `onErrorCaptured`, `onMounted`, `onRenderTracked`,
    /// `onRenderTriggered`, `onUnmounted`, and `onUpdated`.
    ///
    /// A call that passes the component as the second argument is allowed, because it doesn't
    /// depend on the component that is currently being set up.
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
    /// import { onMounted } from "vue";
    ///
    /// export default {
    ///   async setup() {
    ///     await fetchUser();
    ///     onMounted(() => {});
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { onMounted } from "vue";
    ///
    /// export default {
    ///   async setup() {
    ///     onMounted(() => {});
    ///     await fetchUser();
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueLifecycleAfterAwait {
        version: "next",
        name: "noVueLifecycleAfterAwait",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-lifecycle-after-await").same()],
    }
}

impl Rule for NoVueLifecycleAfterAwait {
    type Query = VueComponentQuery;
    type State = LifecycleHookAfterAwait;
    type Signals = Box<[Self::State]>;
    type Options = NoVueLifecycleAfterAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };

        let setup_body = match component.kind() {
            AnyVueComponent::OptionsApi(component) => setup_function_body(component),
            AnyVueComponent::CreateApp(component) => setup_function_body(component),
            AnyVueComponent::DefineComponent(component) => setup_function_body(component),
            AnyVueComponent::Setup(_) => None,
        };

        match setup_body {
            Some(body) => find_hooks_after_await(&body, ctx.model()),
            None => [].into(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.hook.range(),
                markup! {
                    "This lifecycle hook is registered after an "<Emphasis>"await"</Emphasis>", so it never runs."
                },
            )
            .detail(
                state.first_await,
                markup! {
                    "The "<Emphasis>"setup()"</Emphasis>" function pauses here. After this point, Vue no longer knows which component the hook belongs to."
                },
            )
            .note(markup! {
                "Call the lifecycle hook before the first "<Emphasis>"await"</Emphasis>" in "<Emphasis>"setup()"</Emphasis>"."
            }),
        )
    }
}

pub struct LifecycleHookAfterAwait {
    hook: JsCallExpression,
    /// The range of the first `await` keyword in `setup()` that pauses before the hook is called.
    first_await: TextRange,
}

/// Returns the body of the function in the `setup` property of the component's options object.
fn setup_function_body(component: &impl VueOptionsApiBasedComponent) -> Option<AnyJsFunctionBody> {
    let (_, member) = component
        .iter_declaration_groups()
        .find(|(name, _)| name.text() == "setup")?;
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            method.body().ok().map(AnyJsFunctionBody::JsFunctionBody)
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            match property.value().ok()?.omit_parentheses() {
                AnyJsExpression::JsFunctionExpression(function) => {
                    function.body().ok().map(AnyJsFunctionBody::JsFunctionBody)
                }
                AnyJsExpression::JsArrowFunctionExpression(function) => function.body().ok(),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Finds the lifecycle hook calls in `body` that run after `setup()` pauses at an `await`.
///
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of `setup()`.
fn find_hooks_after_await(
    body: &AnyJsFunctionBody,
    model: &SemanticModel,
) -> Box<[LifecycleHookAfterAwait]> {
    let mut first_await = None;
    let mut hooks = Vec::new();
    let mut preorder = body.syntax().preorder();
    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => {
                if AnyJsControlFlowRoot::KIND_SET.matches(node.kind()) {
                    preorder.skip_subtree();
                } else if first_await.is_none()
                    && let Some(for_of) = JsForOfStatement::cast(node)
                {
                    first_await = for_of.await_token().map(|token| token.text_trimmed_range());
                }
            }
            // Leave events come after the node's children, which matches evaluation order:
            // `setup()` pauses only once the awaited value has been computed, and a function is
            // called only once its arguments have been computed. So `await onMounted()` registers
            // the hook before pausing, while `onMounted(await callback())` registers it after.
            WalkEvent::Leave(node) => {
                if let Some(first_await) = first_await {
                    if let Some(call) = JsCallExpression::cast(node)
                        && is_lifecycle_hook_call(&call, model)
                    {
                        hooks.push(LifecycleHookAfterAwait {
                            hook: call,
                            first_await,
                        });
                    }
                } else if let Some(await_expression) = JsAwaitExpression::cast(node) {
                    first_await = await_expression
                        .await_token()
                        .ok()
                        .map(|token| token.text_trimmed_range());
                }
            }
        }
    }
    hooks.into_boxed_slice()
}

/// Returns `true` if `call` registers a lifecycle hook for the component that is currently being
/// set up.
fn is_lifecycle_hook_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    let Some(name) = vue_api_name(&callee, model) else {
        return false;
    };
    if LIFECYCLE_HOOKS.binary_search(&name.text()).is_err() {
        return false;
    }
    // The second argument is the component instance to attach the hook to.
    call.arguments()
        .is_ok_and(|arguments| arguments.args().len() < 2)
}

/// Vue functions that attach a lifecycle hook to a component, sorted for binary search.
const LIFECYCLE_HOOKS: &[&str] = &[
    "onActivated",
    "onBeforeMount",
    "onBeforeUnmount",
    "onBeforeUpdate",
    "onDeactivated",
    "onErrorCaptured",
    "onMounted",
    "onRenderTracked",
    "onRenderTriggered",
    "onUnmounted",
    "onUpdated",
];

#[cfg(test)]
mod tests {
    use super::LIFECYCLE_HOOKS;

    #[test]
    fn lifecycle_hooks_are_sorted() {
        for pair in LIFECYCLE_HOOKS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come after {} in LIFECYCLE_HOOKS",
                pair[0],
                pair[1]
            );
        }
    }
}

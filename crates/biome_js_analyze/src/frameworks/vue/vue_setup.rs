//! Helpers for inspecting the `setup()` function of Vue components that are defined with an
//! options object.

use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember, JsAwaitExpression, JsCallExpression,
    JsForOfStatement, WalkEvent,
};
use biome_rowan::{AstNode, TextRange};

use crate::frameworks::vue::vue_component::{AnyVueComponent, VueOptionsApiBasedComponent};
use crate::services::control_flow::AnyJsControlFlowRoot;

/// A call in `setup()` that runs after `setup()` pauses at an `await`.
pub struct VueSetupCallAfterAwait {
    pub call: JsCallExpression,
    /// The range of the first `await` keyword in `setup()` that pauses before the call runs.
    pub first_await: TextRange,
}

/// Returns the body of the function in the `setup` property of the component's options object.
///
/// Returns `None` for `<script setup>` components, because they have no `setup()` function.
pub fn vue_setup_function_body(component: &AnyVueComponent) -> Option<AnyJsFunctionBody> {
    match component {
        AnyVueComponent::OptionsApi(component) => setup_function_body(component),
        AnyVueComponent::CreateApp(component) => setup_function_body(component),
        AnyVueComponent::DefineComponent(component) => setup_function_body(component),
        AnyVueComponent::Setup(_) => None,
    }
}

/// Finds the calls in `body` that match `predicate` and run after `setup()` pauses at an `await`.
///
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of `setup()`.
pub fn find_vue_setup_calls_after_await(
    body: &AnyJsFunctionBody,
    mut predicate: impl FnMut(&JsCallExpression) -> bool,
) -> Box<[VueSetupCallAfterAwait]> {
    let mut first_await = None;
    let mut calls = Vec::new();
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
                        && predicate(&call)
                    {
                        calls.push(VueSetupCallAfterAwait { call, first_await });
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
    calls.into_boxed_slice()
}

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

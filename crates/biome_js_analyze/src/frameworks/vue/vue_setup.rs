use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember, JsAwaitExpression, JsCallExpression,
    JsForOfStatement, JsLanguage, WalkEvent,
};
use biome_rowan::{AstNode, TextRange, syntax::Preorder};

use crate::frameworks::vue::vue_component::{AnyVueComponent, VueOptionsApiBasedComponent};
use crate::services::control_flow::AnyJsControlFlowRoot;

/// Returns the body of the function in the `setup` property of the component's options object.
///
/// Returns `None` for `<script setup>`, which has no `setup()` function.
pub fn setup_function_body(component: &AnyVueComponent) -> Option<AnyJsFunctionBody> {
    match component {
        AnyVueComponent::OptionsApi(component) => options_setup_function_body(component),
        AnyVueComponent::CreateApp(component) => options_setup_function_body(component),
        AnyVueComponent::DefineComponent(component) => options_setup_function_body(component),
        AnyVueComponent::Setup(_) => None,
    }
}

/// Iterates over the calls in a `setup()` function body that run after `setup()` pauses at an
/// `await`.
///
/// Each item is a call along with the range of the first `await` keyword that pauses before it.
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of `setup()`.
pub struct CallsAfterAwait {
    preorder: Preorder<JsLanguage>,
    first_await: Option<TextRange>,
}

impl CallsAfterAwait {
    pub fn new(body: &AnyJsFunctionBody) -> Self {
        Self {
            preorder: body.syntax().preorder(),
            first_await: None,
        }
    }
}

impl Iterator for CallsAfterAwait {
    type Item = (JsCallExpression, TextRange);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.preorder.next() {
            match event {
                WalkEvent::Enter(node) => {
                    if AnyJsControlFlowRoot::KIND_SET.matches(node.kind()) {
                        self.preorder.skip_subtree();
                    } else if self.first_await.is_none()
                        && let Some(for_of) = JsForOfStatement::cast(node)
                    {
                        self.first_await =
                            for_of.await_token().map(|token| token.text_trimmed_range());
                    }
                }
                // Leave events come after the node's children, which matches evaluation order:
                // `setup()` pauses only once the awaited value has been computed, and a function
                // is called only once its arguments have been computed. So `await f()` calls `f`
                // before pausing, while `f(await g())` calls `f` after.
                WalkEvent::Leave(node) => {
                    if let Some(first_await) = self.first_await {
                        if let Some(call) = JsCallExpression::cast(node) {
                            return Some((call, first_await));
                        }
                    } else if let Some(await_expression) = JsAwaitExpression::cast(node) {
                        self.first_await = await_expression
                            .await_token()
                            .ok()
                            .map(|token| token.text_trimmed_range());
                    }
                }
            }
        }
        None
    }
}

fn options_setup_function_body(
    component: &impl VueOptionsApiBasedComponent,
) -> Option<AnyJsFunctionBody> {
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

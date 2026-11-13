//! Helpers for inspecting the code that runs while a Vue component is being set up: its
//! `setup()` function, or all the code in `<script setup>`.

use biome_js_syntax::{
    AnyFunctionLike, AnyJsFunctionBody, JsAwaitExpression, JsCallExpression, JsForOfStatement,
    JsLanguage, JsModule, JsParameters, JsSyntaxNode, WalkEvent,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, declare_node_union};
use biome_vue_semantic::{AnalyzedVueModel, Component};

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_node_union! {
    /// The code that runs while a component is being set up: the body of its `setup()`
    /// function, or a `<script setup>` module.
    pub AnyVueSetupCode = AnyJsFunctionBody | JsModule
}

/// A call in the setup code that runs after the setup code pauses at an `await`.
pub struct VueSetupCallAfterAwait {
    pub call: JsCallExpression,
    /// The range of the first `await` keyword in the setup code that pauses before the call runs.
    pub first_await: TextRange,
}

/// Returns the `setup()` function of `component`, in any of the forms
/// [`Component::setup_function_node`] accepts.
///
/// Returns `None` for `<script setup>` components, which have no `setup()` function, and when the
/// function is written in another tree than the analyzed one.
pub fn vue_setup_function(
    component: &Component,
    vue_model: &AnalyzedVueModel<JsLanguage>,
) -> Option<AnyFunctionLike> {
    AnyFunctionLike::cast(vue_model.node(&component.setup_function_node()?)?)
}

/// Returns the parenthesized parameters of a `setup()` function, as in `(props, context)`.
///
/// Returns `None` for an arrow function whose only parameter has no parentheses, as in
/// `props => {}`.
pub fn vue_setup_function_parameters(function: &AnyFunctionLike) -> Option<JsParameters> {
    match function {
        AnyFunctionLike::AnyJsFunction(function) => function.parenthesized_parameters(),
        AnyFunctionLike::JsMethodObjectMember(method) => method.parameters().ok(),
        AnyFunctionLike::JsMethodClassMember(_) | AnyFunctionLike::JsConstructorClassMember(_) => {
            None
        }
    }
}

/// Finds the calls in `code` that match `predicate` and run after the setup code pauses at an
/// `await`.
///
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of the setup code.
pub fn find_vue_setup_calls_after_await(
    code: &AnyVueSetupCode,
    mut predicate: impl FnMut(&JsCallExpression) -> bool,
) -> Box<[VueSetupCallAfterAwait]> {
    filter_map_vue_setup_calls_after_await(code, |call, first_await| {
        predicate(&call).then_some(VueSetupCallAfterAwait { call, first_await })
    })
}

/// Calls `f` for each call in `code` that runs after the setup code pauses at an `await`, along
/// with the range of the first `await` keyword that pauses before the call runs, and collects the
/// values it returns.
///
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of the setup code.
pub fn filter_map_vue_setup_calls_after_await<T>(
    code: &AnyVueSetupCode,
    mut f: impl FnMut(JsCallExpression, TextRange) -> Option<T>,
) -> Box<[T]> {
    // A module is a control flow root, which the walk skips, so its items are walked instead.
    let root: JsSyntaxNode = match code {
        AnyVueSetupCode::AnyJsFunctionBody(body) => body.syntax().clone(),
        AnyVueSetupCode::JsModule(module) => module.items().into_syntax(),
    };
    let mut first_await = None;
    let mut results = Vec::new();
    let mut preorder = root.preorder_of_kinds(VISITED_KINDS);
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
            // Leave events come after the node's children, which matches evaluation order: the
            // setup code pauses only once the awaited value has been computed, and a function is
            // called only once its arguments have been computed. So `await onMounted()` registers
            // the hook before pausing, while `onMounted(await callback())` registers it after.
            WalkEvent::Leave(node) => {
                if let Some(first_await) = first_await {
                    if let Some(call) = JsCallExpression::cast(node)
                        && let Some(result) = f(call, first_await)
                    {
                        results.push(result);
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
    results.into_boxed_slice()
}

/// The nodes [`filter_map_vue_setup_calls_after_await`] inspects: nested functions, which it skips,
/// and the nodes that pause the setup code or call a function.
const VISITED_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(JsForOfStatement::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsCallExpression::KIND_SET);

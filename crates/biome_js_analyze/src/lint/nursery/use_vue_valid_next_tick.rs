use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsArrayElementList, JsArrayExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsAwaitExpression, JsCallArgumentList, JsCallExpression,
    JsComputedMemberExpression, JsConditionalExpression, JsExpressionStatement,
    JsIdentifierExpression, JsInitializerClause, JsLanguage, JsReturnStatement, JsSequenceExpression,
    JsStaticMemberExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, declare_node_union};
use biome_rule_options::use_vue_valid_next_tick::UseVueValidNextTickOptions;

use crate::JsRuleAction;
use crate::ast_utils::is_await_allowed;
use crate::frameworks::vue::vue_call::is_vue_next_tick_reference;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce valid calls to Vue's `nextTick` function.
    ///
    /// Vue updates the page some time after your data changes. `nextTick` lets you wait until
    /// that update is done, either by passing it a callback function or by awaiting the Promise
    /// it returns.
    ///
    /// This rule reports uses of `nextTick` that don't wait for the update:
    ///
    /// - `nextTick` is used without being called, such as `nextTick.then(callback)` or
    ///   `await nextTick`.
    /// - `nextTick()` is called without a callback, and the returned Promise isn't awaited,
    ///   returned, assigned or chained with `.then()`.
    /// - `nextTick` is called with more than one argument. It only accepts one callback.
    /// - `nextTick` is called with a callback, and the returned Promise is also awaited or
    ///   chained with `.then()`. Use one of the two, not both.
    ///
    /// This applies to `nextTick` imported from `vue`, `Vue.nextTick`, and `this.$nextTick`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { nextTick } from "vue";
    ///
    /// nextTick();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { nextTick } from "vue";
    ///
    /// nextTick.then(() => {});
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { nextTick } from "vue";
    ///
    /// nextTick(callback, anotherCallback);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   async mounted() {
    ///     await this.$nextTick(callback);
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { nextTick } from "vue";
    ///
    /// export default {
    ///   async mounted() {
    ///     await nextTick();
    ///     nextTick().then(callback);
    ///     nextTick(callback);
    ///     await this.$nextTick();
    ///     promise.then(nextTick);
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub UseVueValidNextTick {
        version: "2.6.5",
        name: "useVueValidNextTick",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-next-tick").same()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsNextTickReference =
        JsIdentifierExpression | JsStaticMemberExpression | JsComputedMemberExpression
}

impl Rule for UseVueValidNextTick {
    type Query = Semantic<AnyJsNextTickReference>;
    type State = InvalidNextTick;
    type Signals = Option<Self::State>;
    type Options = UseVueValidNextTickOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = AnyJsExpression::cast_ref(ctx.query().syntax())?;
        if !is_vue_next_tick_reference(&reference, ctx.model()) {
            return None;
        }

        let reference = reference.outer_expression()?;
        let mut parent = reference.syntax().parent()?;
        // `condition ? nextTick : other` is judged by how the whole conditional is used.
        if let Some(conditional) = JsConditionalExpression::cast_ref(&parent) {
            parent = AnyJsExpression::from(conditional)
                .outer_expression()?
                .syntax()
                .parent()?;
        }

        // Passing `nextTick` to a function or storing it for later are valid uses.
        if STORED_REFERENCE_PARENT_KINDS.matches(parent.kind()) {
            return None;
        }

        let Some(call) = JsCallExpression::cast(parent) else {
            return Some(InvalidNextTick::NotCalled);
        };
        if call.callee().ok()?.syntax() != reference.syntax() {
            // The conditional that contains `nextTick` is called.
            return None;
        }

        let promise_is_used = is_promise_used(&call, ctx.model());
        match call.arguments().ok()?.args().len() {
            0 if !promise_is_used => Some(InvalidNextTick::MissingAwaitOrCallback(call)),
            0 => None,
            1 if promise_is_used => Some(InvalidNextTick::CallbackAndPromise),
            1 => None,
            _ => Some(InvalidNextTick::TooManyArguments),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        let diagnostic = match state {
            InvalidNextTick::NotCalled => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"nextTick"</Emphasis>" is used without being called."
                },
            )
            .note(markup! {
                <Emphasis>"nextTick"</Emphasis>" is a function. It only waits for Vue to update the page when it's called."
            })
            .note(markup! {
                "Call "<Emphasis>"nextTick()"</Emphasis>"."
            }),
            InvalidNextTick::MissingAwaitOrCallback(_) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The Promise returned by "<Emphasis>"nextTick()"</Emphasis>" is ignored."
                },
            )
            .note(markup! {
                "Code after this call doesn't wait for Vue to update the page, so this call has no effect."
            })
            .note(markup! {
                "Await the Promise, or pass a callback function to "<Emphasis>"nextTick"</Emphasis>"."
            }),
            InvalidNextTick::TooManyArguments => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"nextTick"</Emphasis>" is called with more than one argument."
                },
            )
            .note(markup! {
                <Emphasis>"nextTick"</Emphasis>" accepts a single callback function. The other arguments are ignored."
            })
            .note(markup! {
                "Pass at most one callback function to "<Emphasis>"nextTick"</Emphasis>"."
            }),
            InvalidNextTick::CallbackAndPromise => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"nextTick"</Emphasis>" is called with a callback function, and the Promise it returns is also used."
                },
            )
            .note(markup! {
                "Using both splits the code that runs after Vue updates the page into two places."
            })
            .note(markup! {
                "Either await the Promise, or pass a callback function to "<Emphasis>"nextTick"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = match state {
            InvalidNextTick::NotCalled => {
                let reference = AnyJsExpression::cast_ref(ctx.query().syntax())?;
                // Calling the reference inside a type assertion, such as
                // `(nextTick as Tick)`, would no longer match the asserted type.
                if reference.outer_expression()?.omit_parentheses().syntax() != reference.syntax()
                {
                    return None;
                }
                let call = make::js_call_expression(
                    reference.clone().trim_comments_and_trivia()?,
                    make::js_call_arguments(
                        make::token(T!['(']),
                        make::js_call_argument_list([], []),
                        make::token(T![')']),
                    ),
                )
                .build();
                mutation.replace_node_transfer_trivia(
                    reference,
                    AnyJsExpression::JsCallExpression(call),
                );
                markup! { "Call "<Emphasis>"nextTick"</Emphasis>"." }
            }
            InvalidNextTick::MissingAwaitOrCallback(call) => {
                // Only a call that is a whole statement can be awaited without adding
                // parentheses around it. In `other(), nextTick();`, `await` would apply to
                // `other()` instead.
                let call = AnyJsExpression::JsCallExpression(call.clone()).outer_expression()?;
                if !JsExpressionStatement::can_cast(call.syntax().parent()?.kind())
                    || JsSequenceExpression::can_cast(call.syntax().kind())
                    || !is_await_allowed(call.syntax())
                {
                    return None;
                }
                let await_expression = make::js_await_expression(
                    make::token(JsSyntaxKind::AWAIT_KW).with_trailing_space(),
                    call.clone().trim_comments_and_trivia()?,
                );
                mutation.replace_node_transfer_trivia(
                    call,
                    AnyJsExpression::JsAwaitExpression(await_expression),
                );
                markup! { "Add "<Emphasis>"await"</Emphasis>"." }
            }
            InvalidNextTick::TooManyArguments | InvalidNextTick::CallbackAndPromise => {
                return None;
            }
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message.to_owned(),
            mutation,
        ))
    }
}

pub enum InvalidNextTick {
    /// `nextTick` is referenced without being called, such as `nextTick.then(callback)`.
    NotCalled,
    /// `nextTick()` is called without a callback, and its Promise is ignored.
    MissingAwaitOrCallback(JsCallExpression),
    /// `nextTick` is called with more than one argument.
    TooManyArguments,
    /// `nextTick` is called with a callback, and its Promise is also used.
    CallbackAndPromise,
}

/// Parents of a `nextTick` reference that pass it to a function or store it, such as
/// `promise.then(nextTick)` or `const tick = nextTick`.
const STORED_REFERENCE_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsCallArgumentList::KIND_SET
    .union(JsInitializerClause::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET);

/// Parents of a `nextTick()` call that always use the Promise it returns, such as
/// `await nextTick()` or `const promise = nextTick()`.
const PROMISE_USING_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsAwaitExpression::KIND_SET
    .union(JsReturnStatement::KIND_SET)
    .union(JsInitializerClause::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET);

/// Returns `true` if the Promise returned by the `nextTick()` call is awaited, returned, stored,
/// chained with `.then()`, or passed to a `Promise` method in an array.
fn is_promise_used(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Some(call) = AnyJsExpression::JsCallExpression(call.clone()).outer_expression() else {
        return false;
    };
    let Some(parent) = call.syntax().parent() else {
        return false;
    };
    if PROMISE_USING_PARENT_KINDS.matches(parent.kind()) {
        return true;
    }
    if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&parent) {
        return arrow
            .body()
            .is_ok_and(|body| body.syntax() == call.syntax());
    }
    if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
        return member
            .member_name()
            .is_some_and(|name| name.text() == "then");
    }
    JsArrayElementList::cast(parent)
        .is_some_and(|elements| is_promise_method_array_element(&elements, model))
}

/// Returns `true` if `elements` belong to an array passed to a method of the global `Promise`,
/// such as `Promise.all([nextTick(), promise])`.
fn is_promise_method_array_element(elements: &JsArrayElementList, model: &SemanticModel) -> bool {
    let Some(array) = elements.parent::<JsArrayExpression>() else {
        return false;
    };
    let Some(array) = AnyJsExpression::JsArrayExpression(array).outer_expression() else {
        return false;
    };
    let Some(call) = array
        .syntax()
        .parent()
        .filter(|parent| JsCallArgumentList::can_cast(parent.kind()))
        .and_then(|list| list.grand_parent())
        .and_then(JsCallExpression::cast)
    else {
        return false;
    };
    call.callee()
        .ok()
        .and_then(|callee| AnyJsMemberExpression::cast(callee.omit_parentheses().into_syntax()))
        .and_then(|member| member.object().ok())
        .and_then(|object| object.omit_parentheses().as_js_reference_identifier())
        .is_some_and(|reference| {
            reference.has_name("Promise") && model.binding(&reference).is_none()
        })
}

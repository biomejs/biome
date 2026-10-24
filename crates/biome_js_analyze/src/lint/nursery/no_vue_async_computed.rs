use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, AnyJsObjectMember,
    JsAwaitExpression, JsCallExpression, JsNewExpression, JsSyntaxToken, WalkEvent, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_async_computed::NoVueAsyncComputedOptions;

use crate::frameworks::vue::vue_call::{is_vue_api_reference, vue_api_name};
use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueMethod, VueComponent, VueComponentDeclarations,
    VueComponentQuery, VueDeclaration, VueDeclarationCollectionFilter,
};
use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow asynchronous code in Vue computed properties.
    ///
    /// Vue calls the function of a computed property and uses the value it returns right away.
    /// Asynchronous code finishes later, so its result never becomes the value of the computed
    /// property. An `async` function returns a `Promise` instead of the value you expect, and
    /// code that runs later, like a `setTimeout()` callback, can't change what was already
    /// returned.
    ///
    /// This rule checks the computed properties declared in the `computed` option of a component,
    /// and the functions passed to `computed()` from `vue`. It reports:
    ///
    /// - `async` functions and the `await` keyword;
    /// - `new Promise()`, and calls to `Promise` functions such as `Promise.all()` and
    ///   `Promise.resolve()`;
    /// - calls to the `then()`, `catch()`, and `finally()` methods of any object;
    /// - `setTimeout()`, `setInterval()`, `setImmediate()`, and `requestAnimationFrame()`;
    /// - `nextTick()` from `vue`, `Vue.nextTick()`, and `this.$nextTick()`.
    ///
    /// Functions defined inside a computed property aren't checked, because they don't run
    /// when Vue computes the value.
    ///
    /// If you need a value that is loaded asynchronously, store it in a `ref()` or in `data`,
    /// update it from a watcher or a method, and read it from the computed property. You can
    /// also use [`computedAsync`](https://vueuse.org/core/computedAsync) from VueUse.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     async user() {
    ///       return fetchUser(this.userId);
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     user() {
    ///       return fetchUser(this.userId).then((response) => response.data);
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { computed } from "vue";
    ///
    /// const doubled = computed(() => {
    ///   setTimeout(() => console.log(count.value), 100);
    ///   return count.value * 2;
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName() {
    ///       return `${this.firstName} ${this.lastName}`;
    ///     },
    ///     loader() {
    ///       return async () => fetchUser(this.userId);
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreObjects`
    ///
    /// Default: `[]`
    ///
    /// Names of objects whose `then()`, `catch()`, and `finally()` methods aren't reported.
    /// Some libraries use these method names for code that isn't asynchronous. For example,
    /// `catch()` in the Zod validation library sets a fallback value.
    ///
    /// The name is the first variable of the chain of calls and property accesses, so
    /// `"z"` matches `z.number().catch(0)` and `z.object({}).shape.catch(0)`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreObjects": ["z"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { computed } from "vue";
    ///
    /// const age = computed(() => schema.number().catch(0));
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { computed } from "vue";
    ///
    /// const age = computed(() => z.number().catch(0));
    /// ```
    ///
    pub NoVueAsyncComputed {
        version: "next",
        name: "noVueAsyncComputed",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-async-in-computed-properties").inspired()],
    }
}

impl Rule for NoVueAsyncComputed {
    type Query = VueComponentQuery;
    type State = AsyncOperation;
    type Signals = Box<[Self::State]>;
    type Options = NoVueAsyncComputedOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut operations = Vec::new();

        if let AnyPotentialVueComponent::JsCallExpression(call) = ctx.query()
            && let Some(getter) = computed_call_getter(call, model)
        {
            collect_async_operations(&getter, model, ctx.options(), &mut operations);
            return operations.into_boxed_slice();
        }

        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            model,
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };

        for declaration in component.declarations(VueDeclarationCollectionFilter::Computed.into())
        {
            let VueDeclaration::Computed(computed) = declaration else {
                continue;
            };
            let getter = match computed {
                AnyVueMethod::JsMethodObjectMember(method) => {
                    Some(AnyFunctionLike::JsMethodObjectMember(method))
                }
                AnyVueMethod::JsPropertyObjectMember(property) => property
                    .value()
                    .ok()
                    .and_then(getter_from_expression),
            };
            if let Some(getter) = getter {
                collect_async_operations(&getter, model, ctx.options(), &mut operations);
            }
        }

        operations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state.kind {
            AsyncOperationKind::AsyncFunction => markup! {
                "This computed property is an "<Emphasis>"async"</Emphasis>" function, so it returns a "<Emphasis>"Promise"</Emphasis>" instead of a value."
            },
            AsyncOperationKind::Await => markup! {
                "This computed property waits for asynchronous code with "<Emphasis>"await"</Emphasis>"."
            },
            AsyncOperationKind::PromiseObject => markup! {
                "This computed property creates a "<Emphasis>"Promise"</Emphasis>"."
            },
            AsyncOperationKind::PromiseMethod => markup! {
                "This computed property runs asynchronous code."
            },
            AsyncOperationKind::Timer => markup! {
                "This computed property schedules code to run later."
            },
            AsyncOperationKind::NextTick => markup! {
                "This computed property schedules code to run after Vue updates the page."
            },
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, message)
                .note(markup! {
                    "Vue uses the value that a computed property returns right away. Asynchronous code finishes later, so its result never becomes the value of the computed property."
                })
                .note(markup! {
                    "Store the result in a "<Emphasis>"ref()"</Emphasis>" or in "<Emphasis>"data"</Emphasis>" from a watcher or a method, and read it from the computed property instead."
                }),
        )
    }
}

/// Asynchronous code found in the function of a computed property.
pub struct AsyncOperation {
    kind: AsyncOperationKind,
    range: TextRange,
}

#[derive(Clone, Copy)]
enum AsyncOperationKind {
    /// The function of the computed property is `async`.
    AsyncFunction,
    /// An `await` expression.
    Await,
    /// `new Promise()`.
    PromiseObject,
    /// A call to `then()`, `catch()`, or `finally()`, or to a static method of `Promise`.
    PromiseMethod,
    /// A call to a function that runs a callback later, like `setTimeout()`.
    Timer,
    /// A call to `nextTick()` or `$nextTick()`.
    NextTick,
}

/// Returns the getter of `call` if it is a call to `computed()` from `vue`.
///
/// The getter is either the first argument, as in `computed(() => 1)`, or its `get` property, as
/// in `computed({ get: () => 1, set: (value) => {} })`.
fn computed_call_getter(call: &JsCallExpression, model: &SemanticModel) -> Option<AnyFunctionLike> {
    let callee = call.callee().ok()?;
    if !is_vue_api_reference(&callee, model, "computed") {
        return None;
    }
    let argument = call.arguments().ok()?.args().first()?.ok()?;
    getter_from_expression(argument.as_any_js_expression()?.clone())
}

/// Returns the getter of a computed property defined by `expression`.
///
/// `expression` is either the getter function itself or an object with a `get` property.
fn getter_from_expression(expression: AnyJsExpression) -> Option<AnyFunctionLike> {
    let expression = expression.omit_parentheses();
    if let AnyJsExpression::JsObjectExpression(object) = &expression {
        return object
            .members()
            .iter()
            .flatten()
            .find(|member| member.name().is_some_and(|name| name.text() == "get"))
            .and_then(|member| match member {
                AnyJsObjectMember::JsMethodObjectMember(method) => {
                    Some(AnyFunctionLike::JsMethodObjectMember(method))
                }
                AnyJsObjectMember::JsPropertyObjectMember(property) => {
                    function_from_expression(property.value().ok()?)
                }
                _ => None,
            });
    }
    function_from_expression(expression)
}

fn function_from_expression(expression: AnyJsExpression) -> Option<AnyFunctionLike> {
    match expression.omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(function) => Some(
            AnyFunctionLike::AnyJsFunction(AnyJsFunction::JsArrowFunctionExpression(function)),
        ),
        AnyJsExpression::JsFunctionExpression(function) => Some(AnyFunctionLike::AnyJsFunction(
            AnyJsFunction::JsFunctionExpression(function),
        )),
        _ => None,
    }
}

/// Collects the asynchronous code that runs when `getter` is called.
///
/// Code inside nested functions is ignored, because it runs only when that function is called.
fn collect_async_operations(
    getter: &AnyFunctionLike,
    model: &SemanticModel,
    options: &NoVueAsyncComputedOptions,
    operations: &mut Vec<AsyncOperation>,
) {
    if let Some(async_token) = async_token(getter) {
        operations.push(AsyncOperation {
            kind: AsyncOperationKind::AsyncFunction,
            range: async_token.text_trimmed_range(),
        });
    }

    let Ok(body) = getter.body() else {
        return;
    };
    let mut preorder = body.syntax().preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyJsControlFlowRoot::KIND_SET.matches(node.kind()) {
            preorder.skip_subtree();
            continue;
        }
        let kind = if JsAwaitExpression::can_cast(node.kind()) {
            Some(AsyncOperationKind::Await)
        } else if let Some(new_expression) = JsNewExpression::cast_ref(&node) {
            is_new_promise(&new_expression, model).then_some(AsyncOperationKind::PromiseObject)
        } else if let Some(call) = JsCallExpression::cast_ref(&node) {
            async_call_kind(&call, model, options)
        } else {
            None
        };
        if let Some(kind) = kind {
            operations.push(AsyncOperation {
                kind,
                range: node.text_trimmed_range(),
            });
        }
    }
}

fn async_token(function: &AnyFunctionLike) -> Option<JsSyntaxToken> {
    match function {
        AnyFunctionLike::AnyJsFunction(function) => function.async_token(),
        AnyFunctionLike::JsMethodObjectMember(method) => method.async_token(),
        AnyFunctionLike::JsMethodClassMember(method) => method.async_token(),
        AnyFunctionLike::JsConstructorClassMember(_) => None,
    }
}

/// Checks if `new_expression` creates a `Promise` with the global `Promise` constructor.
fn is_new_promise(new_expression: &JsNewExpression, model: &SemanticModel) -> bool {
    new_expression
        .callee()
        .is_ok_and(|callee| is_global(&callee, "Promise", model))
}

/// Returns the kind of asynchronous code that `call` runs, if any.
fn async_call_kind(
    call: &JsCallExpression,
    model: &SemanticModel,
    options: &NoVueAsyncComputedOptions,
) -> Option<AsyncOperationKind> {
    let callee = call.callee().ok()?.inner_expression()?;

    if let Some(member) = AnyJsMemberExpression::cast_ref(callee.syntax()) {
        let name = member.member_name()?;
        let object = member.object().ok()?;
        if PROMISE_INSTANCE_METHODS.contains(&name.text()) {
            let is_ignored = root_object_name(&object).is_some_and(|root| {
                options
                    .ignore_objects()
                    .iter()
                    .any(|ignored| ignored.as_ref() == root.text_trimmed())
            });
            return (!is_ignored).then_some(AsyncOperationKind::PromiseMethod);
        }
        if PROMISE_STATIC_METHODS.binary_search(&name.text()).is_ok()
            && is_global(&object, "Promise", model)
        {
            return Some(AsyncOperationKind::PromiseMethod);
        }
        if name.text() == "$nextTick"
            && matches!(
                object.omit_parentheses(),
                AnyJsExpression::JsThisExpression(_)
            )
        {
            return Some(AsyncOperationKind::NextTick);
        }
    }

    if vue_api_name(&callee, model).is_some_and(|name| name.text() == "nextTick") {
        return Some(AsyncOperationKind::NextTick);
    }

    let (reference, name) = global_identifier(&callee.as_any_global_identifier_expression()?)?;
    if TIMER_FUNCTIONS.contains(&name.text())
        && model.binding(&reference).is_none()
        && !call.arguments().ok()?.args().is_empty()
    {
        return Some(AsyncOperationKind::Timer);
    }

    None
}

/// Checks if `expression` refers to the global variable `name`, as in `name`, `window.name`, or
/// `globalThis.name`, and no local variable shadows it.
fn is_global(expression: &AnyJsExpression, name: &str, model: &SemanticModel) -> bool {
    expression
        .inner_expression()
        .and_then(|expression| expression.as_any_global_identifier_expression())
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, global_name)| {
            global_name.text() == name && model.binding(&reference).is_none()
        })
}

/// Returns the name of the variable that a chain of calls and property accesses starts with, as
/// in `z` for `z.string().optional`.
fn root_object_name(expression: &AnyJsExpression) -> Option<JsSyntaxToken> {
    let mut current = expression.inner_expression()?;
    loop {
        current = match current {
            AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok()?,
            AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok()?,
            AnyJsExpression::JsCallExpression(call) => call.callee().ok()?,
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return identifier.name().ok()?.value_token().ok();
            }
            _ => return None,
        }
        .inner_expression()?;
    }
}

/// Methods of a `Promise` instance.
const PROMISE_INSTANCE_METHODS: &[&str] = &["catch", "finally", "then"];

/// Static methods of `Promise` that return a `Promise`, sorted for binary search.
const PROMISE_STATIC_METHODS: &[&str] = &[
    "all",
    "allSettled",
    "any",
    "race",
    "reject",
    "resolve",
    "try",
    "withResolvers",
];

/// Global functions that run a callback later.
const TIMER_FUNCTIONS: &[&str] = &[
    "requestAnimationFrame",
    "setImmediate",
    "setInterval",
    "setTimeout",
];

#[cfg(test)]
mod tests {
    use super::PROMISE_STATIC_METHODS;

    #[test]
    fn promise_static_methods_are_sorted() {
        for pair in PROMISE_STATIC_METHODS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come after {} in PROMISE_STATIC_METHODS",
                pair[0],
                pair[1]
            );
        }
    }
}

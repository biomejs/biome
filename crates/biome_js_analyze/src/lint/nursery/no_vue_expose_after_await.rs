use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsFormalParameter, AnyJsFunctionBody, AnyJsMemberExpression,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsParameter, JsAwaitExpression,
    JsCallExpression, JsExpressionStatement, JsForOfStatement, JsIdentifierBinding, JsLanguage,
    JsModule, JsModuleItemList, JsParameters, WalkEvent,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange, declare_node_union};
use biome_rule_options::no_vue_expose_after_await::NoVueExposeAfterAwaitOptions;

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueComponentQuery,
    VueOptionsApiBasedComponent,
};
use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow calling `expose()` or `defineExpose()` after an `await` in `setup()`.
    ///
    /// `expose()` and `defineExpose()` choose which properties of a component a parent component
    /// can use through a template ref. Vue expects them to be called before the component's setup
    /// code pauses at its first `await`. The setup code is the `setup()` function, or all the code
    /// in `<script setup>`. A call that comes after the pause runs too late, so the parent
    /// component might not receive the exposed properties.
    ///
    /// This rule reports these calls when they come after an `await`:
    ///
    /// - `expose()` from the second parameter of `setup()`, written as `setup(props, { expose })`
    ///   or as `setup(props, context)` followed by `context.expose()`.
    /// - `defineExpose()` in `<script setup>`.
    ///
    /// An `await` inside a nested function doesn't count, because it doesn't pause the setup code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   async setup(props, { expose }) {
    ///     await fetchUser();
    ///     expose({ reload });
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// await fetchUser();
    /// defineExpose({ reload });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   async setup(props, { expose }) {
    ///     expose({ reload });
    ///     await fetchUser();
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// defineExpose({ reload });
    /// await fetchUser();
    /// </script>
    /// ```
    ///
    pub NoVueExposeAfterAwait {
        version: "2.6.7",
        name: "noVueExposeAfterAwait",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-expose-after-await").same()],
    }
}

impl Rule for NoVueExposeAfterAwait {
    type Query = VueComponentQuery;
    type State = ExposeAfterAwait;
    type Signals = Box<[Self::State]>;
    type Options = NoVueExposeAfterAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };
        let model = ctx.model();

        let setup = match component.kind() {
            AnyVueComponent::OptionsApi(component) => setup_function(component),
            AnyVueComponent::CreateApp(component) => setup_function(component),
            AnyVueComponent::DefineComponent(component) => setup_function(component),
            AnyVueComponent::Setup(_) => {
                let AnyPotentialVueComponent::JsModule(module) = ctx.query() else {
                    return [].into();
                };
                return find_calls_after_await(
                    &AnyVueSetupCode::JsModule(module.clone()),
                    &ExposeFunction::DefineExpose,
                    model,
                );
            }
        };

        let Some((parameters, body)) = setup else {
            return [].into();
        };
        match ExposeFunction::from_setup_parameters(&parameters) {
            Some(expose) => {
                find_calls_after_await(&AnyVueSetupCode::AnyJsFunctionBody(body), &expose, model)
            }
            None => [].into(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = state.kind;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.call.range(),
                markup! {
                    "This "<Emphasis>{kind}</Emphasis>" call comes after an "<Emphasis>"await"</Emphasis>", so it runs too late."
                },
            )
            .detail(
                state.first_await,
                markup! {
                    "The component's setup code pauses here. A parent component might not receive the exposed properties."
                },
            )
            .note(markup! {
                "Call "<Emphasis>{kind}</Emphasis>" before the first "<Emphasis>"await"</Emphasis>"."
            }),
        )
    }
}

pub struct ExposeAfterAwait {
    call: JsCallExpression,
    kind: ExposeKind,
    /// The range of the first `await` keyword that pauses the setup code before the call.
    first_await: TextRange,
}

#[derive(Clone, Copy)]
pub enum ExposeKind {
    Expose,
    DefineExpose,
}

impl biome_console::fmt::Display for ExposeKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Expose => "expose()",
            Self::DefineExpose => "defineExpose()",
        })
    }
}

declare_node_union! {
    /// The code that runs while a component is being set up.
    pub AnyVueSetupCode = AnyJsFunctionBody | JsModule
}

/// The function that exposes properties of the component being set up.
enum ExposeFunction {
    /// `expose` taken out of the setup context, as in `setup(props, { expose })`.
    Destructured(JsIdentifierBinding),
    /// The setup context itself, as in `setup(props, context)`, called as `context.expose()`.
    Context(JsIdentifierBinding),
    /// The `defineExpose()` compiler macro in `<script setup>`.
    DefineExpose,
}

impl ExposeFunction {
    /// Returns how `expose` can be called, based on the second parameter of `setup()`.
    ///
    /// Returns `None` when `setup()` has no second parameter, or when its second parameter is
    /// destructured without taking out `expose`.
    fn from_setup_parameters(parameters: &JsParameters) -> Option<Self> {
        let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(context)) =
            parameters.items().iter().nth(1)?.ok()?
        else {
            return None;
        };
        match context.binding().ok()? {
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(context)) => {
                Some(Self::Context(context))
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => pattern
                .properties()
                .iter()
                .flatten()
                .find_map(|property| match property {
                    // `{ expose }`
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        property,
                    ) => {
                        let binding = property.identifier().ok()?;
                        let binding = binding.as_js_identifier_binding()?;
                        (binding.name_token().ok()?.text_trimmed() == "expose")
                            .then(|| binding.clone())
                    }
                    // `{ expose: exposeProperties }`
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                        if property.member().ok()?.name()?.text() != "expose" {
                            return None;
                        }
                        property
                            .pattern()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()
                            .cloned()
                    }
                    _ => None,
                })
                .map(Self::Destructured),
            _ => None,
        }
    }

    const fn kind(&self) -> ExposeKind {
        match self {
            Self::Destructured(_) | Self::Context(_) => ExposeKind::Expose,
            Self::DefineExpose => ExposeKind::DefineExpose,
        }
    }

    /// Returns `true` if `call` exposes properties of the component through this function.
    fn is_called_by(&self, call: &JsCallExpression, model: &SemanticModel) -> bool {
        match self {
            Self::Destructured(expose) => call
                .callee()
                .ok()
                .and_then(|callee| callee.inner_expression())
                .is_some_and(|callee| reads_binding(&callee, expose, model)),
            Self::Context(context) => {
                let Some(callee) = call
                    .callee()
                    .ok()
                    .and_then(|callee| callee.inner_expression())
                    .and_then(|callee| AnyJsMemberExpression::cast(callee.into_syntax()))
                else {
                    return false;
                };
                callee
                    .member_name()
                    .is_some_and(|name| name.text() == "expose")
                    && callee
                        .object()
                        .ok()
                        .and_then(|object| object.inner_expression())
                        .is_some_and(|object| reads_binding(&object, context, model))
            }
            // Vue only compiles `defineExpose()` when it is a statement of its own at the top
            // level of `<script setup>`.
            Self::DefineExpose => {
                is_vue_compiler_macro_call(call, model, "defineExpose")
                    && call
                        .parent::<JsExpressionStatement>()
                        .is_some_and(|statement| statement.parent::<JsModuleItemList>().is_some())
            }
        }
    }
}

/// Returns the parameters and body of the component's `setup()` function.
fn setup_function(
    component: &impl VueOptionsApiBasedComponent,
) -> Option<(JsParameters, AnyJsFunctionBody)> {
    // `defineComponent((props, context) => { ... }, { ... })`
    if let Some(setup) = component.setup_func() {
        return function_parts(&setup);
    }
    let (_, member) = component
        .iter_declaration_groups()
        .find(|(name, _)| name.text() == "setup")?;
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => Some((
            method.parameters().ok()?,
            AnyJsFunctionBody::JsFunctionBody(method.body().ok()?),
        )),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            function_parts(&property.value().ok()?)
        }
        _ => None,
    }
}

/// Returns the parameters and body of `function` when it is a function or arrow function
/// expression.
fn function_parts(function: &AnyJsExpression) -> Option<(JsParameters, AnyJsFunctionBody)> {
    match function.inner_expression()? {
        AnyJsExpression::JsFunctionExpression(function) => Some((
            function.parameters().ok()?,
            AnyJsFunctionBody::JsFunctionBody(function.body().ok()?),
        )),
        AnyJsExpression::JsArrowFunctionExpression(function) => match function.parameters().ok()? {
            AnyJsArrowFunctionParameters::JsParameters(parameters) => {
                Some((parameters, function.body().ok()?))
            }
            AnyJsArrowFunctionParameters::AnyJsBinding(_) => None,
        },
        _ => None,
    }
}

/// Returns `true` if `expression` is a read of `binding`.
fn reads_binding(
    expression: &AnyJsExpression,
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> bool {
    expression
        .as_js_identifier_expression()
        .and_then(|expression| expression.name().ok())
        .and_then(|reference| model.binding(&reference))
        .is_some_and(|declaration| &declaration.syntax() == binding.syntax())
}

/// Finds the calls to `expose` in `code` that run after the setup code pauses at an `await`.
///
/// Code inside nested functions is ignored, because it runs when that function is called, not as
/// part of the setup code.
fn find_calls_after_await(
    code: &AnyVueSetupCode,
    expose: &ExposeFunction,
    model: &SemanticModel,
) -> Box<[ExposeAfterAwait]> {
    let root = code.syntax();
    let mut first_await = None;
    let mut calls = Vec::new();
    let mut preorder = root.preorder_of_kinds(WALKED_KINDS);
    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => {
                if &node != root && AnyJsControlFlowRoot::KIND_SET.matches(node.kind()) {
                    preorder.skip_subtree();
                } else if first_await.is_none()
                    && let Some(for_of) = JsForOfStatement::cast(node)
                {
                    first_await = for_of.await_token().map(|token| token.text_trimmed_range());
                }
            }
            // Leave events come after the node's children, which matches evaluation order: the
            // setup code pauses only once the awaited value has been computed, and a function is
            // called only once its arguments have been computed. So `await expose()` exposes the
            // properties before pausing, while `expose(await load())` exposes them after.
            WalkEvent::Leave(node) => {
                if let Some(first_await) = first_await {
                    if let Some(call) = JsCallExpression::cast(node)
                        && expose.is_called_by(&call, model)
                    {
                        calls.push(ExposeAfterAwait {
                            call,
                            kind: expose.kind(),
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
    calls.into_boxed_slice()
}

/// The nodes that [find_calls_after_await] needs to see: the nested functions it skips, the
/// `await`s that pause the setup code, and the calls it checks.
const WALKED_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(JsForOfStatement::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsCallExpression::KIND_SET);

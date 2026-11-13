use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter,
    AnyJsMemberExpression, AnyJsObjectBindingPatternMember, AnyJsParameter, JsCallExpression,
    JsExpressionStatement, JsIdentifierBinding, JsModule, JsModuleItemList, JsParameters,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_expose_after_await::NoVueExposeAfterAwaitOptions;
use biome_vue_semantic::ComponentKind;

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::frameworks::vue::vue_setup::{
    AnyVueSetupCode, filter_map_vue_setup_calls_after_await, vue_setup_function,
    vue_setup_function_parameters,
};
use crate::services::vue::VueComponent;

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
    type Query = VueComponent;
    type State = ExposeAfterAwait;
    type Signals = Box<[Self::State]>;
    type Options = NoVueExposeAfterAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let vue_model = ctx.vue_model();
        let model = ctx.model();

        let (code, expose) = if component.kind() == ComponentKind::Setup {
            let Some(module) = vue_model.node(component).and_then(JsModule::cast) else {
                return [].into();
            };
            (AnyVueSetupCode::JsModule(module), ExposeFunction::DefineExpose)
        } else {
            let Some(setup) = vue_setup_function(component, vue_model) else {
                return [].into();
            };
            let Some(expose) = vue_setup_function_parameters(&setup)
                .as_ref()
                .and_then(ExposeFunction::from_setup_parameters)
            else {
                return [].into();
            };
            let Ok(body) = setup.body() else {
                return [].into();
            };
            (AnyVueSetupCode::AnyJsFunctionBody(body), expose)
        };

        filter_map_vue_setup_calls_after_await(&code, |call, first_await| {
            expose
                .is_called_by(&call, model)
                .then(|| ExposeAfterAwait {
                    call,
                    kind: expose.kind(),
                    first_await,
                })
        })
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

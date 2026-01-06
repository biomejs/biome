use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsFileSource};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_vue_options_api::NoVueOptionsApiOptions;

use crate::frameworks::vue::vue_call::is_vue_api_reference;
use crate::frameworks::vue::vue_component::{
    AnyVueComponent, VueComponent, VueComponentQuery, VueOptionsApiBasedComponent,
};

declare_lint_rule! {
    /// Disallow the use of Vue Options API.
    ///
    /// Vue 3.6's Vapor Mode does not support the Options API.
    /// Components must use the Composition API (`<script setup>` or `defineComponent` with function signature) instead.
    ///
    /// This rule helps prepare codebases for Vapor Mode by detecting Options API
    /// patterns that are incompatible with the new rendering mode.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   data() {
    ///     return { count: 0 }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   methods: {
    ///     increment() {
    ///       this.count++
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     doubled() {
    ///       return this.count * 2
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   mounted() {
    ///     console.log('Component mounted')
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from 'vue'
    ///
    /// defineComponent({
    ///   name: 'MyComponent',
    ///   data() {
    ///     return { count: 0 }
    ///   }
    /// })
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { ref } from 'vue'
    /// const count = ref(0)
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// import { ref, computed } from 'vue'
    ///
    /// const count = ref(0)
    /// const doubled = computed(() => count.value * 2)
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// import { onMounted } from 'vue'
    ///
    /// onMounted(() => {
    ///   console.log('Component mounted')
    /// })
    /// </script>
    /// ```
    ///
    /// ## Related Rules
    ///
    /// - [useVueVapor](https://biomejs.dev/linter/rules/use-vue-vapor): Enforces the use of Vapor mode in Vue components
    ///
    /// ## Resources
    ///
    /// - [Vue 3 Composition API](https://vuejs.org/api/composition-api-setup.html)
    /// - [Options API vs Composition API](https://vuejs.org/guide/introduction.html#api-styles)
    ///
    pub NoVueOptionsApi {
        version: "next",
        name: "noVueOptionsApi",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
    }
}

/// State for detected Options API component.
pub struct RuleState {
    range: TextRange,
}

impl Rule for NoVueOptionsApi {
    type Query = VueComponentQuery;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoVueOptionsApiOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        )?;

        // <script setup> or defineComponent with function signature are valid for Vapor Mode
        match component.kind() {
            AnyVueComponent::Setup(_) => None,
            AnyVueComponent::OptionsApi(opts) => {
                let expr = opts.definition_expression()?;
                if is_define_component_or_create_app(&expr, ctx.model()) {
                    return None;
                }
                Some(RuleState {
                    range: expr.range(),
                })
            }
            AnyVueComponent::DefineComponent(component) => {
                if component.setup_func().is_some() {
                    return None;
                }
                let definition = component.definition_expression()?;
                if definition.as_js_object_expression().is_some() {
                    Some(RuleState {
                        range: definition.range(),
                    })
                } else {
                    None
                }
            }
            AnyVueComponent::CreateApp(component) => {
                let definition = component.definition_expression()?;
                if definition.as_js_object_expression().is_some() {
                    Some(RuleState {
                        range: definition.range(),
                    })
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Options API is not supported in Vue Vapor Mode."
                },
            )
            .note(markup! {
                "Although the Options API is still supported by Vue, using the Composition API is recommended, and makes it possible to use Vue's Vapor mode for better performance."
            })
            .note(markup! {
                "Use "<Emphasis>"<script setup>"</Emphasis>" or "<Emphasis>"defineComponent"</Emphasis>" with a function signature to use the "<Hyperlink href="https://vuejs.org/guide/introduction.html#composition-api">"Composition API"</Hyperlink>" instead."
            }),
        )
    }
}

fn is_define_component_or_create_app(
    expr: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    let Some(call_expr) = expr.as_js_call_expression() else {
        return false;
    };
    let Some(callee) = call_expr.callee().ok().and_then(|c| c.inner_expression()) else {
        return false;
    };
    is_vue_api_reference(&callee, model, "defineComponent")
        || is_vue_api_reference(&callee, model, "createApp")
}

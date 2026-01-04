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
    /// Components must use the Composition API with `<script setup>` instead.
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

/// State for detected Options API component
pub struct RuleState {
    /// The range of the detected Options API component
    range: TextRange,
}

/// Checks if the expression is a defineComponent or createApp call.
/// Used to avoid duplicate diagnostics when `export default defineComponent({...})` is used.
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

/// Extracts the range for a component definition.
/// Falls back to query range if definition expression is not available.
fn extract_component_range(
    query_range: TextRange,
    definition_expr: Option<AnyJsExpression>,
) -> RuleState {
    let range = definition_expr.map_or(query_range, |expr| expr.range());
    RuleState { range }
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

        // Only <script setup> is valid for Vapor Mode
        match component.kind() {
            AnyVueComponent::Setup(_) => None,
            AnyVueComponent::OptionsApi(opts) => {
                let expr = opts.definition_expression()?;
                // Skip if the expression is a defineComponent or createApp call
                // to avoid duplicate diagnostics (they will be caught by their own cases)
                if is_define_component_or_create_app(&expr, ctx.model()) {
                    return None;
                }
                Some(RuleState {
                    range: expr.range(),
                })
            }
            AnyVueComponent::DefineComponent(dc) => Some(extract_component_range(
                ctx.query().range(),
                dc.definition_expression(),
            )),
            AnyVueComponent::CreateApp(ca) => Some(extract_component_range(
                ctx.query().range(),
                ca.definition_expression(),
            )),
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
                "Use "<Emphasis>"<script setup>"</Emphasis>" with the Composition API instead."
            }),
        )
    }
}

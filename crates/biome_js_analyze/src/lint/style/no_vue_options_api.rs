use biome_analyze::{Rule, RuleDiagnostic, RuleSource, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_options_api::NoVueOptionsApiOptions;
use biome_vue_semantic::ComponentKind;

use crate::services::vue::VueComponent;

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
        version: "2.3.12",
        name: "noVueOptionsApi",
        language: "js",
        sources: &[
            RuleSource::EslintVueJs("component-api-style").inspired(),
        ],
        recommended: false,
        severity: Severity::Information,
        domains: &[RuleDomain::Vue],
    }
}

/// State for detected Options API component.
pub struct RuleState {
    range: TextRange,
}

impl Rule for NoVueOptionsApi {
    type Query = VueComponent;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoVueOptionsApiOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let range = match component.kind() {
            // <script setup> is valid for Vapor Mode
            ComponentKind::Setup => return None,
            ComponentKind::OptionsObject => {
                // `export default createApp({ ... })` is reported as a `createApp()` component.
                let is_create_app = ctx.vue_model().components().any(|other| {
                    other.kind() == ComponentKind::CreateApp
                        && other.host_range() == component.host_range()
                });
                if is_create_app {
                    return None;
                }
                component.range()
            }
            ComponentKind::DefineComponent => {
                // defineComponent with function signature is valid for Vapor Mode
                let call = JsCallExpression::cast(ctx.vue_model().node(component)?)?;
                if call.arguments().ok()?.args().len() == 2 {
                    return None;
                }
                component.options_object_range()?
            }
            ComponentKind::CreateApp => component.options_object_range()?,
        };
        Some(RuleState { range })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Using the Options API is not allowed."
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

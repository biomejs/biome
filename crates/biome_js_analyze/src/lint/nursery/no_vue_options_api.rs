use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsFileSource;
use biome_rowan::{AstNode, TextRange};

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
    /// <script>
    /// export default {
    ///   setup() {
    ///     const count = ref(0)
    ///     return { count }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    pub NoVueOptionsApi {
        version: "next",
        name: "noVueOptionsApi",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[],
    }
}

/// Options API properties that should be flagged
const OPTIONS_API_PROPERTIES: &[&str] = &[
    // State
    "data",
    "props",
    "propsData",
    "emits",
    "computed",
    "watch",
    // Methods & Components
    "methods",
    "components",
    "directives",
    "mixins",
    "extends",
    // Component Identity
    "name",
    "inheritAttrs",
    // Lifecycle hooks (Vue 3)
    "beforeCreate",
    "created",
    "beforeMount",
    "mounted",
    "beforeUpdate",
    "updated",
    "beforeUnmount",
    "unmounted",
    "errorCaptured",
    "renderTracked",
    "renderTriggered",
    "activated",
    "deactivated",
    "serverPrefetch",
    // Deprecated lifecycle hooks (Vue 2)
    "beforeDestroy",
    "destroyed",
];

/// Get the Composition API alternative for a given Options API property
fn get_composition_api_alternative(property_name: &str) -> &'static str {
    match property_name {
        // State
        "data" => "ref() or reactive()",
        "props" => "defineProps()",
        "propsData" => "defineProps()",
        "emits" => "defineEmits()",
        "computed" => "computed()",
        "watch" => "watch() or watchEffect()",
        // Methods & Components
        "methods" => "regular functions",
        "components" => "direct imports in <script setup>",
        "directives" => "direct imports in <script setup>",
        "mixins" => "composables (reusable functions)",
        "extends" => "composables (reusable functions)",
        // Component Identity
        "name" => "defineOptions({ name })",
        "inheritAttrs" => "defineOptions({ inheritAttrs })",
        // Lifecycle hooks (Vue 3)
        "beforeCreate" => "<script setup> (runs before creation)",
        "created" => "<script setup> (runs at creation)",
        "beforeMount" => "onBeforeMount()",
        "mounted" => "onMounted()",
        "beforeUpdate" => "onBeforeUpdate()",
        "updated" => "onUpdated()",
        "beforeUnmount" => "onBeforeUnmount()",
        "unmounted" => "onUnmounted()",
        "errorCaptured" => "onErrorCaptured()",
        "renderTracked" => "onRenderTracked()",
        "renderTriggered" => "onRenderTriggered()",
        "activated" => "onActivated()",
        "deactivated" => "onDeactivated()",
        "serverPrefetch" => "onServerPrefetch()",
        // Deprecated lifecycle hooks (Vue 2)
        "beforeDestroy" => "onBeforeUnmount()",
        "destroyed" => "onUnmounted()",
        _ => "Composition API equivalents",
    }
}

/// State for each detected Options API property
pub struct RuleState {
    /// The range of the detected Options API property
    range: TextRange,
    /// The name of the detected property
    property_name: String,
}

impl Rule for NoVueOptionsApi {
    type Query = VueComponentQuery;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        ) else {
            return vec![];
        };

        // <script setup> components are valid - skip
        if matches!(component.kind(), AnyVueComponent::Setup(_)) {
            return vec![];
        }

        let mut signals = vec![];

        // Use iter_declaration_groups() to detect Options API properties
        match component.kind() {
            AnyVueComponent::OptionsApi(opts) => {
                for (name, member) in opts.iter_declaration_groups() {
                    let name_text = name.text();
                    // Allow pure setup() - only flag if has other Options API props
                    if name_text != "setup" && OPTIONS_API_PROPERTIES.contains(&name_text) {
                        signals.push(RuleState {
                            range: member.range(),
                            property_name: name_text.to_string(),
                        });
                    }
                }
            }
            AnyVueComponent::DefineComponent(dc) => {
                for (name, member) in dc.iter_declaration_groups() {
                    let name_text = name.text();
                    if name_text != "setup" && OPTIONS_API_PROPERTIES.contains(&name_text) {
                        signals.push(RuleState {
                            range: member.range(),
                            property_name: name_text.to_string(),
                        });
                    }
                }
            }
            AnyVueComponent::CreateApp(ca) => {
                for (name, member) in ca.iter_declaration_groups() {
                    let name_text = name.text();
                    if name_text != "setup" && OPTIONS_API_PROPERTIES.contains(&name_text) {
                        signals.push(RuleState {
                            range: member.range(),
                            property_name: name_text.to_string(),
                        });
                    }
                }
            }
            _ => {}
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let alternative = get_composition_api_alternative(&state.property_name);

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Options API property "<Emphasis>{&state.property_name}</Emphasis>" is not supported in Vue Vapor Mode."
                },
            )
            .note(markup! {
                "Vue 3.6's Vapor Mode requires the Composition API with "<Emphasis>"<script setup>"</Emphasis>" syntax."
            })
            .note(markup! {
                "Use "<Emphasis>{alternative}</Emphasis>" from the Composition API instead."
            }),
        )
    }
}

use crate::services::vue::VueProp;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_boolean_default::NoVueBooleanDefaultOptions;
use biome_vue_semantic::PropType;

declare_lint_rule! {
    /// Disallow default values for Boolean props in Vue components.
    ///
    /// Vue treats a Boolean prop like an HTML boolean attribute: it is `true` when present and
    /// `false` when absent. A default value breaks that convention. With `default: true`, the prop
    /// can only be turned off by explicitly binding `false`, and `default: false` restates what
    /// Vue already does.
    ///
    /// This rule checks props declared through the `props` option, `defineProps()`,
    /// `withDefaults()`, and destructured `defineProps()` with default values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     disabled: {
    ///       type: Boolean,
    ///       default: true,
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// withDefaults(defineProps<{ disabled?: boolean }>(), {
    ///   disabled: false,
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const { disabled = false } = defineProps<{ disabled?: boolean }>();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: {
    ///     disabled: Boolean,
    ///     size: {
    ///       type: Number,
    ///       default: 1,
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{ disabled?: boolean }>();
    /// </script>
    /// ```
    ///
    pub NoVueBooleanDefault {
        version: "next",
        name: "noVueBooleanDefault",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-boolean-default").same()],
    }
}

impl Rule for NoVueBooleanDefault {
    type Query = VueProp;
    /// The range of a default value given to a Boolean prop.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueBooleanDefaultOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        if !prop.types().is_only(PropType::Boolean) {
            return Box::default();
        }
        prop.defaults().map(|default| default.range()).collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<TextRange> {
        Some(*range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This Boolean prop has a default value."
                },
            )
            .note(markup! {
                "Vue already defaults an absent Boolean prop to "<Emphasis>"false"</Emphasis>". A default of "<Emphasis>"true"</Emphasis>" means the prop can only be turned off by explicitly passing "<Emphasis>"false"</Emphasis>"."
            })
            .note(markup! {
                "Remove the default value. If the prop should be enabled by default, invert its name and meaning instead, for example "<Emphasis>"disabled"</Emphasis>" instead of "<Emphasis>"enabled"</Emphasis>"."
            }),
        )
    }
}

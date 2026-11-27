use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_deprecated_model_definition::NoVueDeprecatedModelDefinitionOptions;

declare_lint_rule! {
    /// Disallow the deprecated `model` option in Vue components.
    ///
    /// In Vue 2, the `model` option let a component choose which prop and event `v-model` uses.
    /// Vue 3 no longer supports this option: `v-model` on a component always passes the
    /// `modelValue` prop and listens for the `update:modelValue` event, so a `model` option has
    /// no effect.
    ///
    /// Rename the prop to `modelValue` and emit `update:modelValue` instead. To keep a custom prop
    /// name, use a named `v-model`, such as `v-model:checked`, which uses the `checked` prop and
    /// the `update:checked` event.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/v-model.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   model: {
    ///     prop: "checked",
    ///     event: "change",
    ///   },
    ///   props: ["checked"],
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: ["modelValue"],
    ///   emits: ["update:modelValue"],
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueDeprecatedModelDefinition {
        version: "2.6.5",
        name: "noVueDeprecatedModelDefinition",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-model-definition").same()],
    }
}

impl Rule for NoVueDeprecatedModelDefinition {
    type Query = VueComponent;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedModelDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().model_option_range().map(|_| ())
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().model_option_range()
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().model_option_range()?,
                markup! {
                    "The "<Emphasis>"model"</Emphasis>" option is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 ignores this option. "<Emphasis>"v-model"</Emphasis>" on a component always uses the "<Emphasis>"modelValue"</Emphasis>" prop and the "<Emphasis>"update:modelValue"</Emphasis>" event."
            })
            .note(markup! {
                "Rename the prop to "<Emphasis>"modelValue"</Emphasis>" and emit "<Emphasis>"update:modelValue"</Emphasis>", or keep the prop name and use a named "<Emphasis>"v-model"</Emphasis>" such as "<Emphasis>"v-model:checked"</Emphasis>". See the "<Hyperlink href="https://v3-migration.vuejs.org/breaking-changes/v-model.html">"Vue 3 migration guide"</Hyperlink>" for more information."
            }),
        )
    }
}

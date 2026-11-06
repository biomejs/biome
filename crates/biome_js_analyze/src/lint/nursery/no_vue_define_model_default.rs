use crate::services::vue::VueModel;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_define_model_default::NoVueDefineModelDefaultOptions;

declare_lint_rule! {
    /// Disallow default values in Vue's `defineModel()`.
    ///
    /// `defineModel()` declares a value that a parent component binds with `v-model`. When the
    /// parent passes `undefined`, for example because its variable has no initial value yet, the
    /// child component uses the `default` value instead. The parent still holds `undefined`, so
    /// the two components show different values for the same model until one of them updates it.
    ///
    /// To avoid this, don't give the model a default value. Give the variable an initial value
    /// in the parent component instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const model = defineModel({ type: String, default: "" });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const count = defineModel<number>("count", { default: 0 });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const model = defineModel({ type: String, required: true });
    /// const count = defineModel<number>("count");
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to check that default values match the type of the prop instead, see
    ///   [`useVueValidDefaultProp`](https://biomejs.dev/linter/rules/use-vue-valid-default-prop/).
    ///
    pub NoVueDefineModelDefault {
        version: "next",
        name: "noVueDefineModelDefault",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-define-model-default").same()],
    }
}

impl Rule for NoVueDefineModelDefault {
    type Query = VueModel;
    /// The range of a default value given to the model.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDefineModelDefaultOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().defaults().map(|default| default.range()).collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<TextRange> {
        Some(*range)
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The model "<Emphasis>{ctx.query().name()}</Emphasis>" has a default value."
                },
            )
            .note(markup! {
                "When the parent component passes "<Emphasis>"undefined"</Emphasis>" with "<Emphasis>"v-model"</Emphasis>", this component uses the default value while the parent keeps "<Emphasis>"undefined"</Emphasis>". The two components then show different values."
            })
            .note(markup! {
                "Remove the default value, and give the variable an initial value in the parent component instead."
            }),
        )
    }
}

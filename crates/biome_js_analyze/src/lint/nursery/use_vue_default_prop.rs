use crate::services::vue::VueProp;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::use_vue_default_prop::UseVueDefaultPropOptions;
use biome_vue_semantic::{PropForm, PropType, Tri, TypeSet};

declare_lint_rule! {
    /// Require a default value for optional props in Vue components.
    ///
    /// When a parent component doesn't pass an optional prop, Vue sets it to `undefined`. Code
    /// that uses the prop then has to handle `undefined` every time, and it's easy to forget.
    /// A default value says what the component should use when the prop is left out.
    ///
    /// This rule reports props that are neither required nor given a default value. Props that
    /// only accept `Boolean` are not reported, because Vue already sets them to `false` when they
    /// are left out.
    ///
    /// Props declared with a type, as in `defineProps<{ size?: number }>()`, are only checked
    /// when the component gives defaults through `withDefaults()` or by destructuring the props.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     size: Number,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({
    ///   size: {
    ///     type: Number,
    ///     required: false,
    ///   },
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// withDefaults(defineProps<{ size?: number; label?: string }>(), {
    ///   label: "Button",
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const { size } = defineProps<{ size?: number }>();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const { size = 1, label, disabled } = defineProps<{
    ///   size?: number;
    ///   label: string;
    ///   disabled?: boolean;
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: {
    ///     size: {
    ///       type: Number,
    ///       default: 1,
    ///     },
    ///     label: {
    ///       type: String,
    ///       required: true,
    ///     },
    ///     disabled: Boolean,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow default values for `Boolean` props, see [`noVueBooleanDefault`](https://biomejs.dev/linter/rules/no-vue-boolean-default/).
    /// - If you want to disallow required props that also have a default value, see [`noVueRequiredPropWithDefault`](https://biomejs.dev/linter/rules/no-vue-required-prop-with-default/).
    ///
    pub UseVueDefaultProp {
        version: "2.6.5",
        name: "useVueDefaultProp",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-default-prop").same()],
    }
}

impl Rule for UseVueDefaultProp {
    type Query = VueProp;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueDefaultPropOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        let types = prop.types();
        if prop.required() != Tri::No
            || types == PropType::Boolean
            || prop.defaults().next().is_some()
            || !prop.has_complete_defaults()
        {
            return None;
        }
        let needs_default = match prop.form()? {
            // `props: ['size']` gives no way to write a default.
            PropForm::ArrayString => false,
            // `size: options` may name a variable that holds the prop's
            // options, default included, so only known constructors count.
            PropForm::Constructor => !types.is_empty() && !types.contains(PropType::Unknown),
            PropForm::Options => true,
            // Vue ignores the parts of a type it cannot turn into a runtime
            // type when the type includes `boolean`, so a prop such as
            // `flag?: boolean | undefined` is a Boolean prop.
            PropForm::TypeMember => {
                (types & !TypeSet::from(PropType::Unknown)) != PropType::Boolean
                    && !prop.external_default_sources().is_empty()
            }
        };
        needs_default.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let prop = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                prop.range(),
                markup! {
                    "The optional prop "<Emphasis>{prop.name()}</Emphasis>" has no default value."
                },
            )
            .note(markup! {
                "When a parent component doesn't pass this prop, its value is "<Emphasis>"undefined"</Emphasis>", which the component has to handle everywhere it uses the prop."
            })
            .note(markup! {
                "Give the prop a default value, or mark it as required if parent components must always pass it."
            }),
        )
    }
}

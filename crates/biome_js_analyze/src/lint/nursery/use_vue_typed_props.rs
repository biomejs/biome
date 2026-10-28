use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::use_vue_typed_props::UseVueTypedPropsOptions;
use biome_vue_semantic::{Prop, PropForm};

declare_lint_rule! {
    /// Require a type for every prop of a Vue component.
    ///
    /// A prop's type tells Vue what kind of value to expect. When a parent passes a value of
    /// the wrong kind, Vue warns about it during development. A prop declared without a type
    /// accepts anything, so these mistakes go unnoticed, and readers of the component can't tell
    /// what the prop is for.
    ///
    /// This rule checks props declared through the `props` option, `defineProps()`, and
    /// `defineModel()`. A prop counts as typed when it names a type, either as a constructor such
    /// as `String`, in a `type` option, or with a TypeScript type, or when it has a `validator`
    /// function.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: ["title"],
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     title: { default: "" },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({ title: {} });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const title = defineModel("title");
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: {
    ///     title: String,
    ///     size: [Number, String],
    ///     status: { type: String, required: true },
    ///     level: { validator: (value) => value > 0 },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{ title: string }>();
    /// const count = defineModel<number>("count");
    /// const visible = defineModel("visible", { type: Boolean });
    /// </script>
    /// ```
    ///
    pub UseVueTypedProps {
        version: "next",
        name: "useVueTypedProps",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-prop-types").same()],
    }
}

impl Rule for UseVueTypedProps {
    type Query = VueComponent;
    type State = Prop;
    type Signals = Box<[Self::State]>;
    type Options = UseVueTypedPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        component
            .props()
            .chain(component.models())
            .filter(|prop| !has_type(prop))
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, prop: &Self::State) -> Option<TextRange> {
        Some(prop.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, prop: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                prop.range(),
                markup! {
                    "The prop "<Emphasis>{prop.name()}</Emphasis>" has no type."
                },
            )
            .note(markup! {
                "Without a type, Vue accepts any value for this prop and can't warn when a component receives the wrong kind of value."
            })
            .note(markup! {
                "Declare the type of the prop, for example with "<Emphasis>"String"</Emphasis>" or "<Emphasis>"{ type: String }"</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` when the declaration of `prop` says what values it accepts.
fn has_type(prop: &Prop) -> bool {
    prop.form() == Some(PropForm::TypeMember)
        || !prop.types().is_empty()
        || prop.validator_range().is_some()
}

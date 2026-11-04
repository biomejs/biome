use crate::services::vue::{VueReference, VueSemanticService};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_vue_undeclared_emits::NoVueUndeclaredEmitsOptions;
use biome_vue_semantic::{RefBase, RefSite, Resolution};

declare_lint_rule! {
    /// Require Vue components to declare the events they emit.
    ///
    /// A component lists the events it can emit with the `emits` option or with `defineEmits()`.
    /// This list tells other developers which events they can listen for. It also tells Vue
    /// which listeners belong to the component: a listener for an event that is not in the
    /// list, such as `@close`, is also added to the root element of the component. If that
    /// element sends a native event with the same name, the listener runs a second time.
    ///
    /// This rule reports each event that is emitted with a name the component does not
    /// declare. Events are emitted with `this.$emit('name')`, with the `emit` function of the
    /// `setup()` context, with the function that `defineEmits()` returns, and with
    /// `$emit('name')` in the template.
    ///
    /// An event named `update:name` counts as declared when the component declares a model
    /// called `name` with `defineModel()`, because Vue declares that event for the model.
    ///
    /// :::note
    /// In `.vue` files, this rule needs to read the template to check the events the template
    /// emits, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule only checks the events emitted in the
    /// `<script>` block.
    /// :::
    ///
    /// The rule doesn't report an event when it can't know every event the component declares.
    /// That is the case when:
    ///
    /// - the list of events is not written directly in the component, as in `emits: eventNames`
    ///   or `emits: [...baseEvents]`;
    /// - `defineEmits()` uses a TypeScript type that is imported or can't be read;
    /// - code written elsewhere adds to the component: `mixins`, `extends`, an object copied
    ///   into the options with `...`, or a `<script src>` block.
    ///
    /// The rule also ignores events whose name is not written as a string, as in
    /// `emit(eventName)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   emits: ["change"],
    ///   methods: {
    ///     close() {
    ///       this.$emit("close");
    ///     },
    ///   },
    /// });
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const emit = defineEmits(["change"]);
    ///
    /// function close() {
    ///   emit("close");
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <button @click="$emit('close')">Close</button>
    /// </template>
    ///
    /// <script setup>
    /// defineEmits(["change"]);
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <button @click="$emit('close')">Close</button>
    /// </template>
    ///
    /// <script setup>
    /// const emit = defineEmits(["change", "close"]);
    ///
    /// function onInput(value) {
    ///   emit("change", value);
    /// }
    /// </script>
    /// ```
    ///
    pub NoVueUndeclaredEmits {
        version: "2.6.5",
        name: "noVueUndeclaredEmits",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-explicit-emits").same()],
    }
}

impl Rule for NoVueUndeclaredEmits {
    type Query = VueReference;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueUndeclaredEmitsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        if reference.site() != RefSite::EventName
            || reference.resolution() != Resolution::Unresolved
        {
            return None;
        }
        // `$emit` in a template whose file defines no component, such as a
        // single-file component without a script, has no declaration the
        // event could be added to.
        if reference.base() == RefBase::Builtin
            && ctx
                .get_service::<VueSemanticService>()?
                .model()
                .template_component()
                .is_none()
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let reference = ctx.query();
        let name = reference.name();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {
                    "The event "<Emphasis>{name}</Emphasis>" is emitted, but the component doesn't declare it."
                },
            )
            .note(markup! {
                "Vue also adds a listener for an undeclared event to the root element of the component. If that element sends a native event with the same name, such as "<Emphasis>"click"</Emphasis>", the listener runs twice."
            })
            .note(markup! {
                "Add "<Emphasis>{name}</Emphasis>" to the "<Emphasis>"emits"</Emphasis>" option or to the "<Emphasis>"defineEmits()"</Emphasis>" call of the component."
            }),
        )
    }
}

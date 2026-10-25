use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_unused_emit_declarations::NoVueUnusedEmitDeclarationsOptions;
use biome_vue_semantic::{Component, ComponentKind, Emit, Namespace, OpenReason, OpenReasons};
use enumflags2::make_bitflags;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallow declaring events that a Vue component never emits.
    ///
    /// A component lists the events it can emit with the `emits` option or with `defineEmits()`.
    /// An event that is declared but never emitted misleads the people who use the component:
    /// they can listen for the event, but their listener is never called. Such a declaration is
    /// usually left behind when the code that emitted the event was removed.
    ///
    /// This rule reports a declared event when neither the script nor the template of the
    /// component emits it. In the template, events are emitted with `$emit('name')` or with the
    /// function that `defineEmits()` returns.
    ///
    /// An event named `update:name` counts as emitted when the component declares a model called
    /// `name` with `defineModel()`, because Vue emits that event whenever the model changes.
    ///
    /// :::note
    /// In `.vue` files, this rule needs to read the template, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule only checks components defined in JavaScript
    /// and TypeScript files.
    /// :::
    ///
    /// The rule reports nothing for a component when it can't know every event the component
    /// emits. That is the case when:
    ///
    /// - an event name is not written as a string, as in `emit(eventName)`;
    /// - the function that emits events, the `setup()` context, or the component itself (`this`)
    ///   is stored or handed to other code, as in `useForm(emit)`;
    /// - code written elsewhere runs as part of the component and can emit the events too:
    ///   `mixins`, `extends`, an object copied into the options with `...`, a `setup` function
    ///   that is not written inside the component, or a `<script src>` block;
    /// - the template can't be read, because it is kept in another file, written in a language
    ///   other than HTML, or given as a string in the `template` option;
    /// - the component is an app created with `createApp()`, whose template can be written in
    ///   the page.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   emits: ["change", "close"],
    ///   methods: {
    ///     close() {
    ///       this.$emit("close");
    ///     },
    ///   },
    /// });
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script setup lang="ts">
    /// const emit = defineEmits<{
    ///   change: [value: string];
    /// }>();
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
    pub NoVueUnusedEmitDeclarations {
        version: "2.6.3",
        name: "noVueUnusedEmitDeclarations",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-unused-emit-declarations").same()],
    }
}

impl Rule for NoVueUnusedEmitDeclarations {
    type Query = VueComponent;
    /// An event the component declares but never emits.
    type State = Emit;
    type Signals = Box<[Self::State]>;
    type Options = NoVueUnusedEmitDeclarationsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let declared: Vec<Emit> = component.emits().collect();
        if declared.is_empty() {
            return Box::default();
        }

        // Vue merges the default export of the plain `<script>` block into a
        // `<script setup>` component, so both can declare and emit an event.
        let other = component
            .plain_block_component()
            .or_else(|| component.setup_block_component());
        if has_unseen_emitters(component) || other.as_ref().is_some_and(has_unseen_emitters) {
            return Box::default();
        }

        // An event can be declared more than once, for example with one call
        // signature for each payload type. An emit call counts for all of
        // them, so events are compared by name.
        let other_declared: Vec<Emit> = other.iter().flat_map(Component::emits).collect();
        let emitted: FxHashSet<&str> = declared
            .iter()
            .chain(&other_declared)
            .filter(|emit| emit.uses().next().is_some())
            .map(|emit| emit.name())
            .collect();
        // `defineModel('name')` emits `update:name` whenever the model changes.
        let models: Vec<_> = std::iter::once(component)
            .chain(other.as_ref())
            .flat_map(Component::models)
            .collect();
        let model_names: FxHashSet<&str> = models.iter().map(|model| model.name()).collect();

        declared
            .iter()
            .filter(|emit| {
                emit.has_complete_uses()
                    && !emitted.contains(emit.name())
                    && emit
                        .name()
                        .strip_prefix("update:")
                        .is_none_or(|model_name| !model_names.contains(model_name))
            })
            .cloned()
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, emit: &Self::State) -> Option<TextRange> {
        Some(emit.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, emit: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                emit.range(),
                markup! {
                    "The event "<Emphasis>{emit.name()}</Emphasis>" is declared but never emitted."
                },
            )
            .note(markup! {
                "Other components can listen for this event, but their listeners are never called."
            })
            .note(markup! {
                "Remove the declaration, or emit the event."
            }),
        )
    }
}

/// The reasons that mean code written outside the definition of a component
/// runs as part of it: a mixin, a component it extends, an object spread into
/// its options, or a script kept in another file.
const UNSEEN_EMITTERS: OpenReasons = make_bitflags!(OpenReason::{Extends | Spread | ExternalSrc});

/// Returns `true` when `component` can emit events from code the rule does
/// not see.
///
/// An app created with `createApp()` counts, because its template can be
/// written in the page.
fn has_unseen_emitters(component: &Component) -> bool {
    component.kind() == ComponentKind::CreateApp
        || component
            .open_reasons(Namespace::Emit)
            .intersects(UNSEEN_EMITTERS)
}

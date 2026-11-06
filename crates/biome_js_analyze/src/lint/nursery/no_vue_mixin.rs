use crate::services::vue::VueComponent;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_vue_mixin::NoVueMixinOptions;

declare_lint_rule! {
    /// Disallow the `mixins` option in Vue components.
    ///
    /// A mixin is an object of component options, such as `data` or `methods`, that Vue merges
    /// into every component that lists it in `mixins`. Vue 3 still supports mixins, but they cause
    /// several problems:
    ///
    /// - It is hard to tell which mixin a property of the component comes from.
    /// - Two mixins, or a mixin and the component, can declare a property with the same name, and
    ///   one silently replaces the other.
    /// - Mixins often depend on properties that another mixin or the component is expected to
    ///   declare, without anything in the code that makes this visible.
    ///
    /// In Vue 3, share logic between components with composables instead. A composable is a
    /// function that uses the Composition API, such as `ref()` and `computed()`, and returns the
    /// values a component needs. The component calls it and names what it gets back, so every
    /// property has a visible origin.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import counterMixin from "./counterMixin";
    ///
    /// export default {
    ///   mixins: [counterMixin],
    /// };
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    /// import counterMixin from "./counterMixin";
    ///
    /// export default defineComponent({
    ///   mixins: [counterMixin],
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { useCounter } from "./useCounter";
    ///
    /// const { count, increment } = useCounter();
    /// </script>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [Composables](https://vuejs.org/guide/reusability/composables.html)
    /// - [Comparison of composables with mixins](https://vuejs.org/guide/reusability/composables.html#vs-mixins)
    ///
    pub NoVueMixin {
        version: "2.6.5",
        name: "noVueMixin",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for NoVueMixin {
    type Query = VueComponent;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueMixinOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().mixins_range().map(|_| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().mixins_range()?,
                markup! {
                    "This component uses mixins."
                },
            )
            .note(markup! {
                "Mixins hide where the properties of a component come from, and properties with the same name in different mixins silently replace each other."
            })
            .note(markup! {
                "Move the shared logic into a "<Hyperlink href="https://vuejs.org/guide/reusability/composables.html">"composable"</Hyperlink>" and call it from "<Emphasis>"setup()"</Emphasis>" or "<Emphasis>"<script setup>"</Emphasis>" instead."
            }),
        )
    }
}

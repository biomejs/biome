use crate::services::vue::{VueComponent, VueSemanticService};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rule_options::use_vue_one_component_per_file::UseVueOneComponentPerFileOptions;
use biome_vue_semantic::{Component, ComponentKind};

declare_lint_rule! {
    /// Enforce that each file defines at most one Vue component.
    ///
    /// When every component lives in its own file, components are easier to find, and each file
    /// stays small enough to read and change on its own.
    ///
    /// The rule counts every component that a file defines:
    ///
    /// - a `<script setup>` block;
    /// - `export default { ... }` in a `.vue` file;
    /// - `defineComponent()` called with an options object or a setup function;
    /// - `createApp()` called with an options object.
    ///
    /// A component that is only imported, as in `createApp(App)`, isn't counted. The default
    /// export of a plain `<script>` block that sits next to a `<script setup>` block is part of the
    /// same component, so it isn't counted either.
    ///
    /// When a file defines more than one component, the rule reports each of them, except the
    /// main component of a `.vue` file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { defineComponent, h } from "vue";
    ///
    /// const Badge = defineComponent({
    ///   render: () => h("span", "New"),
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { defineComponent } from "vue";
    ///
    /// const TodoItem = defineComponent({
    ///   props: ["title"],
    /// });
    ///
    /// export default {
    ///   components: { TodoItem },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import Badge from "./Badge.vue";
    /// </script>
    /// ```
    ///
    /// ```js
    /// import { createApp } from "vue";
    /// import App from "./App.vue";
    ///
    /// createApp(App).mount("#app");
    /// ```
    ///
    /// ## Resources
    ///
    /// - [Vue style guide: component files](https://vuejs.org/style-guide/rules-strongly-recommended.html#component-files)
    ///
    pub UseVueOneComponentPerFile {
        version: "next",
        name: "useVueOneComponentPerFile",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("one-component-per-file").inspired()],
    }
}

impl Rule for UseVueOneComponentPerFile {
    type Query = VueComponent;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueOneComponentPerFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        if !is_counted(component) || is_main_component(component) {
            return None;
        }
        let model = ctx.get_service::<VueSemanticService>()?.model();
        let count = model
            .components()
            .filter(is_counted)
            .take(2)
            .count();
        (count > 1).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This file defines more than one component."
                },
            )
            .note(markup! {
                "Components that share a file are harder to find, and the file grows harder to read and change."
            })
            .note(markup! {
                "Move this component to its own file."
            }),
        )
    }
}

/// Returns `true` when `component` is defined in the file, and isn't merged
/// into the `<script setup>` component of the same file.
fn is_counted(component: &Component) -> bool {
    component.is_defined_in_place() && component.setup_block_component().is_none()
}

/// Returns `true` when `component` is the component of a `.vue` file: its
/// `<script setup>` block, or the default export of its plain `<script>`
/// block.
fn is_main_component(component: &Component) -> bool {
    component.kind() == ComponentKind::Setup || component.is_default_export()
}

use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rule_options::use_vue_define_options::UseVueDefineOptionsOptions;

declare_lint_rule! {
    /// Enforce `defineOptions()` instead of a default export in components that use `<script setup>`.
    ///
    /// A Vue single-file component can have two script blocks: a plain `<script>` block and a
    /// `<script setup>` block. Before Vue 3.3, options such as `name` or `inheritAttrs` could
    /// only be set with `export default { ... }` in the plain `<script>` block, so the component
    /// was split across both blocks.
    ///
    /// Since Vue 3.3, the `defineOptions()` macro sets these options directly inside
    /// `<script setup>`. A macro is a function that the Vue compiler handles at build time, so it
    /// doesn't need to be imported. Using it keeps the whole component in one block.
    ///
    /// This rule reports the default export of a plain `<script>` block when the same file also
    /// has a `<script setup>` block.
    ///
    /// :::note
    /// This rule needs to read every script block of a `.vue` file, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, Biome only checks the first script block of the file,
    /// and this rule reports nothing.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script>
    /// export default {
    ///   name: "MyComponent",
    ///   inheritAttrs: false,
    /// };
    /// </script>
    ///
    /// <script setup>
    /// const props = defineProps(["title"]);
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// defineOptions({
    ///   name: "MyComponent",
    ///   inheritAttrs: false,
    /// });
    ///
    /// const props = defineProps(["title"]);
    /// </script>
    /// ```
    ///
    /// A component without `<script setup>` still uses a default export:
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   name: "MyComponent",
    /// };
    /// </script>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [`defineOptions()`](https://vuejs.org/api/sfc-script-setup.html#defineoptions)
    ///
    pub UseVueDefineOptions {
        version: "next",
        name: "useVueDefineOptions",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-define-options").same()],
    }
}

impl Rule for UseVueDefineOptions {
    type Query = VueComponent;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueDefineOptionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Only the default export of a plain `<script>` block is merged into
        // a `<script setup>` component.
        ctx.query().setup_block_component().map(|_| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This component sets its options with a default export, but it also uses "<Emphasis>"<script setup>"</Emphasis>"."
                },
            )
            .note(markup! {
                "Splitting the component across two script blocks makes it harder to read. The "<Emphasis>"defineOptions()"</Emphasis>" macro sets the same options inside "<Emphasis>"<script setup>"</Emphasis>"."
            })
            .note(markup! {
                "Move these options into a "<Emphasis>"defineOptions()"</Emphasis>" call in the "<Emphasis>"<script setup>"</Emphasis>" block, and remove the default export. Declare props and events with "<Emphasis>"defineProps()"</Emphasis>" and "<Emphasis>"defineEmits()"</Emphasis>" instead."
            }),
        )
    }
}

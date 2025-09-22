use crate::frameworks::vue::vue_component::{
    VueComponent, VueComponentDeclarations, VueComponentQuery, VueDeclaration,
    VueDeclarationCollectionFilter, VueDeclarationName,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsFileSource;
use biome_rule_options::no_vue_reserved_props::NoVueReservedPropsOptions;
use enumflags2::make_bitflags;

declare_lint_rule! {
    /// Disallow reserved names to be used as props.
    ///
    /// Vue reserves certain prop names for its internal use. Using these reserved names
    /// as prop names can cause conflicts and unexpected behavior in your Vue components.
    ///
    /// This rule prevents the use of the following reserved prop names:
    /// - `key` - Used by Vue for list rendering and component identification
    /// - `ref` - Used by Vue for template refs
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({
    ///     ref: String,
    /// });
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import {defineComponent} from 'vue';
    ///
    /// export default defineComponent({
    ///     props: [
    ///         'key',
    ///     ]
    /// });
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineProps<{
    ///     ref: string,
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     props: {
    ///         key: String,
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import {defineComponent} from 'vue';
    ///
    /// export default defineComponent({
    ///     props: ['foo']
    /// });
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// defineProps({ foo: String });
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{
    ///     foo: string,
    ///     bar: string,
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///     props: {
    ///         foo: String,
    ///         bar: String,
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueReservedProps {
        version: "2.1.2",
        name: "noVueReservedProps",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-reserved-props").same()],
    }
}

impl Rule for NoVueReservedProps {
    type Query = VueComponentQuery;
    type State = VueDeclaration;
    type Signals = Box<[Self::State]>;
    type Options = NoVueReservedPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        ) else {
            return Box::new([]);
        };

        component
            .declarations(make_bitflags!(VueDeclarationCollectionFilter::Prop))
            .into_iter()
            .filter_map(|declaration| {
                let name = declaration.declaration_name()?;
                if RESERVED_PROPS.contains(&name.text()) {
                    Some(declaration)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.declaration_name_range()?,
            markup! {
                <Emphasis>{state.declaration_name()?.text()}</Emphasis>" is a reserved attribute and cannot be used as props."
            },
        ).note(
            markup! {
                "Rename the prop to avoid possible conflicts."
            },
        ))
    }
}

/// List of reserved Vue props for Vue version 3.x.
const RESERVED_PROPS: &[&str] = &["key", "ref"];

use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_excessive_props::NoVueExcessivePropsOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Limit the number of props a Vue component can declare.
    ///
    /// A component with many props usually does too many things. Each prop is
    /// another input that the component has to handle and that its users have to
    /// understand. Splitting the component into smaller components keeps each of
    /// them easier to read, use, and test.
    ///
    /// This rule counts the props declared with the `props` option, and with
    /// `defineProps()` in `<script setup>`. A prop name that is declared more
    /// than once is counted once.
    ///
    /// The rule only counts props whose names it can read. For example, it can't
    /// count props declared with a spread (`...sharedProps`), or with a type
    /// imported from another file.
    ///
    /// ## Examples
    ///
    /// The examples below set `maxProps` to `2`.
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxProps": 2
    ///     }
    /// }
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup lang="ts">
    /// defineProps<{
    ///   title: string;
    ///   subtitle: string;
    ///   icon: string;
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script>
    /// export default {
    ///   props: {
    ///     title: String,
    ///     subtitle: String,
    ///     icon: String,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// defineProps(["title", "subtitle"]);
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxProps`
    ///
    /// Default: `10`
    ///
    /// The maximum number of props a component can declare. It must be at least `1`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxProps": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// defineProps({
    ///   title: String,
    ///   subtitle: String,
    /// });
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// defineProps({
    ///   title: String,
    /// });
    /// </script>
    /// ```
    ///
    pub NoVueExcessiveProps {
        version: "next",
        name: "noVueExcessiveProps",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("max-props").same()],
    }
}

impl Rule for NoVueExcessiveProps {
    type Query = VueComponent;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoVueExcessivePropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let max_props = usize::from(ctx.options().max_props().get());
        // Duplicate names only lower the count, so a component whose
        // declarations fit within the limit needs no deduplication.
        if component.props().count() <= max_props {
            return None;
        }
        let props: Vec<_> = component.props().collect();
        let prop_count = props
            .iter()
            .map(|prop| prop.name())
            .collect::<FxHashSet<_>>()
            .len();
        if prop_count <= max_props {
            return None;
        }
        let range = props
            .iter()
            .map(|prop| prop.declaration_range())
            .reduce(|range, declaration| range.cover(declaration))?;
        Some(RuleState { prop_count, range })
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This component declares too many props ("{state.prop_count}"). The maximum allowed is "{ctx.options().max_props().get()}"."
                },
            )
            .note(markup! {
                "Each prop is another input that the component has to handle, which makes it harder to read, use, and test."
            })
            .note(markup! {
                "Split the component into smaller components that each need fewer props."
            }),
        )
    }
}

pub struct RuleState {
    /// The number of distinct prop names the component declares.
    prop_count: usize,
    /// The range covering every prop declaration of the component.
    range: TextRange,
}

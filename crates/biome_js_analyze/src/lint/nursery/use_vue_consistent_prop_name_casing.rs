use crate::services::vue::VueProp;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rule_options::use_vue_consistent_prop_name_casing::{
    PropNameCasing, UseVueConsistentPropNameCasingOptions,
};
use biome_string_case::Case;

declare_lint_rule! {
    /// Enforce a consistent casing style for prop names in Vue components.
    ///
    /// Vue converts prop names between camelCase and kebab-case automatically, so a prop
    /// declared as `greetingText` can be passed in a template as `greeting-text`. The
    /// [Vue style guide](https://vuejs.org/style-guide/rules-strongly-recommended.html#prop-name-casing)
    /// recommends declaring prop names in camelCase, the usual naming style for JavaScript
    /// variables. Mixing naming styles makes props harder to find and to use.
    ///
    /// The rule checks props declared with the `props` option, `defineProps()`, and
    /// TypeScript types passed to `defineProps()`.
    ///
    /// Names written only with characters from writing systems without uppercase and
    /// lowercase letters, such as `漢字`, are accepted in both styles. Names that mix
    /// those characters with Latin letters are reported. Names that start with `_` or `$`,
    /// or that contain other symbols, don't follow either style.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     greeting_text: String,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps(["greeting-text"]);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineProps<{ GreetingText: string }>();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{ greetingText: string; size: number }>();
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `casing`
    ///
    /// Default: `"camelCase"`
    ///
    /// The casing that prop names must follow: `"camelCase"` or `"snake_case"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "casing": "snake_case"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// defineProps({
    ///   greetingText: String,
    /// });
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// defineProps({
    ///   greeting_text: String,
    /// });
    /// </script>
    /// ```
    ///
    /// ### `ignore`
    ///
    /// Default: `[]`
    ///
    /// A list of prop names that aren't checked. Names are matched exactly.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": ["greeting_text"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// defineProps({
    ///   greeting_text: String,
    /// });
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to enforce hyphenated prop names in templates, see [`useVueHyphenatedAttributes`](https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes/).
    ///
    pub UseVueConsistentPropNameCasing {
        version: "next",
        name: "useVueConsistentPropNameCasing",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prop-name-casing").inspired()],
    }
}

impl Rule for UseVueConsistentPropNameCasing {
    type Query = VueProp;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentPropNameCasingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let name = ctx.query().name();
        let accepted = match options.casing() {
            PropNameCasing::Camel => Case::Camel | Case::Uni,
            PropNameCasing::Snake => Case::Snake | Case::Uni,
        };
        if accepted.contains(Case::identify(name, false))
            || options.ignore().iter().any(|ignored| ignored.as_ref() == name)
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let prop = ctx.query();
        let casing = CasingName(ctx.options().casing());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                prop.range(),
                markup! {
                    "The prop name "<Emphasis>{prop.name()}</Emphasis>" isn't in "{casing}"."
                },
            )
            .note(markup! {
                "Using one naming style for all props makes them easier to find and to use."
            })
            .note(markup! {
                "Rename the prop to "{casing}"."
            }),
        )
    }
}

/// Writes the name of a casing style as users write it in the rule options.
struct CasingName(PropNameCasing);

impl biome_console::fmt::Display for CasingName {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            PropNameCasing::Camel => "camelCase",
            PropNameCasing::Snake => "snake_case",
        })
    }
}

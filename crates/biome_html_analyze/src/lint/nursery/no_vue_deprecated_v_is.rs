use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::VueDirective;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_v_is::NoVueDeprecatedVIsOptions;

declare_lint_rule! {
    /// Disallow the deprecated Vue `v-is` directive.
    ///
    /// The `v-is` directive renders a component in place of a native HTML element.
    /// It is deprecated since Vue 3.1.
    ///
    /// Use the `is` attribute with a `vue:` prefix instead, for example `is="vue:MyComponent"`.
    /// Unlike `v-is`, the value of `is` is plain text, so the component name doesn't need extra quotes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-is="'MyComponent'"></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div is="vue:MyComponent"></div>
    ///   <component is="MyComponent"></component>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `is` attribute](https://vuejs.org/api/built-in-special-attributes.html#is)
    /// - [Vue 3 migration guide: custom elements](https://v3-migration.vuejs.org/breaking-changes/custom-elements-interop.html)
    pub NoVueDeprecatedVIs {
        version: "next",
        name: "noVueDeprecatedVIs",
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-v-is").same()],
    }
}

impl Rule for NoVueDeprecatedVIs {
    type Query = Ast<VueDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedVIsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query().name_token().ok()?;
        (name.text_trimmed() == "v-is").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"v-is"</Emphasis>" directive is deprecated."
                },
            )
            .note(markup! {
                <Emphasis>"v-is"</Emphasis>" is deprecated since Vue 3.1 and may be removed in a future version."
            })
            .note(markup! {
                "Use the "<Emphasis>"is"</Emphasis>" attribute with a "<Emphasis>"vue:"</Emphasis>" prefix instead, like "<Emphasis>"is=\"vue:MyComponent\""</Emphasis>"."
            }),
        )
    }
}

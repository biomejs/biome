use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlAttribute;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_deprecated_inline_template::NoVueDeprecatedInlineTemplateOptions;

declare_lint_rule! {
    /// Disallow the `inline-template` attribute on Vue components.
    ///
    /// In Vue 2, the `inline-template` attribute made a component use the content between
    /// its tags as its own template. Vue 3 removed this feature. The content is now passed
    /// to the component like any other content between its tags, and the component only
    /// shows it if its template has a `<slot>`.
    ///
    /// Move the content into the component's own template instead. If the parent needs to
    /// decide what is shown, pass the content through a slot.
    ///
    /// Only the attribute written exactly as `inline-template` is reported.
    /// A bound value such as `:inline-template="value"` is an ordinary prop and is allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <my-component inline-template>
    ///     <div>{{ message }}</div>
    ///   </my-component>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <my-component>
    ///     <div>{{ message }}</div>
    ///   </my-component>
    ///   <my-component :inline-template="value" />
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue 3 migration guide: `inline-template` attribute](https://v3-migration.vuejs.org/breaking-changes/inline-template-attribute.html)
    /// - [Vue slots](https://vuejs.org/guide/components/slots.html)
    pub NoVueDeprecatedInlineTemplate {
        version: "2.6.5",
        name: "noVueDeprecatedInlineTemplate",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-inline-template").same()],
    }
}

impl Rule for NoVueDeprecatedInlineTemplate {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedInlineTemplateOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let name = ctx.query().name().ok()?.value_token().ok()?;
        (name.text_trimmed() == "inline-template").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query().name().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.range(),
                markup! {
                    "The "<Emphasis>"inline-template"</Emphasis>" attribute is not supported in Vue 3."
                },
            )
            .note(markup! {
                "Vue 3 no longer uses the content between the component's tags as its template. The component only shows that content if its template has a "<Emphasis>"<slot>"</Emphasis>"."
            })
            .note(markup! {
                "Move the content into the component's own template, or pass it to the component through a slot."
            }),
        )
    }
}

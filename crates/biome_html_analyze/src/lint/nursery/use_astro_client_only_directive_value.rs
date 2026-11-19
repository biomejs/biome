use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::AstroClientDirective;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_astro_client_only_directive_value::UseAstroClientOnlyDirectiveValueOptions;

declare_lint_rule! {
    /// Require a value for Astro's `client:only` directive.
    ///
    /// `client:only` skips server rendering, so Astro needs a framework value to select the client renderer.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <Component client:only />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <Component client:only="react" />
    /// ```
    ///
    /// Dynamic expression values are accepted without resolving their bindings.
    ///
    /// ```astro
    /// <Component client:only={renderer} />
    /// ```
    ///
    /// The rule only checks whether an initializer is present. Empty values are accepted.
    ///
    /// ## References
    ///
    /// - [Astro client directives](https://docs.astro.build/en/reference/directives-reference/#clientonly)
    pub UseAstroClientOnlyDirectiveValue {
        version: "2.5.9",
        name: "useAstroClientOnlyDirectiveValue",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("missing-client-only-directive-value").inspired()],
    }
}

impl Rule for UseAstroClientOnlyDirectiveValue {
    type Query = Ast<AstroClientDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseAstroClientOnlyDirectiveValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }
        let value = ctx.query().value().ok()?;
        let name = value.name().ok()?.token_text_trimmed()?;
        (name.text() == "only" && value.initializer().is_none()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    <Emphasis>"client:only"</Emphasis>" is missing a framework value."
                },
            )
            .note(markup! {
                "Astro skips server rendering for "<Emphasis>"client:only"</Emphasis>", so it cannot infer which renderer should load the component."
            })
            .note(markup! {
                "Add the component framework as the directive value, for example "<Emphasis>"client:only=\"react\""</Emphasis>"."
            }),
        )
    }
}

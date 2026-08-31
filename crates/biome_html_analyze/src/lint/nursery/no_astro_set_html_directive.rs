use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AstroSetDirective;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_astro_set_html_directive::NoAstroSetHtmlDirectiveOptions;

declare_lint_rule! {
    /// Disallow the use of Astro's `set:html` directive.
    ///
    /// `set:html` renders HTML without escaping it. Using `set:html` can introduce cross-site scripting vulnerabilities.
    /// When raw HTML is required, sanitize the value before passing it to `set:html`, then suppress the diagnostic with an explanation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <div set:html={content} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <div>{content}</div>
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro `set:html` directive](https://docs.astro.build/en/reference/directives-reference/#sethtml)
    /// - [OWASP HTML sanitization guidance](https://cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.html#html-sanitization)
    pub NoAstroSetHtmlDirective {
        version: "2.5.11",
        name: "noAstroSetHtmlDirective",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-set-html-directive").same()],
    }
}

impl Rule for NoAstroSetHtmlDirective {
    type Query = Ast<AstroSetDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAstroSetHtmlDirectiveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }

        let value = ctx.query().value().ok()?;
        let name = value.name().ok()?.token_text_trimmed()?;
        (name.text() == "html").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"set:html"</Emphasis>" directive inserts unescaped HTML."
                },
            )
            .note(markup! {
                "Using "<Emphasis>"set:html"</Emphasis>" can introduce cross-site scripting vulnerabilities."
            })
            .note(markup! {
                "Use a regular Astro expression to render text, or suppress this diagnostic with an explanation if raw HTML is required."
            }),
        )
    }
}

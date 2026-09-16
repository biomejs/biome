use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::SvelteHtmlBlock;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_at_html_tags::NoSvelteAtHtmlTagsOptions;

declare_lint_rule! {
    /// Disallow the use of Svelte's `{@html}` tag.
    ///
    /// The `{@html}` tag renders its value as unescaped HTML. This can lead to cross-site
    /// scripting (XSS) vulnerabilities when the value contains untrusted content.
    ///
    /// If raw HTML is required, sanitize the value before passing it to `{@html}`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {@html content}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {content}
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte HTML tag](https://svelte.dev/docs/svelte/@html)
    ///
    pub NoSvelteAtHtmlTags {
        version: "2.5.11",
        name: "noSvelteAtHtmlTags",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-at-html-tags").same()],
    }
}

impl Rule for NoSvelteAtHtmlTags {
    type Query = Ast<SvelteHtmlBlock>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteAtHtmlTagsOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"{@html}"</Emphasis>" tag renders unescaped HTML."
                },
            )
            .note(markup! {
                "Using "<Emphasis>"{@html}"</Emphasis>" can lead to cross-site scripting (XSS) vulnerabilities."
            })
            .note(markup! {
                "Render the value as text, or sanitize it before using "<Emphasis>"{@html}"</Emphasis>"."
            }),
        )
    }
}

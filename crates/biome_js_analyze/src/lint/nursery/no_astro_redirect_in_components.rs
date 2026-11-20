use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_astro_redirect_in_components::NoAstroRedirectInComponentsOptions;

declare_lint_rule! {
    /// Disallow Astro redirects in component frontmatter outside a `pages` directory.
    ///
    /// Astro redirects must be returned from a page or middleware. A child component
    /// cannot redirect the page response and may run after response headers are sent.
    /// This rule identifies pages by a `pages` path segment; custom route integrations
    /// are not resolved.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,file=src/components/Layout.astro
    /// ---
    /// return Astro.redirect("/login");
    /// ---
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,file=src/pages/account.astro
    /// ---
    /// return Astro.redirect("/login");
    /// ---
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro redirect](https://docs.astro.build/en/reference/api-reference/#redirect)
    pub NoAstroRedirectInComponents {
        version: "next",
        name: "noAstroRedirectInComponents",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroRedirectInComponents {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAstroRedirectInComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_frontmatter()
            || ctx
                .file_path()
                .components()
                .any(|part| part.as_str() == "pages")
        {
            return None;
        }
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        if member.member_name()?.text() != "redirect" {
            return None;
        }
        let object = member.object().ok()?.omit_parentheses();
        let identifier = object.as_js_identifier_expression()?.name().ok()?;
        if identifier.value_token().ok()?.text_trimmed() != "Astro"
            || ctx.model().binding(&identifier).is_some()
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! {
            "This Astro redirect is outside a pages directory."
        }).note(markup! { "A child component cannot redirect the page response." })
        .note(markup! { "Move the redirect to the page frontmatter or middleware and return its result there." }))
    }
}

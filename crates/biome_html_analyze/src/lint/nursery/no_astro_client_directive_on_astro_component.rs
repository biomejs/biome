use crate::services::embedded::EmbeddedService;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_embeds::components::component_name_segments;
use biome_html_syntax::{AstroClientDirective, element_ext::AnyHtmlTagElement};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_astro_client_directive_on_astro_component::NoAstroClientDirectiveOnAstroComponentOptions;

declare_lint_rule! {
    /// Disallow client directives on imported Astro components.
    ///
    /// Astro components render HTML on the server. They cannot be hydrated with
    /// `client:*` directives, which load framework components in the browser.
    /// Put interactive code in a framework component or a bundled `<script>`.
    ///
    /// This rule recognizes default `.astro` imports and named `Image` and `Picture`
    /// imports from `astro:assets`, including aliases. It does not resolve
    /// re-exports, namespace imports, or dynamically selected components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// import Accordion from "./Accordion.astro";
    /// ---
    /// <Accordion client:visible />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// import Accordion from "./Accordion.astro";
    /// import Button from "./Button.svelte";
    /// ---
    /// <Accordion />
    /// <Button client:load />
    /// ```
    pub NoAstroClientDirectiveOnAstroComponent {
        version: "next",
        name: "noAstroClientDirectiveOnAstroComponent",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroClientDirectiveOnAstroComponent {
    type Query = Ast<AstroClientDirective>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoAstroClientDirectiveOnAstroComponentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }
        let directive = ctx.query();
        directive.value().ok()?.name().ok()?.value_token().ok()?;
        let tag = directive
            .syntax()
            .ancestors()
            .find_map(AnyHtmlTagElement::cast)?;
        let names = component_name_segments(&tag)?;
        let [name] = names.as_slice() else {
            return None;
        };
        Some(
            ctx.get_service::<EmbeddedService>()?
                .astro()?
                .component_import(name.text())?
                .range,
        )
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(
            ctx.query()
                .syntax()
                .ancestors()
                .find_map(AnyHtmlTagElement::cast)?
                .range(),
        )
    }

    fn diagnostic(ctx: &RuleContext<Self>, import: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! {
            "This client directive is applied to an Astro component."
        }).detail(*import, markup! { "This import is an Astro component." })
        .note(markup! { "Astro components render on the server and cannot be hydrated in the browser." })
        .note(markup! { "Remove the directive. Use a framework component or a bundled script for browser interactivity." }))
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AstroDefineDirective, HtmlOpeningElement, HtmlSelfClosingElement, HtmlSingleTextExpression,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange, WalkEvent};
use biome_rule_options::no_astro_unapplied_style_vars::NoAstroUnappliedStyleVarsOptions;

declare_lint_rule! {
    /// Disallow style variables in Astro files with no element to receive them.
    ///
    /// Astro applies `<style define:vars={...}>` variables through inline styles
    /// on elements rendered by the same component. A component containing only
    /// styles and text has no element on which to define those CSS variables.
    /// Importing it into another component does not define variables on that
    /// component's elements, even when the style uses `is:global`.
    ///
    /// Add an element that wraps the styled content, or set the variables on an
    /// element in the component that renders that content.
    /// This rule skips files containing other tags or template expressions because
    /// they may render an element. It does not check CSS variable usage.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <style define:vars={{ color: "red" }}>
    ///   p { color: var(--color); }
    /// </style>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <p>Hello</p>
    /// <style define:vars={{ color: "red" }}>
    ///   p { color: var(--color); }
    /// </style>
    /// ```
    pub NoAstroUnappliedStyleVars {
        version: "next",
        name: "noAstroUnappliedStyleVars",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroUnappliedStyleVars {
    type Query = Ast<AstroDefineDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAstroUnappliedStyleVarsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }
        let directive = ctx.query();
        let value = directive.value().ok()?;
        if value.name().ok()?.value_token().ok()?.text_trimmed() != "vars" {
            return None;
        }
        value.initializer()?.value().ok()?;
        let tag = directive
            .syntax()
            .ancestors()
            .find_map(AnyHtmlTagElement::cast)?;
        if tag.tag_name()?.text() != "style" {
            return None;
        }
        let kinds = HtmlOpeningElement::KIND_SET
            .union(HtmlSelfClosingElement::KIND_SET)
            .union(HtmlSingleTextExpression::KIND_SET);
        for event in ctx.root().syntax().preorder_of_kinds(kinds) {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if HtmlSingleTextExpression::can_cast(node.kind()) {
                return None;
            }
            let element = AnyHtmlTagElement::cast(node)?;
            if element.tag_name()?.text() != "style" {
                return None;
            }
        }
        Some(())
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

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! {
            "These style variables have no element to receive them."
        }).note(markup! { "Astro defines CSS variables on elements rendered by this component, not on its parent's elements." })
        .note(markup! { "Add an element around the styled content, or define the variables in the component that renders it." }))
    }
}

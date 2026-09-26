use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, AnySvelteTemplateElement, HtmlAttribute};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_tailwind_static_class_strings::UseTailwindStaticClassStringsOptions;

use biome_tailwind_logic::class_context::TailwindClassContext;

declare_lint_rule! {
    /// Require complete, statically written class names.
    ///
    /// Tailwind detects class names in source text. Dynamic string construction
    /// can hide complete class names from its source scanner. Choose between
    /// complete class names instead. Class strings containing interpolation are
    /// reported, including interpolations separated by whitespace.
    ///
    /// This rule checks Svelte class attribute interpolation. JavaScript expressions
    /// inside HTML attributes, including Vue bindings and Astro expressions, are not inspected.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <div class="bg-{color}"></div>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <div class="p-4 {classes}"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <div class="p-4 text-white"></div>
    /// <div class={active ? "bg-red-500" : "bg-blue-500"}></div>
    /// ```
    pub UseTailwindStaticClassStrings {
        version: "next",
        name: "useTailwindStaticClassStrings",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        sources: &[RuleSource::EslintShadcn("require-static-classes").inspired()],
    }
}

impl Rule for UseTailwindStaticClassStrings {
    type Query = TailwindClassContext<HtmlAttribute>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseTailwindStaticClassStringsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let Some(AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(template)) = attribute
            .initializer()
            .and_then(|initializer| initializer.value().ok())
        else {
            return None;
        };
        template
            .elements()
            .iter()
            .any(|element| {
                matches!(
                    element,
                    AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(_)
                )
            })
            .then(|| template.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(), range,
            markup! { "This class string is constructed dynamically." },
        )
        .note(markup! { "Tailwind scans source text for complete class names." })
        .note(markup! { "Use complete class names, choosing between them with a conditional expression or a lookup." }))
    }
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_iframe_title::UseIframeTitleOptions;

declare_lint_rule! {
    /// Enforces the usage of the attribute `title` for the element `iframe`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <iframe></iframe>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <iframe title=""></iframe>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <iframe title="This is a unique title"></iframe>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.1](https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub UseIframeTitle {
        version: "next",
        name: "useIframeTitle",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("iframe-has-title").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseIframeTitle {
    type Query = Ast<AnyHtmlElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseIframeTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if !is_iframe_element(element) {
            return None;
        }

        if let Some(title_attribute) = element.find_attribute_by_name("title")
            && let Some(initializer) = title_attribute.initializer()
            && let Ok(value) = initializer.value()
            && let Some(value) = value.string_value()
            && !value.trim_ascii().is_empty()
        {
            return None;
        }

        Some(element.syntax().text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                "Provide a "<Emphasis>"title"</Emphasis>" attribute when using "<Emphasis>"iframe"</Emphasis>" elements."
            }
            )
            .note(markup! {
                "Screen readers rely on the title set on an iframe to describe the content being displayed."
            }),
        )
    }
}

fn is_iframe_element(element: &AnyHtmlElement) -> bool {
    element
        .name()
        .is_some_and(|token_text| token_text.eq_ignore_ascii_case("iframe"))
}

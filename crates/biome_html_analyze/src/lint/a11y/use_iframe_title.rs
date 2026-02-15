use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_iframe_title::UseIframeTitleOptions;

use crate::a11y::has_non_empty_attribute;

declare_lint_rule! {
    /// Enforces the usage of the attribute `title` for the element `iframe`.
    ///
    /// :::note
    /// In `.html` files, this rule matches `iframe` elements case-insensitively (e.g., `<IFRAME>`, `<IFrame>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase `<iframe>` is checked. PascalCase variants like `<Iframe>` are assumed to be custom components and are ignored.
    /// :::
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
    /// <iframe title="title"></iframe>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.1](https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub UseIframeTitle {
        version: "2.4.0",
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
        let file_extension = ctx.file_path().extension()?;

        if !is_iframe_element(element, file_extension) {
            return None;
        }

        if has_non_empty_attribute(element, "title") {
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

/// Checks if the element is an iframe element.
///
/// - In `.html` files, matching is case-insensitive.
/// - In component-based frameworks, only lowercase `iframe` is matched to avoid flagging custom components like `<Iframe>`.
fn is_iframe_element(element: &AnyHtmlElement, file_extension: &str) -> bool {
    element.name().is_some_and(|token_text| {
        let is_html_file = file_extension == "html";
        if is_html_file {
            token_text.eq_ignore_ascii_case("iframe")
        } else {
            token_text == "iframe"
        }
    })
}

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{T, element_ext::AnyHtmlTagElement};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_iframe_title::UseIframeTitleOptions;

use crate::a11y::has_non_empty_attribute;

declare_lint_rule! {
    /// Require a `title` on every `<iframe>`.
    ///
    /// Screen readers use the title to identify the embedded content before a user enters the
    /// frame. A static HTML title must be non-empty and should describe the frame's purpose. Vue
    /// bindings are accepted without evaluating the value they produce at runtime.
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
        sources: &[RuleSource::EslintJsxA11y("iframe-has-title").inspired(), RuleSource::HtmlEslint("require-frame-title").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseIframeTitle {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseIframeTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.tag_name_kind() != Some(T![iframe]) {
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

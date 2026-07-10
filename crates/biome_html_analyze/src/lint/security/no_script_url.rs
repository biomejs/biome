use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlOpeningElement, element_ext::AnyHtmlTagElement};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_script_url::NoScriptUrlOptions;
use biome_string_case::StrOnlyExtension;

use crate::utils::is_html_tag;

declare_lint_rule! {
    /// Disallow `javascript:` URLs in HTML.
    ///
    /// Using `javascript:` URLs is considered a form of `eval` and can be a security risk.
    /// These URLs can execute arbitrary JavaScript code, which can lead to cross-site scripting (XSS) vulnerabilities.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <a href="javascript:void(0)">Click me</a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a href="javascript:alert('XSS')">Click me</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <a href="https://example.com">Click me</a>
    /// <a href="/path/to/page">Click me</a>
    /// <a href="#section">Click me</a>
    /// <span href="javascript:void(0)">Not a real href</span>
    /// ```
    ///
    pub NoScriptUrl {
        version: "2.3.9",
        name: "noScriptUrl",
        language: "html",
        // Show equivalents from related ecosystems
        sources: &[
            RuleSource::Eslint("no-script-url").same(),
            RuleSource::EslintReact("jsx-no-script-url").same(),
            RuleSource::EslintQwik("jsx-no-script-url").same(),
            RuleSource::EslintSolid("jsx-no-script-url").same(),
            RuleSource::EslintReactDom("no-script-url").same(),
            RuleSource::EslintReactXyz("dom-no-script-url").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoScriptUrl {
    type Query = Ast<HtmlOpeningElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoScriptUrlOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let element = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        if !is_html_tag(&AnyHtmlTagElement::from(element.clone()), source_type, "a") {
            return None;
        }

        let attrs = element.attributes();
        let href_attribute = attrs.find_attribute_or_vue_binding("href")?;
        let value = href_attribute.as_static_value()?;

        if value
            .text()
            .trim()
            .to_lowercase_cow()
            .starts_with("javascript:")
        {
            return Some(href_attribute.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Avoid using "<Emphasis>"javascript:"</Emphasis>" URLs, as they can be a security risk."
                },
            )
            .note(markup! {
                "Using "<Emphasis>"javascript:"</Emphasis>" URLs can lead to security vulnerabilities such as cross-site scripting (XSS)."
            })
            .note(markup! {
                "Consider using regular URLs, or if you need to handle click events, use event handlers instead."
            }),
        )
    }
}

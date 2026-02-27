use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlOpeningElement, inner_string_text};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_script_url::NoScriptUrlOptions;
use biome_string_case::StrOnlyExtension;

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

        // Only check <a> elements for HTML (unlike JSX where components/custom elements exist)
        let name = element.name().ok()?;
        let tag = name.token_text_trimmed()?;
        if !tag.eq_ignore_ascii_case("a") {
            return None;
        }

        let attrs = element.attributes();
        let attr = attrs.find_by_name("href")?;
        let initializer = attr.initializer()?;
        let value = initializer.value().ok()?;

        if let AnyHtmlAttributeInitializer::HtmlString(html_string) = value
            && let Ok(token) = html_string.value_token()
        {
            let inner = inner_string_text(&token);
            if inner.trim().to_lowercase_cow().starts_with("javascript:") {
                return Some(initializer.range());
            }
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

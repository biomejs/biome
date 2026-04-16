use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::use_iframe_sandbox::UseIframeSandboxOptions;

declare_lint_rule! {
    /// Enforce the 'sandbox' attribute for 'iframe' elements.
    ///
    /// The sandbox attribute enables an extra set of restrictions for the content in the iframe.
    /// Using the sandbox attribute is considered a good security practice.
    ///
    /// See [the Mozilla docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#sandbox) for details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <iframe src="https://example.com"></iframe>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <iframe src="https://example.com" sandbox="allow-popups"></iframe>
    /// ```
    ///
    pub UseIframeSandbox {
        version: "next",
        name: "useIframeSandbox",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for UseIframeSandbox {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseIframeSandboxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if !is_iframe_element(element, ctx) {
            return None;
        }

        if element.find_attribute_by_name("sandbox").is_none() {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Iframe doesn't have the "<Emphasis>"sandbox"</Emphasis>" attribute."
                }
            )
            .note(markup! {
                "The sandbox attribute enables an extra set of restrictions for the content in the iframe, protecting against malicious scripts and other security threats."
            })
            .note(markup! {
                "Provide a "<Emphasis>"sandbox"</Emphasis>" attribute when using iframe elements."
            }),
        )
    }
}

fn is_iframe_element(element: &AnyHtmlElement, ctx: &RuleContext<UseIframeSandbox>) -> bool {
    let Some(element_name) = element.name() else {
        return false;
    };

    let source_type = ctx.source_type::<HtmlFileSource>();

    // In HTML files: case-insensitive (IFRAME, Iframe, iframe all match)
    // In component frameworks (Vue, Svelte, Astro): case-sensitive (only "iframe" matches)
    // This means <Iframe> in Vue/Svelte is treated as a component and ignored
    if source_type.is_html() {
        element_name.text().eq_ignore_ascii_case("iframe")
    } else {
        element_name.text() == "iframe"
    }
}

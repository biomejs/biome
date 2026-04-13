use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
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
    /// ```jsx,expect_diagnostic
    /// function MyComponent() {
    ///   return <iframe src="https://example.com" />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyComponent() {
    ///   return <iframe src="https://example.com" sandbox="allow-popups" />;
    /// }
    /// ```
    ///
    pub UseIframeSandbox {
        version: "next",
        name: "useIframeSandbox",
        language: "jsx",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintReactDom("no-missing-iframe-sandbox").inspired(), RuleSource::EslintReactXyz("dom-no-missing-iframe-sandbox").same()],
    }
}

impl Rule for UseIframeSandbox {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseIframeSandboxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?.name_value_token().ok()?;

        if name.text_trimmed() != "iframe" {
            return None;
        }

        if element
            .find_attribute_by_name("sandbox")
            .is_none_or(|sandbox_attribute| sandbox_attribute.as_static_value().is_none())
            && !element.has_spread_prop()
        {
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
                "Provide a "<Emphasis>"sandbox"</Emphasis>" attribute when using "<Emphasis>"iframe"</Emphasis>" elements."
            }),
        )
    }
}

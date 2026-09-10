use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_unsafe_iframe_sandbox::NoUnsafeIframeSandboxOptions;

declare_lint_rule! {
    /// Disallow an unsafe combination of the `sandbox` attribute.
    ///
    /// This rule reports cases where the attribute may contain `allow-scripts` and `allow-same-origin` at the same time,
    /// as this combination allows the embedded document to remove the `sandbox` attribute and bypass the restrictions.
    ///
    /// See [Play safely in sandboxed IFrames](https://web.dev/articles/sandboxed-iframes) or [this Stack Overflow answer](https://stackoverflow.com/a/62431584) for more details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function MyComponent() {
    ///   return <iframe src="https://example.com" sandbox="allow-scripts allow-same-origin" />;
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
    pub NoUnsafeIframeSandbox {
        version: "2.5.13",
        name: "noUnsafeIframeSandbox",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintReactDom("no-unsafe-iframe-sandbox").same(), RuleSource::EslintReactXyz("dom-no-unsafe-iframe-sandbox").same()],
    }
}

impl Rule for NoUnsafeIframeSandbox {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeIframeSandboxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token().ok()?;

        if name.text_trimmed() != "iframe" {
            return None;
        }

        let attr = node.find_attribute_by_name("sandbox")?;
        let value = attr.as_static_value()?;
        let text = value.text();

        let mut has_scripts = false;
        let mut has_same_origin = false;
        for token in text.split_ascii_whitespace() {
            if token.eq_ignore_ascii_case("allow-scripts") {
                has_scripts = true;
            } else if token.eq_ignore_ascii_case("allow-same-origin") {
                has_same_origin = true;
            } else {
                continue;
            }
            if has_scripts && has_same_origin {
                return Some(attr.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Unsafe combination of "<Emphasis>"allow-scripts"</Emphasis>" and "<Emphasis>"allow-same-origin"</Emphasis>" in the "<Emphasis>"sandbox"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "This lets the embedded document run scripts in the origin of the embedding page, so it can remove its own "<Emphasis>"sandbox"</Emphasis>" attribute and escape the restrictions."
            })
            .note(markup! {
                "Remove "<Emphasis>"allow-scripts"</Emphasis>" or "<Emphasis>"allow-same-origin"</Emphasis>" from the "<Emphasis>"sandbox"</Emphasis>" attribute."
            })
            .note(markup!{
                "See "<Hyperlink href="https://web.dev/articles/sandboxed-iframes">"Play safely in sandboxed IFrames"</Hyperlink>" or "<Hyperlink href="https://stackoverflow.com/a/62431584">"this Stack Overflow answer"</Hyperlink>" for more details."
            }),
        )
    }
}

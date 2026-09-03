use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::T;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::TextRange;
use biome_rule_options::no_unsafe_iframe_sandbox::NoUnsafeIframeSandboxOptions;

declare_lint_rule! {
    /// Disallow an unsafe combination of the sandbox attribute.
    ///
    /// This rule reports cases where the attribute contains `allow-scripts` and `allow-same-origin` at the same time,
    /// as this combination allows the embedded document to remove the sandbox attribute and bypass the restrictions.
    ///
    /// See [Play safely in sandboxed IFrames](https://web.dev/articles/sandboxed-iframes) for more details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <iframe src="https://example.com" sandbox="allow-scripts allow-same-origin"></iframe>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <iframe src="https://example.com" sandbox="allow-popups"></iframe>
    /// ```
    ///
    pub NoUnsafeIframeSandbox {
        version: "next",
        name: "noUnsafeIframeSandbox",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintReactDom("no-unsafe-iframe-sandbox").inspired(), RuleSource::EslintReactXyz("dom-no-unsafe-iframe-sandbox").inspired()],
    }
}

impl Rule for NoUnsafeIframeSandbox {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeIframeSandboxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.tag_name_kind() != Some(T![iframe]) {
            return None;
        }

        let attr = element.find_attribute_or_vue_binding("sandbox")?;
        let value = attr.as_static_value()?;
        let text = value.text();

        let has_scripts = text
            .split_ascii_whitespace()
            .any(|token| token == "allow-scripts");
        let has_same_origin = text
            .split_ascii_whitespace()
            .any(|token| token == "allow-same-origin");

        if has_scripts && has_same_origin {
            return Some(attr.range());
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
            }),
        )
    }
}

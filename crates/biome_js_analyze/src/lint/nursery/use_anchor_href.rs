use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_anchor_href::UseAnchorHrefOptions;

declare_lint_rule! {
    /// Enforces `href` attribute for `<a>` elements.
    ///
    /// Ensures `<a>` tags are either valid links (with href) or replaced with buttons for actions.
    /// See [WCAG 4.1.2](https://www.w3.org/WAI/WCAG22/Understanding/name-role-value) for accessibility requirements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a>Link</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a target="_blank">External</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href="/home">Home</a>
    /// ```
    ///
    /// ```jsx
    /// <a href="https://example.com" target="_blank">External</a>
    /// ```
    pub UseAnchorHref {
        version: "next",
        name: "useAnchorHref",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("jsx-a").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseAnchorHref {
    type Query = Ast<AnyJsxElement>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseAnchorHrefOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let tag_token = element.name().ok()?.name_value_token().ok()?;
        if tag_token.text_trimmed() != "a" {
            return None;
        }
        if element.find_attribute_by_name("href").is_none() {
            return Some(element.range());
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Missing required "<Emphasis>"href"</Emphasis>" attribute on "<Emphasis>"a"</Emphasis>" element"
            },
        )
        .note(markup! {
            "Anchor tags without href attributes are inaccessible to keyboard navigation and screen readers, violating WCAG 2.2 Success Criterion 4.1.2 (Name, Role, Value)."
        })
        .note(markup! {
            "Use "<Emphasis>"href"</Emphasis>" for navigation or "<Emphasis>"<button>"</Emphasis>" for actions. "
            "Reference: "
            <Hyperlink href="https://www.w3.org/WAI/WCAG22/Understanding/name-role-value">"WCAG 4.1.2"</Hyperlink>
        })
        .into()
    }
}

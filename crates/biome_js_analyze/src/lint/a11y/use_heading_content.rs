use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{jsx_ext::AnyJsxElement, JsxElement};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1><div aria-hidden /></h1>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 aria-label="Screen reader content" aria-hidden>invisible content</h1>
    /// ```
    ///
    ///
    /// ```jsx,expect_diagnostic
    /// <h1></h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <h1>heading</h1>
    /// ```
    ///
    /// ```jsx
    /// <h1><div aria-hidden="true"></div>visible content</h1>
    /// ```
    ///
    /// ```jsx
    /// <h1 aria-label="Screen reader content"><div aria-hidden="true">invisible content</div></h1>
    /// ```
    ///
    /// ```jsx
    /// <h1 dangerouslySetInnerHTML={{ __html: "heading" }} />
    /// ```
    ///
    /// ```jsx
    /// <h1><div aria-hidden />visible content</h1>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.6](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html)
    ///
    pub UseHeadingContent {
        version: "1.0.0",
        name: "useHeadingContent",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("heading-has-content")],
        recommended: true,
        severity: Severity::Error,
    }
}

const HEADING_ELEMENTS: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];

impl Rule for UseHeadingContent {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token().ok()?;

        if HEADING_ELEMENTS.contains(&name.text_trimmed()) {
            if node.has_truthy_attribute("aria-hidden") {
                return Some(());
            }

            // When node has `aria-label` (and doesn't have `aria-hidden`), the label will be read by screen readers
            if node.has_truthy_attribute("aria-label") {
                return None;
            }

            if has_valid_heading_content(node) {
                return None;
            }

            match node {
                AnyJsxElement::JsxOpeningElement(opening_element) => {
                    if !opening_element.has_accessible_child() {
                        return Some(());
                    }
                }
                AnyJsxElement::JsxSelfClosingElement(_) => return Some(()),
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            AnyJsxElement::JsxOpeningElement(node) => node
                .parent::<JsxElement>()?
                .syntax()
                .text_range_with_trivia(),
            AnyJsxElement::JsxSelfClosingElement(node) => node.syntax().text_trimmed_range(),
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"heading"</Emphasis>"  elements."
            },
        ).note(
            "All headings on a page should have content that is accessible to screen readers."
        ))
    }
}

/// check if the node has a valid heading attribute
fn has_valid_heading_content(node: &AnyJsxElement) -> bool {
    node.find_attribute_by_name("dangerouslySetInnerHTML")
        .is_some()
        || node
            .find_attribute_by_name("children")
            .is_some_and(|attribute| {
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .map_or(true, |attribute| !attribute.is_falsy())
            })
        || node.has_spread_prop()
}

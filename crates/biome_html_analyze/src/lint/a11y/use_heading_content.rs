use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlElementList, HtmlFileSource,
};
use biome_rowan::AstNode;

use crate::a11y::{
    get_truthy_aria_hidden_attribute, has_accessible_name, html_element_has_truthy_aria_hidden,
    html_self_closing_element_has_accessible_name,
    html_self_closing_element_has_non_empty_attribute,
    html_self_closing_element_has_truthy_aria_hidden,
};

declare_lint_rule! {
    /// Enforce that heading elements (h1, h2, h3, h4, h5, h6) have content and that the content is accessible to screen readers.
    ///
    /// Accessible means that the content is not hidden using the `aria-hidden` attribute.
    /// Heading elements should have text content that describes the section for screen reader users.
    /// Alternatively, headings can have an accessible name via the `aria-label`, `aria-labelledby`, or `title` attribute.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<H1>`, `<h1>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<H1>` are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <h1></h1>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <h1 />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <h1><div aria-hidden="true">content</div></h1>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <h1 aria-hidden="true">content</h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <h1>heading</h1>
    /// ```
    ///
    /// ```html
    /// <h1><div aria-hidden="true"></div>visible content</h1>
    /// ```
    ///
    /// ```html
    /// <h1 aria-label="Screen reader content"></h1>
    /// ```
    ///
    /// ```html
    /// <h2 title="Section heading"></h2>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.6](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html)
    ///
    pub UseHeadingContent {
        version: "2.5.0",
        name: "useHeadingContent",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("heading-has-content").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

const HEADING_ELEMENTS: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];

impl Rule for UseHeadingContent {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Check if element is a heading tag (h1-h6)
        // In HTML files, tag names are case-insensitive
        // In component frameworks (Vue, Svelte, Astro), only lowercase is checked
        let element_name = node.name()?;
        let element_text = element_name.text();
        let source_type = ctx.source_type::<HtmlFileSource>();
        let is_heading = if source_type.is_html() {
            HEADING_ELEMENTS.iter().any(|h| element_text.eq_ignore_ascii_case(h))
        } else {
            HEADING_ELEMENTS.contains(&element_text)
        };
        if !is_heading {
            return None;
        }

        // Check if the heading itself has aria-hidden attribute
        if get_truthy_aria_hidden_attribute(node).is_some() {
            return Some(());
        }

        // Check if heading has accessible name via aria-label, aria-labelledby, or title
        if has_accessible_name(node) {
            return None;
        }

        // Handle self-closing headings - they have no content
        if node.as_html_element().is_none() {
            return Some(());
        }

        let html_element = node.as_html_element()?;
        // Skip analysis if we can't fully parse the element to avoid false positives
        if html_element.opening_element().is_err() {
            return None;
        }

        // Check if the heading has accessible content
        if has_accessible_content(&html_element.children()) {
            return None;
        }

        // No accessible content found - emit diagnostic
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"heading"</Emphasis>" elements."
            },
        )
        .note(markup! {
            "All headings on a page should have content that is accessible to screen readers."
        })
        .note(markup! {
            "Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies."
        })
        .note(markup! {
            "Follow this link for more information,\n "<Hyperlink href="https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html">"WCAG 2.4.6"</Hyperlink>""
        });

        Some(diagnostic)
    }
}

/// Checks if `HtmlElementList` contains accessible content (non-empty text or visible elements).
fn has_accessible_content(html_child_list: &HtmlElementList) -> bool {
    html_child_list.into_iter().any(|child| match &child {
        AnyHtmlElement::AnyHtmlContent(content) => is_accessible_text_content(content),
        AnyHtmlElement::HtmlElement(element) => {
            if html_element_has_truthy_aria_hidden(element) {
                false
            } else {
                has_accessible_content(&element.children())
            }
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            if html_self_closing_element_has_truthy_aria_hidden(element) {
                return false;
            }

            if html_self_closing_element_has_accessible_name(element) {
                return true;
            }

            let tag_text = element.name().ok().and_then(|n| n.token_text_trimmed());

            match tag_text.as_ref().map(|t| t.as_ref()) {
                Some(name) if name.eq_ignore_ascii_case("img") => {
                    html_self_closing_element_has_non_empty_attribute(element, "alt")
                }
                Some(name)
                    if name.eq_ignore_ascii_case("br")
                        || name.eq_ignore_ascii_case("hr")
                        || name.eq_ignore_ascii_case("wbr")
                        || name.eq_ignore_ascii_case("meta")
                        || name.eq_ignore_ascii_case("link")
                        || name.eq_ignore_ascii_case("base")
                        || name.eq_ignore_ascii_case("col") =>
                {
                    false
                }
                Some(name) if name.eq_ignore_ascii_case("input") => {
                    let is_hidden = element.find_attribute_by_name("type").is_some_and(|attr| {
                        attr.initializer()
                            .and_then(|init| init.value().ok())
                            .and_then(|value| value.string_value())
                            .is_some_and(|s| s.eq_ignore_ascii_case("hidden"))
                    });
                    !is_hidden
                }
                _ => false,
            }
        }
        AnyHtmlElement::HtmlBogusElement(_) | AnyHtmlElement::HtmlCdataSection(_) => true,
    })
}

/// Checks if the content node contains non-empty text.
fn is_accessible_text_content(content: &AnyHtmlContent) -> bool {
    match content {
        AnyHtmlContent::HtmlContent(html_content) => html_content
            .value_token()
            .is_ok_and(|token| !token.text_trimmed().is_empty()),
        // Text expressions (e.g., {{ variable }}) are considered accessible
        AnyHtmlContent::AnyHtmlTextExpression(_) => true,
        // Embedded content is treated as potentially accessible to avoid false positives
        AnyHtmlContent::HtmlEmbeddedContent(_) => true,
    }
}

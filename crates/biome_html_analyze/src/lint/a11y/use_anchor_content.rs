use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlAttribute, HtmlElementList, HtmlFileSource,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_anchor_content::UseAnchorContentOptions;

use crate::HtmlRuleAction;
use crate::a11y::{
    get_truthy_aria_hidden_attribute, has_accessible_name, html_element_has_truthy_aria_hidden,
    html_self_closing_element_has_accessible_name,
    html_self_closing_element_has_non_empty_attribute,
    html_self_closing_element_has_truthy_aria_hidden,
};

declare_lint_rule! {
    /// Enforce that anchors have content and that the content is accessible to screen readers.
    ///
    /// Accessible means the content is not hidden using the `aria-hidden` attribute.
    /// Anchor tags should have text content that describes the link destination for screen reader users.
    /// Alternatively, the anchor can have an accessible name via the `aria-label` or `title` attribute.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<A>`, `<a>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<A>` are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <a></a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a>   </a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a aria-hidden="true">content</a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a><span aria-hidden="true">content</span></a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <a>content</a>
    /// ```
    ///
    /// ```html
    /// <a><span>content</span></a>
    /// ```
    ///
    /// ```html
    /// <a><span aria-hidden="true"></span>content</a>
    /// ```
    ///
    /// ```html
    /// <a aria-label="Navigate to home"></a>
    /// ```
    ///
    /// ```html
    /// <a title="Home page"></a>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.4](https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub UseAnchorContent {
        version: "2.4.0",
        name: "useAnchorContent",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("anchor-has-content").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

/// State to track whether the issue is aria-hidden on the anchor itself
pub struct UseAnchorContentState {
    aria_hidden_attribute: Option<HtmlAttribute>,
}

impl Rule for UseAnchorContent {
    type Query = Ast<AnyHtmlElement>;
    type State = UseAnchorContentState;
    type Signals = Option<Self::State>;
    type Options = UseAnchorContentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Check if element is an anchor tag
        // In HTML files, tag names are case-insensitive
        // In component frameworks (Vue, Svelte, Astro), only lowercase is checked
        let element_name = node.name()?;
        let source_type = ctx.source_type::<HtmlFileSource>();
        let is_anchor = if source_type.is_html() {
            element_name.text().eq_ignore_ascii_case("a")
        } else {
            element_name.text() == "a"
        };
        if !is_anchor {
            return None;
        }

        // Check if the anchor itself has aria-hidden attribute
        if let Some(aria_hidden_attr) = get_truthy_aria_hidden_attribute(node) {
            return Some(UseAnchorContentState {
                aria_hidden_attribute: Some(aria_hidden_attr),
            });
        }

        // Check if anchor has accessible name via aria-label or title
        if has_accessible_name(node) {
            return None;
        }

        // Handle self-closing anchors - they have no content
        if node.as_html_element().is_none() {
            return Some(UseAnchorContentState {
                aria_hidden_attribute: None,
            });
        }

        let html_element = node.as_html_element()?;
        // Skip analysis if we can't fully parse the element to avoid false positives
        if html_element.opening_element().is_err() {
            return None;
        }

        // Check if the anchor has accessible content
        if has_accessible_content(&html_element.children()) {
            return None;
        }

        // No accessible content found - emit diagnostic
        Some(UseAnchorContentState {
            aria_hidden_attribute: None,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"a"</Emphasis>" elements."
            },
        )
        .note(markup! {
            "All links on a page should have content that is accessible to screen readers."
        })
        .note(markup! {
            "Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies."
        })
        .note(markup! {
            "Follow these links for more information,\n "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">"WCAG 2.4.4"</Hyperlink>"\n "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">"WCAG 4.1.2"</Hyperlink>""
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let aria_hidden = state.aria_hidden_attribute.as_ref()?;
        let mut mutation = ctx.root().begin();
        mutation.remove_node(aria_hidden.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"aria-hidden"</Emphasis>" attribute to allow the anchor element and its content visible to assistive technologies." }.to_owned(),
            mutation,
        ))
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

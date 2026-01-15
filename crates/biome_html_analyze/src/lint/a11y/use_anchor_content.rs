use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlAttribute, HtmlElementList, HtmlFileSource,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::HtmlRuleAction;

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
        version: "next",
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
    type Options = ();

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
        if let Some(aria_hidden_attr) = get_truthy_aria_hidden(node) {
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

/// Returns the aria-hidden attribute if it has a truthy value.
fn get_truthy_aria_hidden(node: &AnyHtmlElement) -> Option<HtmlAttribute> {
    let attribute = node.find_attribute_by_name("aria-hidden")?;
    let is_truthy = attribute
        .initializer()
        .and_then(|init| init.value().ok())
        .and_then(|value| value.string_value())
        .is_none_or(|value| !value.eq_ignore_ascii_case("false"));

    if is_truthy { Some(attribute) } else { None }
}

/// Checks if the element has an accessible name via aria-label or title attribute.
fn has_accessible_name(node: &AnyHtmlElement) -> bool {
    // Check aria-label attribute
    if let Some(attr) = node.find_attribute_by_name("aria-label")
        && attr
            .initializer()
            .and_then(|init| init.value().ok())
            .and_then(|value| value.string_value())
            .is_some_and(|s| !s.trim().is_empty())
    {
        return true;
    }

    // Check title attribute
    if let Some(attr) = node.find_attribute_by_name("title")
        && attr
            .initializer()
            .and_then(|init| init.value().ok())
            .and_then(|value| value.string_value())
            .is_some_and(|s| !s.trim().is_empty())
    {
        return true;
    }

    false
}

/// Checks if the given `HtmlElementList` has accessible content.
/// Accessible content is either:
/// - Non-empty text content
/// - Child elements that don't have `aria-hidden="true"`
fn has_accessible_content(html_child_list: &HtmlElementList) -> bool {
    html_child_list.into_iter().any(|child| match &child {
        AnyHtmlElement::AnyHtmlContent(content) => is_accessible_text_content(content),
        AnyHtmlElement::HtmlElement(element) => {
            // Check if this child element has aria-hidden
            let has_aria_hidden =
                element
                    .find_attribute_by_name("aria-hidden")
                    .is_some_and(|attribute| {
                        attribute
                            .initializer()
                            .and_then(|init| init.value().ok())
                            .and_then(|value| value.string_value())
                            .is_none_or(|value| !value.eq_ignore_ascii_case("false"))
                    });

            if has_aria_hidden {
                // This element is hidden, check if there's other accessible content at this level
                false
            } else {
                // Element is not hidden, check if it has accessible content recursively
                has_accessible_content(&element.children())
            }
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            // Check if element is hidden with aria-hidden
            let has_aria_hidden =
                element
                    .find_attribute_by_name("aria-hidden")
                    .is_some_and(|attribute| {
                        attribute
                            .initializer()
                            .and_then(|init| init.value().ok())
                            .and_then(|value| value.string_value())
                            .is_none_or(|value| !value.eq_ignore_ascii_case("false"))
                    });
            if has_aria_hidden {
                return false;
            }

            // Check for explicit accessible name via aria-label or title
            let has_aria_label = element
                .find_attribute_by_name("aria-label")
                .is_some_and(|attr| {
                    attr.initializer()
                        .and_then(|init| init.value().ok())
                        .and_then(|value| value.string_value())
                        .is_some_and(|s| !s.trim().is_empty())
                });
            if has_aria_label {
                return true;
            }

            let has_title = element.find_attribute_by_name("title").is_some_and(|attr| {
                attr.initializer()
                    .and_then(|init| init.value().ok())
                    .and_then(|value| value.string_value())
                    .is_some_and(|s| !s.trim().is_empty())
            });
            if has_title {
                return true;
            }

            // Check tag-specific accessible content
            let tag_name = element.name().ok().and_then(|n| n.value_token().ok());
            let tag_text = tag_name.as_ref().map(|t| t.text_trimmed());

            match tag_text {
                // <img> requires non-empty alt attribute
                Some(name) if name.eq_ignore_ascii_case("img") => element
                    .find_attribute_by_name("alt")
                    .is_some_and(|attr| {
                        attr.initializer()
                            .and_then(|init| init.value().ok())
                            .and_then(|value| value.string_value())
                            .is_some_and(|s| !s.trim().is_empty())
                    }),
                // Void elements without meaningful content are not accessible
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
                // <input type="hidden"> is not accessible, other inputs may be
                Some(name) if name.eq_ignore_ascii_case("input") => {
                    let is_hidden = element.find_attribute_by_name("type").is_some_and(|attr| {
                        attr.initializer()
                            .and_then(|init| init.value().ok())
                            .and_then(|value| value.string_value())
                            .is_some_and(|s| s.eq_ignore_ascii_case("hidden"))
                    });
                    !is_hidden
                }
                // Other self-closing elements without explicit accessible name are not accessible
                _ => false,
            }
        }
        // Bogus elements and CDATA sections - treat as potentially accessible to avoid false positives
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

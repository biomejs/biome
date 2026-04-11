use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlElementList, HtmlFileSource,
};
use biome_rowan::AstNode;
use biome_rule_options::no_label_without_control::NoLabelWithoutControlOptions;

use crate::a11y::{
    has_accessible_name, html_element_has_truthy_aria_hidden,
    html_self_closing_element_has_accessible_name,
    html_self_closing_element_has_non_empty_attribute,
};

declare_lint_rule! {
    /// Enforce that a label element has a text label and an associated control.
    ///
    /// A label element without a text label or an associated control is meaningless to users
    /// who rely on assistive technologies such as screen readers.
    ///
    /// There are two supported ways to associate a label with a control:
    /// - Wrapping a control inside a label element.
    /// - Adding a `for` attribute to a label and assigning it the DOM ID of a control on the page.
    ///
    /// A label is considered to have accessible text content when it meets one of the following
    /// conditions:
    /// - Has non-whitespace text content.
    /// - Has an `aria-label` attribute with a non-empty value.
    /// - Has an `aria-labelledby` attribute with a non-empty value.
    ///
    /// :::note
    /// In `.html` files, element names are matched case-insensitively.
    /// In component-based frameworks (Vue, Svelte, Astro), only the lowercase `label` is checked.
    /// PascalCase variants are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <label></label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <label>A label</label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <label for="js_id"></label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <label for="js_id"><input /></label>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <label for="js_id" aria-label="A label"></label>
    /// ```
    ///
    /// ```html
    /// <label for="js_id">A label</label>
    /// ```
    ///
    /// ```html
    /// <label>A label<input /></label>
    /// ```
    ///
    /// ```html
    /// <label>A label<textarea></textarea></label>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 3.3.2](https://www.w3.org/WAI/WCAG21/Understanding/labels-or-instructions)
    ///
    pub NoLabelWithoutControl {
        version: "next",
        name: "noLabelWithoutControl",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("label-has-associated-control").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub struct NoLabelWithoutControlState {
    pub has_text_content: bool,
    pub has_control_association: bool,
}

impl Rule for NoLabelWithoutControl {
    type Query = Ast<AnyHtmlElement>;
    type State = NoLabelWithoutControlState;
    type Signals = Option<Self::State>;
    type Options = NoLabelWithoutControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        let element_name = node.name()?;

        // In HTML, tag names are case-insensitive; in framework files only lowercase matches
        let is_label = if source_type.is_html() {
            element_name.text().eq_ignore_ascii_case("label")
        } else {
            element_name.text() == "label"
        };

        if !is_label {
            return None;
        }

        let is_html = source_type.is_html();

        let has_text_content = element_has_accessible_label(node);
        let has_control_association =
            element_has_for_attribute(node) || element_has_nested_control(node, is_html);

        if has_text_content && has_control_association {
            return None;
        }

        Some(NoLabelWithoutControlState {
            has_text_content,
            has_control_association,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "A form label must be associated with a control."
            },
        );

        if !state.has_text_content {
            diagnostic = diagnostic.note(
                markup! { "Consider adding accessible text content to the label element." },
            );
        }

        if !state.has_control_association {
            diagnostic = diagnostic.note(
                markup! { "Consider adding a `for` attribute to the label element or moving the control element inside the label." },
            );
        }

        Some(diagnostic)
    }
}

/// Returns `true` if the element has an accessible label: either via an aria-label/
/// aria-labelledby attribute or via non-empty text content.
fn element_has_accessible_label(element: &AnyHtmlElement) -> bool {
    // Check aria-label / aria-labelledby / title on the element itself
    if has_accessible_name(element) {
        return true;
    }

    // For self-closing labels there can be no child content
    let html_element = match element.as_html_element() {
        Some(el) => el,
        None => return false,
    };

    if html_element.opening_element().is_err() {
        return false;
    }

    has_accessible_text_in_children(&html_element.children())
}

/// Returns `true` if the children contain non-empty accessible text content.
fn has_accessible_text_in_children(children: &HtmlElementList) -> bool {
    children.into_iter().any(|child| match &child {
        AnyHtmlElement::AnyHtmlContent(content) => is_non_empty_text(content),
        AnyHtmlElement::HtmlElement(element) => {
            if html_element_has_truthy_aria_hidden(element) {
                return false;
            }
            has_accessible_text_in_children(&element.children())
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            // A self-closing img with a non-empty alt attribute contributes accessible text
            if let Some(tag_name) = element.tag_name() {
                if tag_name.text().eq_ignore_ascii_case("img") {
                    return html_self_closing_element_has_non_empty_attribute(element, "alt");
                }
            }
            // Other self-closing elements with an accessible name (aria-label, etc.)
            html_self_closing_element_has_accessible_name(element)
        }
        AnyHtmlElement::HtmlBogusElement(_) | AnyHtmlElement::HtmlCdataSection(_) => false,
    })
}

/// Returns `true` if the content node contains non-empty, non-whitespace text.
fn is_non_empty_text(content: &AnyHtmlContent) -> bool {
    match content {
        AnyHtmlContent::HtmlContent(html_content) => html_content
            .value_token()
            .is_ok_and(|token| !token.text_trimmed().is_empty()),
        // Template expressions ({{ var }}, {expr}) are treated as accessible
        AnyHtmlContent::AnyHtmlTextExpression(_) => true,
        // Embedded script/style blocks are not text content
        AnyHtmlContent::HtmlEmbeddedContent(_) => false,
    }
}

/// Returns `true` if the element has a `for` attribute (HTML only, not `htmlFor`).
fn element_has_for_attribute(element: &AnyHtmlElement) -> bool {
    element.find_attribute_by_name("for").is_some()
}

/// Returns `true` if the element has a nested control element
/// (`input`, `meter`, `output`, `progress`, `select`, `textarea`, `button`).
fn element_has_nested_control(element: &AnyHtmlElement, is_html: bool) -> bool {
    let html_element = match element.as_html_element() {
        Some(el) => el,
        None => return false,
    };

    if html_element.opening_element().is_err() {
        return false;
    }

    children_have_control(&html_element.children(), is_html)
}

const CONTROL_ELEMENTS: [&str; 7] = [
    "input", "meter", "output", "progress", "select", "textarea", "button",
];

/// Recursively checks whether the children contain a control element.
fn children_have_control(children: &HtmlElementList, is_html: bool) -> bool {
    children.into_iter().any(|child| match &child {
        AnyHtmlElement::HtmlElement(element) => {
            if let Some(tag_name) = element.tag_name() {
                let is_control = if is_html {
                    CONTROL_ELEMENTS
                        .iter()
                        .any(|&c| tag_name.text().eq_ignore_ascii_case(c))
                } else {
                    CONTROL_ELEMENTS.contains(&tag_name.text())
                };
                if is_control {
                    return true;
                }
            }
            children_have_control(&element.children(), is_html)
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            let tag_name = match element.tag_name() {
                Some(name) => name,
                None => return false,
            };
            if is_html {
                CONTROL_ELEMENTS
                    .iter()
                    .any(|&c| tag_name.text().eq_ignore_ascii_case(c))
            } else {
                CONTROL_ELEMENTS.contains(&tag_name.text())
            }
        }
        _ => false,
    })
}

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_aria_hidden_on_focusable::NoAriaHiddenOnFocusableOptions;

use crate::HtmlRuleAction;
use crate::a11y::get_truthy_aria_hidden_attribute;

declare_lint_rule! {
    /// Enforce that aria-hidden="true" is not set on focusable elements.
    ///
    /// `aria-hidden="true"` can be used to hide purely decorative content from screen reader users.
    /// A focusable element with `aria-hidden="true"` can be reached by keyboard.
    /// This can lead to confusion or unexpected behavior for screen reader users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div aria-hidden="true" tabindex="0"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a href="/" aria-hidden="true">link</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div aria-hidden="true"></div>
    /// ```
    ///
    /// ```html
    /// <button aria-hidden="true" tabindex="-1"></button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [aria-hidden elements do not contain focusable elements](https://dequeuniversity.com/rules/axe/html/4.4/aria-hidden-focus)
    /// - [Element with aria-hidden has no content in sequential focus navigation](https://www.w3.org/WAI/standards-guidelines/act/rules/6cfa84/proposed/)
    /// - [MDN aria-hidden](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)
    ///
    pub NoAriaHiddenOnFocusable {
        version: "next",
        name: "noAriaHiddenOnFocusable",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-aria-hidden-on-focusable").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct NoAriaHiddenOnFocusableState {
    aria_hidden_attribute: HtmlAttribute,
}

impl Rule for NoAriaHiddenOnFocusable {
    type Query = Ast<AnyHtmlElement>;
    type State = NoAriaHiddenOnFocusableState;
    type Signals = Option<Self::State>;
    type Options = NoAriaHiddenOnFocusableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let aria_hidden_attr = get_truthy_aria_hidden_attribute(element)?;
        let file_source = ctx.source_type::<HtmlFileSource>();
        let is_html = file_source.is_html();

        // Tabindex overrides native focusability: negative removes from tab order,
        // non-negative makes the element focusable regardless of element type.
        if let Some(tabindex_attr) = element.find_attribute_by_name("tabindex")
            && let Some(tabindex_value) = get_tabindex_value(&tabindex_attr)
        {
            if tabindex_value < 0 {
                return None;
            }
            return Some(NoAriaHiddenOnFocusableState {
                aria_hidden_attribute: aria_hidden_attr,
            });
        }

        // Check if element is natively focusable or has contenteditable
        if is_focusable_element(element, is_html)? {
            return Some(NoAriaHiddenOnFocusableState {
                aria_hidden_attribute: aria_hidden_attr,
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Incorrect use of "<Emphasis>"aria-hidden=\"true\""</Emphasis>" detected."
                },
            )
            .note(markup! {
                ""<Emphasis>"aria-hidden"</Emphasis>" should not be set to "<Emphasis>"true"</Emphasis>" on focusable elements because this can lead to confusing behavior for screen reader users."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.aria_hidden_attribute.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"aria-hidden"</Emphasis>" attribute from the element." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Parses the tabindex attribute value as an integer.
///
/// Returns `None` if the attribute has no value or cannot be parsed as an integer.
/// Non-integer values (e.g., `tabindex="abc"`) are ignored and treated as if
/// tabindex was not set.
fn get_tabindex_value(attribute: &HtmlAttribute) -> Option<i32> {
    let value = attribute.value()?;
    value.trim().parse::<i32>().ok()
}

/// Returns whether the element is natively focusable per the HTML spec.
///
/// Returns `Some(true)` when the element is one of:
/// - Interactive elements: `<button>`, `<select>`, `<textarea>`, `<details>`, `<summary>`
/// - Elements with `href` attribute: `<a href="...">`, `<area href="...">`
/// - `<input>` elements, except `<input type="hidden">` which is not focusable
/// - Elements with a truthy `contenteditable` attribute (editing hosts)
///
/// Returns `Some(false)` when the element is recognized but not focusable.
/// Returns `None` when the element name cannot be determined (e.g., bogus elements).
fn is_focusable_element(element: &AnyHtmlElement, is_html: bool) -> Option<bool> {
    let element_name = element.name()?;

    let name_matches = |name: &str| -> bool {
        if is_html {
            element_name.eq_ignore_ascii_case(name)
        } else {
            element_name.text() == name
        }
    };

    // <a> and <area> are only focusable when they have an href attribute
    if (name_matches("a") || name_matches("area"))
        && element.find_attribute_by_name("href").is_some()
    {
        return Some(true);
    }

    // These elements are always natively focusable
    if name_matches("button")
        || name_matches("select")
        || name_matches("textarea")
        || name_matches("details")
        || name_matches("summary")
    {
        return Some(true);
    }

    // <input> is focusable unless type="hidden"
    if name_matches("input") {
        let is_hidden = element
            .find_attribute_by_name("type")
            .and_then(|attr| attr.value())
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("hidden"));
        return Some(!is_hidden);
    }

    // Check contenteditable attribute
    Some(has_contenteditable_true(element))
}

/// Returns `true` when the element has a truthy `contenteditable` attribute,
/// making it an editing host (and therefore focusable).
///
/// Per the HTML spec (§6.8.1), `contenteditable` is an enumerated attribute with:
/// - Bare attribute (`<div contenteditable>`) → empty value default = **True** state
/// - `""` (empty string) → empty value default = **True** state
/// - `"true"` → **True** state (editing host)
/// - `"plaintext-only"` → **Plaintext-Only** state (editing host)
/// - `"false"` → **False** state (not editable)
/// - Invalid values (e.g., `"banana"`) → **Inherit** state (not an editing host)
///
/// Ref: <https://html.spec.whatwg.org/multipage/interaction.html#attr-contenteditable>
fn has_contenteditable_true(element: &AnyHtmlElement) -> bool {
    element
        .find_attribute_by_name("contenteditable")
        .is_some_and(|attr| match attr.value() {
            None => true, // bare attribute = True state per HTML spec
            Some(value) => {
                let trimmed = value.trim();
                trimmed.is_empty()
                    || trimmed.eq_ignore_ascii_case("true")
                    || trimmed.eq_ignore_ascii_case("plaintext-only")
            }
        })
}

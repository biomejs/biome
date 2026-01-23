use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};
use biome_rowan::Text;

// ============================================================================
// Core attribute value helpers (shared logic extracted here)
// ============================================================================

/// Extracts the string value from an attribute's initializer.
/// This is the fundamental building block for all attribute value checks.
fn get_attribute_string_value(attribute: &HtmlAttribute) -> Option<Text> {
    attribute
        .initializer()
        .and_then(|init| init.value().ok())
        .and_then(|value| value.string_value())
}

/// Checks if an attribute has a truthy `aria-hidden` value.
/// Returns `true` if value is not "false" (case-insensitive) or if no value is provided.
fn is_truthy_aria_hidden_value(attribute: &HtmlAttribute) -> bool {
    get_attribute_string_value(attribute).is_none_or(|value| !value.eq_ignore_ascii_case("false"))
}

/// Checks if an attribute value equals "true" exactly (case-sensitive).
fn is_strict_true_value(attribute: &HtmlAttribute) -> bool {
    get_attribute_string_value(attribute).is_some_and(|value| value == "true")
}

/// Checks if an attribute has a non-empty trimmed value.
fn has_non_empty_value(attribute: &HtmlAttribute) -> bool {
    get_attribute_string_value(attribute).is_some_and(|value| !value.trim().is_empty())
}

/// Checks if an attribute value matches a specific string (case-insensitive).
pub(crate) fn attribute_value_equals_ignore_case(
    attribute: &HtmlAttribute,
    expected: &str,
) -> bool {
    get_attribute_string_value(attribute).is_some_and(|value| value.eq_ignore_ascii_case(expected))
}

// ============================================================================
// Element-level helpers (use core helpers above)
// ============================================================================

/// Check the element is hidden from screen reader.
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
pub(crate) fn is_hidden_from_screen_reader(element: &AnyHtmlTagElement) -> bool {
    if element.has_truthy_attribute("aria-hidden") {
        return true;
    }

    match element.name_value_token().ok() {
        Some(name) if name.text_trimmed() == "input" => element
            .find_attribute_by_name("type")
            .is_some_and(|attr| attribute_value_equals_ignore_case(&attr, "hidden")),
        _ => false,
    }
}

/// Strict check: returns `true` only when `aria-hidden="true"` (case-sensitive).
///
/// Ref: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
pub(crate) fn is_aria_hidden_true(element: &AnyHtmlElement) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_strict_true_value(&attr))
}

/// Returns the `aria-hidden` attribute if it has a truthy value.
///
/// Returns `Some` if present and not "false" (case-insensitive), `None` otherwise.
/// Useful for code fixes that need to reference the attribute.
///
/// Ref: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
pub(crate) fn get_truthy_aria_hidden_attribute(element: &AnyHtmlElement) -> Option<HtmlAttribute> {
    let attribute = element.find_attribute_by_name("aria-hidden")?;
    if is_truthy_aria_hidden_value(&attribute) {
        Some(attribute)
    } else {
        None
    }
}

/// Returns `true` if attribute exists with non-empty trimmed value.
pub(crate) fn has_non_empty_attribute(element: &AnyHtmlElement, name: &str) -> bool {
    element
        .find_attribute_by_name(name)
        .is_some_and(|attr| has_non_empty_value(&attr))
}

/// Returns `true` if element has `aria-label`, `aria-labelledby`, or `title` with non-empty value.
pub(crate) fn has_accessible_name(element: &AnyHtmlElement) -> bool {
    has_non_empty_attribute(element, "aria-label")
        || has_non_empty_attribute(element, "aria-labelledby")
        || has_non_empty_attribute(element, "title")
}

// ============================================================================
// Type-specific variants (avoid wrapping/cloning in recursive code)
// ============================================================================

/// Type-specific variant for `HtmlElement`. Checks truthy `aria-hidden`.
pub(crate) fn html_element_has_truthy_aria_hidden(
    element: &biome_html_syntax::HtmlElement,
) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_truthy_aria_hidden_value(&attr))
}

/// Type-specific variant for `HtmlSelfClosingElement`. Checks truthy `aria-hidden`.
pub(crate) fn html_self_closing_element_has_truthy_aria_hidden(
    element: &biome_html_syntax::HtmlSelfClosingElement,
) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_truthy_aria_hidden_value(&attr))
}

/// Type-specific variant for `HtmlSelfClosingElement`. Checks accessible name.
pub(crate) fn html_self_closing_element_has_accessible_name(
    element: &biome_html_syntax::HtmlSelfClosingElement,
) -> bool {
    let has_aria_label = element
        .find_attribute_by_name("aria-label")
        .is_some_and(|attr| has_non_empty_value(&attr));
    let has_aria_labelledby = element
        .find_attribute_by_name("aria-labelledby")
        .is_some_and(|attr| has_non_empty_value(&attr));
    let has_title = element
        .find_attribute_by_name("title")
        .is_some_and(|attr| has_non_empty_value(&attr));
    has_aria_label || has_aria_labelledby || has_title
}

/// Type-specific variant for `HtmlSelfClosingElement`. Checks non-empty attribute.
pub(crate) fn html_self_closing_element_has_non_empty_attribute(
    element: &biome_html_syntax::HtmlSelfClosingElement,
    name: &str,
) -> bool {
    element
        .find_attribute_by_name(name)
        .is_some_and(|attr| has_non_empty_value(&attr))
}

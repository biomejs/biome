//! Shared accessibility helper functions for HTML lint rules.
//!
//! This module provides reusable utilities for checking accessibility-related
//! attributes and element states. All helpers follow a layered architecture:
//!
//! 1. **Core helpers** (private): Low-level attribute value extraction and checks
//! 2. **Element helpers** (public): Higher-level checks on HTML elements
//! 3. **Type-specific variants** (public): Optimized versions that avoid cloning

use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};

// ============================================================================
// Core attribute value helpers (private)
// ============================================================================

/// Checks if an `aria-hidden` attribute has a truthy value.
///
/// Per ARIA spec, `aria-hidden` accepts only `"true"` or `"false"` as valid values.
/// Returns `true` only for non-empty values that are not `"false"` (case-insensitive).
/// Missing values, empty strings, and whitespace-only values are considered falsy.
///
/// Ref: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-hidden>
fn is_aria_hidden_value_truthy(attribute: &HtmlAttribute) -> bool {
    attribute.value().is_some_and(|value| {
        let trimmed = value.trim();
        !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("false")
    })
}

/// Checks if an attribute value equals `"true"` exactly (case-sensitive).
fn is_strict_true_value(attribute: &HtmlAttribute) -> bool {
    attribute.value().is_some_and(|value| value == "true")
}

/// Checks if an attribute has a non-empty value after trimming whitespace.
///
/// Returns `false` for attributes with no value, empty strings, or whitespace-only values.
fn has_non_empty_value(attribute: &HtmlAttribute) -> bool {
    attribute
        .value()
        .is_some_and(|value| !value.trim().is_empty())
}

/// Checks if an attribute value matches the expected string (case-insensitive).
///
/// Useful for checking HTML attribute values like `type="hidden"` or `role="button"`
/// where the comparison should be case-insensitive per HTML spec.
pub(crate) fn attribute_value_equals_ignore_case(
    attribute: &HtmlAttribute,
    expected: &str,
) -> bool {
    attribute
        .value()
        .is_some_and(|value| value.eq_ignore_ascii_case(expected))
}

// ============================================================================
// Element-level helpers (public)
// ============================================================================

/// Returns `true` if the element is hidden from assistive technologies.
///
/// An element is hidden from screen readers when:
/// - It has a truthy `aria-hidden` attribute
/// - It is an `<input type="hidden">` element
///
/// Ref:
/// - <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
/// - <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden>
/// - <https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js>
pub(crate) fn is_hidden_from_screen_reader(element: &AnyHtmlTagElement) -> bool {
    if element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_aria_hidden_value_truthy(&attr))
    {
        return true;
    }

    match element.name_value_token() {
        Some(name) if name.text_trimmed() == "input" => element
            .find_attribute_by_name("type")
            .is_some_and(|attr| attribute_value_equals_ignore_case(&attr, "hidden")),
        _ => false,
    }
}

/// Strict check: returns `true` only when `aria-hidden="true"` (case-sensitive).
///
/// Unlike [`is_aria_hidden_value_truthy`], this only matches the exact string `"true"`.
///
/// Ref: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
pub(crate) fn is_aria_hidden_true(element: &AnyHtmlElement) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_strict_true_value(&attr))
}

/// Returns the `aria-hidden` attribute if it has a truthy value.
///
/// Returns `Some` if present and not `"false"` (case-insensitive), `None` otherwise.
/// Useful for code fixes that need to reference the attribute node.
///
/// Ref: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
pub(crate) fn get_truthy_aria_hidden_attribute(element: &AnyHtmlElement) -> Option<HtmlAttribute> {
    let attribute = element.find_attribute_by_name("aria-hidden")?;
    if is_aria_hidden_value_truthy(&attribute) {
        Some(attribute)
    } else {
        None
    }
}

/// Returns `true` if the element has the named attribute with a non-empty value.
///
/// Whitespace-only values are considered empty.
pub(crate) fn has_non_empty_attribute(element: &AnyHtmlElement, name: &str) -> bool {
    element
        .find_attribute_by_name(name)
        .is_some_and(|attr| has_non_empty_value(&attr))
}

/// Returns `true` if the element has an accessible name via `aria-label`,
/// `aria-labelledby`, or `title` attributes.
pub(crate) fn has_accessible_name(element: &AnyHtmlElement) -> bool {
    has_non_empty_attribute(element, "aria-label")
        || has_non_empty_attribute(element, "aria-labelledby")
        || has_non_empty_attribute(element, "title")
}

/// Checks if an [`HtmlElement`] has a truthy `aria-hidden` attribute.
///
/// [`HtmlElement`]: biome_html_syntax::HtmlElement
pub(crate) fn html_element_has_truthy_aria_hidden(
    element: &biome_html_syntax::HtmlElement,
) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_aria_hidden_value_truthy(&attr))
}

/// Checks if an [`HtmlSelfClosingElement`] has a truthy `aria-hidden` attribute.
///
/// [`HtmlSelfClosingElement`]: biome_html_syntax::HtmlSelfClosingElement
pub(crate) fn html_self_closing_element_has_truthy_aria_hidden(
    element: &biome_html_syntax::HtmlSelfClosingElement,
) -> bool {
    element
        .find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| is_aria_hidden_value_truthy(&attr))
}

/// Checks if an [`HtmlSelfClosingElement`] has an accessible name via `aria-label`,
/// `aria-labelledby`, or `title` attributes.
///
/// [`HtmlSelfClosingElement`]: biome_html_syntax::HtmlSelfClosingElement
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

/// Checks if an [`HtmlSelfClosingElement`] has the named attribute with a non-empty value.
///
/// [`HtmlSelfClosingElement`]: biome_html_syntax::HtmlSelfClosingElement
pub(crate) fn html_self_closing_element_has_non_empty_attribute(
    element: &biome_html_syntax::HtmlSelfClosingElement,
    name: &str,
) -> bool {
    element
        .find_attribute_by_name(name)
        .is_some_and(|attr| has_non_empty_value(&attr))
}

#[cfg(test)]
#[path = "a11y_tests.rs"]
mod tests;

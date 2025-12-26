use biome_html_syntax::element_ext::AnyHtmlTagElement;

/// Check the element is hidden from screen reader.
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
pub(crate) fn is_hidden_from_screen_reader(element: &AnyHtmlTagElement) -> bool {
    let is_aria_hidden = element.has_truthy_attribute("aria-hidden");
    if is_aria_hidden {
        return true;
    }

    match element.name_value_token().ok() {
        Some(name) if name.text_trimmed() == "input" => element
            .find_attribute_by_name("type")
            .and_then(|attribute| attribute.initializer()?.value().ok()?.string_value())
            .is_some_and(|value| value.text() == "hidden"),
        _ => false,
    }
}

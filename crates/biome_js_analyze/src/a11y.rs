use biome_js_syntax::jsx_ext::AnyJsxElement;

/// Check the element is hidden from screen reader.
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
pub(crate) fn is_hidden_from_screen_reader(element: &AnyJsxElement) -> bool {
    let is_aria_hidden = element.has_truthy_attribute("aria-hidden");
    if is_aria_hidden {
        return true;
    }

    match element.name_value_token().ok() {
        Some(name) if name.text_trimmed() == "input" => element
            .find_attribute_by_name("type")
            .and_then(|attribute| attribute.as_static_value())
            .and_then(|value| value.as_string_constant().map(|value| value == "hidden"))
            .unwrap_or_default(),
        _ => false,
    }
}

/// Check if the element is `contentEditable`
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isContentEditable.js
pub(crate) fn is_content_editable(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("contentEditable")
        .and_then(|attribute| attribute.as_static_value())
        .and_then(|value| value.as_string_constant().map(|value| value == "true"))
        .unwrap_or_default()
}

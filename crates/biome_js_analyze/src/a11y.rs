use biome_aria::event_handlers::matches_event_handler;
use biome_js_syntax::{AnyJsxAttributeName, jsx_ext::AnyJsxElement};
use biome_rowan::AstNodeList;

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

/// Check if the element contains event handler
pub fn has_event_handler(handler_types: &[&str], element: &AnyJsxElement) -> bool {
    element.attributes().iter().any(|attribute| {
        if let Some(jsx_attribute) = attribute.as_jsx_attribute()
            && let Ok(AnyJsxAttributeName::JsxName(name)) = jsx_attribute.name()
            && let Ok(value_token) = name.value_token()
            && matches_event_handler(handler_types, value_token.text_trimmed())
        {
            return !jsx_attribute.is_value_null_or_undefined();
        }

        false
    })
}

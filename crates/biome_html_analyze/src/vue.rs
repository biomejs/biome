//! Shared helpers for Vue lint rules.

use biome_html_syntax::{AnyHtmlTagName, HtmlSyntaxKind, element_ext::AnyHtmlTagElement};
use biome_parser::{TokenSet, token_set};

/// Returns `true` if `name` is a known HTML, SVG, or MathML element, which Vue renders as a
/// native element instead of resolving it as a component.
///
/// Names such as `<MyComponent>`, `<my-element>`, and `<component>` are not native.
pub(crate) fn is_native_element_name(name: &AnyHtmlTagName) -> bool {
    name.tag_name_kind()
        .is_some_and(|kind| !COMPONENT_TAG_NAMES.contains(kind))
}

/// Tag names that Vue renders as components even though the lexer gives them a plain tag name.
const COMPONENT_TAG_NAMES: TokenSet<HtmlSyntaxKind> = token_set!(
    HtmlSyntaxKind::COMPONENT_KW,
    HtmlSyntaxKind::HTML_UNKNOWN_TAG
);

/// Returns `true` if Vue renders `element` as a component instead of a native element.
///
/// An element is a component if its name isn't a known HTML, SVG, or MathML element, such as
/// `<MyComponent>`, `<my-element>`, or `<component>`, or if it has an `is` attribute whose
/// value starts with `vue:`, such as `<tr is="vue:my-row">`.
pub(crate) fn is_component(element: &AnyHtmlTagElement) -> bool {
    let Ok(name) = element.name() else {
        return false;
    };
    !is_native_element_name(&name) || has_vue_is_attribute(element)
}

fn has_vue_is_attribute(element: &AnyHtmlTagElement) -> bool {
    element
        .find_attribute_by_name("is")
        .and_then(|attribute| {
            attribute
                .as_html_attribute()?
                .initializer()?
                .value()
                .ok()?
                .as_static_value()
        })
        .is_some_and(|value| value.text().starts_with("vue:"))
}

//! Shared helpers for Vue lint rules.

use biome_html_syntax::{AnyHtmlTagName, HtmlSyntaxKind, element_ext::AnyHtmlTagElement};
use biome_parser::{TokenSet, token_set};

/// Returns `true` if `name` is a known HTML, SVG, or MathML element, which Vue renders as a
/// native element instead of resolving it as a component.
///
/// Names such as `<MyComponent>`, `<my-element>`, and `<component>` are not native.
pub(crate) fn is_native_element_name(name: &AnyHtmlTagName) -> bool {
    match name.tag_name_kind() {
        Some(kind) if !COMPONENT_TAG_NAMES.contains(kind) => true,
        _ => name
            .token_text_trimmed()
            .is_some_and(|name| MATHML_ELEMENTS.binary_search(&name.text()).is_ok()),
    }
}

/// Tag names that Vue renders as components even though the lexer gives them a plain tag name.
const COMPONENT_TAG_NAMES: TokenSet<HtmlSyntaxKind> = token_set!(
    HtmlSyntaxKind::COMPONENT_KW,
    HtmlSyntaxKind::HTML_UNKNOWN_TAG
);

/// MathML element names that Vue renders as native elements. They have no tag-name keyword,
/// so they lex as `HTML_UNKNOWN_TAG` (or as a component name, for `annotation-xml`).
const MATHML_ELEMENTS: &[&str] = &[
    "annotation",
    "annotation-xml",
    "maction",
    "maligngroup",
    "malignmark",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mfraction",
    "mglyph",
    "mi",
    "mlabeledtr",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mscarries",
    "mscarry",
    "msgroup",
    "msline",
    "mspace",
    "msqrt",
    "msrow",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "none",
    "semantics",
];

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

#[cfg(test)]
mod tests {
    use super::MATHML_ELEMENTS;

    #[test]
    fn mathml_elements_are_sorted() {
        for pair in MATHML_ELEMENTS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "`{}` should come after `{}`",
                pair[0],
                pair[1]
            );
        }
    }
}

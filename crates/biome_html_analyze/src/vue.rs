//! Helpers shared by the lint rules for Vue templates.

use biome_html_syntax::{AnyHtmlTagName, HtmlSyntaxKind};
use biome_parser::{TokenSet, token_set};

/// Returns `true` if Vue renders an element with this tag name as a native HTML, SVG, or MathML
/// element instead of resolving it as a component.
///
/// `<div>`, `<svg>`, and `<mi>` are native. `<MyComponent>`, `<my-element>`, `<foo>`, and
/// `<component>` aren't.
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

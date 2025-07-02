use crate::HtmlSelfClosingElement;
use biome_rowan::SyntaxResult;

/// https://html.spec.whatwg.org/#void-elements
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

impl HtmlSelfClosingElement {
    /// Whether the current self-closing element is a void element.
    ///
    /// <https://html.spec.whatwg.org/#void-elements>
    pub fn is_void_element(&self) -> SyntaxResult<bool> {
        let name = self.name()?;
        Ok(VOID_ELEMENTS
            .binary_search(&name.value_token()?.text_trimmed())
            .is_ok())
    }
}

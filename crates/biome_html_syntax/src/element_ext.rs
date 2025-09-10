use crate::{AnyHtmlElement, HtmlElement, HtmlSelfClosingElement};
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

impl AnyHtmlElement {
    pub fn is_script_tag(&self) -> SyntaxResult<bool> {
        match self {
            Self::AnyHtmlContent(_)
            | Self::HtmlBogusElement(_)
            | Self::HtmlSelfClosingElement(_)
            | Self::HtmlCdataSection(_) => Ok(false),
            Self::HtmlElement(element) => element.is_script_tag(),
        }
    }

    pub fn is_style_tag(&self) -> SyntaxResult<bool> {
        match self {
            Self::AnyHtmlContent(_)
            | Self::HtmlBogusElement(_)
            | Self::HtmlSelfClosingElement(_)
            | Self::HtmlCdataSection(_) => Ok(false),
            Self::HtmlElement(element) => element.is_style_tag(),
        }
    }
}

impl HtmlElement {
    pub fn is_script_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;

        Ok(name_text.text_trimmed() == "script")
    }

    pub fn is_style_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;
        Ok(name_text.text_trimmed() == "style")
    }
}

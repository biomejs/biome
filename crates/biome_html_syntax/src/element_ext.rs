use crate::{AnyHtmlElement, HtmlElement, HtmlSelfClosingElement};
use biome_rowan::{AstNodeList, SyntaxResult};

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
    pub fn is_javascript_tag(&self) -> SyntaxResult<bool> {
        match self {
            Self::AnyHtmlContent(_)
            | Self::HtmlBogusElement(_)
            | Self::HtmlSelfClosingElement(_)
            | Self::HtmlCdataSection(_) => Ok(false),
            Self::HtmlElement(element) => element.is_javascript_tag(),
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
    pub fn is_javascript_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;
        let is_script = name_text.text_trimmed().eq_ignore_ascii_case("script");

        let type_attribute = opening_element.attributes().iter().find_map(|attribute| {
            let name = attribute
                .as_html_attribute()
                .and_then(|attribute| attribute.name().ok())?;

            let is_type = name
                .value_token()
                .map(|token| token.text_trimmed().eq_ignore_ascii_case("type"))
                .unwrap_or_default();

            if is_type { Some(name) } else { None }
        });

        let is_type_javascript = type_attribute.is_none_or(|attribute| {
            attribute
                .value_token()
                .map(|token| token.text_trimmed().eq_ignore_ascii_case("text/javascript"))
                .unwrap_or_default()
        });

        Ok(is_script && is_type_javascript)
    }

    pub fn is_style_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;
        Ok(name_text.text_trimmed().eq_ignore_ascii_case("style"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_javascript_tag() {}
}

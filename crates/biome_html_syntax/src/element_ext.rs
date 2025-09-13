use crate::{
    AnyHtmlElement, HtmlAttribute, HtmlElement, HtmlSelfClosingElement, inner_string_text,
};
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

    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        match self {
            Self::HtmlElement(element) => element.find_attribute_by_name(name_to_lookup),
            Self::HtmlSelfClosingElement(element) => element.find_attribute_by_name(name_to_lookup),
            // Other variants don't have attributes
            Self::AnyHtmlContent(_) | Self::HtmlBogusElement(_) | Self::HtmlCdataSection(_) => None,
        }
    }
}

impl HtmlSelfClosingElement {
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        self.attributes().iter().find_map(|attr| {
            let attribute = attr.as_html_attribute()?;
            let name = attribute.name().ok()?;
            let name_token = name.value_token().ok()?;
            if name_token.text_trimmed() == name_to_lookup {
                Some(attribute.clone())
            } else {
                None
            }
        })
    }
}

impl HtmlElement {
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        self.opening_element()
            .ok()?
            .attributes()
            .iter()
            .find_map(|attr| {
                let attribute = attr.as_html_attribute()?;
                let name = attribute.name().ok()?;
                let name_token = name.value_token().ok()?;
                if name_token.text_trimmed() == name_to_lookup {
                    Some(attribute.clone())
                } else {
                    None
                }
            })
    }

    pub fn is_javascript_tag(&self) -> SyntaxResult<bool> {
        let is_script = self.is_script_tag()?;
        let type_attribute = self.find_attribute_by_name("type");

        let is_type_javascript = type_attribute.is_none_or(|attribute| {
            attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_string().cloned())
                .and_then(|value| value.value_token().ok())
                .is_some_and(|token| {
                    let text = inner_string_text(&token);
                    text.eq_ignore_ascii_case("text/javascript")
                        || text.eq_ignore_ascii_case("application/javascript")
                        || text.eq_ignore_ascii_case("application/ecmascript")
                        || text.eq_ignore_ascii_case("module")
                })
        });

        Ok(is_script && is_type_javascript)
    }

    pub fn is_javascript_module(&self) -> SyntaxResult<bool> {
        let is_script = self.is_script_tag()?;
        let type_attribute = self.find_attribute_by_name("type");
        let is_type_module = type_attribute.is_none_or(|attribute| {
            attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_string().cloned())
                .and_then(|value| value.value_token().ok())
                .is_some_and(|token| {
                    let text = inner_string_text(&token);
                    text.eq_ignore_ascii_case("module")
                })
        });

        Ok(is_script && is_type_module)
    }

    pub fn is_style_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;
        Ok(name_text.text_trimmed().eq_ignore_ascii_case("style"))
    }

    pub fn is_script_tag(&self) -> SyntaxResult<bool> {
        let opening_element = self.opening_element()?;
        let name = opening_element.name()?;
        let name_text = name.value_token()?;
        Ok(name_text.text_trimmed().eq_ignore_ascii_case("script"))
    }
}

#[cfg(test)]
mod tests {
    use biome_html_factory::syntax::HtmlElement;
    use biome_html_parser::{HtmlParseOptions, parse_html};
    use biome_rowan::AstNode;

    #[test]
    fn test_is_javascript_tag() {
        let html = r#"
        <script type="text/javascript">
        </script>
        "#;
        let syntax = parse_html(html, HtmlParseOptions::default());
        let element = syntax
            .tree()
            .syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .unwrap();

        assert!(element.is_javascript_tag().unwrap());

        let html = r#"
        <script type="application/javascript">
        </script>
        "#;
        let syntax = parse_html(html, HtmlParseOptions::default());
        let element = syntax
            .tree()
            .syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .unwrap();

        assert!(element.is_javascript_tag().unwrap());

        let html = r#"
        <script type="application/ecmascript">
        </script>
        "#;
        let syntax = parse_html(html, HtmlParseOptions::default());
        let element = syntax
            .tree()
            .syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .unwrap();

        assert!(element.is_javascript_tag().unwrap());

        let html = r#"
        <script type="module">
        </script>
        "#;
        let syntax = parse_html(html, HtmlParseOptions::default());
        let element = syntax
            .tree()
            .syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .unwrap();

        assert!(element.is_javascript_tag().unwrap());
    }
}

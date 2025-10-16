use crate::{
    AnyHtmlElement, AstroEmbeddedContent, HtmlAttribute, HtmlElement, HtmlEmbeddedContent,
    HtmlSelfClosingElement, HtmlSyntaxToken, ScriptType, inner_string_text,
};
use biome_rowan::{AstNodeList, SyntaxResult, TokenText, declare_node_union};

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
    pub fn is_javascript_tag(&self) -> bool {
        match self {
            Self::AnyHtmlContent(_)
            | Self::HtmlBogusElement(_)
            | Self::HtmlSelfClosingElement(_)
            | Self::HtmlCdataSection(_) => false,
            Self::HtmlElement(element) => element.is_javascript_tag(),
        }
    }

    pub fn is_style_tag(&self) -> bool {
        match self {
            Self::AnyHtmlContent(_)
            | Self::HtmlBogusElement(_)
            | Self::HtmlSelfClosingElement(_)
            | Self::HtmlCdataSection(_) => false,
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

    pub fn name(&self) -> Option<TokenText> {
        match self {
            Self::HtmlElement(el) => {
                let opening_element = el.opening_element().ok()?;
                let name = opening_element.name().ok()?;
                let name_token = name.value_token().ok()?;
                Some(name_token.token_text_trimmed())
            }
            Self::HtmlSelfClosingElement(el) => {
                let name = el.name().ok()?;
                let name_token = name.value_token().ok()?;
                Some(name_token.token_text_trimmed())
            }
            _ => None,
        }
    }
}

impl HtmlSelfClosingElement {
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        self.attributes().iter().find_map(|attr| {
            let attribute = attr.as_html_attribute()?;
            let name = attribute.name().ok()?;
            let name_token = name.value_token().ok()?;
            if name_token
                .text_trimmed()
                .eq_ignore_ascii_case(name_to_lookup)
            {
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
                if name_token
                    .text_trimmed()
                    .eq_ignore_ascii_case(name_to_lookup)
                {
                    Some(attribute.clone())
                } else {
                    None
                }
            })
    }

    pub fn is_javascript_tag(&self) -> bool {
        self.get_script_type()
            .is_some_and(ScriptType::is_javascript)
    }

    pub fn is_supported_script_tag(&self) -> bool {
        self.get_script_type().is_some_and(ScriptType::is_supported)
    }

    /// Returns the type of script for a `<script>` tag.
    ///
    /// Returns `Some` [`ScriptType`] if this is a `<script>` tag, even if it
    /// has no `type` attribute, because the omission of the `type` attribute
    /// implies [`ScriptType::Classic`].
    ///
    /// Returns `None` for non-`<script>` tags.
    pub fn get_script_type(&self) -> Option<ScriptType> {
        if !self.is_script_tag() {
            return None;
        }

        let script_type = self
            .find_attribute_by_name("type")
            .and_then(|attribute| {
                let initializer = attribute.initializer()?;
                let value = initializer.value().ok()?.string_value()?;
                Some(ScriptType::from_type_value(&value))
            })
            .unwrap_or_default();
        Some(script_type)
    }

    pub fn is_style_tag(&self) -> bool {
        let Ok(name_token) = self
            .opening_element()
            .and_then(|el| el.name())
            .and_then(|name| name.value_token())
        else {
            return false;
        };

        name_token.text_trimmed().eq_ignore_ascii_case("style")
    }

    pub fn is_script_tag(&self) -> bool {
        let Ok(name_token) = self
            .opening_element()
            .and_then(|el| el.name())
            .and_then(|name| name.value_token())
        else {
            return false;
        };

        name_token.text_trimmed().eq_ignore_ascii_case("script")
    }

    /// Returns `true` if the element is a `<script type="module">`
    pub fn is_javascript_module(&self) -> SyntaxResult<bool> {
        let is_script = self.is_script_tag();
        let type_attribute = self.find_attribute_by_name("type");
        let is_type_module = type_attribute.is_some_and(|attribute| {
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

    /// Returns `true` if the element is a `<script lang="ts">`
    pub fn is_typescript_lang(&self) -> bool {
        let is_script = self.is_script_tag();
        let lang_attribute = self.find_attribute_by_name("lang");
        let is_lang_typescript = lang_attribute.is_some_and(|attribute| {
            attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_string().cloned())
                .and_then(|value| value.value_token().ok())
                .is_some_and(|token| {
                    let text = inner_string_text(&token);
                    text.eq_ignore_ascii_case("ts")
                })
        });
        is_script && is_lang_typescript
    }

    /// Returns `true` if the element is a `<style lang="sass">` or `<style lang="scss">`
    pub fn is_sass_lang(&self) -> bool {
        let is_style = self.is_style_tag();
        let lang_attribute = self.find_attribute_by_name("lang");
        let is_lang_typescript = lang_attribute.is_some_and(|attribute| {
            attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_string().cloned())
                .and_then(|value| value.value_token().ok())
                .is_some_and(|token| {
                    let text = inner_string_text(&token);
                    text.eq_ignore_ascii_case("sass") || text.eq_ignore_ascii_case("scss")
                })
        });
        is_style && is_lang_typescript
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

        assert!(element.is_javascript_tag());

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

        assert!(element.is_javascript_tag());

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

        assert!(element.is_javascript_tag());

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

        assert!(element.is_javascript_tag());
    }
}

declare_node_union! {
    pub AnyEmbeddedContent = HtmlEmbeddedContent | AstroEmbeddedContent
}

impl AnyEmbeddedContent {
    pub fn value_token(&self) -> Option<HtmlSyntaxToken> {
        match self {
            Self::HtmlEmbeddedContent(node) => node.value_token().ok(),
            Self::AstroEmbeddedContent(node) => node.content_token(),
        }
    }
}

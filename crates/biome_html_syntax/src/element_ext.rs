use crate::{
    AnyHtmlContent, AnyHtmlElement, AnyHtmlTagName, AnyHtmlTextExpression, AnySvelteBlock,
    AstroEmbeddedContent, HtmlAttribute, HtmlAttributeList, HtmlElement, HtmlEmbeddedContent,
    HtmlOpeningElement, HtmlSelfClosingElement, HtmlSyntaxToken, HtmlTagName, ScriptType,
    inner_string_text,
};
use biome_rowan::{AstNodeList, SyntaxResult, TokenText, declare_node_union};
use biome_string_case::StrOnlyExtension;

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
            .binary_search(&name.name_value_token()?.text_trimmed())
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
            Self::HtmlElement(el) => el.tag_name(),
            Self::HtmlSelfClosingElement(el) => el.tag_name(),
            _ => None,
        }
    }

    /// Returns the closing `>` token from this element's closing tag, if it has one.
    ///
    /// This is used for "borrowing" the closing `>` when formatting adjacent inline elements
    /// to avoid introducing whitespace between them.
    ///
    /// Only returns a token for `HtmlElement` (which has actual closing tags like `</span>`).
    /// Self-closing elements like `<img />` don't have a separate closing tag to borrow from.
    pub fn closing_r_angle_token(&self) -> Option<HtmlSyntaxToken> {
        match self {
            Self::HtmlElement(el) => el.closing_element().ok()?.r_angle_token().ok(),
            // Self-closing elements don't have a closing tag to borrow from
            _ => None,
        }
    }

    pub fn is_svelte_block(&self) -> bool {
        matches!(
            self,
            Self::AnyHtmlContent(AnyHtmlContent::AnyHtmlTextExpression(
                AnyHtmlTextExpression::AnySvelteBlock(_)
            ))
        )
    }

    pub fn as_svelte_block(self) -> Option<AnySvelteBlock> {
        if let Self::AnyHtmlContent(AnyHtmlContent::AnyHtmlTextExpression(
            AnyHtmlTextExpression::AnySvelteBlock(block),
        )) = self
        {
            Some(block)
        } else {
            None
        }
    }
}

impl HtmlSelfClosingElement {
    /// Returns the tag name of the element (trimmed), if it has one.
    pub fn tag_name(&self) -> Option<TokenText> {
        let name = self.name().ok()?;
        let name_token = name.name_value_token().ok()?;
        Some(name_token.token_text_trimmed())
    }

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

impl HtmlOpeningElement {
    /// Returns the tag name of the element (trimmed), if it has one.
    pub fn tag_name(&self) -> Option<TokenText> {
        let name = self.name().ok()?;
        let name_token = name.name_value_token().ok()?;
        Some(name_token.token_text_trimmed())
    }

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
    /// Returns the tag name of the element (trimmed), if it has one.
    pub fn tag_name(&self) -> Option<TokenText> {
        let opening_element = self.opening_element().ok()?;
        opening_element.tag_name()
    }

    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        self.opening_element()
            .ok()?
            .find_attribute_by_name(name_to_lookup)
    }

    pub fn is_javascript_tag(&self) -> bool {
        self.get_script_type()
            .is_some_and(ScriptType::is_javascript)
    }

    pub fn is_supported_script_tag(&self) -> bool {
        self.get_script_type().is_some_and(ScriptType::is_supported)
    }

    /// It's a style tag, and it doesn't contain `scss` as `lang`
    pub fn is_supported_style_tag(&self) -> bool {
        self.is_style_tag() && !self.is_sass_lang()
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
            .and_then(|name| name.name_value_token())
        else {
            return false;
        };

        name_token.text_trimmed().eq_ignore_ascii_case("style")
    }

    pub fn is_script_tag(&self) -> bool {
        let Ok(name_token) = self
            .opening_element()
            .and_then(|el| el.name())
            .and_then(|name| name.name_value_token())
        else {
            return false;
        };

        name_token.text_trimmed().eq_ignore_ascii_case("script")
    }

    fn has_attribute(&self, name: &str, value: &str) -> bool {
        let attribute = self.find_attribute_by_name(name);
        attribute.is_some_and(|attribute| {
            attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_string().cloned())
                .and_then(|value| value.value_token().ok())
                .is_some_and(|token| {
                    let text = inner_string_text(&token);
                    text.eq_ignore_ascii_case(value)
                })
        })
    }

    /// Returns `true` if the element is a `<script type="module">`
    pub fn is_javascript_module(&self) -> SyntaxResult<bool> {
        Ok(self.is_script_tag() && self.has_attribute("type", "module"))
    }

    /// Returns `true` if the element is a `<script lang="ts">`
    pub fn is_typescript_lang(&self) -> bool {
        self.is_script_tag() && self.has_attribute("lang", "ts")
    }

    /// Returns `true` if the element is a `<script setup>` tag.
    pub fn is_script_with_setup_attribute(&self) -> bool {
        self.is_script_tag() && self.find_attribute_by_name("setup").is_some()
    }

    /// Returns `true` if the element is a `<script lang="jsx">`
    pub fn is_jsx_lang(&self) -> bool {
        self.is_script_tag() && self.has_attribute("lang", "jsx")
    }

    /// Returns `true` if the element is a `<script lang="tsx">`
    pub fn is_tsx_lang(&self) -> bool {
        self.is_script_tag() && self.has_attribute("lang", "tsx")
    }

    /// Returns `true` if the element is a `<style lang="sass">` or `<style lang="scss">`
    pub fn is_sass_lang(&self) -> bool {
        self.is_style_tag() && self.has_attribute("lang", "scss")
    }

    pub fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        self.opening_element()?.name()
    }
}

impl AnyHtmlTagName {
    /// Returns the token text of the attribute name.
    pub fn token_text(&self) -> Option<TokenText> {
        self.name_value_token().ok().map(|token| token.token_text())
    }

    /// Returns the value token.
    pub fn name_value_token(&self) -> SyntaxResult<HtmlSyntaxToken> {
        match self {
            Self::HtmlTagName(name) => name.value_token(),
            Self::HtmlReferenceIdentifier(name) => name.value_token(),
        }
    }

    /// Returns the trimmed token text of the attribute name.
    pub fn token_text_trimmed(&self) -> Option<TokenText> {
        self.name_value_token()
            .ok()
            .map(|token| token.token_text_trimmed())
    }
}

impl HtmlTagName {
    /// Returns the token text of the attribute name.
    pub fn token_text(&self) -> Option<TokenText> {
        self.value_token().ok().map(|token| token.token_text())
    }

    /// Returns the trimmed token text of the attribute name.
    pub fn token_text_trimmed(&self) -> Option<TokenText> {
        self.value_token()
            .ok()
            .map(|token| token.token_text_trimmed())
    }
}

declare_node_union! {
    pub AnyHtmlTagElement = HtmlOpeningElement | HtmlSelfClosingElement
}

impl AnyHtmlTagElement {
    pub fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        match self {
            Self::HtmlOpeningElement(element) => element.name(),
            Self::HtmlSelfClosingElement(element) => element.name(),
        }
    }

    pub fn attributes(&self) -> HtmlAttributeList {
        match self {
            Self::HtmlOpeningElement(element) => element.attributes(),
            Self::HtmlSelfClosingElement(element) => element.attributes(),
        }
    }

    pub fn name_value_token(&self) -> SyntaxResult<HtmlSyntaxToken> {
        self.name()?.name_value_token()
    }

    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        match self {
            Self::HtmlOpeningElement(element) => element.find_attribute_by_name(name_to_lookup),
            Self::HtmlSelfClosingElement(element) => element.find_attribute_by_name(name_to_lookup),
        }
    }

    pub fn has_truthy_attribute(&self, name_to_lookup: &str) -> bool {
        self.find_attribute_by_name(name_to_lookup)
            .is_some_and(|attribute| {
                attribute
                    .initializer()
                    .and_then(|init| init.value().ok())
                    .and_then(|value| value.string_value())
                    .is_none_or(|value| value != "false")
            })
    }

    /// Returns `true` if the current element is actually a component.
    ///
    /// - `<Span />` is a component and it would return `true`
    /// - `<span ></span>` is **not** component and it returns `false`
    pub fn is_custom_component(&self) -> bool {
        self.name().is_ok_and(|it| it.as_html_tag_name().is_none())
    }

    /// Whether the current element is a custom element.
    ///
    /// A custom element must contain dashes and its name is all lower case.
    pub fn is_custom_element(&self) -> bool {
        self.name()
            .ok()
            .and_then(|it| it.as_html_tag_name().cloned())
            .and_then(|element| element.value_token().ok())
            .is_some_and(|token| {
                token.text_trimmed().contains('-')
                    && token
                        .text_trimmed()
                        .eq(token.text_trimmed().to_lowercase_cow().as_ref())
            })
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

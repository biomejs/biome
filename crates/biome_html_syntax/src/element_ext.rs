use crate::{
    AnyHtmlContent, AnyHtmlElement, AnyHtmlTagName, AnyHtmlTextExpression, AnySvelteBlock,
    AstroEmbeddedContent, HtmlAttribute, HtmlAttributeList, HtmlElement, HtmlEmbeddedContent,
    HtmlOpeningElement, HtmlSelfClosingElement, HtmlSyntaxToken, HtmlTagName, ScriptType,
    inner_string_text,
};
use biome_rowan::{AstNodeList, SyntaxResult, TokenText, declare_node_union};

/// https://html.spec.whatwg.org/#void-elements
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

/// Helper to get the text value from any tag name variant
fn get_tag_name_text(name: &AnyHtmlTagName) -> Option<TokenText> {
    match name {
        AnyHtmlTagName::HtmlTagName(tag) => {
            let token = tag.value_token().ok()?;
            Some(token.token_text_trimmed())
        }
        AnyHtmlTagName::HtmlComponentName(component) => {
            let token = component.value_token().ok()?;
            Some(token.token_text_trimmed())
        }
        AnyHtmlTagName::HtmlMemberName(_) => None,
    }
}

impl HtmlSelfClosingElement {
    /// Whether the current self-closing element is a void element.
    ///
    /// <https://html.spec.whatwg.org/#void-elements>
    pub fn is_void_element(&self) -> Option<bool> {
        let name = self.name().ok()?;
        let name_text = get_tag_name_text(&name)?;
        Some(VOID_ELEMENTS.binary_search(&&*name_text).is_ok())
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

    /// Find an attribute by name (case-insensitive) within this element, if it has attributes.
    ///
    /// This will not detect attributes in Svelte attribute shorthand like `<div {foo}>`.
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

    /// Returns the list of attributes for this element, if it has any.
    pub fn attributes(&self) -> Option<HtmlAttributeList> {
        match self {
            Self::HtmlElement(element) => Some(element.opening_element().ok()?.attributes()),
            Self::HtmlSelfClosingElement(element) => Some(element.attributes()),
            // Other variants don't have attributes
            _ => None,
        }
    }
}

impl HtmlSelfClosingElement {
    /// Returns the tag name of the element (trimmed), if it has one.
    pub fn tag_name(&self) -> Option<TokenText> {
        let name = self.name().ok()?;
        get_tag_name_text(&name)
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
        get_tag_name_text(&name)
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
        let Ok(name) = self.opening_element().and_then(|el| el.name()) else {
            return false;
        };

        let Some(name_text) = name.token_text_trimmed() else {
            return false;
        };

        name_text.eq_ignore_ascii_case("style")
    }

    pub fn is_script_tag(&self) -> bool {
        let Ok(name) = self.opening_element().and_then(|el| el.name()) else {
            return false;
        };

        let Some(name_text) = name.token_text_trimmed() else {
            return false;
        };

        name_text.eq_ignore_ascii_case("script")
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

impl AnyHtmlTagName {
    /// Returns the trimmed token text of the tag name.
    /// For member names like Component.Member, returns the full member expression text.
    pub fn token_text_trimmed(&self) -> Option<TokenText> {
        get_tag_name_text(self)
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

    pub fn name_value_token(&self) -> Option<HtmlSyntaxToken> {
        let name = self.name().ok()?;
        match name {
            AnyHtmlTagName::HtmlTagName(tag) => tag.value_token().ok(),
            AnyHtmlTagName::HtmlComponentName(_) => None,
            AnyHtmlTagName::HtmlMemberName(_) => None,
        }
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

use crate::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnySvelteTemplateElement, HtmlAttribute,
    HtmlAttributeList, HtmlAttributeName, inner_string_text, static_value::StaticValue,
};
use biome_aria::Attribute;
use biome_rowan::{AstNodeList, Text, TokenText};

impl AnyHtmlAttributeInitializer {
    /// Returns the string value of the attribute, if available, without quotes.
    pub fn string_value(&self) -> Option<Text> {
        match self {
            Self::HtmlAttributeSingleTextExpression(text) => text.expression().ok()?.string_value(),
            Self::HtmlString(string) => Some(
                string
                    .value_token()
                    .map(|token| inner_string_text(&token).into())
                    .unwrap_or_default(),
            ),
            // When all elements are plain text chunks (no {expression} interpolations),
            // the static text can be recovered by concatenating the chunk tokens.
            Self::SvelteTemplateAttributeValue(value) => {
                let elements = value.elements();
                let mut text = String::new();
                for element in elements.iter() {
                    match element {
                        AnySvelteTemplateElement::SvelteTemplateChunkElement(chunk) => {
                            text.push_str(chunk.html_template_chunk_token().ok()?.text_trimmed());
                        }
                        AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(_) => {
                            // Dynamic interpolation — no static value available.
                            return None;
                        }
                    }
                }
                Some(Text::from(text))
            }
            Self::VueVForValue(_) => None,
        }
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            Self::HtmlAttributeSingleTextExpression(_) => None,
            Self::HtmlString(value) => Some(StaticValue::String(value.value_token().ok()?)),
            // No single token to wrap — static values with interpolations are not representable.
            Self::SvelteTemplateAttributeValue(_) => None,
            Self::VueVForValue(_) => None,
        }
    }
}

impl HtmlAttributeList {
    pub fn find_by_name(&self, name_to_lookup: &str) -> Option<HtmlAttribute> {
        self.iter().find_map(|attribute| {
            if let AnyHtmlAttribute::HtmlAttribute(attribute) = attribute
                && let Ok(name) = attribute.name()
                && name.value_token().ok()?.text_trimmed() == name_to_lookup
            {
                return Some(attribute);
            }
            None
        })
    }
}

impl HtmlAttribute {
    /// Extracts the value from an attribute's initializer.
    ///
    /// Returns `None` if the attribute has no initializer or the value cannot be extracted.
    pub fn value(&self) -> Option<Text> {
        self.initializer()
            .and_then(|init| init.value().ok())
            .and_then(|value| value.string_value())
    }
}

impl HtmlAttributeName {
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

impl Attribute for AnyHtmlAttribute {
    fn name(&self) -> Option<impl AsRef<str>> {
        self.name()
    }

    fn value(&self) -> Option<impl AsRef<str>> {
        self.value()
    }
}

impl AnyHtmlAttribute {
    pub fn name(&self) -> Option<TokenText> {
        match self {
            Self::HtmlAttribute(attr) => attr.name().ok()?.token_text_trimmed(),
            Self::AnySvelteDirective(_)
            | Self::AnyVueDirective(_)
            | Self::HtmlAttributeDoubleTextExpression(_)
            | Self::HtmlAttributeSingleTextExpression(_)
            | Self::HtmlBogusAttribute(_)
            | Self::HtmlSpreadAttribute(_)
            | Self::AnyAstroDirective(_)
            | Self::SvelteAttachAttribute(_) => None,
        }
    }

    pub fn value(&self) -> Option<StaticValue> {
        match self {
            Self::HtmlAttribute(attr) => attr.initializer()?.value().ok()?.as_static_value(),
            Self::AnySvelteDirective(_)
            | Self::AnyVueDirective(_)
            | Self::HtmlAttributeDoubleTextExpression(_)
            | Self::HtmlAttributeSingleTextExpression(_)
            | Self::HtmlBogusAttribute(_)
            | Self::HtmlSpreadAttribute(_)
            | Self::AnyAstroDirective(_)
            | Self::SvelteAttachAttribute(_) => None,
        }
    }
}

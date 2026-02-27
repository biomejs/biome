use crate::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, HtmlAttribute, HtmlAttributeList,
    HtmlAttributeName, inner_string_text,
};
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

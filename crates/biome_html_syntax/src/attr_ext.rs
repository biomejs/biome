use crate::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, HtmlAttribute, HtmlAttributeList,
    inner_string_text,
};
use biome_rowan::{AstNodeList, Text};

impl AnyHtmlAttributeInitializer {
    /// Returns the string value of the attribute, if available, without quotes.
    pub fn string_value(&self) -> Option<Text> {
        match self {
            Self::HtmlSingleTextExpression(_) => None,
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

use crate::{AnyHtmlAttributeInitializer, inner_string_text};
use biome_rowan::Text;

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

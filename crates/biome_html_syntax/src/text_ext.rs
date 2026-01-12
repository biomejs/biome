use crate::HtmlTextExpression;
use biome_rowan::Text;

impl HtmlTextExpression {
    /// Returns the string value of the attribute, if available, without quotes.
    pub fn string_value(&self) -> Option<Text> {
        self.html_literal_token()
            .ok()
            .map(|token| token.token_text_trimmed().into())
    }
}

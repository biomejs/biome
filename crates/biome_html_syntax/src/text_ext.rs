use crate::{HtmlTextExpression, static_value::StaticValue};
use biome_rowan::Text;

impl HtmlTextExpression {
    /// Returns the string value of the attribute, if available, without quotes.
    pub fn string_value(&self) -> Option<Text> {
        self.html_literal_token()
            .ok()
            .map(|token| token.token_text_trimmed().into())
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        let token = self.html_literal_token().ok()?;
        let value = &token.token_text_trimmed();

        if value.is_empty() {
            return None;
        }

        let text = value.text();
        match text {
            "true" | "false" => Some(StaticValue::Boolean(token)),
            "undefined" => Some(StaticValue::Undefined(token)),
            "null" => Some(StaticValue::Null(token)),
            _ => {
                if (text.starts_with('"') && text.ends_with('"'))
                    || (text.starts_with('\'') && text.ends_with('\''))
                {
                    return Some(StaticValue::String(token));
                }

                None
            }
        }
    }
}

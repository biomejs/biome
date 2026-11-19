use crate::{
    HtmlAttributeSingleTextExpression, HtmlSingleTextExpression, HtmlTextExpression,
    SvelteTemplateElementList, is_quoted, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstNodeList, Text};

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
                if is_quoted(text) {
                    return Some(StaticValue::String(token));
                }

                None
            }
        }
    }

    /// Returns `true` if Svelte converts the value of this expression to a string when
    /// rendering it.
    ///
    /// That is the case for a mustache in element content, such as `<p>{value}</p>`,
    /// and for a mustache inside a quoted attribute value that contains other parts,
    /// such as `class="item {value}"`. A mustache that forms an entire attribute value,
    /// such as `prop={value}` or `prop="{value}"`, returns `false`.
    pub fn is_svelte_text_interpolation(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        if HtmlSingleTextExpression::can_cast(parent.kind()) {
            return true;
        }
        HtmlAttributeSingleTextExpression::cast(parent)
            .and_then(|mustache| mustache.parent::<SvelteTemplateElementList>())
            .is_some_and(|list| list.len() > 1)
    }
}

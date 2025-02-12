use crate::{inner_string_text, AnyJsonValue, JsonMember, JsonMemberName, JsonSyntaxToken};
use biome_rowan::{AstSeparatedList, SyntaxResult, TokenText};

impl JsonMemberName {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl JsonMember {
    /// If the value of the member is a [JsonObjectValue](crate::JsonObjectValue), it returns a tuple
    /// that contains a list of [JsonMember] and of its separators. Returns [None] otherwise
    pub fn unzip_elements(&self) -> Option<(Vec<JsonMember>, Vec<JsonSyntaxToken>)> {
        let value = self.value().ok()?;
        let AnyJsonValue::JsonObjectValue(value) = value else {
            return None;
        };

        let members = value.json_member_list().iter().flatten().collect();
        let separators = value.json_member_list().separators().flatten().collect();
        Some((members, separators))
    }
}

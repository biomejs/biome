use crate::{
    AnyJsonMemberName, AnyJsonValue, JsonMember, JsonMemberName, JsonSyntaxToken, inner_string_text,
};
use biome_rowan::{AstSeparatedList, SyntaxResult, TokenText};

impl JsonMemberName {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl AnyJsonMemberName {
    /// Returns the inner string text if this is a [JsonMemberName], [None] otherwise
    pub fn inner_string_text(&self) -> Option<TokenText> {
        match self {
            Self::JsonMemberName(name) => name.inner_string_text().ok(),
            _ => None,
        }
    }

    /// Returns the value token if this is a [JsonMemberName], [None] otherwise
    pub fn value_token(&self) -> Option<JsonSyntaxToken> {
        match self {
            Self::JsonMemberName(name) => name.value_token().ok(),
            _ => None,
        }
    }
}

impl JsonMember {
    /// If the value of the member is a [JsonObjectValue](crate::JsonObjectValue), it returns a tuple
    /// that contains a list of [JsonMember] and of its separators. Returns [None] otherwise
    pub fn unzip_elements(&self) -> Option<(Vec<Self>, Vec<JsonSyntaxToken>)> {
        let value = self.value().ok()?;
        let AnyJsonValue::JsonObjectValue(value) = value else {
            return None;
        };

        let members = value.json_member_list().iter().flatten().collect();
        let separators = value.json_member_list().separators().flatten().collect();
        Some((members, separators))
    }
}

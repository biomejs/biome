use crate::{AnyJsonValue, JsonMember, JsonMemberName, JsonSyntaxToken, inner_string_text};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxResult, TokenText};

impl JsonMemberName {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
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

    pub fn with_name(self, name: JsonMemberName) -> Self {
        Self::unwrap_cast(
            self.into_syntax()
                .splice_slots(0..=0, [Some(name.into_syntax().into())]),
        )
    }

    pub fn with_value(self, value: AnyJsonValue) -> Self {
        Self::unwrap_cast(
            self.into_syntax()
                .splice_slots(2..=2, [Some(value.into_syntax().into())]),
        )
    }
}

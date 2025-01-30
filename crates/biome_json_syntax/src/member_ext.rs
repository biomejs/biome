use crate::{inner_string_text, AnyJsonValue, JsonMember, JsonMemberName, JsonSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxResult, TokenText};

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

    /// If the value of the member is a [JsonObjectValue](crate::JsonObjectValue), it returns
    /// the list of its items allocated into a vector. It returns [None] otherwise
    pub fn map_members(&self) -> Option<Vec<JsonMember>> {
        self.value()
            .ok()?
            .as_json_object_value()
            .map(|value| value.json_member_list().iter().flatten().collect())
    }

    /// If the value of the current member is an object, it returns the [JsonMember] that has
    /// a name that matches the input `name`
    ///
    /// It returns [None] otherwise
    pub fn find_member_by_name(&self, name: &str) -> Option<JsonMember> {
        self.value().ok()?.as_json_object_value().and_then(|value| {
            value
                .json_member_list()
                .iter()
                .flatten()
                .find_map(|member| {
                    if member.name().ok()?.inner_string_text().ok()?.text() == name {
                        Some(member)
                    } else {
                        None
                    }
                })
        })
    }

    /// From this member it traverses all its ancestors until it finds a [JsonMember] that matches the given name
    pub fn find_member_by_name_upwards(&self, name: &str) -> Option<JsonMember> {
        self.syntax.ancestors().find_map(|node| {
            if let Some(member) = JsonMember::cast(node) {
                let member_name = member.name().ok()?.inner_string_text().ok()?;
                if member_name.text() == name {
                    return Some(member);
                }
            }
            None
        })
    }
}

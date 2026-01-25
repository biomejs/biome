use crate::{AnyJsonMemberName, JsonMember, JsonMemberList, JsonObjectValue};
use biome_rowan::AstSeparatedList;

impl JsonObjectValue {
    pub fn find_member(&self, name: &str) -> Option<JsonMember> {
        self.json_member_list().find_member(name)
    }
}

impl JsonMemberList {
    pub fn find_member(&self, name: &str) -> Option<JsonMember> {
        for member in self.iter().flatten() {
            let Ok(member_name) = member.name() else {
                continue;
            };
            let member_name_text = match member_name {
                AnyJsonMemberName::JsonMemberName(n) => n.inner_string_text().ok(),
                // Metavariables and bogus nodes don't have string text
                _ => None,
            };
            let Some(member_name_text) = member_name_text else {
                continue;
            };
            if member_name_text == name {
                return Some(member);
            }
        }
        None
    }
}

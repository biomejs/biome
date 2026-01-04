use crate::{JsonMember, JsonMemberList, JsonObjectValue};
use biome_rowan::AstSeparatedList;

impl JsonObjectValue {
    pub fn find_member(&self, name: &str) -> Option<JsonMember> {
        self.json_member_list().find_member(name)
    }
}

impl JsonMemberList {
    pub fn find_member(&self, name: &str) -> Option<JsonMember> {
        for member in self.iter().flatten() {
            let Ok(member_name) = member.name().and_then(|n| n.inner_string_text()) else {
                continue;
            };
            if member_name == name {
                return Some(member);
            }
        }
        None
    }
}

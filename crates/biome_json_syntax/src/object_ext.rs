use crate::{JsonMemberList, JsonObjectValue};
use biome_rowan::AstNode;

impl JsonObjectValue {
    pub fn with_json_member_list(self, list: JsonMemberList) -> Self {
        Self::unwrap_cast(
            self.into_syntax()
                .splice_slots(1..=1, [Some(list.into_syntax().into())]),
        )
    }
}

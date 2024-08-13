use biome_json_syntax::{JsonMember, JsonMemberList, JsonMemberName, JsonObjectValue};
use biome_rowan::AstNode;

/// Matches a JSON member name node against a path
pub fn matches_path(optional_node: Option<&JsonMemberName>, path: &[&str]) -> bool {
    if path.is_empty() {
        return true;
    }

    let Some(node) = optional_node else {
        return false;
    };

    if !node
        .inner_string_text()
        .is_ok_and(|text| Some(&text.text()) == path.last())
    {
        return false;
    }

    let optional_parent_node = node
        .syntax()
        .parent()
        .and_then(|p| {
            if JsonMember::can_cast(p.kind()) {
                p.parent()
            } else {
                None
            }
        })
        .and_then(|p| {
            if JsonMemberList::can_cast(p.kind()) {
                p.parent()
            } else {
                None
            }
        })
        .and_then(|p| {
            if JsonObjectValue::can_cast(p.kind()) {
                p.parent()
            } else {
                None
            }
        })
        .and_then(|p| JsonMember::cast(p)?.name().ok());

    matches_path(optional_parent_node.as_ref(), &path[..path.len() - 1])
}

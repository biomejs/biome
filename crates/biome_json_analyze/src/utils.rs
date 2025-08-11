use biome_json_syntax::JsonMember;
use biome_rowan::AstNode;

/// Finds the first ancestor [JsonMember], and returns [true] if it's name matches the given input
pub(crate) fn matches_parent_object(node: &JsonMember, name: &str) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find_map(JsonMember::cast)
        .and_then(|member| member.name().ok())
        .and_then(|member| member.inner_string_text().ok())
        .is_some_and(|text| text.text() == name)
}

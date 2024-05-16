use biome_json_syntax::{JsonSyntaxKind, JsonSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn ident(text: &str) -> JsonSyntaxToken {
    JsonSyntaxToken::new_detached(JsonSyntaxKind::IDENT, text, [], [])
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: JsonSyntaxKind) -> JsonSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsonSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new string literal token with no attached trivia
pub fn json_string_literal(text: &str) -> JsonSyntaxToken {
    JsonSyntaxToken::new_detached(
        JsonSyntaxKind::JSON_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

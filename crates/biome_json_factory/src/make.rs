use biome_json_syntax::{JsonSyntaxKind, JsonSyntaxToken};

pub use crate::generated::node_factory::*;

pub fn ident(text: &str) -> JsonSyntaxToken {
    JsonSyntaxToken::new_detached(JsonSyntaxKind::IDENT, text, [], [])
}

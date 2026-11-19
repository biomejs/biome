#![deny(clippy::use_self)]

mod events;
mod format_semantic_model;

mod db;
mod semantic_model;
#[cfg(test)]
mod tests;

use biome_js_syntax::unescape_js_identifier;
use biome_rowan::{Text, TokenText};

pub use db::{js_semantic_model, semantic_model_from_snippet, semantic_model_from_source};
pub use events::*;
pub use semantic_model::*;

/// Returns the semantic name represented by parsed identifier token text.
///
/// Unicode escapes are decoded without applying Unicode normalization. Names
/// without escapes retain their token-backed storage.
fn identifier_name(text: TokenText) -> Text {
    let decoded = match unescape_js_identifier(text.text()) {
        std::borrow::Cow::Borrowed(_) => None,
        std::borrow::Cow::Owned(decoded) => Some(decoded),
    };
    decoded.map_or_else(|| text.into(), Into::into)
}

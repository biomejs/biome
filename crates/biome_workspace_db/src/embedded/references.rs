use biome_rowan::{TextRange, TokenText};

#[salsa::input]
pub struct EmbeddedValueReference {
    /// Where it's been used
    pub range: TextRange,

    /// The text of the reference
    pub text: TokenText,
}

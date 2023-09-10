use crate::{inner_string_text, JsonStringValue};
use biome_rowan::{SyntaxResult, TokenText};

impl JsonStringValue {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

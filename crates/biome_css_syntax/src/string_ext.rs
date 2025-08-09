use crate::{CssString, inner_string_text};
use biome_rowan::{SyntaxResult, TokenText};

impl CssString {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

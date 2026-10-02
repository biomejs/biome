use crate::{YamlAliasNode, YamlAnchorProperty, YamlSyntaxToken};
use biome_rowan::{SyntaxResult, TextRange, TextSize, TokenText};

impl YamlAnchorProperty {
    /// Returns the name of the anchor, without its `&`.
    pub fn name(&self) -> SyntaxResult<TokenText> {
        Ok(text_without_indicator(&self.value_token()?))
    }
}

impl YamlAliasNode {
    /// Returns the name of the anchor that the alias refers to, without its `*`.
    pub fn name(&self) -> SyntaxResult<TokenText> {
        Ok(text_without_indicator(&self.value_token()?))
    }
}

/// Returns the text of an anchor or alias token without the indicator that starts it.
fn text_without_indicator(token: &YamlSyntaxToken) -> TokenText {
    let text = token.token_text_trimmed();
    let range = TextRange::new(TextSize::from(1), text.len());
    text.slice(range)
}

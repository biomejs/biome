use crate::{YamlAliasNode, YamlAnchorProperty, YamlSyntaxKind, YamlSyntaxToken};
use biome_rowan::{SyntaxResult, TokenText};

impl YamlAnchorProperty {
    /// Returns the name of the anchor, without its `&`.
    pub fn name(&self) -> SyntaxResult<TokenText> {
        let token = self.value_token()?;
        debug_assert_eq!(token.kind(), YamlSyntaxKind::ANCHOR_PROPERTY_LITERAL);
        Ok(text_without_indicator(&token, '&'))
    }
}

impl YamlAliasNode {
    /// Returns the name of the anchor that the alias refers to, without its `*`.
    pub fn name(&self) -> SyntaxResult<TokenText> {
        let token = self.value_token()?;
        debug_assert_eq!(token.kind(), YamlSyntaxKind::ALIAS_LITERAL);
        Ok(text_without_indicator(&token, '*'))
    }
}

/// Returns the text of an anchor or alias token without the indicator that starts it.
fn text_without_indicator(token: &YamlSyntaxToken, indicator: char) -> TokenText {
    let text = token.token_text_trimmed();
    debug_assert!(text.starts_with(indicator));
    text.strip_prefix(indicator).unwrap_or(text)
}

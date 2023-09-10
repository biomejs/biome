use crate::parentheses::NeedsParentheses;
use crate::FormatBogusNodeRule;
use biome_js_syntax::{JsSyntaxNode, TsBogusType};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsBogusType;

impl FormatBogusNodeRule<TsBogusType> for FormatTsBogusType {}

impl NeedsParentheses for TsBogusType {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

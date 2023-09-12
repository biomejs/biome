use crate::parentheses::NeedsParentheses;
use crate::FormatBogusNodeRule;
use biome_js_syntax::{JsBogusExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusExpression;

impl FormatBogusNodeRule<JsBogusExpression> for FormatJsBogusExpression {}

impl NeedsParentheses for JsBogusExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        self.needs_parentheses()
    }
}

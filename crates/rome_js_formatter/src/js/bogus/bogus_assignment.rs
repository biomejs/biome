use crate::parentheses::NeedsParentheses;
use crate::FormatBogusNodeRule;
use biome_js_syntax::{JsBogusAssignment, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusAssignment;

impl FormatBogusNodeRule<JsBogusAssignment> for FormatJsBogusAssignment {}

impl NeedsParentheses for JsBogusAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}

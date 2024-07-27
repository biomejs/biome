use crate::parentheses::NeedsParentheses;
use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusAssignment;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusAssignment;

impl FormatBogusNodeRule<JsBogusAssignment> for FormatJsBogusAssignment {}

impl NeedsParentheses for JsBogusAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

use crate::parentheses::NeedsParentheses;
use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusExpression;

impl FormatBogusNodeRule<JsBogusExpression> for FormatJsBogusExpression {}

impl NeedsParentheses for JsBogusExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

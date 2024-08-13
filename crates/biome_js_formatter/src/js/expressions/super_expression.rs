use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsSuperExpression;
use biome_js_syntax::JsSuperExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSuperExpression;

impl FormatNodeRule<JsSuperExpression> for FormatJsSuperExpression {
    fn fmt_fields(&self, node: &JsSuperExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSuperExpressionFields { super_token } = node.as_fields();

        write![f, [super_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsSuperExpression) -> bool {
        item.needs_parentheses()
    }
}

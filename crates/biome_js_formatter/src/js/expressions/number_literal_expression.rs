use crate::prelude::*;

use biome_formatter::token::number::format_number_token;
use biome_formatter::token::number::NumberFormatOptions;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsNumberLiteralExpression;
use biome_js_syntax::JsNumberLiteralExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNumberLiteralExpression;

impl FormatNodeRule<JsNumberLiteralExpression> for FormatJsNumberLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsNumberLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsNumberLiteralExpressionFields { value_token } = node.as_fields();
        format_number_token(
            &value_token?,
            NumberFormatOptions::default().keep_one_trailing_decimal_zero(),
        )
        .fmt(f)
    }

    fn needs_parentheses(&self, item: &JsNumberLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsNumberLiteralExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(5).test", JsNumberLiteralExpression);
        assert_needs_parentheses!("(5)[test]", JsNumberLiteralExpression);
        assert_not_needs_parentheses!("test[5]", JsNumberLiteralExpression);
    }
}

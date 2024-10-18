use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsBigintLiteralExpression;
use biome_js_syntax::JsBigintLiteralExpressionFields;
use biome_string_case::StrLikeExtension;
use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBigintLiteralExpression;

impl FormatNodeRule<JsBigintLiteralExpression> for FormatJsBigintLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsBigintLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBigintLiteralExpressionFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let original = value_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [value_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &dynamic_text(&lowercase, value_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }

    fn needs_parentheses(&self, item: &JsBigintLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

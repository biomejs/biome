use rome_formatter::token::string::ToAsciiLowercaseCow;
use rome_formatter::write;
use std::borrow::Cow;

use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::JsBigintLiteralExpressionFields;
use biome_js_syntax::{JsBigintLiteralExpression, JsSyntaxNode};

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

impl NeedsParentheses for JsBigintLiteralExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

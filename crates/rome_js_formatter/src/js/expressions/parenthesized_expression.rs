use crate::prelude::*;
use rome_formatter::{format_args, write, CstFormatContext};

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsParenthesizedExpression;

impl FormatNodeRule<JsParenthesizedExpression> for FormatJsParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &JsParenthesizedExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let expression = expression?;
        let comments = f.context().comments();

        let should_hug = !comments.has_comments(expression.syntax())
            && (matches!(
                expression,
                AnyJsExpression::JsObjectExpression(_) | AnyJsExpression::JsArrayExpression(_)
            ));

        if should_hug {
            write!(
                f,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&expression.format()),
                    r_paren_token.format()
                ])]
            )
        }
    }

    fn needs_parentheses(&self, item: &JsParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsParenthesizedExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

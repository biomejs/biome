use crate::prelude::*;

use biome_formatter::{CstFormatContext, format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields,
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
        let should_insert_space = f.options().delimiter_spacing().value();

        let should_hug = !comments.has_comments(expression.syntax())
            && (matches!(
                expression,
                AnyJsExpression::JsObjectExpression(_) | AnyJsExpression::JsArrayExpression(_)
            ));

        let l_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [l_paren_token.format(), space()])
            } else {
                write!(f, [l_paren_token.format()])
            }
        });

        let r_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space(), r_paren_token.format()])
            } else {
                write!(f, [r_paren_token.format()])
            }
        });

        if should_hug {
            write!(f, [l_paren, expression.format(), r_paren])
        } else {
            write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&expression.format(), should_insert_space),
                    r_paren_token.format()
                ])]
            )
        }
    }

    fn needs_parentheses(&self, item: &JsParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}

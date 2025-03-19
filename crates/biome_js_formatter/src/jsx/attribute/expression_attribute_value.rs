use crate::prelude::*;

use biome_formatter::{CstFormatContext, format_args, write};
use biome_js_syntax::{
    AnyJsExpression, AnyJsxTag, JsxExpressionAttributeValue, JsxExpressionAttributeValueFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionAttributeValue;

impl FormatNodeRule<JsxExpressionAttributeValue> for FormatJsxExpressionAttributeValue {
    fn fmt_fields(
        &self,
        node: &JsxExpressionAttributeValue,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsxExpressionAttributeValueFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let expression = expression?;

        let should_inline = should_inline_jsx_expression(&expression, f.context().comments());

        if should_inline {
            write!(
                f,
                [
                    l_curly_token.format(),
                    expression.format(),
                    line_suffix_boundary(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_curly_token.format(),
                    soft_block_indent(&expression.format()),
                    line_suffix_boundary(),
                    r_curly_token.format()
                ])]
            )
        }
    }
}

/// Tests if an expression inside of a [JsxExpressionChild] or [JsxExpressionAttributeValue] should be inlined.
/// Good:
/// ```jsx
///  <ColorPickerPage
///     colors={[
///        "blue",
///        "brown",
///        "green",
///        "orange",
///        "purple",
///     ]} />
/// ```
///
/// Bad:
/// ```jsx
///  <ColorPickerPage
///     colors={
///       [
///         "blue",
///          "brown",
///         "green",
///         "orange",
///         "purple",
///       ]
///     } />
/// ```
pub(crate) fn should_inline_jsx_expression(
    expression: &AnyJsExpression,
    comments: &JsComments,
) -> bool {
    use AnyJsExpression::*;

    if comments.has_comments(expression.syntax()) {
        return false;
    }

    match expression {
        JsArrayExpression(_)
        | JsObjectExpression(_)
        | JsArrowFunctionExpression(_)
        | JsCallExpression(_)
        | JsImportCallExpression(_)
        | JsImportMetaExpression(_)
        | JsFunctionExpression(_)
        | JsTemplateExpression(_) => true,
        JsAwaitExpression(await_expression) => match await_expression.argument() {
            Ok(JsxTagExpression(argument)) => {
                matches!(argument.tag(), Ok(AnyJsxTag::JsxElement(_)))
                    && should_inline_jsx_expression(&argument.into(), comments)
            }
            _ => false,
        },
        _ => false,
    }
}

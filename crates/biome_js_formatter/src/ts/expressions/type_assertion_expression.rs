use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::AnyJsExpression;
use biome_js_syntax::{TsTypeAssertionExpression, TsTypeAssertionExpressionFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionExpression;

impl FormatNodeRule<TsTypeAssertionExpression> for FormatTsTypeAssertionExpression {
    fn fmt_fields(
        &self,
        node: &TsTypeAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = node.as_fields();

        let expression = expression?;

        let break_after_cast = !matches!(
            expression,
            AnyJsExpression::JsArrayExpression(_) | AnyJsExpression::JsObjectExpression(_)
        );

        let format_cast = format_with(|f| {
            write!(
                f,
                [
                    l_angle_token.format(),
                    group(&soft_block_indent(&ty.format())),
                    r_angle_token.format(),
                ]
            )
        });

        if break_after_cast {
            let format_cast = format_cast.memoized();
            let format_expression = expression.format().memoized();

            write!(
                f,
                [best_fitting![
                    format_args![format_cast, format_expression],
                    format_args![
                        format_cast,
                        group(&format_args![
                            text("("),
                            block_indent(&format_expression),
                            text(")")
                        ])
                    ],
                    format_args![format_cast, format_expression]
                ]]
            )
        } else {
            write![f, [format_cast, expression.format()]]
        }
    }

    fn needs_parentheses(&self, item: &TsTypeAssertionExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsTypeAssertionExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(<number> x) as any", TsTypeAssertionExpression);

        assert_needs_parentheses!("class X extends (<number>B) {}", TsTypeAssertionExpression);

        assert_needs_parentheses!("(<Function>x)()", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<Function>x)?.()", TsTypeAssertionExpression);
        assert_needs_parentheses!("new (<Function>x)()", TsTypeAssertionExpression);

        assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
        assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
        assert_needs_parentheses!("(<any>x)`template`", TsTypeAssertionExpression);
        assert_needs_parentheses!("!(<any>x)", TsTypeAssertionExpression);
        assert_needs_parentheses!("[...(<any>x)]", TsTypeAssertionExpression);
        assert_needs_parentheses!("({...(<any>x)})", TsTypeAssertionExpression);

        assert_needs_parentheses!("await (<any>x)", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<any>x)!", TsTypeAssertionExpression);

        assert_needs_parentheses!("(<any>x).member", TsTypeAssertionExpression);
        assert_needs_parentheses!("(<any>x)[member]", TsTypeAssertionExpression);
        assert_not_needs_parentheses!("object[<any>x]", TsTypeAssertionExpression);
    }
}

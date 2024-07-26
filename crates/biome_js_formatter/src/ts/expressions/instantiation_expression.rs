use crate::{parentheses::NeedsParentheses, prelude::*};
use biome_formatter::write;
use biome_js_syntax::{
    AnyJsMemberExpression, TsInstantiationExpression, TsInstantiationExpressionFields,
};
#[derive(Debug, Clone, Default)]
pub struct FormatTsInstantiationExpression;
impl FormatNodeRule<TsInstantiationExpression> for FormatTsInstantiationExpression {
    fn fmt_fields(
        &self,
        node: &TsInstantiationExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsInstantiationExpressionFields {
            expression,
            arguments,
        } = node.as_fields();

        write![f, [expression.format(), arguments.format()]]
    }

    fn needs_parentheses(&self, item: &TsInstantiationExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsInstantiationExpression {
    fn needs_parentheses(&self) -> bool {
        self.parent::<AnyJsMemberExpression>()
            .is_some_and(|member_expr| {
                member_expr
                    .object()
                    .map(|object| object.syntax() == self.syntax())
                    .unwrap_or(false)
            })
    }
}

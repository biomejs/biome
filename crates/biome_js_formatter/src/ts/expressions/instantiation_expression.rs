use crate::{
    parentheses::{unary_like_expression_needs_parentheses, NeedsParentheses},
    prelude::*,
};
use biome_formatter::write;
use biome_js_syntax::{TsInstantiationExpression, TsInstantiationExpressionFields};
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
}

impl NeedsParentheses for TsInstantiationExpression {
    fn needs_parentheses_with_parent(&self, parent: &biome_js_syntax::JsSyntaxNode) -> bool {
        unary_like_expression_needs_parentheses(self.syntax(), parent)
    }
}

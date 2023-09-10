use crate::prelude::*;

use crate::js::expressions::static_member_expression::member_chain_callee_needs_parens;
use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxKind, TsNonNullAssertionExpressionFields};
use biome_js_syntax::{JsSyntaxNode, TsNonNullAssertionExpression};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionExpression;

impl FormatNodeRule<TsNonNullAssertionExpression> for FormatTsNonNullAssertionExpression {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionExpressionFields {
            expression,
            excl_token,
        } = node.as_fields();

        write![f, [expression.format(), excl_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNonNullAssertionExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNonNullAssertionExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
            || member_chain_callee_needs_parens(self.clone().into(), parent)
    }
}

use crate::prelude::*;
use crate::utils::format_class::FormatClass;
use biome_formatter::{format_args, write};

use crate::parentheses::{
    is_callee, is_first_in_statement, FirstInStatementMode, NeedsParentheses,
};
use biome_js_syntax::{JsClassExpression, JsSyntaxKind, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsClassExpression;

impl FormatNodeRule<JsClassExpression> for FormatJsClassExpression {
    fn fmt_fields(&self, node: &JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        if node.decorators().is_empty() || !self.needs_parentheses(node) {
            FormatClass::from(&node.clone().into()).fmt(f)
        } else {
            write!(
                f,
                [
                    indent(&format_args![
                        soft_line_break_or_space(),
                        &FormatClass::from(&node.clone().into()),
                    ]),
                    soft_line_break_or_space()
                ]
            )
        }
    }

    fn needs_parentheses(&self, item: &JsClassExpression) -> bool {
        /*!item.decorators().is_empty() || */
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsClassExpression,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted as part of `FormatClass`
        Ok(())
    }
}

impl NeedsParentheses for JsClassExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        (parent.kind() == JsSyntaxKind::JS_EXTENDS_CLAUSE && !self.decorators().is_empty())
            || is_callee(self.syntax(), parent)
            || is_first_in_statement(
                self.clone().into(),
                FirstInStatementMode::ExpressionOrExportDefault,
            )
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsClassExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("console.log((class {})())", JsClassExpression);
        assert_needs_parentheses!("console.log(new (class {})())", JsClassExpression);

        assert_needs_parentheses!("(class {}).test", JsClassExpression);
        assert_not_needs_parentheses!("a => class {} ", JsClassExpression);

        assert_needs_parentheses!("export default (class  {})", JsClassExpression);
    }
}

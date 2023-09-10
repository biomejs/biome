use crate::prelude::*;

use crate::js::declarations::function_declaration::{FormatFunction, FormatFunctionOptions};
use crate::parentheses::{
    is_callee, is_first_in_statement, is_tag, FirstInStatementMode, NeedsParentheses,
};

use biome_js_syntax::{JsFunctionExpression, JsSyntaxNode};
use rome_formatter::FormatRuleWithOptions;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsFunctionExpression {
    options: FormatFunctionOptions,
}

impl FormatRuleWithOptions<JsFunctionExpression> for FormatJsFunctionExpression {
    type Options = FormatFunctionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<JsFunctionExpression> for FormatJsFunctionExpression {
    fn fmt_fields(&self, node: &JsFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatFunction::from(node.clone()).fmt_with_options(f, &self.options)?;
        Ok(())
    }

    fn needs_parentheses(&self, item: &JsFunctionExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsFunctionExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        is_callee(self.syntax(), parent)
            || is_tag(self.syntax(), parent)
            || is_first_in_statement(
                self.clone().into(),
                FirstInStatementMode::ExpressionOrExportDefault,
            )
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsFunctionExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("console.log((function () {})())", JsFunctionExpression);
        assert_needs_parentheses!("console.log(new (function () {})())", JsFunctionExpression);

        assert_needs_parentheses!("(function() {}).test", JsFunctionExpression);
        assert_not_needs_parentheses!("a => function () {} ", JsFunctionExpression);

        assert_needs_parentheses!(
            "console.log((function () {})`template`)",
            JsFunctionExpression
        );

        assert_needs_parentheses!("export default (function () {})", JsFunctionExpression);
    }
}

use crate::js::declarations::function_declaration::{FormatFunction, FormatFunctionOptions};
use crate::prelude::*;

use biome_formatter::FormatRuleWithOptions;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsFunctionExpression;

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

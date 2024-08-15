use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsComputedMember, AnyJsExpression, AnyJsLiteralExpression, JsComputedMemberExpression,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberExpression;

impl FormatNodeRule<JsComputedMemberExpression> for FormatJsComputedMemberExpression {
    fn fmt_fields(
        &self,
        node: &JsComputedMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsComputedMember::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsComputedMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

impl Format<JsFormatContext> for AnyJsComputedMember {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.object().format()])?;

        FormatComputedMemberLookup(self).fmt(f)
    }
}

/// Formats the lookup portion (everything except the object) of a computed member like.
pub(crate) struct FormatComputedMemberLookup<'a>(&'a AnyJsComputedMember);

impl<'a> FormatComputedMemberLookup<'a> {
    pub(crate) fn new(member_like: &'a AnyJsComputedMember) -> Self {
        Self(member_like)
    }
}

impl Format<JsFormatContext> for FormatComputedMemberLookup<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.0.member()? {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                write!(
                    f,
                    [
                        self.0.optional_chain_token().format(),
                        self.0.l_brack_token().format(),
                        literal.format(),
                        self.0.r_brack_token().format()
                    ]
                )
            }
            member => {
                write![
                    f,
                    [group(&format_args![
                        self.0.optional_chain_token().format(),
                        self.0.l_brack_token().format(),
                        soft_block_indent(&member.format()),
                        self.0.r_brack_token().format()
                    ])]
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsComputedMemberExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (test()[a])()", JsComputedMemberExpression);
        assert_needs_parentheses!("new (test().a[b])()", JsComputedMemberExpression);
        assert_needs_parentheses!(
            "new (test()`template`[index])()",
            JsComputedMemberExpression
        );
        assert_needs_parentheses!("new (test()![member])()", JsComputedMemberExpression);

        assert_not_needs_parentheses!("new (test[a])()", JsComputedMemberExpression);
    }
}

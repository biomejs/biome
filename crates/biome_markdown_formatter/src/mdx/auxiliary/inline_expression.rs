use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdxInlineExpression, MdxInlineExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxInlineExpression;
impl FormatNodeRule<MdxInlineExpression> for FormatMdxInlineExpression {
    fn fmt_fields(
        &self,
        node: &MdxInlineExpression,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdxInlineExpressionFields {
            l_curly_token,
            content,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                content.format(),
                r_curly_token.format()
            ]
        )
    }
}

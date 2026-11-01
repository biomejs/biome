use crate::prelude::*;
use crate::shared::follows_space;
use biome_formatter::write;
use biome_markdown_syntax::{MdxBlockExpression, MdxBlockExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxBlockExpression;
impl FormatNodeRule<MdxBlockExpression> for FormatMdxBlockExpression {
    fn fmt_fields(&self, node: &MdxBlockExpression, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxBlockExpressionFields {
            indent,
            l_curly_token,
            content,
            r_curly_token,
        } = node.as_fields();
        let l_curly_token = l_curly_token?;

        if follows_space(&l_curly_token) {
            write!(f, [space()])?;
        }
        write!(
            f,
            [
                indent.format(),
                l_curly_token.format(),
                content.format(),
                r_curly_token.format()
            ]
        )
    }
}

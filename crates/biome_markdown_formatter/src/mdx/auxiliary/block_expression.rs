use crate::prelude::*;
use crate::shared::follows_construct_with_space;
use biome_formatter::write;
use biome_markdown_syntax::{MdxBlockExpression, MdxBlockExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxBlockExpression;
impl FormatNodeRule<MdxBlockExpression> for FormatMdxBlockExpression {
    fn fmt_fields(&self, node: &MdxBlockExpression, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxBlockExpressionFields {
            l_curly_token,
            content,
            r_curly_token,
        } = node.as_fields();
        let l_curly_token = l_curly_token?;

        if follows_construct_with_space(node.syntax(), &l_curly_token) {
            write!(f, [space()])?;
        }
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

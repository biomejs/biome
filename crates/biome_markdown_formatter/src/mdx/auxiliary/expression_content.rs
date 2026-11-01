use crate::prelude::*;
use crate::shared::FormatMdxSnippet;
use biome_formatter::write;
use biome_markdown_syntax::{MdxExpressionContent, MdxExpressionContentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxExpressionContent;
impl FormatNodeRule<MdxExpressionContent> for FormatMdxExpressionContent {
    fn fmt_fields(
        &self,
        node: &MdxExpressionContent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdxExpressionContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        write!(
            f,
            [FormatMdxSnippet {
                token: &value_token
            }]
        )
    }
}

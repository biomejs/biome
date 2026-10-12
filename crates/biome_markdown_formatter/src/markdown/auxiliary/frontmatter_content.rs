use crate::FormatEmbedded;
use crate::prelude::*;
use crate::shared::FormatLiteralLines;
use biome_formatter::write;
use biome_markdown_syntax::{MdFrontmatterContent, MdFrontmatterContentFields};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdFrontmatterContent;
impl FormatNodeRule<MdFrontmatterContent> for FormatMdFrontmatterContent {
    fn fmt_fields(
        &self,
        node: &MdFrontmatterContent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdFrontmatterContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let range = value_token.text_range();
        if !f.context().is_embedded_node_range(range) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // The literal starts with the line break after the opening fence and
        // ends with the line break before the closing fence. The fences must
        // stay on their own lines whether or not the snippet is replaced.
        let lines = FormatLiteralLines {
            token: &value_token,
            max_indent: 0,
        };
        let content = format_replaced(&value_token, &lines);
        let embedded = FormatEmbedded::new(range, &content, f)?;
        write!(f, [hard_line_break(), embedded, hard_line_break()])
    }
}

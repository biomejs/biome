use crate::prelude::*;
use crate::shared::{FormatMdxSnippet, follows_space};
use biome_formatter::write;
use biome_markdown_syntax::{MdxJsxBlockTag, MdxJsxBlockTagFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxJsxBlockTag;
impl FormatNodeRule<MdxJsxBlockTag> for FormatMdxJsxBlockTag {
    fn fmt_fields(&self, node: &MdxJsxBlockTag, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxJsxBlockTagFields {
            indent,
            value_token,
        } = node.as_fields();
        let value_token = value_token?;

        if follows_space(&value_token) {
            write!(f, [space()])?;
        }
        write!(
            f,
            [
                indent.format(),
                FormatMdxSnippet {
                    token: &value_token
                }
            ]
        )
    }
}

use crate::prelude::*;
use crate::shared::{FormatMdxSnippet, follows_construct_with_space};
use biome_formatter::write;
use biome_markdown_syntax::{MdxJsxBlockTag, MdxJsxBlockTagFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxJsxBlockTag;
impl FormatNodeRule<MdxJsxBlockTag> for FormatMdxJsxBlockTag {
    fn fmt_fields(&self, node: &MdxJsxBlockTag, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdxJsxBlockTagFields { value_token } = node.as_fields();
        let value_token = value_token?;

        if follows_construct_with_space(node.syntax(), &value_token) {
            write!(f, [space()])?;
        }
        write!(
            f,
            [FormatMdxSnippet {
                token: &value_token
            }]
        )
    }
}

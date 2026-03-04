use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdHeader, MdHeaderFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHeader;
impl FormatNodeRule<MdHeader> for FormatMdHeader {
    fn fmt_fields(&self, node: &MdHeader, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdHeaderFields {
            before,
            content,
            after,
        } = node.as_fields();

        write!(f, [before.format()])?;

        if let Some(content) = content {
            write!(f, [content.format()])?;
        }

        for hash in after.iter() {
            f.context()
                .comments()
                .mark_suppression_checked(hash.syntax());
            write!(f, [format_removed(&hash.hash_token()?)])?;
        }
        Ok(())
    }
}

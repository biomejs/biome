use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdQuotePrefix, MdQuotePrefixFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuotePrefix;
impl FormatNodeRule<MdQuotePrefix> for FormatMdQuotePrefix {
    fn fmt_fields(&self, node: &MdQuotePrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdQuotePrefixFields {
            marker_token,
            post_marker_space_token,
            pre_marker_indent,
        } = node.as_fields();

        write!(f, [pre_marker_indent.format(), marker_token.format(),])?;

        if let Some(post_marker_space_token) = post_marker_space_token {
            write!(f, [post_marker_space_token.format()])?;
        }

        Ok(())
    }
}

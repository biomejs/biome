use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdQuote, MdQuoteFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuote;
impl FormatNodeRule<MdQuote> for FormatMdQuote {
    fn fmt_fields(&self, node: &MdQuote, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdQuoteFields { content, prefix } = node.as_fields();

        write!(f, [prefix.format(), content.format()])
    }
}

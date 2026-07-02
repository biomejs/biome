use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::markdown::lists::block_list::{
    FormatMdBlockListOptions, QuoteBoundaryTrim, quote_boundary_trim_range,
};
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdQuote, MdQuoteFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuote;
impl FormatNodeRule<MdQuote> for FormatMdQuote {
    fn fmt_fields(&self, node: &MdQuote, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdQuoteFields { content, prefix } = node.as_fields();
        let prefix = prefix?;
        let quote_boundary_trim = if node.syntax().next_sibling().is_none() {
            QuoteBoundaryTrim::LeadingAndTrailing
        } else {
            QuoteBoundaryTrim::Leading
        };
        let trim_range = quote_boundary_trim_range(&content, quote_boundary_trim);
        let starts_with_blank_line = content
            .iter()
            .next()
            .is_some_and(|block| block.is_newline());
        let remove_prefix = starts_with_blank_line && !trim_range.is_empty();

        if remove_prefix {
            write!(
                f,
                [prefix.format().with_options(FormatMdQuotePrefixOptions {
                    should_remove: true,
                })]
            )?;
        } else {
            write!(f, [prefix.format()])?;
        }

        write!(
            f,
            [content.format().with_options(FormatMdBlockListOptions {
                quote_boundary_trim,
            })]
        )
    }
}

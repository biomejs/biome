use crate::context::ProseWrap;
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::markdown::lists::block_list::{
    FormatMdBlockListOptions, QuoteBoundaryTrim, quote_boundary_trim_range,
};
use crate::prelude::*;
use crate::quote::{Quote, should_format_quote_structurally};
use biome_formatter::write;
use biome_markdown_syntax::{MdQuote, MdQuoteFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdQuote;
impl FormatNodeRule<MdQuote> for FormatMdQuote {
    fn fmt_fields(&self, node: &MdQuote, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let prose_wrap = f.options().prose_wrap();
        if prose_wrap == ProseWrap::Preserve && should_format_quote_structurally(node)? {
            return Quote::new(node.clone()).fmt(f);
        }

        let MdQuoteFields { content, prefix } = node.as_fields();
        let prefix = prefix?;
        let quote_boundary_trim = if node.syntax().next_sibling().is_none() {
            QuoteBoundaryTrim::LeadingAndTrailing
        } else {
            QuoteBoundaryTrim::Leading
        };
        let trim_range = quote_boundary_trim_range(&content, quote_boundary_trim);
        let remove_prefix = trim_range.start > 0 && !trim_range.is_empty();

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

        let content = content.format().with_options(FormatMdBlockListOptions {
            quote_boundary_trim,
        });
        if prose_wrap == ProseWrap::Preserve {
            write!(f, [content])
        } else {
            write!(f, [align("> ", &content)])
        }
    }
}

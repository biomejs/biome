use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdInline, MdInlineItemList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItemList;
impl FormatRule<MdInlineItemList> for FormatMdInlineItemList {
    type Context = MdFormatContext;
    fn fmt(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        let mut seen_new_line = false;
        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    if text.is_empty()? && seen_new_line {
                        let entry = format_with(|f| {
                            write!(
                                f,
                                [text.format().with_options(FormatMdTextualOptions {
                                    should_remove: true
                                })]
                            )
                        });
                        joiner.entry(&entry);
                    } else if text.is_newline()? {
                        let entry = format_with(|f| {
                            write!(
                                f,
                                [
                                    text.format().with_options(FormatMdTextualOptions {
                                        should_remove: true
                                    }),
                                    hard_line_break()
                                ]
                            )
                        });
                        seen_new_line = true;
                        joiner.entry(&entry);
                    } else {
                        joiner.entry(&text.format());
                    }
                }
                _ => {
                    joiner.entry(&item.format());
                    seen_new_line = false;
                }
            }
        }

        joiner.finish()
    }
}

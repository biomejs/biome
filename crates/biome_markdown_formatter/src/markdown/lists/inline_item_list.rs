use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{AnyMdInline, MdInlineItemList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItemList {
    trime_start: bool,
}
impl FormatRule<MdInlineItemList> for FormatMdInlineItemList {
    type Context = MdFormatContext;
    fn fmt(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        let mut seen_new_line = false;
        for (index, item) in node.iter().enumerate() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    if text.is_empty()? && seen_new_line {
                        let entry = format_with(|f| {
                            write!(
                                f,
                                [text.format().with_options(FormatMdTextualOptions {
                                    should_remove: true,
                                    trime_start: false
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
                                        should_remove: true,
                                        trime_start: false
                                    }),
                                    hard_line_break()
                                ]
                            )
                        });
                        seen_new_line = true;
                        joiner.entry(&entry);
                    } else {
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            should_remove: false,
                            trime_start: self.trime_start && index == 0,
                        }));
                    }
                }

                AnyMdInline::MdHardLine(hard_line) => {
                    seen_new_line = true;
                    joiner.entry(&format_with(|f| write!(f, [hard_line.format()])));
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

pub(crate) struct FormatMdFormatInlineItemListOptions {
    pub(crate) trime_start: bool,
}

impl FormatRuleWithOptions<MdInlineItemList> for FormatMdInlineItemList {
    type Options = FormatMdFormatInlineItemListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.trime_start = options.trime_start;
        self
    }
}

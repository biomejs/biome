use crate::markdown::auxiliary::hard_line::FormatMdFormatHardLineOptions;
use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{AnyMdInline, MdInlineItemList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItemList {
    print_mode: TextPrintMode,
}

impl FormatRule<MdInlineItemList> for FormatMdInlineItemList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if self.print_mode.is_auto_link_like() {
            return self.fmt_auto_link_like(node, f);
        } else if self.print_mode.is_normalize_words() {
            return self.fmt_normalize_words(node, f);
        } else if self.print_mode.is_all() {
            return self.fmt_trim_all(node, f);
        } else if self.print_mode.is_pristine() {
            return self.fmt_pristine(node, f);
        } else if self.print_mode.is_clean() {
            return self.fmt_clean(node, f);
        }

        let mut joiner = f.join();

        let mut seen_new_line = false;
        for (index, item) in node.iter().enumerate() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    if text.is_empty_and_not_newline()? && seen_new_line {
                        let entry = format_with(|f| {
                            write!(
                                f,
                                [text.format().with_options(FormatMdTextualOptions {
                                    should_remove: true,
                                    trim_start: false,
                                    ..Default::default()
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
                                        trim_start: false,
                                        ..Default::default()
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
                            trim_start: self.print_mode.is_start() && index == 0,
                            ..Default::default()
                        }));
                    }
                }

                AnyMdInline::MdHardLine(hard_line) => {
                    seen_new_line = true;
                    joiner.entry(&format_with(|f| {
                        write!(
                            f,
                            [hard_line
                                .format()
                                .with_options(FormatMdFormatHardLineOptions {
                                    print_mode: self.print_mode,
                                })]
                        )
                    }));
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

impl FormatMdInlineItemList {
    /// If the first and last [MdTextual] are `<` and `>` respectively,
    /// they are removed. Otherwise falls back to [TrimMode::All].
    fn fmt_auto_link_like(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let items: Vec<_> = node.iter().collect();

        let starts_with_lt = matches!(items.first(), Some(AnyMdInline::MdTextual(t)) if t.value_token().is_ok_and(|tok| tok.text() == "<"));
        let ends_with_gt = matches!(items.last(), Some(AnyMdInline::MdTextual(t)) if t.value_token().is_ok_and(|tok| tok.text() == ">"));

        let is_auto_link = starts_with_lt && ends_with_gt && items.len() > 2;

        if !is_auto_link {
            return self.fmt_trim_all(node, f);
        }

        let mut joiner = f.join();
        for (index, item) in items.iter().enumerate() {
            if (index == 0 || index == items.len() - 1)
                && let AnyMdInline::MdTextual(text) = item
            {
                joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                    should_remove: true,
                    ..Default::default()
                }));
                continue;
            }
            joiner.entry(&item.format());
        }
        joiner.finish()
    }

    /// Strips leading and trailing whitespace/hard-lines around the content.
    /// Items between the first and last non-empty nodes are kept as-is;
    /// items outside those boundaries are removed.
    fn fmt_trim_all(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let items: Vec<_> = node.iter().collect();
        let mut joiner = f.join();

        let is_content = |item: &AnyMdInline| match item {
            AnyMdInline::MdTextual(text) => !text.is_empty().unwrap_or_default(),
            AnyMdInline::MdHardLine(_) => false,
            _ => true,
        };

        // Find the first non-empty item from the left.
        let first_content = items.iter().position(&is_content);

        // Find the first non-empty item from the right.
        let last_content = items
            .iter()
            .rev()
            .position(is_content)
            .map(|pos| items.len() - 1 - pos);

        for (index, item) in items.iter().enumerate() {
            let is_before_content = first_content.is_none_or(|first| index < first);
            let is_after_content = last_content.is_none_or(|last| index > last);

            if is_before_content || is_after_content {
                // Outside content boundaries: remove empty nodes.
                match item {
                    AnyMdInline::MdTextual(text) => {
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            should_remove: true,
                            trim_start: true,
                            ..Default::default()
                        }));
                    }
                    AnyMdInline::MdHardLine(hard_line) => {
                        joiner.entry(&hard_line.format().with_options(
                            FormatMdFormatHardLineOptions {
                                print_mode: TextPrintMode::Trim(TrimMode::All),
                            },
                        ));
                    }
                    _ => {
                        joiner.entry(&item.format());
                    }
                }
            } else {
                // Inside content boundaries: keep as-is.
                joiner.entry(&item.format());
            }
        }

        joiner.finish()
    }

    /// Normalizes all whitespace in textual nodes to `hard_space`.
    ///
    /// For example, `[  Foo   Bar  ]` becomes `[ Foo Bar ]`.
    fn fmt_normalize_words(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let mut joiner = f.join();

        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::NormalizeWords),
                        ..Default::default()
                    }));
                }
                _ => {
                    joiner.entry(&item.format());
                }
            }
        }

        joiner.finish()
    }

    /// Clean mode: formats content verbatim, but removes the first
    /// whitespace-only textual token.
    fn fmt_clean(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();
        let mut handled_first = false;

        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(text) if !handled_first => {
                    handled_first = true;
                    if text.is_empty_and_not_newline().unwrap_or(false)
                        || text.is_newline().unwrap_or(false)
                    {
                        // First token is trailing whitespace/newline from the
                        // info string line — remove it entirely.
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            should_remove: true,
                            ..Default::default()
                        }));
                    } else {
                        let token = text.value_token()?;
                        let token_text = token.text();
                        if token_text.trim().is_empty() {
                            // Mixed whitespace + newline (e.g. "    \n") — remove.
                            joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                                should_remove: true,
                                ..Default::default()
                            }));
                        } else {
                            // First token has content — keep as-is.
                            joiner.entry(&text.format());
                        }
                    }
                }
                AnyMdInline::MdHardLine(hd) => {
                    joiner.entry(&hd.format().with_options(FormatMdFormatHardLineOptions {
                        print_mode: TextPrintMode::Pristine,
                    }));
                }
                node => {
                    joiner.entry(&node.format());
                }
            }
        }

        joiner.finish()
    }

    /// Formats all items verbatim, preserving the original text exactly.
    /// Hard lines are explicitly set to pristine mode to prevent
    /// normalization (e.g. collapsing multiple trailing spaces to two).
    fn fmt_pristine(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        for item in node.iter() {
            match item {
                AnyMdInline::MdHardLine(hd) => {
                    joiner.entry(&hd.format().with_options(FormatMdFormatHardLineOptions {
                        print_mode: TextPrintMode::Pristine,
                    }));
                }
                node => {
                    joiner.entry(&node.format());
                }
            }
        }

        joiner.finish()
    }
}

pub(crate) struct FormatMdFormatInlineItemListOptions {
    pub(crate) print_mode: TextPrintMode,
}

impl FormatRuleWithOptions<MdInlineItemList> for FormatMdInlineItemList {
    type Options = FormatMdFormatInlineItemListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.print_mode = options.print_mode;
        self
    }
}

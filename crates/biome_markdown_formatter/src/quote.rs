use crate::bullet_list::FmtAnyList;
use crate::markdown::auxiliary::hard_line::FormatMdFormatHardLineOptions;
use crate::markdown::auxiliary::inline_italic::FormatMdInlineItalicOptions;
use crate::markdown::auxiliary::paragraph::FormatMdParagraphOptions;
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::markdown::lists::block_list::{QuoteBoundaryTrim, quote_boundary_trim_range};
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode, format_removed_quote_boundary};
use crate::{AsFormat, MarkdownFormatContext, MarkdownFormatter};
use biome_formatter::{Format, FormatResult, format_args, write};
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdContainerBlock, AnyMdInline, AnyMdLeafBlock, MarkdownSyntaxNode, MdBlockList,
    MdBullet, MdParagraph, MdQuote, MdQuoteFields,
};
use biome_rowan::AstNode;
use biome_rowan::TextSize;
use std::ops::Add;

pub(crate) struct Quote {
    node: MdQuote,
}

impl Quote {
    pub(crate) fn new(node: MdQuote) -> Self {
        Self { node }
    }
}

pub(crate) fn should_format_quote_structurally(node: &MdQuote) -> FormatResult<bool> {
    block_list_has_quote_continuation(&node.content())
}

impl Format<MarkdownFormatContext> for Quote {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdQuoteFields { content, prefix } = self.node.as_fields();
        let prefix = prefix?;
        let quote_boundary_trim = if self.node.syntax().next_sibling().is_none() {
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
            f.context().comments().is_suppressed(prefix.syntax());

            let fields = prefix.as_fields();
            for indent in fields.pre_marker_indent.iter() {
                f.context().comments().is_suppressed(indent.syntax());
                write!(
                    f,
                    [format_removed(&indent.md_quote_pre_marker_indent_token()?)]
                )?;
            }

            let marker = fields.marker_token?;
            write!(f, [marker.format()])?;

            if let Some(post_marker_space_token) = fields.post_marker_space_token {
                write!(f, [post_marker_space_token.format()])?;
            } else {
                let next_has_text = marker.next_token().is_some_and(|token| {
                    token.text().starts_with(|char: char| !char.is_whitespace())
                });
                if next_has_text {
                    write!(f, [space()])?;
                }
            }
        }

        let content = QuoteBlockList {
            content,
            quote_boundary_trim,
        };

        write!(f, [align("  ", &content)])
    }
}

struct QuoteBlockList {
    content: MdBlockList,
    quote_boundary_trim: QuoteBoundaryTrim,
}

impl Format<MarkdownFormatContext> for QuoteBlockList {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        // We custom format the content, so mwe mark it as suppressed
        f.context().comments().is_suppressed(self.content.syntax());

        let quote_trim_range = quote_boundary_trim_range(&self.content, self.quote_boundary_trim);
        let mut prev_content = PrevContentBlock::None;
        let mut iter = self.content.iter().enumerate().peekable();
        let mut joiner = f.join();

        while let Some((index, block)) = iter.next() {
            if !quote_trim_range.contains(&index) {
                joiner.entry(&format_with(|f| format_removed_quote_boundary(&block, f)));
                continue;
            }

            match &block {
                AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                    prev_content = PrevContentBlock::Paragraph;
                    if should_format_quote_paragraph(paragraph)? {
                        joiner.entry(&QuoteParagraph { paragraph });
                    } else {
                        joiner.entry(&paragraph.format().with_options(FormatMdParagraphOptions {
                            trim_mode: TextPrintMode::Pristine,
                            text_context: TextContext::Neutral,
                        }));
                    }
                }

                block if block.is_any_header() => {
                    prev_content = PrevContentBlock::Header;
                    joiner.entry(&block.format());
                }

                block if block.is_newline() => {
                    if prev_content == PrevContentBlock::Header
                        && iter.peek().is_some_and(|(_, next)| !next.is_newline())
                    {
                        let line_prefix = quote_line_prefix(self.content.syntax())?;
                        let mut removed_blocks = Vec::new();

                        if matches!(iter.peek(), Some((_, AnyMdBlock::MdQuotePrefix(_))))
                            && let Some((_, prefix)) = iter.next()
                        {
                            removed_blocks.push(prefix);
                        }

                        if matches!(iter.peek(), Some((_, next)) if next.is_newline()) {
                            if let Some((_, newline)) = iter.next() {
                                removed_blocks.push(newline);
                            }

                            if matches!(iter.peek(), Some((_, AnyMdBlock::MdQuotePrefix(_))))
                                && let Some((_, prefix)) = iter.next()
                            {
                                removed_blocks.push(prefix);
                            }
                        }

                        let entry = format_with(move |f| {
                            format_removed_quote_boundary(block, f)?;
                            for block in &removed_blocks {
                                format_removed_quote_boundary(block, f)?;
                            }
                            write!(
                                f,
                                [dedent_to_root(&format_args![
                                    hard_line_break(),
                                    line_prefix.format(false),
                                    hard_line_break(),
                                    line_prefix.format(true)
                                ])]
                            )
                        });
                        joiner.entry(&entry);
                    } else {
                        joiner.entry(&block.format());
                    }
                    prev_content = PrevContentBlock::Other;
                }

                AnyMdBlock::MdQuotePrefix(prefix)
                    if prev_content == PrevContentBlock::Paragraph
                        && iter.peek().is_some_and(|(_, next)| next.is_fenced_block()) =>
                {
                    prev_content = PrevContentBlock::Other;
                    let line_prefix = quote_line_prefix(self.content.syntax())?;
                    let prefix = prefix.clone();
                    let entry = format_with(move |f| {
                        write!(
                            f,
                            [
                                prefix.format().with_options(FormatMdQuotePrefixOptions {
                                    should_remove: true,
                                }),
                                dedent_to_root(&format_args![
                                    hard_line_break(),
                                    line_prefix.format(true)
                                ])
                            ]
                        )
                    });
                    joiner.entry(&entry);
                }

                AnyMdBlock::MdQuotePrefix(prefix) => {
                    joiner.entry(&prefix.format());
                }

                _ => {
                    prev_content = PrevContentBlock::Other;
                    if let Some(list_item) = block.as_any_list_item() {
                        joiner.entry(&format_with(|f| FmtAnyList::new(list_item.clone()).fmt(f)));
                    } else {
                        joiner.entry(&block.format());
                    }
                }
            }
        }

        joiner.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PrevContentBlock {
    None,
    Header,
    Paragraph,
    Other,
}

struct QuoteParagraph<'a> {
    paragraph: &'a MdParagraph,
}

impl<'a> Format<MarkdownFormatContext> for QuoteParagraph<'a> {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        // Some items won't be formatted, so we mark them as suppressed
        f.context()
            .comments()
            .is_suppressed(self.paragraph.syntax());
        let line_prefix = quote_line_prefix(self.paragraph.syntax())?;
        let mut joiner = f.join();
        let mut after_quote_continuation_newline = false;
        let mut after_removed_continuation_prefix = false;

        for (index, item) in self.paragraph.list().iter().enumerate() {
            match item {
                AnyMdInline::MdTextual(textual) => {
                    if after_removed_continuation_prefix
                        && textual.value_token()?.text().starts_with(' ')
                    {
                        let token = textual.value_token()?;
                        let trimmed = &token.text()[1..];
                        joiner.entry(&format_with(|f: &mut MarkdownFormatter| {
                            f.context()
                                .comments()
                                .mark_suppression_checked(textual.syntax());
                            write!(
                                f,
                                [format_replaced(
                                    &token,
                                    &text(
                                        trimmed,
                                        Some(token.text_range().start().add(TextSize::from(1)))
                                    )
                                )]
                            )
                        }));
                        after_quote_continuation_newline = false;
                        after_removed_continuation_prefix = false;
                    } else if textual.is_newline()?
                        && should_format_quote_continuation_after_newline(
                            self.paragraph,
                            index + 1,
                        )?
                    {
                        let line_prefix = line_prefix.clone();
                        joiner.entry(&format_with(|f| {
                            write!(
                                f,
                                [
                                    textual.format().with_options(FormatMdTextualOptions {
                                        print_mode: TextPrintMode::Remove,
                                        ..FormatMdTextualOptions::default()
                                    }),
                                    dedent_to_root(&format_args![
                                        hard_line_break(),
                                        line_prefix.format(true)
                                    ])
                                ]
                            )
                        }));
                        after_quote_continuation_newline = true;
                        after_removed_continuation_prefix = false;
                    } else {
                        joiner.entry(&textual.format());
                        after_quote_continuation_newline = false;
                        after_removed_continuation_prefix = false;
                    }
                }
                AnyMdInline::MdQuotePrefix(prefix) if after_quote_continuation_newline => {
                    joiner.entry(&prefix.format().with_options(FormatMdQuotePrefixOptions {
                        should_remove: true,
                    }));
                    after_removed_continuation_prefix = true;
                }
                AnyMdInline::MdHardLine(hard_line) => {
                    joiner.entry(
                        &hard_line
                            .format()
                            .with_options(FormatMdFormatHardLineOptions {
                                print_mode: TextPrintMode::Pristine,
                            }),
                    );
                    after_quote_continuation_newline = false;
                    after_removed_continuation_prefix = false;
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: false,
                    }));
                    after_quote_continuation_newline = false;
                    after_removed_continuation_prefix = false;
                }
                item => {
                    joiner.entry(&item.format());
                    after_quote_continuation_newline = false;
                    after_removed_continuation_prefix = false;
                }
            }
        }

        joiner.finish()
    }
}

#[derive(Clone)]
pub(crate) struct QuoteLinePrefix {
    parts: Vec<QuoteLinePrefixPart>,
}

#[derive(Clone)]
enum QuoteLinePrefixPart {
    Quote,
    ListAlign(usize),
}

impl QuoteLinePrefix {
    pub(crate) fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub(crate) fn format(&self, trailing_space: bool) -> impl Format<MarkdownFormatContext> + '_ {
        format_with(move |f| {
            let last_quote_index = self
                .parts
                .iter()
                .rposition(|part| matches!(part, QuoteLinePrefixPart::Quote));

            for (index, part) in self.parts.iter().enumerate() {
                match part {
                    QuoteLinePrefixPart::Quote => {
                        write!(f, [token(">")])?;
                        if trailing_space || Some(index) != last_quote_index {
                            write!(f, [space()])?;
                        }
                    }
                    QuoteLinePrefixPart::ListAlign(width) => {
                        for _ in 0..*width {
                            write!(f, [token(" ")])?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}

pub(crate) fn quote_line_prefix(syntax: &MarkdownSyntaxNode) -> FormatResult<QuoteLinePrefix> {
    let mut parts = Vec::new();
    let mut ancestors: Vec<_> = syntax.ancestors().skip(1).collect();
    ancestors.reverse();

    for ancestor in ancestors {
        if MdQuote::can_cast(ancestor.kind()) {
            parts.push(QuoteLinePrefixPart::Quote);
        } else if let Some(bullet) = MdBullet::cast(ancestor) {
            parts.push(QuoteLinePrefixPart::ListAlign(list_marker_alignment(
                &bullet,
            )?));
        }
    }

    Ok(QuoteLinePrefix { parts })
}

fn list_marker_alignment(bullet: &MdBullet) -> FormatResult<usize> {
    let prefix = bullet.as_fields().prefix?;
    let marker = prefix.marker()?;
    Ok(prefix.pre_marker_indent().len()
        + marker.text_trimmed().len()
        + prefix.post_marker_len().unwrap_or(2))
}

fn should_format_quote_paragraph(paragraph: &MdParagraph) -> FormatResult<bool> {
    for (index, item) in paragraph.list().iter().enumerate() {
        if let AnyMdInline::MdTextual(text) = item
            && text.is_newline()?
            && should_format_quote_continuation_after_newline(paragraph, index + 1)?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn block_list_has_quote_continuation(content: &MdBlockList) -> FormatResult<bool> {
    for block in content.iter() {
        match &block {
            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(paragraph)) => {
                if should_format_quote_paragraph(paragraph)? {
                    return Ok(true);
                }
            }
            AnyMdBlock::AnyMdContainerBlock(AnyMdContainerBlock::MdQuote(quote)) => {
                if should_format_quote_structurally(quote)? {
                    return Ok(true);
                }
            }
            _ => {
                if let Some(list_item) = block.as_any_list_item() {
                    for bullet in list_item.list().iter() {
                        if block_list_has_quote_continuation(&bullet.content())? {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }

    Ok(false)
}

fn should_format_quote_continuation_after_newline(
    paragraph: &MdParagraph,
    start: usize,
) -> FormatResult<bool> {
    for item in paragraph.list().iter().skip(start) {
        match item {
            AnyMdInline::MdQuotePrefix(_) => {}
            AnyMdInline::MdIndentToken(_) => return Ok(false),
            AnyMdInline::MdTextual(text) => {
                if text.is_newline()? {
                    return Ok(false);
                }

                if !text.is_empty()? {
                    return Ok(true);
                }
            }
            AnyMdInline::MdHardLine(_) => return Ok(false),
            _ => return Ok(true),
        }
    }

    Ok(false)
}

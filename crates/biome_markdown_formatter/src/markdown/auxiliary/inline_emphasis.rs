use crate::context::ProseWrap;
use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::{Format, write};
use biome_markdown_syntax::{
    AnyMdInline, MdBlockList, MdInlineEmphasis, MdInlineEmphasisFields, MdInlineItemList,
    MdParagraph, MdReferenceImage, MdReferenceLink, emphasis_ext::MdEmphasisFence,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineEmphasis;

impl FormatNodeRule<MdInlineEmphasis> for FormatMdInlineEmphasis {
    fn fmt_fields(&self, node: &MdInlineEmphasis, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineEmphasisFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        let l_fence = l_fence?;
        let r_fence = r_fence?;

        let mut content_items = content.iter();
        let single_textual = match content_items.next() {
            Some(AnyMdInline::MdTextual(textual)) if content_items.next().is_none() => {
                Some(textual)
            }
            _ => None,
        };
        let should_escape_star_content = if let Some(textual) = &single_textual {
            textual.value_token()?.text() == "*"
        } else {
            false
        };
        let content = format_with(|f| {
            if should_escape_star_content && let Some(textual) = &single_textual {
                textual
                    .format()
                    .with_options(FormatMdTextualOptions {
                        should_escape: true,
                        ..FormatMdTextualOptions::default()
                    })
                    .fmt(f)
            } else {
                FormatEmphasisContent::new(&content).fmt(f)
            }
        });

        if node.fence()? == MdEmphasisFence::DoubleStar {
            write!(f, [l_fence.format(), content, r_fence.format()])
        } else {
            write!(
                f,
                [
                    format_replaced(
                        &l_fence,
                        &text(
                            MdEmphasisFence::DoubleStar.as_str(),
                            Some(l_fence.text_range().start())
                        )
                    ),
                    content,
                    format_replaced(
                        &r_fence,
                        &text(
                            MdEmphasisFence::DoubleStar.as_str(),
                            Some(r_fence.text_range().start())
                        )
                    ),
                ]
            )
        }
    }
}

/// Reflows strong-emphasis content in wrapping paragraphs without adding a block-ending break.
pub(crate) struct FormatEmphasisContent<'a> {
    content: &'a MdInlineItemList,
    options: FormatMdFormatInlineItemListOptions,
}

impl<'a> FormatEmphasisContent<'a> {
    pub(crate) fn new(content: &'a MdInlineItemList) -> Self {
        Self::new_with_options(content, FormatMdFormatInlineItemListOptions::default())
    }

    pub(crate) fn new_with_options(
        content: &'a MdInlineItemList,
        options: FormatMdFormatInlineItemListOptions,
    ) -> Self {
        Self { content, options }
    }
}

impl Format<MarkdownFormatContext> for FormatEmphasisContent<'_> {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut ancestors = self.content.syntax().ancestors().take_while(|ancestor| {
            !MdReferenceImage::can_cast(ancestor.kind())
                && !MdReferenceLink::can_cast(ancestor.kind())
        });
        let reflow_prose = f.options().prose_wrap() == ProseWrap::Always
            && ancestors
                .by_ref()
                .any(|ancestor| MdInlineEmphasis::can_cast(ancestor.kind()))
            && ancestors
                .find_map(MdParagraph::cast)
                .and_then(|paragraph| paragraph.syntax().parent())
                .is_some_and(|parent| MdBlockList::can_cast(parent.kind()));

        if reflow_prose {
            self.content
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: TextPrintMode::fill(),
                    text_context: TextContext::Inline,
                    ..self.options
                })
                .fmt(f)
        } else {
            self.content.format().with_options(self.options).fmt(f)
        }
    }
}

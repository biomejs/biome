use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MdInlineEmphasis, MdInlineEmphasisFields, emphasis_ext::MdEmphasisFence,
};

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
                content.format().fmt(f)
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

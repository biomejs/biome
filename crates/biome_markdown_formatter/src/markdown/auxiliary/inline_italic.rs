use crate::{markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions, prelude::*};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{
    MarkdownSyntaxKind, MdInlineItalic, MdInlineItalicFields, MdReferenceImage, MdReferenceLink,
};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItalic {
    should_keep_fences: bool,
}
impl FormatNodeRule<MdInlineItalic> for FormatMdInlineItalic {
    fn fmt_fields(&self, node: &MdInlineItalic, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineItalicFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        let l_fence = l_fence?;
        let r_fence = r_fence?;

        if node
            .syntax()
            .descendants()
            .skip(1)
            .any(|d| d.kind() == MarkdownSyntaxKind::MD_INLINE_ITALIC)
        {
            return write!(
                f,
                [
                    l_fence.format(),
                    content
                        .format()
                        .with_options(FormatMdFormatInlineItemListOptions {
                            keep_fences_in_italics: true,
                            ..Default::default()
                        }),
                    r_fence.format()
                ]
            );
        }

        // Inside reference images the alt text doubles as the reference label.
        // Normalizing `*` → `_` would change the label and break reference resolution.
        // E.g. `![foo *bar*]` with `[foo *bar*]: url` must keep `*`.
        if node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|a| MdReferenceImage::can_cast(a.kind()))
            | self.should_keep_fences
        {
            return write!(f, [l_fence.format(), content.format(), r_fence.format()]);
        }

        let prev_is_alphanum = l_fence
            .prev_token()
            .and_then(|t| t.text_trimmed().chars().last())
            .is_some_and(|c| c.is_alphanumeric());
        let next_is_alphanum = r_fence
            .next_token()
            .and_then(|t| t.text_trimmed().chars().next())
            .is_some_and(|c| c.is_alphanumeric());

        // See https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis
        // Prefer `_` but use `*` when adjacent to alphanumeric,
        // For example, `a_b_c` won't parse `b` as italic, but `a*b*c` will).
        let target_kind = if prev_is_alphanum || next_is_alphanum {
            MarkdownSyntaxKind::STAR
        } else {
            MarkdownSyntaxKind::UNDERSCORE
        };

        write_fence(&l_fence, target_kind, f)?;
        write!(f, [content.format()])?;
        write_fence(&r_fence, target_kind, f)
    }
}

/// Write a fence token, reusing it if it already matches the target kind.
fn write_fence(
    fence: &biome_markdown_syntax::MarkdownSyntaxToken,
    target_kind: MarkdownSyntaxKind,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
    if fence.kind() == target_kind {
        write!(f, [fence.format()])
    } else {
        let text = if target_kind == MarkdownSyntaxKind::STAR {
            "*"
        } else {
            "_"
        };
        write!(f, [format_replaced(fence, &token(text))])
    }
}

pub(crate) struct FormatMdInlineItalicOptions {
    pub(crate) should_keep_fences: bool,
}

impl FormatRuleWithOptions<MdInlineItalic> for FormatMdInlineItalic {
    type Options = FormatMdInlineItalicOptions;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_keep_fences = options.should_keep_fences;
        self
    }
}

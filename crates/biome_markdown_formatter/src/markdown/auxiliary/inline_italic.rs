use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    MarkdownSyntaxKind, MdInlineItalic, MdInlineItalicFields, MdReferenceImage,
};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItalic;
impl FormatNodeRule<MdInlineItalic> for FormatMdInlineItalic {
    fn fmt_fields(&self, node: &MdInlineItalic, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineItalicFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        let l_fence = l_fence?;
        let r_fence = r_fence?;

        // Nested italic anywhere in the subtree → keep entire node verbatim.
        // Normalizing to `_` could create `__` (bold) adjacency: `_*foo*_` → `__foo__`.
        if node
            .syntax()
            .descendants()
            .skip(1)
            .any(|d| d.kind() == MarkdownSyntaxKind::MD_INLINE_ITALIC)
        {
            // TODO: instead of format_verbatim_node, pass options to child formatters so
            // other normalizations (bold, code, etc.) still run inside nested italic content.
            // See example-383.md for a case where Prettier handles escapes inside italic.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // Inside reference images the alt text doubles as the reference label.
        // Normalizing `*` → `_` would change the label and break reference resolution.
        // E.g. `![foo *bar*]` with `[foo *bar*]: url` must keep `*`.
        if node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|a| MdReferenceImage::can_cast(a.kind()))
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

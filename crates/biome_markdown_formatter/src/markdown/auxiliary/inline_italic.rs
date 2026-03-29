use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MdInlineEmphasis, MdInlineEmphasisFields, MdInlineItalic, MdInlineItalicFields,
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

        // If the content subtree contains any nested italic node, keep `*` verbatim.
        // Direct children that are italic would create `__` (= bold) adjacency when
        // normalized to `_`. Deeper descendants (e.g. italic inside a link) also trigger
        // this guard because Prettier keeps `*` to avoid visual ambiguity.
        let items: Vec<_> = content.iter().collect();
        let has_nested_italic = node.syntax().descendants().any(|d| {
            use biome_markdown_syntax::MarkdownSyntaxKind;
            d.kind() == MarkdownSyntaxKind::MD_INLINE_ITALIC && d != *node.syntax()
        });
        if has_nested_italic {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // `_` has stricter CommonMark flanking rules than `*`.
        let prev_is_alphanum = l_fence
            .prev_token()
            .and_then(|t| t.text_trimmed().chars().last())
            .is_some_and(|c| c.is_alphanumeric());
        let next_is_alphanum = r_fence
            .next_token()
            .and_then(|t| t.text_trimmed().chars().next())
            .is_some_and(|c| c.is_alphanumeric());

        // When the source uses `*` fences (e.g. `***x***`) and the content is a single
        // bold node, Prettier prefers **_x_** over _**x**_ (same HTML, different nesting).
        // Only rewrite for `*` fences so `_____x_____` is left untouched.
        if l_fence.text_trimmed() == "*"
            && !prev_is_alphanum
            && !next_is_alphanum
            && items.len() == 1
            && let Some(AnyMdInline::MdInlineEmphasis(emphasis)) = items.first()
        {
            return fmt_italic_wrapping_emphasis(&l_fence, emphasis, &r_fence, f);
        }

        if l_fence.text_trimmed() == "*" {
            // `*` fence: if adjacent to alphanum, `_` would be invalid. Keep `*`.
            if prev_is_alphanum || next_is_alphanum {
                return write!(
                    f,
                    [
                        format_replaced(&l_fence, &token("*")),
                        content.format(),
                        format_replaced(&r_fence, &token("*")),
                    ]
                );
            }
        } else {
            // `_` fence: check for CommonMark violations.
            // `_` cannot open if preceded by alphanumeric → convert to `*`.
            if prev_is_alphanum {
                return write!(
                    f,
                    [
                        format_replaced(&l_fence, &token("*")),
                        content.format(),
                        format_replaced(&r_fence, &token("*")),
                    ]
                );
            }
            // `_` cannot close if followed by alphanumeric → escape the opening `_`.
            if next_is_alphanum {
                return write!(
                    f,
                    [
                        format_replaced(&l_fence, &token("\\_")),
                        content.format(),
                        format_replaced(&r_fence, &token("_")),
                    ]
                );
            }
        }

        write!(
            f,
            [
                format_replaced(&l_fence, &token("_")),
                content.format(),
                format_replaced(&r_fence, &token("_")),
            ]
        )
    }
}

/// Format an italic node whose content is a single bold (emphasis) node as `**_x_**`
/// rather than `_**x**_`. Both render identically but Prettier prefers the former.
fn fmt_italic_wrapping_emphasis(
    outer_l: &biome_markdown_syntax::MarkdownSyntaxToken,
    emphasis: &MdInlineEmphasis,
    outer_r: &biome_markdown_syntax::MarkdownSyntaxToken,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
    let MdInlineEmphasisFields {
        l_fence: em_l,
        content: em_content,
        r_fence: em_r,
    } = emphasis.as_fields();
    // Mark the emphasis node as suppression-checked since we format it manually.
    f.context()
        .comments()
        .mark_suppression_checked(emphasis.syntax());
    write!(
        f,
        [
            format_replaced(outer_l, &token("**")),
            format_replaced(&em_l?, &token("_")),
            em_content.format(),
            format_replaced(&em_r?, &token("_")),
            format_replaced(outer_r, &token("**")),
        ]
    )
}

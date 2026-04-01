use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdInline, MarkdownSyntaxKind, MdInlineEmphasis, MdInlineEmphasisFields, MdInlineItalic,
    MdInlineItalicFields,
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

        // If descendant is also `_`, normalizing to `_` would create `__` (bold)
        // So we keep verbatim.
        let has_nested_italic = node
            .syntax()
            .descendants()
            .any(|d| d.kind() == MarkdownSyntaxKind::MD_INLINE_ITALIC && d != *node.syntax());
        if has_nested_italic {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let prev_is_alphanum = l_fence
            .prev_token()
            .and_then(|t| t.text_trimmed().chars().last())
            .is_some_and(|c| c.is_alphanumeric());
        let next_is_alphanum = r_fence
            .next_token()
            .and_then(|t| t.text_trimmed().chars().next())
            .is_some_and(|c| c.is_alphanumeric());

        // `***x***` → `**_x_**`: Prettier prefers bold-wrapping-italic.
        let items: Vec<_> = content.iter().collect();
        if l_fence.text_trimmed() == "*"
            && !prev_is_alphanum
            && !next_is_alphanum
            && items.len() == 1
            && let Some(AnyMdInline::MdInlineEmphasis(emphasis)) = items.first()
        {
            return fmt_italic_wrapping_emphasis(&l_fence, emphasis, &r_fence, f);
        }

        // See https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis
        // By default, we prefer `_` but fall back to "*"
        // when CommonMark flanking rules make `_` invalid:
        let (open, close) = match (prev_is_alphanum, next_is_alphanum) {
            // `a*b*c`: Can't use `_` because `a_` won't open emphasis
            (true, _) => ("*", "*"),
            // `_b_2` where source is `_`: we can't't switch to `*` (which would change
            // intended semantic), so we escape the opener: `\_b_2`
            (false, true) if l_fence.text_trimmed() == "_" => ("\\_", "_"),
            // e.g. `*b*2` — can't use `_` because `_2` won't close emphasis
            (false, true) => ("*", "*"),
            // e.g. `!*b*!` — no adjacent alphanumeric, safe to normalize to `_`
            (false, false) => ("_", "_"),
        };

        write!(
            f,
            [
                format_replaced(&l_fence, &token(open)),
                content.format(),
                format_replaced(&r_fence, &token(close)),
            ]
        )
    }
}

/// Format `*__x__*` (italic wrapping bold) as `**_x_**` (bold wrapping italic).
/// Both render identically but Prettier prefers the latter nesting.
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

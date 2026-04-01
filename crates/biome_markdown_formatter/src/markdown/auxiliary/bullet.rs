use crate::markdown::auxiliary::list_marker_prefix::FormatMdListMarkerPrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdLeafBlock, MarkdownSyntaxKind, MdBullet, MdBulletFields,
};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBullet;
impl FormatNodeRule<MdBullet> for FormatMdBullet {
    fn fmt_fields(&self, node: &MdBullet, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdBulletFields { prefix, content } = node.as_fields();

        let prefix = prefix.map_err(|_| FormatError::SyntaxError)?;
        let marker = prefix.marker().map_err(|_| FormatError::SyntaxError)?;
        let marker_kind = marker.kind();

        // `* - - -` is a bullet containing a thematic break. Normalizing `*`
        // to `-` produces `- - - -` which, in CommonMark, means a thematic break
        // which breaks the semantic. Same for `+ - - -`.
        // So we don't want to normalize it into `-`.
        if marker_kind != MarkdownSyntaxKind::MINUS && first_block_is_dash_thematic_break(&content)
        {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let target_marker = target_marker_for_bullet(node);
        write!(
            f,
            [prefix
                .format()
                .with_options(FormatMdListMarkerPrefixOptions { target_marker })]
        )?;
        for block in content.iter() {
            block.format().fmt(f)?;
        }
        Ok(())
    }
}

/// This algorithm is based on the fact that CommonMark treats list with
/// different markers as different groups.
/// See https://spec.commonmark.org/0.31.2/#lists
/// Instead of normlalizing everything to `-`, this function walks up the tree
/// and see whether its siblings are also MD_BULLET_LIST_ITEM.
/// If so, it alternates between `-` and `*` to preserve list
/// separation.
/// The corresponding test for this is separate.md.
fn target_marker_for_bullet(node: &MdBullet) -> &'static str {
    use biome_markdown_syntax::MarkdownSyntaxKind;

    let item = node
        .syntax()
        .parent()
        .and_then(|n| n.parent())
        .filter(|n| n.kind() == MarkdownSyntaxKind::MD_BULLET_LIST_ITEM);
    let Some(item) = item else { return "-" };

    let count = std::iter::successors(item.prev_sibling(), |s| s.prev_sibling())
        .take_while(|s| {
            matches!(
                s.kind(),
                MarkdownSyntaxKind::MD_NEWLINE | MarkdownSyntaxKind::MD_BULLET_LIST_ITEM
            )
        })
        .filter(|s| s.kind() == MarkdownSyntaxKind::MD_BULLET_LIST_ITEM)
        .count();

    if count % 2 == 0 { "-" } else { "*" }
}

/// Returns true if the first block in `content` is a thematic break using `-`.
fn first_block_is_dash_thematic_break(content: &biome_markdown_syntax::MdBlockList) -> bool {
    let Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdThematicBreakBlock(tb))) =
        content.iter().next()
    else {
        return false;
    };
    tb.parts()
        .into_iter()
        .find_map(|p| p.as_md_thematic_break_char().cloned())
        .and_then(|c| c.value().ok())
        .is_some_and(|t| t.text_trimmed() == "-")
}

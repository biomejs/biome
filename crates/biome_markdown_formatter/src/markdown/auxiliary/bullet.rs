use crate::markdown::auxiliary::list_marker_prefix::FormatMdListMarkerPrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdBlock, AnyMdLeafBlock, MdBullet, MdBulletFields};
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBullet;
impl FormatNodeRule<MdBullet> for FormatMdBullet {
    fn fmt_fields(&self, node: &MdBullet, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdBulletFields { prefix, content } = node.as_fields();

        let prefix = match prefix {
            Ok(p) => p,
            Err(_) => return format_verbatim_node(node.syntax()).fmt(f),
        };

        let marker = match prefix.marker() {
            Ok(m) => m,
            Err(_) => return format_verbatim_node(node.syntax()).fmt(f),
        };

        // Only normalize `*` and `+`; `-` is already correct.
        if marker.text_trimmed() != "*" && marker.text_trimmed() != "+" {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // Guard: if the first content block is a `-`-based thematic break, normalizing
        // the marker to `-` would produce a line like `- - - -` which is re-parsed as a
        // thematic break (not a list item). Keep verbatim in that case.
        if first_block_is_dash_thematic_break(&content) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        // Determine the target marker. Consecutive bullet lists alternate between
        // `-` and `*` to preserve list separation (Prettier's behavior).
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

/// Determine the target marker for a bullet based on how many preceding
/// `MdBulletListItem` siblings exist. First list → `-`, second → `*`,
/// third → `-`, etc. (Prettier's alternation pattern.)
fn target_marker_for_bullet(node: &MdBullet) -> &'static str {
    use biome_markdown_syntax::MarkdownSyntaxKind;
    let Some(bullet_list) = node.syntax().parent() else {
        return "-";
    };
    let Some(bullet_list_item) = bullet_list.parent() else {
        return "-";
    };
    if bullet_list_item.kind() != MarkdownSyntaxKind::MD_BULLET_LIST_ITEM {
        return "-";
    }
    let mut count = 0u32;
    let mut sibling = bullet_list_item.prev_sibling();
    while let Some(s) = sibling {
        match s.kind() {
            MarkdownSyntaxKind::MD_NEWLINE => {}
            MarkdownSyntaxKind::MD_BULLET_LIST_ITEM => count += 1,
            _ => break,
        }
        sibling = s.prev_sibling();
    }
    if count.is_multiple_of(2) { "-" } else { "*" }
}

/// Returns true if the first block in `content` is a thematic break using `-` characters.
fn first_block_is_dash_thematic_break(content: &biome_markdown_syntax::MdBlockList) -> bool {
    let Some(first_block) = content.iter().next() else {
        return false;
    };
    let leaf = match first_block {
        AnyMdBlock::AnyMdLeafBlock(leaf) => leaf,
        _ => return false,
    };
    let thematic_break = match leaf {
        AnyMdLeafBlock::MdThematicBreakBlock(tb) => tb,
        _ => return false,
    };
    // Check if the thematic break uses `-` characters.
    for part in thematic_break.parts() {
        if let Some(char_node) = part.as_md_thematic_break_char() {
            return char_node
                .value()
                .map(|t| t.text_trimmed() == "-")
                .unwrap_or(false);
        }
    }
    false
}
